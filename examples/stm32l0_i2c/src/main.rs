// reads correctly in one shot mode
// reads correctly in continuous mode



#![no_main]
#![no_std]

extern crate panic_halt;

use cortex_m_rt::entry;
use stm32l0xx_hal::{pac, 
    prelude::*, 
    rcc::{Config},    
    serial,
    };

use lsm9ds1::interface::{I2cInterface,
        i2c::{AgAddress, MagAddress}};
use lsm9ds1::{accel, gyro, mag, LSM9DS1Init};



use core::fmt::Write;

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();
    let cp = cortex_m::Peripherals::take().unwrap();

    //configure the clock
    let mut rcc = dp.RCC.freeze(Config::hsi16());
    
    //acquire the GPIOA and GPIOB peripherals
    let mut gpioa = dp.GPIOA.split(&mut rcc);
    let mut gpiob = dp.GPIOB.split(&mut rcc);
  
    //get the delay provider
    let mut delay = cp.SYST.delay(rcc.clocks);

    //configure PB3 as output (green LED)
    let mut green = gpiob.pb3.into_push_pull_output(); // SPI CLOCK!

    // configure UART TX/RX pins
    let tx_pin = gpioa.pa2;
    let rx_pin = gpioa.pa3;
    
    // configure serial (default 9600 bps)
    let mut serial = dp.USART2.usart(tx_pin, rx_pin, serial::Config::default().baudrate(9600.Bd()), &mut rcc).unwrap();

    let (mut tx, mut _rx) = serial.split();

    // I2C pins
    let scl = gpioa.pa9.into_open_drain_output();
    let sda = gpioa.pa10.into_open_drain_output();
    
    // I2C configuration
    let i2c = dp.I2C1.i2c(sda, scl, 100_000.Hz(), &mut rcc); 
    

    // initialize LSM9DS1 sensor    
    let ag_addr = AgAddress::_2; // 0x6B
    let mag_addr = MagAddress::_2; // 0x1E

    
    let i2c_interface = I2cInterface::init(i2c, ag_addr, mag_addr);
    
    let mut lsm9ds1 = LSM9DS1Init {
                    ..Default::default()
                    }.with_interface(i2c_interface);

    
    //lsm9ds1.begin_accel().unwrap();
    //lsm9ds1.begin_gyro().unwrap();
    //lsm9ds1.begin_mag().unwrap();
    

    let mut val: u8 = 0;

    loop {
        
        let whoami = lsm9ds1.whoami_ag().unwrap();

        writeln!(tx, "my name is {}\r", whoami).unwrap();

        // let (m_x,m_y,m_z) = lsm9ds1.read_mag().unwrap(); // read magnetometer values
        
        // print data to serial
        // writeln!(tx, "magnetometer X: {:.3}, Y: {:.3}, Z: {:.3}\r", m_x, m_y, m_y).unwrap();
         
        green.set_high().unwrap();    
        delay.delay_ms(250_u16);

        // writeln!(tx, "blink! {}\r", val).unwrap();

        val += 1;

        green.set_low().unwrap();
    
        delay.delay_ms(500_u16);
    }


}



