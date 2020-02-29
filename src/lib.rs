//! A platform agnostic driver to interface with LSM9DS1 3D accelerometer, 3D gyroscope, 3D magnetometer sensor module.
//!
//! ### Datasheets
//! - [LSM9DS1](https://www.st.com/resource/en/datasheet/lsm9ds1.pdf)
//!
//! # Examples
//!```rust
//! #[entry]
//! fn main() -> ! {
//!     let cp = cortex_m::Peripherals::take().unwrap();
//!     let mut itm = cp.ITM;
//!     let dp = stm32::Peripherals::take().unwrap();
//!     let mut flash = dp.FLASH.constrain();
//!     let mut rcc = dp.RCC.constrain();
//!     let mut gpiob = dp.GPIOB.split(&mut rcc.ahb);
//!     // Accelerometer/Gyroscope CS pin
//!     let mut ag_cs = gpiob
//!         .pb5
//!         .into_push_pull_output(&mut gpiob.moder, &mut gpiob.otyper);
//!     ag_cs.set_high().unwrap();
//!     // Magnetometer CS pin
//!     let mut m_cs = gpiob
//!         .pb4
//!         .into_push_pull_output(&mut gpiob.moder, &mut gpiob.otyper);
//!     m_cs.set_high().unwrap();
//!
//!     let mut gpioa = dp.GPIOA.split(&mut rcc.ahb);
//!     let clocks = rcc.cfgr.freeze(&mut flash.acr);
//!
//!     // Configure pins for SPI
//!     let sck = gpioa.pa5.into_af5(&mut gpioa.moder, &mut gpioa.afrl);
//!     let miso = gpioa.pa6.into_af5(&mut gpioa.moder, &mut gpioa.afrl);
//!     let mosi = gpioa.pa7.into_af5(&mut gpioa.moder, &mut gpioa.afrl);
//!
//!     let spi = Spi::spi1(
//!         dp.SPI1,
//!         (sck, miso, mosi),
//!         MODE_0,
//!         1.mhz(),
//!         clocks,
//!         &mut rcc.apb2,
//!     );
//!     // Create SPI interface
//!     let spi_interface = SpiInterface::new(spi, ag_cs, m_cs);
//!     // Create LSM9DS1
//!     let mut lsm9ds1 = LSM9DS1::from_interface(spi_interface);
//!     lsm9ds1.init_accel().unwrap();
//!     lsm9ds1.init_gyro().unwrap();
//! 
//!     loop {
//!         let temp = lsm9ds1.read_temp().unwrap();
//!         iprintln!(&mut itm.stim[0], "temp: {}", temp);
//!
//!         let (x, y, z) = lsm9ds1.read_accel().unwrap();
//!         iprintln!(&mut itm.stim[0], "xl: {}, {}, {}", x, y, z);
//!
//!         let (x, y, z) = lsm9ds1.read_gyro().unwrap();
//!         iprintln!(&mut itm.stim[0], "gy: {}, {}, {}", x, y, z);
//! 
//!         cortex_m::asm::delay(8_000_000);
//!     }
//! }
//! ```
#![no_std]
// #![deny(warnings, missing_docs)]
pub mod accel;
pub mod gyro;
pub mod mag;
pub mod register;

use accel::AccelSettings;
use gyro::GyroSettings;
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

/// Axis selection
pub enum Axis {
    X,
    Y,
    Z,
}

/// LSM9DS1 IMU
pub struct LSM9DS1<T>
where
    T: Interface,
{
    interface: T,
    pub accel: AccelSettings,
    pub gyro: GyroSettings,
    pub mag: MagSettings,
}

impl<T> LSM9DS1<T>
where
    T: Interface,
{
    /// Construct a new LSM9DS1 driver instance with a I2C or SPI peripheral.
    ///
    /// # Arguments
    /// * `interface` - `SpiInterface` or `I2cInterface`
    pub fn from_interface(interface: T) -> LSM9DS1<T> {
        Self {
            interface,
            accel: AccelSettings::new(),
            gyro: GyroSettings::new(),
            mag: MagSettings::new(),
        }
    }

    fn reachable(&mut self, sensor: Sensor) -> Result<bool, T::Error> {
        use Sensor::*;
        let mut bytes = [0u8, 1];
        let (who_am_i, register) = match sensor {
            Accelerometer | Gyro | Temperature => (WHO_AM_I_AG, register::AG::WHO_AM_I.addr()),
            Magnetometer => (WHO_AM_I_M, register::Mag::WHO_AM_I.addr()),
        };

        self.interface.read(sensor, register, &mut bytes)?;
        Ok(bytes[0] == who_am_i)
    }

    /// Verify communication with WHO_AM_I register
    pub fn accel_is_reacheable(&mut self) -> Result<bool, T::Error> {
        self.reachable(Sensor::Accelerometer)
    }
    /// Verify communication with WHO_AM_I register
    pub fn mag_is_reacheable(&mut self) -> Result<bool, T::Error> {
        self.reachable(Sensor::Magnetometer)
    }
    /// Initialize Accelerometer with default settings.
    pub fn init_accel(&mut self) -> Result<(), T::Error> {
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
    /// Initialize Gyro with default settings.
    pub fn init_gyro(&mut self) -> Result<(), T::Error> {
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
    /// Initialize Magnetometer with default settings.
    pub fn init_mag(&mut self) -> Result<(), T::Error> {
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

    fn set_scale(&mut self, sensor: Sensor) -> Result<(), T::Error> {
        use Sensor::*;
        let (register, value) = match sensor {
            Accelerometer => (register::AG::CTRL_REG6_XL.addr(), self.accel.ctrl_reg6_xl()),
            Gyro => (register::AG::CTRL_REG1_G.addr(), self.gyro.ctrl_reg1_g()),
            _ => (register::Mag::CTRL_REG2_M.addr(), self.mag.ctrl_reg2_m()),
        };
        self.interface.write(sensor, register, value)
    }
    /// update accelerometer scale
    pub fn set_accel_scale(&mut self, scale: accel::Scale) -> Result<(), T::Error> {
        self.accel.scale = scale;
        self.set_scale(Sensor::Accelerometer)
    }
    /// update gyro scale
    pub fn set_gyro_scale(&mut self, scale: gyro::Scale) -> Result<(), T::Error> {
        self.gyro.scale = scale;
        self.set_scale(Sensor::Gyro)
    }
    /// update magnetometer scale
    pub fn set_mag_scale(&mut self, scale: mag::Scale) -> Result<(), T::Error> {
        self.mag.scale = scale;
        self.set_scale(Sensor::Magnetometer)
    }

    fn set_odr(&mut self, sensor: Sensor) -> Result<(), T::Error> {
        use Sensor::*;
        let (register, value) = match sensor {
            Accelerometer => (register::AG::CTRL_REG6_XL.addr(), self.accel.ctrl_reg6_xl()),
            Gyro => (register::AG::CTRL_REG1_G.addr(), self.gyro.ctrl_reg1_g()),
            _ => (register::Mag::CTRL_REG1_M.addr(), self.mag.ctrl_reg1_m()),
        };
        self.interface.write(sensor, register, value)
    }
    /// update accelerometer sample rate (ODR: output data rate)
    pub fn set_accel_odr(&mut self, sample_rate: accel::ODR) -> Result<(), T::Error> {
        self.accel.sample_rate = sample_rate;
        self.set_odr(Sensor::Accelerometer)
    }
    /// update gyro sample rate (ODR: output data rate)
    pub fn set_gyro_odr(&mut self, sample_rate: gyro::ODR) -> Result<(), T::Error> {
        self.gyro.sample_rate = sample_rate;
        self.set_odr(Sensor::Gyro)
    }
    /// update magnetometer sample rate (ODR: output data rate)
    pub fn set_mag_odr(&mut self, sample_rate: mag::ODR) -> Result<(), T::Error> {
        self.mag.sample_rate = sample_rate;
        self.set_odr(Sensor::Magnetometer)
    }
    /// update accelerometer bandwidth selection
    pub fn set_accel_bandwidth_selection(
        &mut self,
        bandwidth_selection: accel::BandwidthSelection,
    ) -> Result<(), T::Error> {
        self.accel.bandwidth_selection = bandwidth_selection;
        self.interface.write(
            Sensor::Accelerometer,
            register::AG::CTRL_REG6_XL.addr(),
            self.accel.ctrl_reg6_xl(),
        )?;
        Ok(())
    }
    /// update accelerometer Anti-aliasing filter bandwidth
    pub fn set_accel_bandwidth(&mut self, bandwidth: accel::Bandwidth) -> Result<(), T::Error> {
        self.accel.bandwidth = bandwidth;
        self.interface.write(
            Sensor::Accelerometer,
            register::AG::CTRL_REG6_XL.addr(),
            self.accel.ctrl_reg6_xl(),
        )?;
        Ok(())
    }

    /// update gyro bandwidth
    pub fn set_gyro_bandwidth(&mut self, bandwidth: gyro::Bandwidth) -> Result<(), T::Error> {
        self.gyro.bandwidth = bandwidth;
        self.interface.write(
            Sensor::Gyro,
            register::AG::CTRL_REG1_G.addr(),
            self.gyro.ctrl_reg1_g(),
        )?;
        Ok(())
    }

    pub fn enable_axis(&mut self, axis: Axis, enabled: bool) -> Result<(), T::Error> {
        match axis {
            Axis::X => self.accel.enable_x = enabled,
            Axis::Y => self.accel.enable_y = enabled,
            Axis::Z => self.accel.enable_z = enabled,
        }
        self.interface.write(
            Sensor::Accelerometer,
            register::AG::CTRL_REG5_XL.addr(),
            self.accel.ctrl_reg5_xl(),
        )?;
        Ok(())
    }

    fn data_available(&mut self, sensor: Sensor) -> Result<u8, T::Error> {
        use Sensor::*;
        let register = match sensor {
            Accelerometer | Gyro | Temperature => register::AG::STATUS_REG_1.addr(),
            Magnetometer => register::Mag::STATUS_REG_M.addr(),
        };
        let mut bytes = [0u8, 1];
        self.interface.read(sensor, register, &mut bytes)?;
        Ok(bytes[0])
    }
    /// See if new Accelerometer data is available
    pub fn accel_data_available(&mut self) -> Result<bool, T::Error> {
        match self.data_available(Sensor::Accelerometer)? {
            x if x & 0x01 > 0 => Ok(true),
            _ => Ok(false),
        }
    }
    /// See if new Gyro data is available
    pub fn gyro_data_available(&mut self) -> Result<bool, T::Error> {
        match self.data_available(Sensor::Gyro)? {
            x if x & 0x02 > 0 => Ok(true),
            _ => Ok(false),
        }
    }
    /// See if new Magnetometer data is available
    pub fn mag_data_available(&mut self) -> Result<bool, T::Error> {
        match self.data_available(Sensor::Magnetometer)? {
            x if x & 0x01 > 0 => Ok(true),
            _ => Ok(false),
        }
    }
    /// See if new Temperature data is available
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
    /// calculated accelerometer readings
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
    /// calculated gyro readings
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
    /// calculated magnetometer readings
    pub fn read_mag(&mut self) -> Result<(f32, f32, f32), T::Error> {
        let (x, y, z) = self.read_mag_raw()?;
        let sensitivity = self.mag.scale.sensitivity();
        Ok((
            x as f32 * sensitivity,
            y as f32 * sensitivity,
            z as f32 * sensitivity,
        ))
    }
    /// calculated temperature readings
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
}
