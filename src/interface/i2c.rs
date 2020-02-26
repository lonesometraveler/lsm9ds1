use super::CommunicationInterface;
use embedded_hal::blocking::i2c::{Write, WriteRead};

/// Default address
pub const DEFAULT_SLAVE_ADDR: u8 = 0x60;

/// I2C driver
pub struct I2cInterface<I2C> {
    i2c: I2C,
    addr: u8,
}

impl<I2C, E> I2cInterface<I2C>
where
    I2C: WriteRead<Error = E> + Write<Error = E>,
{
    pub fn new(i2c: I2C, addr: u8) -> Self {
        Self { i2c, addr }
    }
}

impl<I2C, E> CommunicationInterface for I2cInterface<I2C>
where
    I2C: WriteRead<Error = E> + Write<Error = E>,
{
    type Error = E;

    fn write_register(&mut self, addr: u8, value: u8) -> Result<(), Self::Error> {
        self.i2c.write(self.addr, &[addr, value])
    }

    fn read_register(&mut self, addr: u8) -> Result<u8, Self::Error> {
        let mut bytes = [0u8; 2];
        self.read_bytes(addr, &mut bytes)?;
        Ok(bytes[1])
    }

    fn read_bytes(&mut self, addr: u8, bytes: &mut [u8]) -> Result<(), Self::Error> {
        self.i2c.write_read(self.addr, &[addr], bytes)
    }
}
