# lsm9ds1

![](https://img.shields.io/crates/v/lsm9ds1.svg)
![](https://docs.rs/lsm9ds1/badge.svg)
![](https://github.com/lonesometraveler/lsm9ds1/workflows/build/badge.svg)

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
* [x] Custom sensor settings
* [x] Sensor reading (Accel, Gyro, Mag, Temperature)
* [x] Raw Sensor reading (Accel, Gyro, Mag) See `read_sensor_raw()`.
* [ ] Calibration
* [ ] Interrupt
* [ ] FIFO Mode


## Usage

### Overview

1. Configure sensor settings with `LSM9DS1Init`. 
2. Initialize a communication interface: either `SpiInterface` or `I2cInterface`.
3. Initialize `LSM9DS1` driver with the interface of your choice.
4. Start the sensors.
5. Get the sensors' readings.

### Sensor Settings

This driver uses `LSM9DS1Init` struct to set sensor configuration. 

```rust
pub struct LSM9DS1Init {
    pub accel: AccelSettings,
    pub gyro: GyroSettings,
    pub mag: MagSettings,
}
```
You can find each sensor's default settings in `accel.rs`, `gyro.rs` and `mag.rs`.

#### Accelerometer default settings

```rust
impl Default for AccelSettings {
    fn default() -> Self {
        AccelSettings {
            enable_x: true,
            enable_y: true,
            enable_z: true,
            sample_rate: ODR::_119Hz,
            scale: Scale::_2G,
            bandwidth_selection: BandwidthSelection::ByODR,
            bandwidth: Bandwidth::_408Hz,
            high_res_bandwidth: HighRes::Disabled,
        }
    }
}
```

#### Gyroscope default settings

```rust
impl Default for GyroSettings {
    fn default() -> Self {
        GyroSettings {
            enable_x: true,
            enable_y: true,
            enable_z: true,
            flip_x: false,
            flip_y: false,
            flip_z: false,
            scale: Scale::_245DPS,
            sample_rate: ODR::_952Hz,
            bandwidth: Bandwidth::LPF_0,
            int_selection: GyroIntSelection::SEL_0,
            out_selection: GyroOutSelection::SEL_0,
            low_power_mode: LowPowerMode::Disabled,
            hpf_mode: HpFilter::Disabled,
            hpf_cutoff: HpFilterCutoff::HPCF_1,
            latch_interrupt: LatchInterrupt::Disabled,
        }
    }
}
```

#### Magnetometer default settings

```rust
impl Default for MagSettings {
    fn default() -> Self {
        MagSettings {
            temp_compensation: TempComp::Disabled,
            x_y_performance: OpModeXY::Low,
            sample_rate: ODR::_10Hz,
            scale: Scale::_4G,
            i2c_mode: I2cMode::Enabled,
            system_op: SysOpMode::Continuous,
            low_power: LowPowerMode::Disabled,
            spi_mode: SpiMode::RW,
            z_performance: OpModeZ::Low,
        }
    }
}
```
### How to configure settings

If you want to use the default settings, initialize `LSM9DS1Init` this way.

```rust
LSM9DS1Init {
    ..Default::default()
}
```
If you want a custom configuration, modify the fields that are different from the default values.

```rust
LSM9DS1Init {
    accel: accel::AccelSettings {
        scale: accel::Scale::_16G, // custom setting
        ..Default::default() // the rest of the fields are the default values.
    },
    ..Default::default() // gyro and mag use the default settings.
}
```

### Communication Interface

LSM9DS1 supports SPI and I2C communication. Create an instance of `SpiInterface` or `I2cInterface` and pass it to `LSM9DS1Init`'s `with_interface()` method to create an instance of `LSM9DS1` driver.

```rust
// Create SPI interface
let spi_interface = SpiInterface::init(spi, ag_cs, m_cs);
// Init LSM9DS1 driver with settings and SPI interface
let mut lsm9ds1 = LSM9DS1Init {
    ..Default::default()
}
.with_interface(spi_interface);
```

### Reading Sensors

Turn on sensors.

```rust
lsm9ds1.begin_accel().unwrap();
lsm9ds1.begin_gyro().unwrap();
lsm9ds1.begin_mag().unwrap();
```

Get readings

```rust
let temp = lsm9ds1.read_temp().unwrap(); // temperature reading in celsius
let (x, y, z) = lsm9ds1.read_accel().unwrap();
let (x, y, z) = lsm9ds1.read_gyro().unwrap();
let (x, y, z) = lsm9ds1.read_mag().unwrap();
```

## Example

This code shows how to read sensor values with SPI interface. (Error handling is omitted for brevity.)

```rust
//! Target board: STM32F3DISCOVERY
#![no_std]
#![no_main]

extern crate panic_semihosting;
pub use cortex_m::{asm::bkpt, iprint, iprintln, peripheral::ITM};
use cortex_m_rt::entry;
use embedded_hal::spi::MODE_0;
use stm32f3xx_hal as hal;

use hal::prelude::*;
use hal::spi::Spi;
use hal::stm32;

use lsm9ds1::interface::SpiInterface;
use lsm9ds1::{accel, gyro, mag, LSM9DS1Init};

#[entry]
fn main() -> ! {
    let cp = cortex_m::Peripherals::take().unwrap();
    let mut itm = cp.ITM;
    let dp = stm32::Peripherals::take().unwrap();
    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();

    // Accelerometer/Gyroscope Chip Select
    let mut gpiob = dp.GPIOB.split(&mut rcc.ahb);
    let mut ag_cs = gpiob
        .pb5
        .into_push_pull_output(&mut gpiob.moder, &mut gpiob.otyper);
    ag_cs.set_high().unwrap();

    // Magnetometer Chip Select
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
    let spi_interface = SpiInterface::init(spi, ag_cs, m_cs);
    // Init LSM9DS1 with default settings and attach SPI interface
    let mut lsm9ds1 = LSM9DS1Init {
        ..Default::default()
    }
    .with_interface(spi_interface);
	
    // start sensors
    lsm9ds1.begin_accel().unwrap();
    lsm9ds1.begin_gyro().unwrap();
    lsm9ds1.begin_mag().unwrap();

    loop {
        // read sensors
        let temp = lsm9ds1.read_temp().unwrap();
        iprintln!(&mut itm.stim[0], "temp: {}", temp);

        let (x, y, z) = lsm9ds1.read_accel().unwrap();
        iprintln!(&mut itm.stim[0], "xl: {}, {}, {}", x, y, z);

        let (x, y, z) = lsm9ds1.read_gyro().unwrap();
        iprintln!(&mut itm.stim[0], "gy: {}, {}, {}", x, y, z);

        let (x, y, z) = lsm9ds1.read_mag().unwrap();
        iprintln!(&mut itm.stim[0], "mg: {}, {}, {}", x, y, z);

        cortex_m::asm::delay(8_000_000);
    }
}
```
