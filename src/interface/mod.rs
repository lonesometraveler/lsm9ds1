pub mod spi;
pub use self::spi::SpiInterface;
pub mod i2c;
pub use self::i2c::I2cInterface;

pub trait Interface {
    type Error;
    fn write_register(&mut self, addr: u8, value: u8) -> Result<(), Self::Error>;
    fn read_register(&mut self, addr: u8) -> Result<u8, Self::Error>;
    fn read_bytes(&mut self, addr: u8, bytes: &mut [u8]) -> Result<(), Self::Error>;
}
