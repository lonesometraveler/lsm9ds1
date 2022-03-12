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

    /*

    let config_xl = IntConfigAccel {                    
                    ..Default::default()
                        };

    lsm9ds1.configure_interrupts_accel(config_xl).unwrap();
    
    println!("register INT_GEN_CFG_XL {:08b}", 
            lsm9ds1.read_register(Sensor::Accelerometer, 
                                register::AG::INT_GEN_CFG_XL.addr()).unwrap());

    let cfg_xl = lsm9ds1.get_accel_int_config().unwrap();

    println!("default configuration:\n{:?}", cfg_xl);

    thread::sleep(Duration::from_millis(500));

    println!("testing single settings:...\n");


    println!("events combination:...");

    lsm9ds1.accel_int_events_combination(COMBINATION::AND).unwrap();

    println!("current setting: {:?}", lsm9ds1.get_accel_int_config().unwrap().events_combination);                

    thread::sleep(Duration::from_millis(250));

    lsm9ds1.accel_int_events_combination(COMBINATION::OR).unwrap();

    println!("current setting: {:?}", lsm9ds1.get_accel_int_config().unwrap().events_combination);                

    thread::sleep(Duration::from_millis(250));

    
    println!("enable 6D:...");

    lsm9ds1.accel_int_enable_6d(FLAG::Enabled).unwrap();

    println!("current setting: {:?}", lsm9ds1.get_accel_int_config().unwrap().enable_6d);                

    thread::sleep(Duration::from_millis(250));

    lsm9ds1.accel_int_enable_6d(FLAG::Disabled).unwrap();

    println!("current setting: {:?}", lsm9ds1.get_accel_int_config().unwrap().enable_6d);                

    thread::sleep(Duration::from_millis(250));


    println!("Z axis high:...");

    lsm9ds1.accel_int_zaxis_high(FLAG::Enabled).unwrap();

    println!("current setting: {:?}", lsm9ds1.get_accel_int_config().unwrap().interrupt_zaxis_high);                

    thread::sleep(Duration::from_millis(250));

    lsm9ds1.accel_int_zaxis_high(FLAG::Disabled).unwrap();

    println!("current setting: {:?}", lsm9ds1.get_accel_int_config().unwrap().interrupt_zaxis_high);                

    thread::sleep(Duration::from_millis(250));


    println!("Z axis low:...");

    lsm9ds1.accel_int_zaxis_low(FLAG::Enabled).unwrap();

    println!("current setting: {:?}", lsm9ds1.get_accel_int_config().unwrap().interrupt_zaxis_low);                

    thread::sleep(Duration::from_millis(250));

    lsm9ds1.accel_int_zaxis_low(FLAG::Disabled).unwrap();

    println!("current setting: {:?}", lsm9ds1.get_accel_int_config().unwrap().interrupt_zaxis_low);                

    thread::sleep(Duration::from_millis(250));


    println!("Y axis high:...");

    lsm9ds1.accel_int_yaxis_high(FLAG::Enabled).unwrap();

    println!("current setting: {:?}", lsm9ds1.get_accel_int_config().unwrap().interrupt_yaxis_high);                

    thread::sleep(Duration::from_millis(250));

    lsm9ds1.accel_int_yaxis_high(FLAG::Disabled).unwrap();

    println!("current setting: {:?}", lsm9ds1.get_accel_int_config().unwrap().interrupt_yaxis_high);                

    thread::sleep(Duration::from_millis(250));


    println!("Y axis low:...");

    lsm9ds1.accel_int_yaxis_low(FLAG::Enabled).unwrap();

    println!("current setting: {:?}", lsm9ds1.get_accel_int_config().unwrap().interrupt_yaxis_low);                

    thread::sleep(Duration::from_millis(250));

    lsm9ds1.accel_int_yaxis_low(FLAG::Disabled).unwrap();

    println!("current setting: {:?}", lsm9ds1.get_accel_int_config().unwrap().interrupt_yaxis_low);                

    thread::sleep(Duration::from_millis(250));


    println!("X axis high:...");

    lsm9ds1.accel_int_xaxis_high(FLAG::Enabled).unwrap();

    println!("current setting: {:?}", lsm9ds1.get_accel_int_config().unwrap().interrupt_xaxis_high);                

    thread::sleep(Duration::from_millis(250));

    lsm9ds1.accel_int_xaxis_high(FLAG::Disabled).unwrap();

    println!("current setting: {:?}", lsm9ds1.get_accel_int_config().unwrap().interrupt_xaxis_high);                

    thread::sleep(Duration::from_millis(250));


    println!("X axis low:...");

    lsm9ds1.accel_int_xaxis_low(FLAG::Enabled).unwrap();

    println!("current setting: {:?}", lsm9ds1.get_accel_int_config().unwrap().interrupt_xaxis_low);                

    thread::sleep(Duration::from_millis(250));

    lsm9ds1.accel_int_xaxis_low(FLAG::Disabled).unwrap();

    println!("current setting: {:?}", lsm9ds1.get_accel_int_config().unwrap().interrupt_xaxis_low);                

    thread::sleep(Duration::from_millis(1000));

 */        


    println!("\nsetting thresholds:...");

    lsm9ds1.set_accel_int_thresholds(0, 254, 253).unwrap();

    println!("register INT_GEN_THS_X_XL: {:08b}",
                lsm9ds1.read_register(Sensor::Accelerometer, register::AG::INT_GEN_THS_X_XL.addr()).unwrap());

    let (x,y,z) = lsm9ds1.get_accel_int_thresholds().unwrap();

    println!("thresholds: x {}, y {}, z {}", x, y, z);


    /*

    lsm9ds1.set_accel_enable_6d(FLAG::Enabled).unwrap();

        

    println!("register INT_GEN_CFG_XL {:08b}", 
            lsm9ds1.read_register(Sensor::Accelerometer, 
                                register::AG::INT_GEN_CFG_XL.addr()).unwrap());


    let cfg_xl = lsm9ds1.get_accel_int_config().unwrap();

    println!("current configuration:\n{:?}", cfg_xl);
 */

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
