# lsm9ds1

A platform agnostic driver to interface with LSM9DS1 3D accelerometer, 3D gyroscope, 3D magnetometer sensor module.

* [LSM9DS1 datasheet](https://www.st.com/resource/en/datasheet/lsm9ds1.pdf)

This library is work in progress. Not all features are implemented yet. Contributions are welcome.

## Features

* [x] SPI communication with Accelerometer/Gyroscope
* [x] SPI communication with Magnetometer
* [x] SPI communication with Temperature Sensor
* [x] I2C communication with Accelerometer/Gyroscope
* [ ] I2C communication with Magnetometer
* [x] I2C communication with Temperature Sensor
* [x] Sensor reading (Accel, Gyro, Mag, Temperature) See `read_accel()`, `read_gyro()`, etc.
* [x] Raw Sensor reading (Accel, Gyro, Mag, Temperature) See `read_accel_raw()`, `read_gyro_raw`
* [x] Output Data Rate config for Accel, Gyro, Mag
* [x] Sensitivity config for Accel, Gyro, Mag
* [x] Bandwidth config for Accel and Gyro
* [ ] Calibration
* [ ] Interrupt
* [ ] FIFO Mode


## Usage

To use this driver, import this crate and instantiate either SPI or I2C interface. The code below shows how to read sensor values with SPI interface.

```rust
//! Target board: STM32F3DISCOVERY
#![no_std]
#![no_main]

extern crate panic_semihosting;
pub use cortex_m::{asm::bkpt, iprint, iprintln, peripheral::ITM};
use cortex_m_rt::entry;
use embedded_hal::spi::MODE_0;
use stm32f3xx_hal as hal;

use hal::i2c::I2c;
use hal::prelude::*;
use hal::spi::Spi;
use hal::stm32;

use lsm9ds1::interface::SpiInterface;
use lsm9ds1::interface::I2cInterface;
use lsm9ds1::interface::i2c::{AgAddress, MagAddress};
use lsm9ds1::LSM9DS1;

#[entry]
fn main() -> ! {
    let cp = cortex_m::Peripherals::take().unwrap();
    let mut itm = cp.ITM;
    let dp = stm32::Peripherals::take().unwrap();
    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();

    // Accelerometer/Gyro CS
    let mut gpiob = dp.GPIOB.split(&mut rcc.ahb);
    let mut ag_cs = gpiob
        .pb5
        .into_push_pull_output(&mut gpiob.moder, &mut gpiob.otyper);
    ag_cs.set_high().unwrap();

    // Magnetometer CS
    let mut m_cs = gpiob
        .pb4
        .into_push_pull_output(&mut gpiob.moder, &mut gpiob.otyper);
    m_cs.set_high().unwrap();

    // SPI
    let mut gpioa = dp.GPIOA.split(&mut rcc.ahb);
    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    // Configure pins for SPI
    let sck = gpioa.pa5.into_af5(&mut gpioa.moder, &mut gpioa.afrl);
    let miso = gpioa.pa6.into_af5(&mut gpioa.moder, &mut gpioa.afrl);
    let mosi = gpioa.pa7.into_af5(&mut gpioa.moder, &mut gpioa.afrl);

    let spi = Spi::spi1(
        dp.SPI1,
        (sck, miso, mosi),
        MODE_0,
        1.mhz(),
        clocks,
        &mut rcc.apb2,
    );

    // Create SPI interface
    let spi_interface = SpiInterface::new(spi, ag_cs, m_cs);
    let mut lsm9ds1 = LSM9DS1::from_interface(spi_interface);

    lsm9ds1.init_accel().unwrap();
    lsm9ds1.init_gyro().unwrap();
    lsm9ds1.init_mag().unwrap();

    loop {
        let temp = lsm9ds1.read_temp().unwrap();
        iprintln!(&mut itm.stim[0], "temp: {}", temp);

        let (x, y, z) = lsm9ds1.read_accel().unwrap();
        iprintln!(&mut itm.stim[0], "xl: {}, {}, {}", x, y, z);

        let (x, y, z) = lsm9ds1.read_gyro().unwrap();
        iprintln!(&mut itm.stim[0], "gy: {}, {}, {}", x, y, z);

        let (x, y, z) = lsm9ds1.read_mag().unwrap() {
            iprintln!(&mut itm.stim[0], "mg: {}, {}, {}", x, y, z);
        }

        cortex_m::asm::delay(8_000_000);
    }
}
```
