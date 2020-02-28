use super::Interface;
use super::Sensor;
use embedded_hal::blocking::i2c::{Write, WriteRead};

/// R/W bit should be high for I2C Read operation
const I2C_READ: u8 = 0x01;

/// Errors in this crate
#[derive(Debug)]
pub enum Error<CommE> {
    /// Communication error
    Comm(CommE),
}

/// Accelerometer/Gyro sensor address for I2C communication
pub enum AgAddress {
    _1 = 0x6A,
    _2 = 0x6B,
}

impl AgAddress {
    pub fn addr(self) -> u8 {
        self as u8
    }
}

/// Magnetometer sensor address for I2C communication
pub enum MagAddress {
    _1 = 0x1C,
    _2 = 0x1E,
}

impl MagAddress {
    pub fn addr(self) -> u8 {
        self as u8
    }
}

/// I2C driver
pub struct I2cInterface<I2C> {
    i2c: I2C,
    ag_addr: u8,
    mag_addr: u8,
}

impl<I2C, CommE> I2cInterface<I2C>
where
    I2C: WriteRead<Error = CommE> + Write<Error = CommE>,
{
    pub fn new(i2c: I2C, ag_addr: AgAddress, mag_addr: MagAddress) -> Self {
        Self {
            i2c,
            ag_addr: ag_addr.addr(),
            mag_addr: mag_addr.addr(),
        }
    }
}

impl<I2C, CommE> Interface for I2cInterface<I2C>
where
    I2C: WriteRead<Error = CommE> + Write<Error = CommE>,
{
    type Error = Error<CommE>;

    fn write_register(&mut self, sensor: Sensor, addr: u8, value: u8) -> Result<(), Self::Error> {
        let sensor_addr = match sensor {
            Sensor::Accelerometer | Sensor::Gyro => self.ag_addr,
            Sensor::Magnetometer => self.mag_addr,
        };
        core::prelude::v1::Ok(
            self.i2c
                .write(sensor_addr, &[addr, value])
                .map_err(Error::Comm)?,
        )
    }

    fn read_register(&mut self, sensor: Sensor, addr: u8) -> Result<u8, Self::Error> {
        let mut bytes = [0u8; 2];
        self.read_bytes(sensor, addr, &mut bytes)?;
        Ok(bytes[1])
    }

    fn read_bytes(
        &mut self,
        sensor: Sensor,
        addr: u8,
        bytes: &mut [u8],
    ) -> Result<(), Self::Error> {
        let sensor_addr = match sensor {
            Sensor::Accelerometer | Sensor::Gyro => self.ag_addr,
            Sensor::Magnetometer => self.mag_addr,
        };
        core::prelude::v1::Ok(
            self.i2c
                .write_read(sensor_addr, &[(addr << 7) | I2C_READ], bytes)
                .map_err(Error::Comm)?,
        )
    }
}
