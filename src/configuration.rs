//! Configuration trait, trait implementations
use crate::{
    accel::AccelSettings,
    fifo::{Decimate, FIFOConfig},
    gyro::GyroSettings,
    interface::Sensor,
    interrupts::{
        accel_int::IntConfigAccel,
        gyro_int::IntConfigGyro,
        mag_int::IntConfigMag,
        pins_config::{IntConfigAG1, IntConfigAG2, PinConfig},
    },
    mag::MagSettings,
    register,
};

pub trait Configuration {
    fn byte(&self) -> u8;
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
    fn byte(&self) -> u8 {
        self.value
    }
}

impl Configuration for PinConfig {
    fn byte(&self) -> u8 {
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
    fn byte(&self) -> u8 {
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
    fn byte(&self) -> u8 {
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
    fn byte(&self) -> u8 {
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
    fn byte(&self) -> u8 {
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
    fn byte(&self) -> u8 {
        self.int_cfg_m()
    }
    fn addr(&self) -> u8 {
        register::Mag::INT_CFG_M.addr()
    }
    fn sensor(&self) -> Sensor {
        Sensor::Magnetometer
    }
}

impl Configuration for Decimate {
    fn byte(&self) -> u8 {
        self.value()
    }
    fn addr(&self) -> u8 {
        register::AG::CTRL_REG5_XL.addr()
    }
    fn sensor(&self) -> Sensor {
        Sensor::Accelerometer
    }
}

impl AccelSettings {
    /// Returns `Configuration` to write to CTRL_REG5_XL (0x1F)
    pub fn ctrl_reg5_xl_config(&self) -> CustomConfiguration {
        CustomConfiguration {
            value: self.ctrl_reg5_xl(),
            sensor: Sensor::Accelerometer,
            register: register::AG::CTRL_REG5_XL.addr(),
        }
    }

    /// Returns `Configuration` to write to CTRL_REG6_XL (0x20)
    pub fn ctrl_reg6_xl_config(&self) -> CustomConfiguration {
        CustomConfiguration {
            value: self.ctrl_reg6_xl(),
            sensor: Sensor::Accelerometer,
            register: register::AG::CTRL_REG6_XL.addr(),
        }
    }

    /// Returns `Configuration` to write to CTRL_REG7_XL (0x21)
    pub fn ctrl_reg7_xl_config(&self) -> CustomConfiguration {
        CustomConfiguration {
            value: self.ctrl_reg7_xl(),
            sensor: Sensor::Accelerometer,
            register: register::AG::CTRL_REG7_XL.addr(),
        }
    }
}

impl GyroSettings {
    /// Returns `Configuration` to write to CTRL_REG1_G. See page 45
    pub fn ctrl_reg1_g_config(&self) -> CustomConfiguration {
        CustomConfiguration {
            value: self.ctrl_reg1_g(),
            sensor: Sensor::Gyro,
            register: register::AG::CTRL_REG1_G.addr(),
        }
    }
    /// Returns `Configuration` to write to CTRL_REG2_G. See page 47
    pub fn ctrl_reg2_g_config(&self) -> CustomConfiguration {
        CustomConfiguration {
            value: self.ctrl_reg2_g(),
            sensor: Sensor::Gyro,
            register: register::AG::CTRL_REG2_G.addr(),
        }
    }
    /// Returns `Configuration` to write to CTRL_REG3_G. See page 47
    pub fn ctrl_reg3_g_config(&self) -> CustomConfiguration {
        CustomConfiguration {
            value: self.ctrl_reg3_g(),
            sensor: Sensor::Gyro,
            register: register::AG::CTRL_REG3_G.addr(),
        }
    }
    /// Returns `Configuration` to write to CTRL_REG4. See page 50
    pub fn ctrl_reg4_config(&self) -> CustomConfiguration {
        CustomConfiguration {
            value: self.ctrl_reg4(),
            sensor: Sensor::Gyro,
            register: register::AG::CTRL_REG4.addr(),
        }
    }
}

impl MagSettings {
    /// Returns `Configuration` to write to CTRL_REG1_M. See page 63.
    pub fn ctrl_reg1_m_config(&self) -> CustomConfiguration {
        CustomConfiguration {
            value: self.ctrl_reg1_m(),
            sensor: Sensor::Magnetometer,
            register: register::Mag::CTRL_REG1_M.addr(),
        }
    }
    /// Returns `Configuration` to write to CTRL_REG2_M.
    pub fn ctrl_reg2_m_config(&self) -> CustomConfiguration {
        CustomConfiguration {
            value: self.ctrl_reg2_m(),
            sensor: Sensor::Magnetometer,
            register: register::Mag::CTRL_REG2_M.addr(),
        }
    }
    /// Returns `Configuration` to write to CTRL_REG3_M.
    pub fn ctrl_reg3_m_config(&self) -> CustomConfiguration {
        CustomConfiguration {
            value: self.ctrl_reg3_m(),
            sensor: Sensor::Magnetometer,
            register: register::Mag::CTRL_REG3_M.addr(),
        }
    }
    /// Returns `Configuration` to write to CTRL_REG4_M.
    pub fn ctrl_reg4_m_config(&self) -> CustomConfiguration {
        CustomConfiguration {
            value: self.ctrl_reg4_m(),
            sensor: Sensor::Magnetometer,
            register: register::Mag::CTRL_REG4_M.addr(),
        }
    }
    /// Returns `Configuration` to write to CTRL_REG5_M.
    pub fn ctrl_reg5_m_config(&self) -> CustomConfiguration {
        CustomConfiguration {
            value: self.ctrl_reg5_m(),
            sensor: Sensor::Magnetometer,
            register: register::Mag::CTRL_REG5_M.addr(),
        }
    }
}

impl FIFOConfig {
    /// Returns `Configuration` to be written to FIFO_CTRL.
    pub fn f_fifo_ctrl_config(&self) -> CustomConfiguration {
        CustomConfiguration {
            value: self.f_fifo_ctrl(),
            sensor: Sensor::Accelerometer,
            register: register::AG::FIFO_CTRL.addr(),
        }
    }

    pub fn f_ctrl_reg9_config(&self) -> CustomConfiguration {
        CustomConfiguration {
            sensor: Sensor::Accelerometer,
            register: register::AG::CTRL_REG9.addr(),
            value: self.f_ctrl_reg9(),
        }
    }
}
