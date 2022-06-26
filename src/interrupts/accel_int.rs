//! Functions related to accelerometer-specific interrupts
///
/// TO DO:
/// - set acceleration threshold for X, Y and Z axis (INT_GEN_THS_X/Y/Z_XL) in mg instead?
/// - LIR_XL1 and 4D_XL1 bits of CTRL_REG4 => should they be incorporated in the Config struct? what's the relation between 4D_XL1 and _6D?
///
use super::*;

/// Accelerometer interrupt generation settings
#[derive(Debug)]
pub struct IntConfigAccel {
    /// Combination of accelerometer's interrupt events
    pub events_combination: Combination,
    /// Enable 6-direction detection
    pub enable_6d: Flag,
    /// Enable interrupt generation on Z-axis high event
    pub interrupt_zaxis_high: Flag,
    /// Enable interrupt generation on Z-axis low event
    pub interrupt_zaxis_low: Flag,
    /// Enable interrupt generation on Y-axis high event
    pub interrupt_yaxis_high: Flag,
    /// Enable interrupt generation on Y-axis low event
    pub interrupt_yaxis_low: Flag,
    /// Enable interrupt generation on X-axis high event
    pub interrupt_xaxis_high: Flag,
    /// Enable interrupt generation on X-axis low event
    pub interrupt_xaxis_low: Flag,
}
impl Default for IntConfigAccel {
    fn default() -> Self {
        IntConfigAccel {
            events_combination: Combination::Or,
            enable_6d: Flag::Disabled,
            interrupt_zaxis_high: Flag::Disabled,
            interrupt_zaxis_low: Flag::Disabled,
            interrupt_yaxis_high: Flag::Disabled,
            interrupt_yaxis_low: Flag::Disabled,
            interrupt_xaxis_high: Flag::Disabled,
            interrupt_xaxis_low: Flag::Disabled,
        }
    }
}

impl IntConfigAccel {
    /// Returns values to be written to INT_GEN_CFG_XL:    
    pub(crate) fn int_gen_cfg_xl(&self) -> u8 {
        let mut data = 0u8;
        data |= self.events_combination.value() << 7;
        data |= self.enable_6d.value() << 6;
        data |= self.interrupt_zaxis_high.value() << 5;
        data |= self.interrupt_zaxis_low.value() << 4;
        data |= self.interrupt_yaxis_high.value() << 3;
        data |= self.interrupt_yaxis_low.value() << 2;
        data |= self.interrupt_xaxis_high.value() << 1;
        data |= self.interrupt_xaxis_low.value();
        data
    }
}

impl From<u8> for IntConfigAccel {
    fn from(reg_value: u8) -> Self {
        IntConfigAccel {
            events_combination: match (reg_value & CfgBitmasks::AOI_XL) >> 7 {
                1 => Combination::And,
                _ => Combination::Or,
            },
            enable_6d: match (reg_value & CfgBitmasks::_6D) >> 6 {
                1 => Flag::Enabled,
                _ => Flag::Disabled,
            },
            interrupt_zaxis_high: match (reg_value & CfgBitmasks::ZHIE_XL) >> 5 {
                1 => Flag::Enabled,
                _ => Flag::Disabled,
            },
            interrupt_zaxis_low: match (reg_value & CfgBitmasks::ZLIE_XL) >> 4 {
                1 => Flag::Enabled,
                _ => Flag::Disabled,
            },
            interrupt_yaxis_high: match (reg_value & CfgBitmasks::YHIE_XL) >> 3 {
                1 => Flag::Enabled,
                _ => Flag::Disabled,
            },
            interrupt_yaxis_low: match (reg_value & CfgBitmasks::XLIE_XL) >> 2 {
                1 => Flag::Enabled,
                _ => Flag::Disabled,
            },
            interrupt_xaxis_high: match (reg_value & CfgBitmasks::XHIE_XL) >> 1 {
                1 => Flag::Enabled,
                _ => Flag::Disabled,
            },
            interrupt_xaxis_low: match reg_value & CfgBitmasks::XLIE_XL {
                1 => Flag::Enabled,
                _ => Flag::Disabled,
            },
        }
    }
}

/// Bitmasks for interrupt-related settings in INT_GEN_SRC_XL register
pub struct InterruptBitmasks;
#[allow(dead_code)]
impl InterruptBitmasks {
    pub const IA_XL: u8 = 0b0100_0000;
    pub const ZH_XL: u8 = 0b0010_0000;
    pub const ZL_XL: u8 = 0b0001_0000;
    pub const YH_XL: u8 = 0b0000_1000;
    pub const YL_XL: u8 = 0b0000_0100;
    pub const XH_XL: u8 = 0b0000_0010;
    pub const XL_XL: u8 = 0b0000_0001;
}

/// Bitmasks for interrupt-related settings in INT_GEN_CFG_XL register
pub struct CfgBitmasks;
#[allow(dead_code)]
impl CfgBitmasks {
    pub const AOI_XL: u8 = 0b1000_0000;
    pub const _6D: u8 = 0b0100_0000;
    pub const ZHIE_XL: u8 = 0b0010_0000;
    pub const ZLIE_XL: u8 = 0b0001_0000;
    pub const YHIE_XL: u8 = 0b0000_1000;
    pub const YLIE_XL: u8 = 0b0000_0100;
    pub const XHIE_XL: u8 = 0b0000_0010;
    pub const XLIE_XL: u8 = 0b0000_0001;

    pub const LIR_XL1: u8 = 0b0000_0010;
    pub const _4D_XL1: u8 = 0b0000_0001;
}

#[derive(Debug)]
/// Contents of the INT_GEN_SRC_XL register (interrupt active and differential pressure events flags)
pub struct IntStatusAccel {
    pub interrupt_active: bool,
    pub xaxis_high_event: bool,
    pub xaxis_low_event: bool,
    pub yaxis_high_event: bool,
    pub yaxis_low_event: bool,
    pub zaxis_high_event: bool,
    pub zaxis_low_event: bool,
}

#[test]
fn configure_accel_int() {
    let config = IntConfigAccel::default();
    assert_eq!(config.int_gen_cfg_xl(), 0b0000_0000);

    let config = IntConfigAccel {
        events_combination: Combination::And,
        enable_6d: Flag::Enabled,
        interrupt_zaxis_high: Flag::Enabled,
        interrupt_zaxis_low: Flag::Enabled,
        interrupt_yaxis_high: Flag::Enabled,
        interrupt_yaxis_low: Flag::Enabled,
        interrupt_xaxis_high: Flag::Enabled,
        interrupt_xaxis_low: Flag::Enabled,
    };
    assert_eq!(config.int_gen_cfg_xl(), 0b1111_1111);

    let config = IntConfigAccel {
        interrupt_zaxis_high: Flag::Enabled,
        interrupt_xaxis_low: Flag::Enabled,
        ..Default::default()
    };
    assert_eq!(config.int_gen_cfg_xl(), 0b0010_0001);
}
