//! Interface trait
pub mod spi;
pub use self::spi::SpiInterface;
pub mod i2c;
pub use self::i2c::I2cInterface;
pub mod fake_interface;
pub use self::fake_interface::FakeInterface;

/// Interface Trait. SpiInterface and I2cInterface implements this.
pub trait Interface {
    type Error;

    /// write a byte to a sensor's specified register address.
    fn write(&mut self, sensor: Sensor, addr: u8, value: u8) -> Result<(), Self::Error>;
    /// Read multiple bytes from a sensor's specified register address.
    fn read(&mut self, sensor: Sensor, addr: u8, buffer: &mut [u8]) -> Result<(), Self::Error>;
}

/// Available Sensors to talk to
pub enum Sensor {
    Accelerometer,
    Gyro,
    Magnetometer,
    Temperature,
}
