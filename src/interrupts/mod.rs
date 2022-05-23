// SPLIT THE INTERRUPTS CODE INTO SMALLER MODULES: ACCEL, GYRO, MAG and PINS

use super::*;

pub mod accel_int;
pub mod gyro_int;
pub mod mag_int;
pub mod pins_config;

/// Interrupt active setting for the INT_DRDY pin: active high (default) or active low
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy)]
pub enum INT_ACTIVE {
    /// Active high
    High,
    /// Active low
    Low,
}

impl INT_ACTIVE {
    pub fn status(self) -> bool {
        match self {
            INT_ACTIVE::High => false,
            INT_ACTIVE::Low => true,
        }
    }
}

/// Interrupt pad setting for INT_DRDY pin: push-pull (default) or open-drain.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy)]
pub enum INT_PIN {
    /// Push-pull
    PushPull,
    /// Open drain
    OpenDrain,
}

impl INT_PIN {
    pub fn status(self) -> bool {
        match self {
            INT_PIN::PushPull => false,
            INT_PIN::OpenDrain => true,
        }
    }
}


/// Interrupt latching setting (interrupt request latched or not latched)
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy)]
pub enum INT_LATCH {
    /// Interrupt request latched
    Latched,
    /// Interrupt request not latched
    NotLatched,
}

impl INT_LATCH {
    pub fn status(self) -> bool {
        match self {
            INT_LATCH::Latched => true,
            INT_LATCH::NotLatched => false,
        }
    }
}


/// 6D or 4D used by interrupt generator for for position recognition
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy)]
pub enum POS_RECOG {
    /// 4D option used for position recognition
    _4D,
    /// 4D option used for position recognition
    _6D,
}

impl POS_RECOG {
    pub fn status(self) -> bool {
        match self {
            POS_RECOG::_4D => true,
            POS_RECOG::_6D => false,
        }
    }
}

/// Decrement or reset counter mode selection.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy)]
pub enum COUNTER {
    /// Decrement counter (see pages 58-61)
    Decrement,
    /// Reset counter
    Reset,
}

impl COUNTER {
    pub fn status(self) -> bool {
        match self {
            COUNTER::Decrement => true,
            COUNTER::Reset => false,
        }
    }
}