//! A platform agnostic driver to interface with LSM9DS1 3D accelerometer, 3D gyroscope, 3D magnetometer sensor module.
//!
//! ### Datasheets
//! - [LSM9DS1](https://www.st.com/resource/en/datasheet/lsm9ds1.pdf)
//!
#![no_std]
// #![deny(warnings, missing_docs)]
pub mod accel;
pub mod fifo;
pub mod gyro;
pub mod interrupts;
pub mod mag;
pub mod register;

use accel::AccelSettings;
use fifo::{Decimate, FIFOBitmasks, FIFOConfig, FIFOStatus};
use gyro::GyroSettings;
use interrupts::pins_config::{IntConfigAG1, IntConfigAG2, PinConfig};
use mag::MagSettings;

pub mod interface;
use interface::{Interface, Sensor};

/// Accelerometer/Gyroscope's ID
const WHO_AM_I_AG: u8 = 0x68;
/// Magnetometer's ID
const WHO_AM_I_M: u8 = 0x3D;
/// temperature scale
const TEMP_SCALE: f32 = 16.0;
/// The output of the temperature sensor is 0 (typ.) at 25 °C. see page 14: Temperature sensor characteristics
const TEMP_BIAS: f32 = 25.0;

/// LSM9DS1 init struct.
/// Use this struct to configure sensors and init LSM9DS1 with an interface of your choice.
#[derive(Default)]
pub struct LSM9DS1Init {
    pub accel: AccelSettings,
    pub gyro: GyroSettings,
    pub mag: MagSettings,
}

impl LSM9DS1Init {
    /// Constructs a new LSM9DS1 driver instance with a I2C or SPI peripheral.
    ///
    /// # Arguments
    /// * `interface` - `SpiInterface` or `I2cInterface`
    pub fn with_interface<T>(self, interface: T) -> LSM9DS1<T>
    where
        T: Interface,
    {
        LSM9DS1 {
            interface,
            accel: self.accel,
            gyro: self.gyro,
            mag: self.mag,
        }
    }
}

/// LSM9DS1 IMU
pub struct LSM9DS1<T>
where
    T: Interface,
{
    interface: T,
    accel: AccelSettings,
    gyro: GyroSettings,
    mag: MagSettings,
}

impl<T> LSM9DS1<T>
where
    T: Interface,
{
    fn reachable(&mut self, sensor: Sensor) -> Result<bool, T::Error> {
        use Sensor::*;
        let mut bytes = [0u8; 1];
        let (who_am_i, register) = match sensor {
            Accelerometer | Gyro | Temperature => (WHO_AM_I_AG, register::AG::WHO_AM_I.addr()),
            Magnetometer => (WHO_AM_I_M, register::Mag::WHO_AM_I.addr()),
        };

        self.interface.read(sensor, register, &mut bytes)?;
        Ok(bytes[0] == who_am_i)
    }

    /// Verifies communication with WHO_AM_I register
    pub fn accel_is_reacheable(&mut self) -> Result<bool, T::Error> {
        self.reachable(Sensor::Accelerometer)
    }
    /// Verifies communication with WHO_AM_I register
    pub fn mag_is_reacheable(&mut self) -> Result<bool, T::Error> {
        self.reachable(Sensor::Magnetometer)
    }
    /// Initializes Accelerometer with sensor settings.
    pub fn begin_accel(&mut self) -> Result<(), T::Error> {
        self.interface.write(
            Sensor::Accelerometer,
            register::AG::CTRL_REG5_XL.addr(),
            self.accel.ctrl_reg5_xl(),
        )?;
        self.interface.write(
            Sensor::Accelerometer,
            register::AG::CTRL_REG6_XL.addr(),
            self.accel.ctrl_reg6_xl(),
        )?;
        self.interface.write(
            Sensor::Accelerometer,
            register::AG::CTRL_REG7_XL.addr(),
            self.accel.ctrl_reg7_xl(),
        )?;
        Ok(())
    }
    /// Initializes Gyro with sensor settings.
    pub fn begin_gyro(&mut self) -> Result<(), T::Error> {
        self.interface.write(
            Sensor::Gyro,
            register::AG::CTRL_REG1_G.addr(),
            self.gyro.ctrl_reg1_g(),
        )?;
        self.interface.write(
            Sensor::Gyro,
            register::AG::CTRL_REG2_G.addr(),
            self.gyro.ctrl_reg2_g(),
        )?;
        self.interface.write(
            Sensor::Gyro,
            register::AG::CTRL_REG3_G.addr(),
            self.gyro.ctrl_reg3_g(),
        )?;
        self.interface.write(
            Sensor::Gyro,
            register::AG::CTRL_REG4.addr(),
            self.gyro.ctrl_reg4(),
        )?;
        Ok(())
    }
    /// Initializes Magnetometer with sensor settings.
    pub fn begin_mag(&mut self) -> Result<(), T::Error> {
        self.interface.write(
            Sensor::Magnetometer,
            register::Mag::CTRL_REG1_M.addr(),
            self.mag.ctrl_reg1_m(),
        )?;
        self.interface.write(
            Sensor::Magnetometer,
            register::Mag::CTRL_REG2_M.addr(),
            self.mag.ctrl_reg2_m(),
        )?;
        self.interface.write(
            Sensor::Magnetometer,
            register::Mag::CTRL_REG3_M.addr(),
            self.mag.ctrl_reg3_m(),
        )?;
        self.interface.write(
            Sensor::Magnetometer,
            register::Mag::CTRL_REG4_M.addr(),
            self.mag.ctrl_reg4_m(),
        )?;
        self.interface.write(
            Sensor::Magnetometer,
            register::Mag::CTRL_REG5_M.addr(),
            self.mag.ctrl_reg5_m(),
        )?;
        Ok(())
    }

    fn data_available(&mut self, sensor: Sensor) -> Result<u8, T::Error> {
        use Sensor::*;
        let register = match sensor {
            Accelerometer | Gyro | Temperature => register::AG::STATUS_REG_1.addr(),
            Magnetometer => register::Mag::STATUS_REG_M.addr(),
        };
        let mut bytes = [0u8; 1];
        self.interface.read(sensor, register, &mut bytes)?;
        Ok(bytes[0])
    }
    /// Sees if new Accelerometer data is available
    pub fn accel_data_available(&mut self) -> Result<bool, T::Error> {
        match self.data_available(Sensor::Accelerometer)? {
            x if x & 0x01 > 0 => Ok(true),
            _ => Ok(false),
        }
    }
    /// Sees if new Gyro data is available
    pub fn gyro_data_available(&mut self) -> Result<bool, T::Error> {
        match self.data_available(Sensor::Gyro)? {
            x if x & 0x02 > 0 => Ok(true),
            _ => Ok(false),
        }
    }
    /// Sees if new Magnetometer data is available
    pub fn mag_data_available(&mut self) -> Result<bool, T::Error> {
        match self.data_available(Sensor::Magnetometer)? {
            x if x & 0x01 > 0 => Ok(true),
            _ => Ok(false),
        }
    }
    /// Sees if new Temperature data is available
    pub fn temp_data_available(&mut self) -> Result<bool, T::Error> {
        match self.data_available(Sensor::Temperature)? {
            x if x & 0x04 > 0 => Ok(true),
            _ => Ok(false),
        }
    }
    /// raw sensor reading for x, y, z axis
    fn read_sensor_raw(&mut self, sensor: Sensor, addr: u8) -> Result<(i16, i16, i16), T::Error> {
        let mut bytes = [0u8; 6];
        self.interface.read(sensor, addr, &mut bytes)?;
        let x: i16 = (bytes[1] as i16) << 8 | bytes[0] as i16;
        let y: i16 = (bytes[3] as i16) << 8 | bytes[2] as i16;
        let z: i16 = (bytes[5] as i16) << 8 | bytes[4] as i16;
        Ok((x, y, z))
    }
    /// raw accelerometer readings
    pub fn read_accel_raw(&mut self) -> Result<(i16, i16, i16), T::Error> {
        self.read_sensor_raw(Sensor::Accelerometer, register::AG::OUT_X_L_XL.addr())
    }
    /// calculated accelerometer readings (x, y, z)
    pub fn read_accel(&mut self) -> Result<(f32, f32, f32), T::Error> {
        let (x, y, z) = self.read_accel_raw()?;
        let sensitivity = self.accel.scale.sensitivity();
        Ok((
            x as f32 * sensitivity,
            y as f32 * sensitivity,
            z as f32 * sensitivity,
        ))
    }
    /// raw gyro readings
    pub fn read_gyro_raw(&mut self) -> Result<(i16, i16, i16), T::Error> {
        self.read_sensor_raw(Sensor::Gyro, register::AG::OUT_X_L_G.addr())
    }
    /// calculated gyro readings (x, y, z)
    pub fn read_gyro(&mut self) -> Result<(f32, f32, f32), T::Error> {
        let (x, y, z) = self.read_gyro_raw()?;
        let sensitivity = self.gyro.scale.sensitivity();
        Ok((
            x as f32 * sensitivity,
            y as f32 * sensitivity,
            z as f32 * sensitivity,
        ))
    }
    /// raw magnetometer readings
    pub fn read_mag_raw(&mut self) -> Result<(i16, i16, i16), T::Error> {
        self.read_sensor_raw(Sensor::Magnetometer, register::Mag::OUT_X_L_M.addr())
    }
    /// calculated magnetometer readings (x, y, z)
    pub fn read_mag(&mut self) -> Result<(f32, f32, f32), T::Error> {
        let (x, y, z) = self.read_mag_raw()?;
        let sensitivity = self.mag.scale.sensitivity();
        Ok((
            x as f32 * sensitivity,
            y as f32 * sensitivity,
            z as f32 * sensitivity,
        ))
    }
    /// Reads calculated temperature in Celsius
    pub fn read_temp(&mut self) -> Result<f32, T::Error> {
        let mut bytes = [0u8; 2];
        self.interface.read(
            Sensor::Accelerometer,
            register::AG::OUT_TEMP_L.addr(),
            &mut bytes,
        )?;
        let result: i16 = (bytes[1] as i16) << 8 | bytes[0] as i16;
        Ok((result as f32) / TEMP_SCALE + TEMP_BIAS)
    }

    /// Enable and configure FIFO
    pub fn configure_fifo(&mut self, config: FIFOConfig) -> Result<(), T::Error> {
        // write values to the FIFO_CTRL register
        self.interface.write(
            Sensor::Accelerometer,
            register::AG::FIFO_CTRL.addr(),
            config.f_fifo_ctrl(),
        )?;

        // write values to specific bits of the CTRL_REG9 register
        let ctrl_reg9: u8 =
            self.read_register(Sensor::Accelerometer, register::AG::CTRL_REG9.addr())?;
        let data: u8 = config.f_ctrl_reg9();
        let mut payload: u8 = ctrl_reg9 & !FIFOBitmasks::CTRL_REG9_FIFO;
        payload |= data;
        self.interface.write(
            Sensor::Accelerometer,
            register::AG::CTRL_REG9.addr(),
            payload,
        )?;

        Ok(())
    }

    /// Get flags and FIFO level from the FIFO_STATUS register
    pub fn get_fifo_status(&mut self) -> Result<FIFOStatus, T::Error> {
        let fifo_src = self.read_register(Sensor::Accelerometer, register::AG::FIFO_SRC.addr())?;
        let fifo_level_value = fifo_src & FIFOBitmasks::FSS;
        let status = FIFOStatus {
            /// Is FIFO filling equal or higher than the threshold?
            fifo_thresh_reached: fifo_src & FIFOBitmasks::FTH != 0,
            /// Is FIFO full and at least one sample has been overwritten?
            fifo_overrun: fifo_src & FIFOBitmasks::OVRN != 0,
            /// Is FIFO empty (no unread samples)?
            fifo_empty: fifo_level_value == 0,
            /// Read FIFO stored data level
            fifo_level: fifo_level_value,
        };
        Ok(status)
    }

    /// Sets decimation of acceleration data on OUT REG and FIFO
    pub fn set_decimation(&mut self, decimation: Decimate) -> Result<(), T::Error> {
        let data: u8 =
            self.read_register(Sensor::Accelerometer, register::AG::CTRL_REG5_XL.addr())?; // read current content of the register
        let mut payload: u8 = data & !FIFOBitmasks::DEC; // use bitmask to affect only bits [7:6]
        payload |= decimation.value(); // set the selected decimation value
        self.interface.write(
            Sensor::Accelerometer,
            register::AG::CTRL_REG5_XL.addr(),
            payload,
        )?;
        Ok(())
    }

    /// Enable interrupts for accelerometer/gyroscope and configure the INT1_A/G interrupt pin
    pub fn configure_interrupts_ag1(&mut self, config: IntConfigAG1) -> Result<(), T::Error> {
        self.interface.write(
            Sensor::Accelerometer,
            register::AG::INT1_CTRL.addr(),
            config.int1_ctrl(),
        )?;
        Ok(())
    }

    /// Enable interrupts for accelerometer/gyroscope and configure the INT1_A/G interrupt pin
    pub fn configure_interrupts_ag2(&mut self, config: IntConfigAG2) -> Result<(), T::Error> {
        let reg_data = self.read_register(Sensor::Accelerometer, register::AG::INT2_CTRL.addr())?;

        let mut data: u8 = reg_data & !0b1100_0000;

        data |= config.int2_ctrl();

        self.interface
            .write(Sensor::Accelerometer, register::AG::INT2_CTRL.addr(), data)?;
        Ok(())
    }

    /// Interrupt pins electrical configuration
    pub fn configure_interrupts_pins(&mut self, config: PinConfig) -> Result<(), T::Error> {
        let reg_data = self.read_register(Sensor::Accelerometer, register::AG::CTRL_REG8.addr())?;

        let mut data: u8 = reg_data & !0b0011_0000;

        data |= config.ctrl_reg8();

        self.interface
            .write(Sensor::Accelerometer, register::AG::CTRL_REG8.addr(), data)?;
        Ok(())
    }

    /// Get the current A/G1 pin configuration
    pub fn get_ag1_config(&mut self) -> Result<IntConfigAG1, T::Error> {
        Ok(IntConfigAG1::from(self.read_register(
            Sensor::Accelerometer,
            register::AG::INT1_CTRL.addr(),
        )?))
    }

    /// Get the current A/G2 pin configuration
    pub fn get_ag2_config(&mut self) -> Result<IntConfigAG2, T::Error> {
        Ok(IntConfigAG2::from(self.read_register(
            Sensor::Accelerometer,
            register::AG::INT2_CTRL.addr(),
        )?))
    }

    /// Get the current common pins configuration
    pub fn get_pins_config(&mut self) -> Result<PinConfig, T::Error> {
        Ok(PinConfig::from(self.read_register(
            Sensor::Accelerometer,
            register::AG::CTRL_REG8.addr(),
        )?))
    }

    /// Read a byte from the given register.
    fn read_register(&mut self, sensor: Sensor, address: u8) -> Result<u8, T::Error> {
        let mut reg_data = [0u8];
        self.interface.read(sensor, address, &mut reg_data)?;
        Ok(reg_data[0])
    }
}
