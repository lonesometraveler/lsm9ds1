#![no_std]
// #![deny(warnings, missing_docs)]
use embedded_hal::{blocking::spi::Transfer, blocking::spi::Write, digital::v2::OutputPin};

pub mod accel;
mod mag;

use accel::AccelSettings;

const SPI_READ: u8 = 0x80;
const WHO_AM_I_AG: u8 = 0x68;
const WHO_AM_I_M: u8 = 0x3D;

const TEMP_OFFSET: u16 = 25;

pub enum Axis {
    X,
    Y,
    Z,
}

pub struct LSM9DS1<SPI, CS> {
    spi: SPI,
    cs: CS,
    accel: AccelSettings,
}

impl<SPI, CS, E> LSM9DS1<SPI, CS>
where
    SPI: Transfer<u8, Error = E> + Write<u8, Error = E>,
    CS: OutputPin,
    // E: core::convert::From<<CS as embedded_hal::digital::v2::OutputPin>::Error>
{
    pub fn new(spi: SPI, cs: CS) -> Result<LSM9DS1<SPI, CS>, E> {
        let mut this = Self {
            spi,
            cs,
            accel: AccelSettings::new(),
        };
        // this.cs.set_high()?;
        this.cs.set_high().ok();
        Ok(this)
    }

    pub fn accel_is_reacheable(&mut self) -> bool {
        match self.read_register(accel::Register::WHO_AM_I.addr()) {
            Ok(x) if x == WHO_AM_I_AG => true,
            _ => false,
        }
    }

    pub fn mag_is_reacheable(&mut self) -> bool {
        match self.read_register(mag::Register::WHO_AM_I.addr()) {
            Ok(x) if x == WHO_AM_I_M => true,
            _ => false,
        }
    }

    pub fn init_accel(&mut self) {
        self.write_register(
            accel::Register::CTRL_REG5_XL.addr(),
            self.accel.ctrl_reg5_xl(),
        );
        self.write_register(
            accel::Register::CTRL_REG6_XL.addr(),
            self.accel.ctrl_reg6_xl(),
        );
        self.write_register(
            accel::Register::CTRL_REG7_XL.addr(),
            self.accel.ctrl_reg7_xl(),
        );
    }

    pub fn set_accel_scale(&mut self, scale: accel::AccelScale) {
        self.accel.scale = scale;
        self.write_register(
            accel::Register::CTRL_REG6_XL.addr(),
            self.accel.ctrl_reg6_xl(),
        );
    }

    pub fn set_accel_odr(&mut self, sample_rate: accel::AccelODR) {
        self.accel.sample_rate = sample_rate;
        self.write_register(
            accel::Register::CTRL_REG6_XL.addr(),
            self.accel.ctrl_reg6_xl(),
        );
    }

    pub fn set_accel_bandwidth_selection(
        &mut self,
        bandwidth_selection: accel::AccelBandwidthSelection,
    ) {
        self.accel.bandwidth_selection = bandwidth_selection;
        self.write_register(
            accel::Register::CTRL_REG6_XL.addr(),
            self.accel.ctrl_reg6_xl(),
        );
    }

    pub fn set_accel_bandwidth(&mut self, bandwidth: accel::AccelBandwidth) {
        self.accel.bandwidth = bandwidth;
        self.write_register(
            accel::Register::CTRL_REG6_XL.addr(),
            self.accel.ctrl_reg6_xl(),
        );
    }

    pub fn enable_axis(&mut self, axis: Axis, enabled: bool) {
        match axis {
            Axis::X => self.accel.enable_x = enabled,
            Axis::Y => self.accel.enable_y = enabled,
            Axis::Z => self.accel.enable_z = enabled,
        }
        self.write_register(
            accel::Register::CTRL_REG5_XL.addr(),
            self.accel.ctrl_reg5_xl(),
        );
    }

    pub fn accel_available(&mut self) -> bool {
        match self.read_register(accel::Register::STATUS_REG_1.addr()) {
            Ok(x) if x & 0x01 > 0 => true,
            _ => false,
        }
    }

    pub fn gyro_available(&mut self) -> bool {
        match self.read_register(accel::Register::STATUS_REG_1.addr()) {
            Ok(x) if x & 0x02 > 0 => true,
            _ => false,
        }
    }

    pub fn temp_available(&mut self) -> bool {
        match self.read_register(accel::Register::STATUS_REG_1.addr()) {
            Ok(x) if x & 0x04 > 0 => true,
            _ => false,
        }
    }

    pub fn read_accel(&mut self) -> (u16, u16, u16) {
        let mut bytes = [0u8; 7];
        bytes[0] = SPI_READ | accel::Register::OUT_X_L_XL.addr();
        let result = self.read_bytes(&mut bytes);
        match result {
            Ok(_) => {
                let x: u16 = (bytes[2] as u16) << 8 | bytes[1] as u16;
                let y: u16 = (bytes[4] as u16) << 8 | bytes[3] as u16;
                let z: u16 = (bytes[6] as u16) << 8 | bytes[5] as u16;
                // if (_autoCalc) {
                //     ax -= aBiasRaw[X_AXIS];
                //     ay -= aBiasRaw[Y_AXIS];
                //     az -= aBiasRaw[Z_AXIS];
                // }
                (x, y, z)
            }
            _ => (0, 0, 0),
        }
    }

    pub fn read_accel_for(&mut self, axis: Axis) -> u16 {
        let mut bytes = [0u8; 3];
        let addr = match axis {
            Axis::X => accel::Register::OUT_X_L_XL.addr(),
            Axis::Y => accel::Register::OUT_Y_L_XL.addr(),
            Axis::Z => accel::Register::OUT_Z_L_XL.addr(),
        };
        bytes[0] = SPI_READ | addr;
        let result = self.read_bytes(&mut bytes);
        match result {
            Ok(_) => {
                let result: u16 = (bytes[2] as u16) << 8 | bytes[1] as u16;
                // if (_autoCalc) {
                //     ax -= aBiasRaw[X_AXIS];
                //     ay -= aBiasRaw[Y_AXIS];
                //     az -= aBiasRaw[Z_AXIS];
                // }
                result
            }
            _ => 0,
        }
    }

    pub fn read_temp(&mut self) -> u16 {
        let mut bytes = [0u8; 3];
        bytes[0] = SPI_READ | accel::Register::OUT_TEMP_L.addr();
        match self.read_bytes(&mut bytes) {
            Ok(_) => {
                let result: u16 = (bytes[2] as u16) << 8 | bytes[1] as u16;
                result + TEMP_OFFSET
            }
            _ => 0,
        }
    }

    pub fn read_gyro(&mut self) -> (u16, u16, u16) {
        let mut bytes = [0u8; 7];
        bytes[0] = SPI_READ | accel::Register::OUT_X_L_G.addr();
        let result = self.read_bytes(&mut bytes);
        match result {
            Ok(_) => {
                let x: u16 = (bytes[2] as u16) << 8 | bytes[1] as u16;
                let y: u16 = (bytes[4] as u16) << 8 | bytes[3] as u16;
                let z: u16 = (bytes[6] as u16) << 8 | bytes[5] as u16;
                // if (_autoCalc) {
                //     ax -= aBiasRaw[X_AXIS];
                //     ay -= aBiasRaw[Y_AXIS];
                //     az -= aBiasRaw[Z_AXIS];
                // }
                (x, y, z)
            }
            _ => (0, 0, 0),
        }
    }

    pub fn read_gyro_for(&mut self, axis: Axis) -> u16 {
        let mut bytes = [0u8; 3];
        let addr = match axis {
            Axis::X => accel::Register::OUT_X_L_G.addr(),
            Axis::Y => accel::Register::OUT_Y_L_G.addr(),
            Axis::Z => accel::Register::OUT_Z_L_G.addr(),
        };
        bytes[0] = SPI_READ | addr;
        let result = self.read_bytes(&mut bytes);
        match result {
            Ok(_) => {
                let result: u16 = (bytes[2] as u16) << 8 | bytes[1] as u16;
                // if (_autoCalc) {
                //     ax -= aBiasRaw[X_AXIS];
                //     ay -= aBiasRaw[Y_AXIS];
                //     az -= aBiasRaw[Z_AXIS];
                // }
                result
            }
            _ => 0,
        }
    }

    // pub fn mag_available(&mut self) -> bool {
    //     match self.read_register(mag::Register::STATUS_REG_M.addr()) {
    //         Ok(x) if x & 0x01 > 0 => true,
    //         _ => false,
    //     }
    // }

    fn write_register(&mut self, addr: u8, value: u8) {
        let bytes = [addr, value];
        self.cs.set_low().ok();
        self.spi.write(&bytes).ok();
        self.cs.set_high().ok();
    }

    pub fn read_register(&mut self, addr: u8) -> Result<u8, E> {
        let mut buffer = [0u8; 2];
        buffer[0] = SPI_READ | (addr & 0x3F);
        self.cs.set_low().ok();
        self.spi.transfer(&mut buffer)?;
        self.cs.set_high().ok();

        Ok(buffer[1])
    }

    pub fn read_bytes(&mut self, bytes: &mut [u8]) -> Result<(), E> {
        // let mut bytes = [0u8; 7];
        // bytes[0] = SPI_READ | (sub_address & 0x3F);
        self.cs.set_low().ok();
        self.spi.transfer(bytes)?;
        self.cs.set_high().ok();

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
