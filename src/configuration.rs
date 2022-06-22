//! Configuration trait, trait implementations
use crate::{
    interface::Sensor,
    interrupts::{
        accel_int::IntConfigAccel,
        gyro_int::IntConfigGyro,
        mag_int::IntConfigMag,
        pins_config::{IntConfigAG1, IntConfigAG2, PinConfig},
    },
    register,
};

pub trait Configuration {
    fn value(&self) -> u8;
    fn sensor(&self) -> Sensor;
    fn addr(&self) -> u8;
}

/// Generic `Configuration` struct
pub struct CustomConfiguration {
    pub sensor: Sensor,
    pub register: u8,
    pub value: u8,
}

impl Configuration for CustomConfiguration {
    fn addr(&self) -> u8 {
        self.register
    }
    fn sensor(&self) -> Sensor {
        self.sensor
    }
    fn value(&self) -> u8 {
        self.value
    }
}

impl Configuration for PinConfig {
    fn value(&self) -> u8 {
        self.ctrl_reg8()
    }
    fn sensor(&self) -> Sensor {
        Sensor::Accelerometer
    }
    fn addr(&self) -> u8 {
        register::AG::CTRL_REG8.addr()
    }
}

impl Configuration for IntConfigAccel {
    fn value(&self) -> u8 {
        self.int_gen_cfg_xl()
    }
    fn sensor(&self) -> Sensor {
        Sensor::Accelerometer
    }
    fn addr(&self) -> u8 {
        register::AG::INT_GEN_CFG_XL.addr()
    }
}

impl Configuration for IntConfigAG1 {
    fn value(&self) -> u8 {
        self.int1_ctrl()
    }
    fn sensor(&self) -> Sensor {
        Sensor::Accelerometer
    }
    fn addr(&self) -> u8 {
        register::AG::INT1_CTRL.addr()
    }
}

impl Configuration for IntConfigAG2 {
    fn value(&self) -> u8 {
        self.int2_ctrl()
    }
    fn sensor(&self) -> Sensor {
        Sensor::Accelerometer
    }
    fn addr(&self) -> u8 {
        register::AG::INT2_CTRL.addr()
    }
}

impl Configuration for IntConfigGyro {
    fn value(&self) -> u8 {
        self.int_gen_cfg_g()
    }
    fn addr(&self) -> u8 {
        register::AG::INT_GEN_CFG_G.addr()
    }
    fn sensor(&self) -> Sensor {
        Sensor::Gyro
    }
}

impl Configuration for IntConfigMag {
    fn value(&self) -> u8 {
        self.int_cfg_m()
    }
    fn addr(&self) -> u8 {
        register::Mag::INT_CFG_M.addr()
    }
    fn sensor(&self) -> Sensor {
        Sensor::Magnetometer
    }
}
