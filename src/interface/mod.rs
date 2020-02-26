pub mod spi;
pub use self::spi::SpiInterface;
pub mod i2c;
pub use self::i2c::I2cInterface;

use embedded_hal::{blocking::spi::Transfer, blocking::spi::Write, digital::v2::OutputPin};

pub trait CommunicationInterface {
    type Error;
    fn write_register(&mut self, addr: u8, value: u8) -> Result<(), Self::Error>;
    fn read_register(&mut self, addr: u8) -> Result<u8, Self::Error>;
    fn read_bytes(&mut self, addr: u8, bytes: &mut [u8]) -> Result<(), Self::Error>;
}

pub enum Selection<SPI, CS> {
    SPI(SpiInterface<SPI, CS>),
    // I2C(I2cInterface<B>)
}

#[derive(Debug)]
pub enum Error<CommE, PinE> {
    /// Communication error
    Comm(CommE),
    /// Pin setting error
    Pin(PinE),
}

impl<CommE, PinE> From<spi::Error<CommE, PinE>> for Error<CommE, PinE> {
    fn from(err: spi::Error<CommE, PinE>) -> Error<CommE, PinE> {
        match err {
            spi::Error::Comm(x) => Error::Comm(x),
            spi::Error::Pin(x) => Error::Pin(x),
        }
    }
}

impl<SPI, CS, CommE, PinE> CommunicationInterface for Selection<SPI, CS>
where
    SPI: Transfer<u8, Error = CommE> + Write<u8, Error = CommE>,
    CS: OutputPin<Error = PinE>,
{
    type Error = Error<CommE, PinE>;
    fn write_register(&mut self, addr: u8, value: u8) -> Result<(), Self::Error> {
        match self {
            Selection::SPI(inner) => core::prelude::v1::Ok(inner.write_register(addr, value)?),
        }
    }
    fn read_register(&mut self, addr: u8) -> Result<u8, Self::Error> {
        match self {
            Selection::SPI(inner) => core::prelude::v1::Ok(inner.read_register(addr)?),
        }
    }
    fn read_bytes(&mut self, addr: u8, bytes: &mut [u8]) -> Result<(), Self::Error> {
        match self {
            Selection::SPI(inner) => core::prelude::v1::Ok(inner.read_bytes(addr, bytes)?),
        }
    }
}
