#![no_std]
// #![deny(warnings, missing_docs)]
use embedded_hal::{blocking::spi::Transfer, blocking::spi::Write, digital::v2::OutputPin};

pub mod accel;
pub mod gyro;
// mod mag;
pub mod register;

use accel::AccelSettings;
use gyro::GyroSettings;

/// R/W bit should be high for SPI Read operation
const SPI_READ: u8 = 0x80;
/// Accelerometer/Gyroscope's ID
const WHO_AM_I_AG: u8 = 0x68;
/// Magnetonomer's ID
const WHO_AM_I_M: u8 = 0x3D;

/// temperature scale
const TEMP_SCALE: f32 = 16.0;
/// The output of the temperature sensor is 0 (typ.) at 25 °C. see page 14: Temperature sensor characteristics
const TEMP_BIAS: f32 = 25.0;

/// Errors in this crate
#[derive(Debug)]
pub enum Error<CommE, PinE> {
    /// Communication error
    Comm(CommE),
    /// Pin setting error
    Pin(PinE),
}

/// Axis selection
pub enum Axis {
    X,
    Y,
    Z,
}

pub struct LSM9DS1<SPI, CS> {
    spi: SPI,
    cs: CS,
    accel: AccelSettings,
    gyro: GyroSettings,
}

impl<SPI, CS, CommE, PinE> LSM9DS1<SPI, CS>
where
    SPI: Transfer<u8, Error = CommE> + Write<u8, Error = CommE>,
    CS: OutputPin<Error = PinE>,
{
    pub fn new(spi: SPI, cs: CS) -> Result<LSM9DS1<SPI, CS>, Error<CommE, PinE>>
    where
        CS: OutputPin<Error = PinE>,
    {
        let mut this = Self {
            spi,
            cs,
            accel: AccelSettings::new(),
            gyro: GyroSettings::new(),
        };
        this.cs.set_high().map_err(Error::Pin)?;
        Ok(this)
    }

    pub fn accel_is_reacheable(&mut self) -> bool {
        match self.read_register(register::AG::WHO_AM_I.addr()) {
            Ok(x) if x == WHO_AM_I_AG => true,
            _ => false,
        }
    }

    pub fn mag_is_reacheable(&mut self) -> bool {
        match self.read_register(register::Mag::WHO_AM_I.addr()) {
            Ok(x) if x == WHO_AM_I_M => true,
            _ => false,
        }
    }

    pub fn init_accel(&mut self) -> Result<(), Error<CommE, PinE>> {
        self.write_register(register::AG::CTRL_REG5_XL.addr(), self.accel.ctrl_reg5_xl())?;
        self.write_register(register::AG::CTRL_REG6_XL.addr(), self.accel.ctrl_reg6_xl())?;
        self.write_register(register::AG::CTRL_REG7_XL.addr(), self.accel.ctrl_reg7_xl())?;
        Ok(())
    }

    pub fn init_gyro(&mut self) -> Result<(), Error<CommE, PinE>> {
        self.write_register(register::AG::CTRL_REG1_G.addr(), self.gyro.crtl_reg1_g())?;
        self.write_register(register::AG::CTRL_REG2_G.addr(), self.gyro.crtl_reg2_g())?;
        self.write_register(register::AG::CTRL_REG3_G.addr(), self.gyro.crtl_reg3_g())?;
        self.write_register(register::AG::CTRL_REG4.addr(), self.gyro.ctrl_reg4())?;
        Ok(())
    }

    pub fn set_accel_scale(&mut self, scale: accel::AccelScale) -> Result<(), Error<CommE, PinE>> {
        self.accel.scale = scale;
        self.write_register(register::AG::CTRL_REG6_XL.addr(), self.accel.ctrl_reg6_xl())?;
        Ok(())
    }

    pub fn set_accel_odr(
        &mut self,
        sample_rate: accel::AccelODR,
    ) -> Result<(), Error<CommE, PinE>> {
        self.accel.sample_rate = sample_rate;
        self.write_register(register::AG::CTRL_REG6_XL.addr(), self.accel.ctrl_reg6_xl())?;
        Ok(())
    }

    pub fn set_accel_bandwidth_selection(
        &mut self,
        bandwidth_selection: accel::AccelBandwidthSelection,
    ) -> Result<(), Error<CommE, PinE>> {
        self.accel.bandwidth_selection = bandwidth_selection;
        self.write_register(register::AG::CTRL_REG6_XL.addr(), self.accel.ctrl_reg6_xl())?;
        Ok(())
    }

    pub fn set_accel_bandwidth(
        &mut self,
        bandwidth: accel::AccelBandwidth,
    ) -> Result<(), Error<CommE, PinE>> {
        self.accel.bandwidth = bandwidth;
        self.write_register(register::AG::CTRL_REG6_XL.addr(), self.accel.ctrl_reg6_xl())?;
        Ok(())
    }

    pub fn enable_axis(&mut self, axis: Axis, enabled: bool) -> Result<(), Error<CommE, PinE>> {
        match axis {
            Axis::X => self.accel.enable_x = enabled,
            Axis::Y => self.accel.enable_y = enabled,
            Axis::Z => self.accel.enable_z = enabled,
        }
        self.write_register(register::AG::CTRL_REG5_XL.addr(), self.accel.ctrl_reg5_xl())?;
        Ok(())
    }

    pub fn accel_available(&mut self) -> bool {
        match self.read_register(register::AG::STATUS_REG_1.addr()) {
            Ok(x) if x & 0x01 > 0 => true,
            _ => false,
        }
    }

    pub fn gyro_available(&mut self) -> bool {
        match self.read_register(register::AG::STATUS_REG_1.addr()) {
            Ok(x) if x & 0x02 > 0 => true,
            _ => false,
        }
    }

    pub fn temp_available(&mut self) -> bool {
        match self.read_register(register::AG::STATUS_REG_1.addr()) {
            Ok(x) if x & 0x04 > 0 => true,
            _ => false,
        }
    }

    fn read_sensor(
        &mut self,
        addr: u8,
        sensitivity: f32,
    ) -> Result<(f32, f32, f32), Error<CommE, PinE>> {
        let mut bytes = [0u8; 7];
        bytes[0] = SPI_READ | addr;
        self.read_bytes(&mut bytes)?;

        let x: i16 = (bytes[2] as i16) << 8 | bytes[1] as i16;
        let y: i16 = (bytes[4] as i16) << 8 | bytes[3] as i16;
        let z: i16 = (bytes[6] as i16) << 8 | bytes[5] as i16;
        // if (_autoCalc) {
        //     ax -= aBiasRaw[X_AXIS];
        //     ay -= aBiasRaw[Y_AXIS];
        //     az -= aBiasRaw[Z_AXIS];
        // }

        Ok((
            x as f32 * sensitivity,
            y as f32 * sensitivity,
            z as f32 * sensitivity,
        ))
    }

    pub fn read_accel(&mut self) -> Result<(f32, f32, f32), Error<CommE, PinE>> {
        self.read_sensor(
            register::AG::OUT_X_L_XL.addr(),
            self.accel.scale.sensitivity(),
        )
    }

    pub fn read_temp(&mut self) -> Result<f32, Error<CommE, PinE>> {
        let mut bytes = [0u8; 3];
        bytes[0] = SPI_READ | register::AG::OUT_TEMP_L.addr();
        self.read_bytes(&mut bytes)?;
        let result: i16 = (bytes[2] as i16) << 8 | bytes[1] as i16;
        Ok((result as f32) / TEMP_SCALE + TEMP_BIAS)
    }

    pub fn read_gyro(&mut self) -> Result<(f32, f32, f32), Error<CommE, PinE>> {
        self.read_sensor(
            register::AG::OUT_X_L_G.addr(),
            self.gyro.scale.sensitivity(),
        )
    }

    pub fn mag_available(&mut self) -> bool {
        match self.read_register(register::Mag::STATUS_REG_M.addr()) {
            Ok(x) if x & 0x01 > 0 => true,
            _ => false,
        }
    }

    fn write_register(&mut self, addr: u8, value: u8) -> Result<(), Error<CommE, PinE>> {
        let bytes = [addr, value];
        self.cs.set_low().map_err(Error::Pin)?;
        self.spi.write(&bytes).map_err(Error::Comm)?;
        self.cs.set_high().map_err(Error::Pin)?;
        Ok(())
    }

    fn read_register(&mut self, addr: u8) -> Result<u8, Error<CommE, PinE>> {
        let mut buffer = [0u8; 2];
        buffer[0] = SPI_READ | (addr & 0x3F);
        self.cs.set_low().map_err(Error::Pin)?;
        self.spi.transfer(&mut buffer).map_err(Error::Comm)?;
        self.cs.set_high().map_err(Error::Pin)?;
        Ok(buffer[1])
    }

    fn read_bytes(&mut self, bytes: &mut [u8]) -> Result<(), Error<CommE, PinE>> {
        // let mut bytes = [0u8; 7];
        // bytes[0] = SPI_READ | (sub_address & 0x3F);
        self.cs.set_low().map_err(Error::Pin)?;
        self.spi.transfer(bytes).map_err(Error::Comm)?;
        self.cs.set_high().map_err(Error::Pin)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
