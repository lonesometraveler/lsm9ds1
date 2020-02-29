use super::Interface;
use super::Sensor;
use Sensor::*;

/// Errors in this crate
#[derive(Debug)]
pub enum Error {
    /// Communication error
    Invalid,
}

/// This holds registers
pub struct FakeInterface {
    ag_registers: [u8; 256],
    mag_registers: [u8; 256],
}

impl Default for FakeInterface {
    fn default() -> Self {
        FakeInterface {
            ag_registers: [0u8; 256],
            mag_registers: [0u8; 256],
        }
    }
}

impl FakeInterface

{
    /// create a fake interface
    pub fn new() -> Self {
        Default::default()
    }
}

/// Implementation of `Interface`
impl Interface for FakeInterface
where
{
    type Error = Error;

    fn write(&mut self, sensor: Sensor, addr: u8, value: u8) -> Result<(), Self::Error> {
        match sensor {
            Accelerometer | Gyro | Temperature => self.ag_registers[addr as usize] = value,
            Magnetometer => self.mag_registers[addr as usize] = value,
        }
        Ok(())
    }

    fn read(&mut self, sensor: Sensor, addr: u8, buffer: &mut [u8]) -> Result<(), Self::Error> {
        let registers = match sensor {
            Accelerometer | Gyro | Temperature => self.ag_registers,
            Magnetometer => self.mag_registers,
        };
        for i in 0..buffer.len() {
            buffer[i] = registers[(addr as usize) + i];
        }
        Ok(())
    }
}