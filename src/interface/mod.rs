pub mod spi;
pub use self::spi::SpiInterface;
pub mod i2c;
pub use self::i2c::I2cInterface;

use embedded_hal::blocking::i2c::{Write as i2cWrite, WriteRead};
use embedded_hal::{blocking::spi::Transfer, blocking::spi::Write, digital::v2::OutputPin};

pub trait CommunicationInterface {
    type Error;
    fn write_register(&mut self, addr: u8, value: u8) -> Result<(), Self::Error>;
    fn read_register(&mut self, addr: u8) -> Result<u8, Self::Error>;
    fn read_bytes(&mut self, addr: u8, bytes: &mut [u8]) -> Result<(), Self::Error>;
}

pub enum Selection<SPI, CS, I2C> {
    SPI(SpiInterface<SPI, CS>),
    I2C(I2cInterface<I2C>),
}

#[derive(Debug)]
pub enum Error<CommE, PinE, E> {
    /// Communication error
    Comm(CommE),
    /// Pin setting error
    Pin(PinE),
    /// I2C error
    I2C(E),
}

impl<CommE, PinE, E> From<spi::Error<CommE, PinE>> for Error<CommE, PinE, E> {
    fn from(err: spi::Error<CommE, PinE>) -> Error<CommE, PinE, E> {
        match err {
            spi::Error::Comm(x) => Error::Comm(x),
            spi::Error::Pin(x) => Error::Pin(x),
        }
    }
}

impl<CommE, PinE, E> From<i2c::Error<E>> for Error<CommE, PinE, E> {
    fn from(err: i2c::Error<E>) -> Error<CommE, PinE, E> {
        match err {
            i2c::Error::I2C(x) => Error::I2C(x),
        }
    }
}

impl<SPI, CS, CommE, PinE, I2C, E> CommunicationInterface for Selection<SPI, CS, I2C>
where
    SPI: Transfer<u8, Error = CommE> + Write<u8, Error = CommE>,
    CS: OutputPin<Error = PinE>,
    I2C: WriteRead<Error = E> + i2cWrite<Error = E>,
{
    type Error = Error<CommE, PinE, E>;
    fn write_register(&mut self, addr: u8, value: u8) -> Result<(), Self::Error> {
        match self {
            Selection::SPI(inner) => core::prelude::v1::Ok(inner.write_register(addr, value)?),
            Selection::I2C(inner) => core::prelude::v1::Ok(inner.write_register(addr, value)?),
        }
    }

    fn read_register(&mut self, addr: u8) -> Result<u8, Self::Error> {
        match self {
            Selection::SPI(inner) => core::prelude::v1::Ok(inner.read_register(addr)?),
            Selection::I2C(inner) => core::prelude::v1::Ok(inner.read_register(addr)?),
        }
    }

    fn read_bytes(&mut self, addr: u8, bytes: &mut [u8]) -> Result<(), Self::Error> {
        match self {
            Selection::SPI(inner) => core::prelude::v1::Ok(inner.read_bytes(addr, bytes)?),
            Selection::I2C(inner) => core::prelude::v1::Ok(inner.read_bytes(addr, bytes)?),
        }
    }
}
