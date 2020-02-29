use super::Interface;
use super::Sensor;
use embedded_hal::{blocking::spi::Transfer, blocking::spi::Write, digital::v2::OutputPin};
use Sensor::*;

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

/// This combines the SPI Interface and chip select pins
pub struct SpiInterface<SPI, AG, M> {
    spi: SPI,
    ag_cs: AG,
    m_cs: M,
}

impl<SPI, AG, M, CommE, PinE> SpiInterface<SPI, AG, M>
where
    SPI: Transfer<u8, Error = CommE> + Write<u8, Error = CommE>,
    AG: OutputPin<Error = PinE>,
    M: OutputPin<Error = PinE>,
{
    /// create Interface with `SPI` instance and AG and M chip select `OutputPin`s
    pub fn new(spi: SPI, ag_cs: AG, m_cs: M) -> Self {
        Self { spi, ag_cs, m_cs }
    }
}

/// Implementation of `Interface`
impl<SPI, AG, M, CommE, PinE> Interface for SpiInterface<SPI, AG, M>
where
    SPI: Transfer<u8, Error = CommE> + Write<u8, Error = CommE>,
    AG: OutputPin<Error = PinE>,
    M: OutputPin<Error = PinE>,
{
    type Error = Error<CommE, PinE>;

    fn write(&mut self, sensor: Sensor, addr: u8, value: u8) -> Result<(), Self::Error> {
        let bytes = [addr, value];
        match sensor {
            Accelerometer | Gyro | Temperature => {
                self.ag_cs.set_low().map_err(Error::Pin)?;
                self.spi.write(&bytes).map_err(Error::Comm)?;
                self.ag_cs.set_high().map_err(Error::Pin)?;
            }
            Magnetometer => {
                self.m_cs.set_low().map_err(Error::Pin)?;
                self.spi.write(&bytes).map_err(Error::Comm)?;
                self.m_cs.set_high().map_err(Error::Pin)?;
            }
        }
        Ok(())
    }

    fn read(&mut self, sensor: Sensor, addr: u8, buffer: &mut [u8]) -> Result<(), Self::Error> {
        match sensor {
            Accelerometer | Gyro | Temperature => {
                self.ag_cs.set_low().map_err(Error::Pin)?;
                self.spi.write(&[SPI_READ | addr]).map_err(Error::Comm)?;
                self.spi.transfer(buffer).map_err(Error::Comm)?;
                self.ag_cs.set_high().map_err(Error::Pin)?;
            }
            Magnetometer => {
                self.m_cs.set_low().map_err(Error::Pin)?;
                self.spi.write(&[SPI_READ | addr]).map_err(Error::Comm)?;
                self.spi.transfer(buffer).map_err(Error::Comm)?;
                self.m_cs.set_high().map_err(Error::Pin)?;
            }
        }
        Ok(())
    }
}
