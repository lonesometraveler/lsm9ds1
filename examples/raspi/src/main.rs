//! talking to the BMA220 module over I2C on Raspberry Pi
//! 

use std::thread;
use std::time::Duration;

use rppal::i2c::I2c;

use lsm9ds1::interface::{Sensor, I2cInterface,
    i2c::{AgAddress, MagAddress}};
use lsm9ds1::{accel, gyro, mag, LSM9DS1Init};
use lsm9ds1::register;
use lsm9ds1::*;

use lsm9ds1::interrupts::accel_int::IntConfigAccel;
use lsm9ds1::interrupts::gyro_int::IntConfigGyro;
//use lsm9ds1::interrupts::mag_int::IntConfigMag;
use lsm9ds1::interrupts::*;

fn main() {
    // new I2C instance with rppal
    let i2c = I2c::new().unwrap();

    // initialize LSM9DS1 sensor    
    let ag_addr = AgAddress::_2; // 0x6B
    let mag_addr = MagAddress::_2; // 0x1E
    
    let i2c_interface = I2cInterface::init(i2c, ag_addr, mag_addr);

    let mut lsm9ds1 = LSM9DS1Init {
        ..Default::default()
        }.with_interface(i2c_interface);

    //thread::sleep(Duration::from_millis(500));

    lsm9ds1.begin_accel().unwrap();
    lsm9ds1.begin_gyro().unwrap();
    lsm9ds1.begin_mag().unwrap();
    
    // let whoami = lsm9ds1.whoami_ag().unwrap();

    // let (a_x,a_y,a_z) = lsm9ds1.read_accel().unwrap();

        

    let config_xl = IntConfigAccel {                    
                    ..Default::default()
                        };

    lsm9ds1.configure_interrupts_accel(config_xl).unwrap();
    
    println!("register INT_GEN_CFG_XL {:08b}", 
            lsm9ds1.read_register(Sensor::Accelerometer, 
                                register::AG::INT_GEN_CFG_XL.addr()).unwrap());

    let cfg_xl = lsm9ds1.get_accel_int_config().unwrap();

    println!("current configuration:\n{:?}", cfg_xl);

    lsm9ds1.set_accel_events_combination(COMBINATION::AND).unwrap();

    lsm9ds1.set_accel_enable_6d(FLAG::Enabled).unwrap();

    println!("register INT_GEN_CFG_XL {:08b}", 
            lsm9ds1.read_register(Sensor::Accelerometer, 
                                register::AG::INT_GEN_CFG_XL.addr()).unwrap());


    let cfg_xl = lsm9ds1.get_accel_int_config().unwrap();



    println!("current configuration:\n{:?}", cfg_xl);
 /*
    let config_xl = IntConfigAccel {                    
        events_combination: COMBINATION::AND,
        enable_6d: FLAG::Enabled,
        ..Default::default()
            };

    lsm9ds1.configure_interrupts_accel(config_xl).unwrap();

    println!("register INT_GEN_CFG_XL {:08b}", 
            lsm9ds1.read_register(Sensor::Accelerometer, 
                                register::AG::INT_GEN_CFG_XL.addr()).unwrap());

    let cfg_xl = lsm9ds1.get_accel_int_config().unwrap();

    println!("current configuration:\n{:?}", cfg_xl);

   

    let status = lsm9ds1.gyro_int_status().unwrap();

    println!("register INT_GEN_CFG_G {:08b}", 
    lsm9ds1.read_register(Sensor::Gyro, 
                    register::AG::INT_GEN_CFG_G.addr()).unwrap());


    println!("gyro int status: {:?}", status);
    */
}
