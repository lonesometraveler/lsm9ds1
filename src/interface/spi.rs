use super::CommunicationInterface;
use embedded_hal::{blocking::spi::Transfer, blocking::spi::Write, digital::v2::OutputPin};

/// R/W bit should be high for SPI Read operation
const SPI_READ: u8 = 0x80;

/// Errors in this crate
#[derive(Debug)]
pub enum Error<CommE, PinE> {
    /// Communication error
    Comm(CommE),
    /// Pin setting error
    Pin(PinE),
}

/// This combines the SPI peripheral and a data/command pin
pub struct SpiInterface<SPI, CS> {
    spi: SPI,
    cs: CS,
}

impl<SPI, CS, CommE, PinE> SpiInterface<SPI, CS>
where
    SPI: Transfer<u8, Error = CommE> + Write<u8, Error = CommE>,
    CS: OutputPin<Error = PinE>,
{
    pub fn new(spi: SPI, cs: CS) -> Self {
        Self { spi, cs }
    }
}

impl<SPI, CS, CommE, PinE> CommunicationInterface for SpiInterface<SPI, CS>
where
    SPI: Transfer<u8, Error = CommE> + Write<u8, Error = CommE>,
    CS: OutputPin<Error = PinE>,
{
    type Error = Error<CommE, PinE>;

    fn write_register(&mut self, addr: u8, value: u8) -> Result<(), Self::Error> {
        let bytes = [addr, value];
        self.cs.set_low().map_err(Error::Pin)?;
        self.spi.write(&bytes).map_err(Error::Comm)?;
        self.cs.set_high().map_err(Error::Pin)?;
        Ok(())
    }

    fn read_register(&mut self, addr: u8) -> Result<u8, Self::Error> {
        let mut buffer = [0u8; 2];
        buffer[0] = SPI_READ | (addr & 0x3F);
        self.cs.set_low().map_err(Error::Pin)?;
        self.spi.transfer(&mut buffer).map_err(Error::Comm)?;
        self.cs.set_high().map_err(Error::Pin)?;
        Ok(buffer[1])
    }

    fn read_bytes(&mut self, addr: u8, bytes: &mut [u8]) -> Result<(), Self::Error> {
        bytes[0] = SPI_READ | addr;
        self.cs.set_low().map_err(Error::Pin)?;
        self.spi.transfer(bytes).map_err(Error::Comm)?;
        self.cs.set_high().map_err(Error::Pin)?;
        Ok(())
    }
}
