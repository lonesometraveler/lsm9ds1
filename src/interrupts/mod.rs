// SPLIT THE INTERRUPTS CODE INTO SMALLER MODULES: ACCEL, GYRO, MAG and PINS
use super::*;

pub mod accel_int;
pub mod gyro_int;
pub mod mag_int;
pub mod pins_config;


// Interrupt active setting for the INT_DRDY pin: active high (default) or active low
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
        let status = match self {
            INT_ACTIVE::High => false,
            INT_ACTIVE::Low => true,
        };
        status
    }
}
