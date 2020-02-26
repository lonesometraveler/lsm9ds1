// pub mod i2c;
pub mod spi;

pub trait CommunicationInterface {
    type Error;
    fn new() -> Self;
    fn write_register(&mut self, addr: u8, value: u8) -> Result<(), Self::Error>;
    fn read_register(&mut self, addr: u8) -> Result<u8, Self::Error>;
    fn read_bytes(&mut self, bytes: &mut [u8]) -> Result<(), Self::Error>;
}

// pub struct ComMode<IF>(pub IF);

// pub use self::{i2c::I2cInterface, spi::SpiInterface};
pub use self::spi::SpiInterface;

// struct Interface<SPI, CS> {
//     device: Selection<SPI, CS>
// }

// // trait Device {
// //     fn write(&self, data: u8);
// // }

// // struct SpiInterface<A> {
// //     foo: A
// // }

// // impl<A> Device for SpiInterface<A> where A: Copy + std::fmt::Debug {
// //     fn write(&self, data: u8) {
// //         println!("spi {:?}, {}", self.foo, data);
// //     }
// // }

// // impl<A> SpiInterface<A> {
// //     pub fn new(foo: A) -> Self where A:  Copy + std::fmt::Debug {
// //         SpiInterface { foo }
// //     }
// // }

// // struct I2cInterface<B> {
// //     bar: B
// // }

// // impl<B> Device for I2cInterface<B> where B: std::fmt::Debug {
// //     fn write(&self, data: u8) {
// //         println!("i2c {:?}, {}", self.bar, data);
// //     }
// // }

// // impl<B> I2cInterface<B> {
// //     pub fn new(bar: B) -> Self where B: std::fmt::Debug {
// //         I2cInterface { bar }
// //     }
// // }

// enum Selection<SPI, CS> {
//     SPI(SpiInterface<SPI, CS>),
//     // I2C(I2cInterface<B>)
// }

// impl<SPI, CS, CommE, PinE> Selection<SPI, CS> 
//     where SPI: Transfer<u8, Error = CommE> + Write<u8, Error = CommE>, CS: OutputPin<Error = PinE> {
//     fn inner(&self) -> &dyn Device {
//         match self {
//             Selection::SPI(ref inner) => inner,
//             // Selection::I2C(ref inner) => inner,
//         }
//     }

//     fn inner_mut(&mut self) -> &mut dyn Device {
//         match self {
//             Selection::SPI(ref mut inner) => inner,
//             // Selection::I2C(ref mut inner) => inner,
//         }
//     }
// }
