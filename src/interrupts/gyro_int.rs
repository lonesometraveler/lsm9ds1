/// Functions related to gyroscope-specific interrupts
///
/// TO DO:
/// - complete gyroscope threshold setting for X, Y and Z axis (INT_GEN_THS_X/Y/Z_G)
/// - ORIENT_CFG_G settings (user orientation selection (???)) -> to be done in gyro.rs
///
use super::*;

/// Gyroscope interrupt generator settings
#[derive(Debug)]
pub struct IntConfigGyro {
    /// Combination of gyroscope interrupt events
    pub events_combination: Combination,
    /// Latch interrupt request
    pub latch_interrupts: IntLatch,
    /// Enable interrupt generation on Z-axis (yaw) high event
    pub interrupt_high_zaxis: Flag,
    /// Enable interrupt generation on Z-axis (yaw) low event
    pub interrupt_low_zaxis: Flag,
    /// Enable interrupt generation on Y-axis (roll) high event
    pub interrupt_high_yaxis: Flag,
    /// Enable interrupt generation on Y-axis (roll) low event
    pub interrupt_low_yaxis: Flag,
    /// Enable interrupt generation on X-axis (pitch) high event
    pub interrupt_high_xaxis: Flag,
    /// Enable interrupt generation on X-axis (pitch) low event
    pub interrupt_low_xaxis: Flag,
}

impl Default for IntConfigGyro {
    fn default() -> Self {
        IntConfigGyro {
            events_combination: Combination::Or,
            latch_interrupts: IntLatch::NotLatched,
            interrupt_high_zaxis: Flag::Disabled,
            interrupt_low_zaxis: Flag::Disabled,
            interrupt_high_yaxis: Flag::Disabled,
            interrupt_low_yaxis: Flag::Disabled,
            interrupt_high_xaxis: Flag::Disabled,
            interrupt_low_xaxis: Flag::Disabled,
        }
    }
}

impl IntConfigGyro {
    /// Returns values to be written to INT_GEN_CFG_G:    
    pub fn int_gen_cfg_g(&self) -> u8 {
        let mut data = 0u8;
        data |= self.events_combination.value() << 7;
        data |= self.latch_interrupts.value() << 6;
        data |= self.interrupt_high_zaxis.value() << 5;
        data |= self.interrupt_low_zaxis.value() << 4;
        data |= self.interrupt_high_yaxis.value() << 3;
        data |= self.interrupt_low_yaxis.value() << 2;
        data |= self.interrupt_high_xaxis.value() << 1;
        data |= self.interrupt_low_xaxis.value();
        data
    }
}

impl From<u8> for IntConfigGyro {
    fn from(reg_value: u8) -> Self {
        IntConfigGyro {
            events_combination: match (reg_value & CfgBitmasks::AOI_G) >> 7 {
                1 => Combination::And,
                _ => Combination::Or,
            },
            latch_interrupts: match (reg_value & CfgBitmasks::LIR_G) >> 6 {
                1 => IntLatch::Latched,
                _ => IntLatch::NotLatched,
            },
            interrupt_high_zaxis: match (reg_value & CfgBitmasks::ZHIE_G) >> 5 {
                1 => Flag::Enabled,
                _ => Flag::Disabled,
            },
            interrupt_low_zaxis: match (reg_value & CfgBitmasks::ZLIE_G) >> 4 {
                1 => Flag::Enabled,
                _ => Flag::Disabled,
            },
            interrupt_high_yaxis: match (reg_value & CfgBitmasks::YHIE_G) >> 3 {
                1 => Flag::Enabled,
                _ => Flag::Disabled,
            },
            interrupt_low_yaxis: match (reg_value & CfgBitmasks::YLIE_G) >> 2 {
                1 => Flag::Enabled,
                _ => Flag::Disabled,
            },
            interrupt_high_xaxis: match (reg_value & CfgBitmasks::XHIE_G) >> 1 {
                1 => Flag::Enabled,
                _ => Flag::Disabled,
            },
            interrupt_low_xaxis: match reg_value & CfgBitmasks::XLIE_G {
                1 => Flag::Enabled,
                _ => Flag::Disabled,
            },
        }
    }
}
pub struct InterruptBitmasks;

#[allow(dead_code)]
/// Bitmasks for interrupt-related settings in INT_GEN_SRC_G register
impl InterruptBitmasks {
    pub(crate) const IA_G: u8 = 0b0100_0000;
    pub(crate) const ZH_G: u8 = 0b0010_0000;
    pub(crate) const ZL_G: u8 = 0b0001_0000;
    pub(crate) const YH_G: u8 = 0b0000_1000;
    pub(crate) const YL_G: u8 = 0b0000_0100;
    pub(crate) const XH_G: u8 = 0b0000_0010;
    pub(crate) const XL_G: u8 = 0b0000_0001;
}
pub struct CfgBitmasks;
#[allow(dead_code)]
/// Bitmasks for interrupt-related settings in INT_GEN_CFG_G register
impl CfgBitmasks {
    pub(crate) const AOI_G: u8 = 0b1000_0000;
    pub(crate) const LIR_G: u8 = 0b0100_0000;
    pub(crate) const ZHIE_G: u8 = 0b0010_0000;
    pub(crate) const ZLIE_G: u8 = 0b0001_0000;
    pub(crate) const YHIE_G: u8 = 0b0000_1000;
    pub(crate) const YLIE_G: u8 = 0b0000_0100;
    pub(crate) const XHIE_G: u8 = 0b0000_0010;
    pub(crate) const XLIE_G: u8 = 0b0000_0001;
}

#[derive(Debug)]
/// Contents of the INT_GEN_SRC_G register (interrupt active and differential pressure events Flags)
pub struct IntStatusGyro {
    pub interrupt_active: bool,
    pub xaxis_high_event: bool,
    pub xaxis_low_event: bool,
    pub yaxis_high_event: bool,
    pub yaxis_low_event: bool,
    pub zaxis_high_event: bool,
    pub zaxis_low_event: bool,
}

#[test]
fn configure_gyro_int() {
    let config = IntConfigGyro::default();
    assert_eq!(config.int_gen_cfg_g(), 0b0000_0000);

    let config = IntConfigGyro {
        events_combination: Combination::And,
        latch_interrupts: IntLatch::Latched,
        interrupt_high_xaxis: Flag::Enabled,
        interrupt_high_yaxis: Flag::Enabled,
        interrupt_high_zaxis: Flag::Enabled,
        interrupt_low_xaxis: Flag::Enabled,
        interrupt_low_yaxis: Flag::Enabled,
        interrupt_low_zaxis: Flag::Enabled,
    };
    assert_eq!(config.int_gen_cfg_g(), 0b1111_1111);
}
