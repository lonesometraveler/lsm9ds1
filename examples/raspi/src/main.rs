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
use lsm9ds1::interrupts::mag_int::IntConfigMag;
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
    
    /*
    println!("current CTRL_REG4: {:08b}", 
            lsm9ds1.read_register(Sensor::Accelerometer,register::AG::CTRL_REG4.addr()).unwrap());

    lsm9ds1.accel_int_latching(INT_LATCH::Latched).unwrap();
 */
   

    let config_m = IntConfigMag {                    
                    ..Default::default()
                        };

    lsm9ds1.configure_interrupts_mag(config_m).unwrap();
    
    println!("register INT_CFG_M {:08b}", 
            lsm9ds1.read_register(Sensor::Magnetometer, 
                                register::Mag::INT_CFG_M.addr()).unwrap());

    let cfg_m = lsm9ds1.get_mag_int_config().unwrap();

    println!("default configuration:\n{:?}", cfg_m);

    
    thread::sleep(Duration::from_millis(500));


    let config_m = IntConfigMag {                    
            interrupt_xaxis: FLAG::Enabled,
            interrupt_yaxis: FLAG::Enabled,
            interrupt_zaxis: FLAG::Enabled,
            active_high_or_low: INT_ACTIVE::High,
            interrupt_latching: INT_LATCH::NotLatched,
            enable_interrupt: FLAG::Enabled,
            };

    lsm9ds1.configure_interrupts_mag(config_m).unwrap();

    println!("register INT_CFG_M {:08b}", 
    lsm9ds1.read_register(Sensor::Magnetometer, 
                        register::Mag::INT_CFG_M.addr()).unwrap());

    let cfg_m = lsm9ds1.get_mag_int_config().unwrap();

    println!("current configuration:\n{:?}", cfg_m);





    println!("testing single settings:...\n");


    println!("X axis:...");

    lsm9ds1.mag_int_xaxis(FLAG::Enabled).unwrap();

    println!("current setting: {:?}", lsm9ds1.get_mag_int_config().unwrap().interrupt_xaxis);                

    thread::sleep(Duration::from_millis(250));

    lsm9ds1.mag_int_xaxis(FLAG::Disabled).unwrap();

    println!("current setting: {:?}", lsm9ds1.get_mag_int_config().unwrap().interrupt_xaxis);                

    thread::sleep(Duration::from_millis(250));


    println!("Y axis:...");

    lsm9ds1.mag_int_yaxis(FLAG::Enabled).unwrap();

    println!("current setting: {:?}", lsm9ds1.get_mag_int_config().unwrap().interrupt_yaxis);                

    thread::sleep(Duration::from_millis(250));

    lsm9ds1.mag_int_yaxis(FLAG::Disabled).unwrap();

    println!("current setting: {:?}", lsm9ds1.get_mag_int_config().unwrap().interrupt_yaxis);                

    thread::sleep(Duration::from_millis(250));


    println!("Z axis:...");

    lsm9ds1.mag_int_zaxis(FLAG::Enabled).unwrap();

    println!("current setting: {:?}", lsm9ds1.get_mag_int_config().unwrap().interrupt_zaxis);                

    thread::sleep(Duration::from_millis(250));

    lsm9ds1.mag_int_zaxis(FLAG::Disabled).unwrap();

    println!("current setting: {:?}", lsm9ds1.get_mag_int_config().unwrap().interrupt_zaxis);                

    thread::sleep(Duration::from_millis(250));


    println!("active high/low:...");

    lsm9ds1.mag_int_pin_active(INT_ACTIVE::High).unwrap();

    println!("current setting: {:?}", lsm9ds1.get_mag_int_config().unwrap().active_high_or_low);                

    thread::sleep(Duration::from_millis(250));

    lsm9ds1.mag_int_pin_active(INT_ACTIVE::Low).unwrap();

    println!("current setting: {:?}", lsm9ds1.get_mag_int_config().unwrap().active_high_or_low);                

    thread::sleep(Duration::from_millis(250));
    

    println!("interupt latching:...");

    lsm9ds1.mag_int_latching(INT_LATCH::Latched).unwrap();

    println!("current setting: {:?}", lsm9ds1.get_mag_int_config().unwrap().interrupt_latching);                

    thread::sleep(Duration::from_millis(250));

    lsm9ds1.mag_int_latching(INT_LATCH::NotLatched).unwrap();

    println!("current setting: {:?}", lsm9ds1.get_mag_int_config().unwrap().interrupt_latching);                

    thread::sleep(Duration::from_millis(250));


    println!("interrupt enabling:...");

    lsm9ds1.mag_int_enable(FLAG::Enabled).unwrap();

    println!("current setting: {:?}", lsm9ds1.get_mag_int_config().unwrap().enable_interrupt);                

    thread::sleep(Duration::from_millis(250));

    lsm9ds1.mag_int_enable(FLAG::Disabled).unwrap();

    println!("current setting: {:?}", lsm9ds1.get_mag_int_config().unwrap().enable_interrupt);                

    thread::sleep(Duration::from_millis(250));

    thread::sleep(Duration::from_millis(1000));


    /*

    println!("\nsetting thresholds:...");

    lsm9ds1.set_accel_int_thresholds(255, 255, 255).unwrap();

    println!("register INT_GEN_THS_X_XL: {:08b}",
                lsm9ds1.read_register(Sensor::Accelerometer, register::AG::INT_GEN_THS_X_XL.addr()).unwrap());

    let (x,y,z) = lsm9ds1.get_accel_int_thresholds().unwrap();

    println!("thresholds: x {}, y {}, z {}", x, y, z);


    println!("\nsetting duration:...");

    lsm9ds1.accel_int_duration(FLAG::Enabled, 0).unwrap();

    println!("register INT_GEN_DUR_XL: {:08b}",
                lsm9ds1.read_register(Sensor::Accelerometer, register::AG::INT_GEN_DUR_XL.addr()).unwrap());

 */
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
