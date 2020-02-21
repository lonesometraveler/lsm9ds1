#![no_std]
// #![deny(warnings, missing_docs)]
use embedded_hal::{blocking::spi::Transfer, blocking::spi::Write, digital::v2::OutputPin};

#[derive(Debug)]
pub struct LSM9DS1<SPI, CS> {
    spi: SPI,
    cs: CS,
}

impl<SPI, CS, E> LSM9DS1<SPI, CS>
where
    SPI: Transfer<u8, Error = E> + Write<u8, Error = E>,
    CS: OutputPin, 
    // E: core::convert::From<<CS as embedded_hal::digital::v2::OutputPin>::Error>
{
    pub fn new(spi: SPI, cs: CS) -> Result<LSM9DS1<SPI, CS>, E> {
        let mut this = Self { spi, cs };
        // this.cs.set_high()?;
        this.cs.set_high();
        Ok(this)
    }

    pub fn read_byte(&mut self, addr: u8) -> Result<u8, E> {
        let mut buffer = [0; 2];
        buffer[0] = addr;
        self.cs.set_low();
        self.spi.transfer(&mut buffer)?;
        self.cs.set_high();

        Ok(buffer[1])
    }

    pub fn read_bytes(&mut self, sub_address: u8, _count: usize) -> Result<(), E> {
        let mut addr = [0x80 | (sub_address & 0x3F); 1];
        self.cs.set_low();
        self.spi.transfer(&mut addr)?;
        let mut data = [0u8; 8];
        self.spi.transfer(&mut data)?;
        self.cs.set_high();

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
