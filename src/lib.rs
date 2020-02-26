#![no_std]
// #![deny(warnings, missing_docs)]
use embedded_hal::blocking::i2c::{Write as i2cWrite, WriteRead};
use embedded_hal::{blocking::spi::Transfer, blocking::spi::Write, digital::v2::OutputPin};

pub mod accel;
pub mod gyro;
pub mod mag;
pub mod register;

use accel::AccelSettings;
use gyro::GyroSettings;
use mag::MagSettings;

mod interface;
use interface::*;

/// Accelerometer/Gyroscope's ID
const WHO_AM_I_AG: u8 = 0x68;
/// Magnetometer's ID
const WHO_AM_I_M: u8 = 0x3D;

/// temperature scale
const TEMP_SCALE: f32 = 16.0;
/// The output of the temperature sensor is 0 (typ.) at 25 °C. see page 14: Temperature sensor characteristics
const TEMP_BIAS: f32 = 25.0;

/// Errors in this crate
#[derive(Debug)]
pub enum Error<CommE, PinE, E> {
    /// Communication error
    Comm(CommE),
    /// Pin setting error
    Pin(PinE),
    /// I2C error
    I2C(E),
}

impl<CommE, PinE, E> From<interface::Error<CommE, PinE, E>> for Error<CommE, PinE, E> {
    fn from(err: interface::Error<CommE, PinE, E>) -> Error<CommE, PinE, E> {
        match err {
            interface::Error::Comm(x) => Error::Comm(x),
            interface::Error::Pin(x) => Error::Pin(x),
            interface::Error::I2C(x) => Error::I2C(x),
        }
    }
}

/// Axis selection
pub enum Axis {
    X,
    Y,
    Z,
}

pub struct LSM9DS1<SPI, CS, I2C> {
    peripheral: interface::Selection<SPI, CS, I2C>,
    accel: AccelSettings,
    gyro: GyroSettings,
    mag: MagSettings,
}

impl<SPI, CS, CommE, PinE, I2C, E> LSM9DS1<SPI, CS, I2C>
where
    SPI: Transfer<u8, Error = CommE> + Write<u8, Error = CommE>,
    CS: OutputPin<Error = PinE>,
    I2C: WriteRead<Error = E> + i2cWrite<Error = E>,
{
    pub fn new(spi: SPI, cs: CS) -> Result<LSM9DS1<SPI, CS, I2C>, Error<CommE, PinE, E>> {
        let this = Self {
            peripheral: Selection::SPI(SpiInterface::new(spi, cs)),
            accel: AccelSettings::new(),
            gyro: GyroSettings::new(),
            mag: MagSettings::new(),
        };
        Ok(this)
    }

    pub fn accel_is_reacheable(&mut self) -> bool {
        match self.peripheral.read_register(register::AG::WHO_AM_I.addr()) {
            Ok(x) if x == WHO_AM_I_AG => true,
            _ => false,
        }
    }

    pub fn mag_is_reacheable(&mut self) -> bool {
        match self
            .peripheral
            .read_register(register::Mag::WHO_AM_I.addr())
        {
            Ok(x) if x == WHO_AM_I_M => true,
            _ => false,
        }
    }

    pub fn init_accel(&mut self) -> Result<(), Error<CommE, PinE, E>> {
        self.peripheral
            .write_register(register::AG::CTRL_REG5_XL.addr(), self.accel.ctrl_reg5_xl())?;
        self.peripheral
            .write_register(register::AG::CTRL_REG6_XL.addr(), self.accel.ctrl_reg6_xl())?;
        self.peripheral
            .write_register(register::AG::CTRL_REG7_XL.addr(), self.accel.ctrl_reg7_xl())?;
        Ok(())
    }

    pub fn init_gyro(&mut self) -> Result<(), Error<CommE, PinE, E>> {
        self.peripheral
            .write_register(register::AG::CTRL_REG1_G.addr(), self.gyro.crtl_reg1_g())?;
        self.peripheral
            .write_register(register::AG::CTRL_REG2_G.addr(), self.gyro.crtl_reg2_g())?;
        self.peripheral
            .write_register(register::AG::CTRL_REG3_G.addr(), self.gyro.crtl_reg3_g())?;
        self.peripheral
            .write_register(register::AG::CTRL_REG4.addr(), self.gyro.ctrl_reg4())?;
        Ok(())
    }

    pub fn init_mag(&mut self) -> Result<(), Error<CommE, PinE, E>> {
        self.peripheral
            .write_register(register::Mag::CTRL_REG1_M.addr(), self.mag.ctrl_reg1_m())?;
        self.peripheral
            .write_register(register::Mag::CTRL_REG2_M.addr(), self.mag.ctrl_reg2_m())?;
        self.peripheral
            .write_register(register::Mag::CTRL_REG3_M.addr(), self.mag.ctrl_reg3_m())?;
        self.peripheral
            .write_register(register::Mag::CTRL_REG4_M.addr(), self.mag.ctrl_reg4_m())?;
        self.peripheral
            .write_register(register::Mag::CTRL_REG5_M.addr(), self.mag.ctrl_reg5_m())?;
        Ok(())
    }

    pub fn set_accel_scale(
        &mut self,
        scale: accel::AccelScale,
    ) -> Result<(), Error<CommE, PinE, E>> {
        self.accel.scale = scale;
        self.peripheral
            .write_register(register::AG::CTRL_REG6_XL.addr(), self.accel.ctrl_reg6_xl())?;
        Ok(())
    }

    pub fn set_accel_odr(
        &mut self,
        sample_rate: accel::AccelODR,
    ) -> Result<(), Error<CommE, PinE, E>> {
        self.accel.sample_rate = sample_rate;
        self.peripheral
            .write_register(register::AG::CTRL_REG6_XL.addr(), self.accel.ctrl_reg6_xl())?;
        Ok(())
    }

    pub fn set_accel_bandwidth_selection(
        &mut self,
        bandwidth_selection: accel::AccelBandwidthSelection,
    ) -> Result<(), Error<CommE, PinE, E>> {
        self.accel.bandwidth_selection = bandwidth_selection;
        self.peripheral
            .write_register(register::AG::CTRL_REG6_XL.addr(), self.accel.ctrl_reg6_xl())?;
        Ok(())
    }

    pub fn set_accel_bandwidth(
        &mut self,
        bandwidth: accel::AccelBandwidth,
    ) -> Result<(), Error<CommE, PinE, E>> {
        self.accel.bandwidth = bandwidth;
        self.peripheral
            .write_register(register::AG::CTRL_REG6_XL.addr(), self.accel.ctrl_reg6_xl())?;
        Ok(())
    }

    pub fn enable_axis(&mut self, axis: Axis, enabled: bool) -> Result<(), Error<CommE, PinE, E>> {
        match axis {
            Axis::X => self.accel.enable_x = enabled,
            Axis::Y => self.accel.enable_y = enabled,
            Axis::Z => self.accel.enable_z = enabled,
        }
        self.peripheral
            .write_register(register::AG::CTRL_REG5_XL.addr(), self.accel.ctrl_reg5_xl())?;
        Ok(())
    }

    pub fn accel_available(&mut self) -> bool {
        match self
            .peripheral
            .read_register(register::AG::STATUS_REG_1.addr())
        {
            Ok(x) if x & 0x01 > 0 => true,
            _ => false,
        }
    }

    pub fn gyro_available(&mut self) -> bool {
        match self
            .peripheral
            .read_register(register::AG::STATUS_REG_1.addr())
        {
            Ok(x) if x & 0x02 > 0 => true,
            _ => false,
        }
    }

    pub fn temp_available(&mut self) -> bool {
        match self
            .peripheral
            .read_register(register::AG::STATUS_REG_1.addr())
        {
            Ok(x) if x & 0x04 > 0 => true,
            _ => false,
        }
    }

    pub fn mag_available(&mut self) -> bool {
        match self
            .peripheral
            .read_register(register::Mag::STATUS_REG_M.addr())
        {
            Ok(x) if x & 0x01 > 0 => true,
            _ => false,
        }
    }

    fn read_sensor(
        &mut self,
        addr: u8,
        sensitivity: f32,
    ) -> Result<(f32, f32, f32), Error<CommE, PinE, E>> {
        let mut bytes = [0u8; 7];
        self.peripheral.read_bytes(addr, &mut bytes)?;

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

    pub fn read_accel(&mut self) -> Result<(f32, f32, f32), Error<CommE, PinE, E>> {
        self.read_sensor(
            register::AG::OUT_X_L_XL.addr(),
            self.accel.scale.sensitivity(),
        )
    }

    pub fn read_temp(&mut self) -> Result<f32, Error<CommE, PinE, E>> {
        let mut bytes = [0u8; 3];
        self.peripheral
            .read_bytes(register::AG::OUT_TEMP_L.addr(), &mut bytes)?;
        let result: i16 = (bytes[2] as i16) << 8 | bytes[1] as i16;
        Ok((result as f32) / TEMP_SCALE + TEMP_BIAS)
    }

    pub fn read_gyro(&mut self) -> Result<(f32, f32, f32), Error<CommE, PinE, E>> {
        self.read_sensor(
            register::AG::OUT_X_L_G.addr(),
            self.gyro.scale.sensitivity(),
        )
    }

    pub fn read_mag(&mut self) -> Result<(f32, f32, f32), Error<CommE, PinE, E>> {
        self.read_sensor(
            register::Mag::OUT_X_L_M.addr(),
            1.0, // TODO: verify
        )
    }
}
