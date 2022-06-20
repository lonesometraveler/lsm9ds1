use super::{
    accel_int::IntConfigAccel,
    pins_config::{IntConfigAG1, IntConfigAG2, PinConfig},
};
use crate::{interface::Sensor, register};

pub trait Configuration {
    fn value(&self) -> u8;
    fn sensor(&self) -> Sensor;
    fn addr(&self) -> u8;
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
