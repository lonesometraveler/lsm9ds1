pub mod spi;
pub use self::spi::SpiInterface;
pub mod i2c;
pub use self::i2c::I2cInterface;

/// Interface Trait. SpiInterface and I2cInterface implements this.
pub trait Interface {
    type Error;

    /// write a byte to a sensor's specified register address.
    fn write_register(&mut self, sensor: Sensor, addr: u8, value: u8) -> Result<(), Self::Error>;
    /// Read a byte from a sensor's specified register address.
    fn read_register(&mut self, sensor: Sensor, addr: u8) -> Result<u8, Self::Error>;
    /// Read multiple bytes from a sensor's specified register address.
    fn read_bytes(&mut self, sensor: Sensor, addr: u8, bytes: &mut [u8])
        -> Result<(), Self::Error>;
}

/// Available Sensors to talk to
pub enum Sensor {
    Accelerometer,
    Gyro,
    Magnetometer,
}
