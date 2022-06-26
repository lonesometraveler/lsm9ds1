//! Functions related to magnetometer-specific interrupts
use super::*;

/// Magnetometer interrupt pin (INT_M) settings
#[derive(Debug)]
pub struct IntConfigMag {
    /// Enable interrupt generation on X-axis, default 0
    pub interrupt_xaxis: Flag,
    /// Enable interrupt generation on Y-axis, default 0
    pub interrupt_yaxis: Flag,
    /// Enable interrupt generation on Z-axis, default 0
    pub interrupt_zaxis: Flag,
    /// Configure interrupt pin INT_M as active high or active low, default 0 (low)
    pub active_high_or_low: IntActive,
    /// Latch interrupt request (Once latched, the INT_M pin remains in the same state until INT_SRC_M is read), default 0 (latched)
    pub interrupt_latching: IntLatch,
    /// Interrupt enable on the INT_M pin, default 0
    pub enable_interrupt: Flag,
}

impl Default for IntConfigMag {
    fn default() -> Self {
        IntConfigMag {
            interrupt_xaxis: Flag::Disabled,
            interrupt_yaxis: Flag::Disabled,
            interrupt_zaxis: Flag::Disabled,
            active_high_or_low: IntActive::Low,       // reversed!
            interrupt_latching: IntLatch::NotLatched, // NOTE: it's reversed, 0 is latched (default)
            enable_interrupt: Flag::Disabled,
        }
    }
}

impl IntConfigMag {
    /// Returns values to be written to INT_CFG_M:    
    pub fn int_cfg_m(&self) -> u8 {
        let mut data = 0u8;
        data |= self.interrupt_xaxis.value() << 7;
        data |= self.interrupt_yaxis.value() << 6;
        data |= self.interrupt_zaxis.value() << 5;
        data |= match self.active_high_or_low {
            IntActive::High => 1,
            IntActive::Low => 0,
        } << 2;
        data |= match self.interrupt_latching {
            IntLatch::Latched => 0,
            IntLatch::NotLatched => 1,
        } << 1;
        data |= self.enable_interrupt.value();
        data
    }
}

impl From<u8> for IntConfigMag {
    fn from(reg_value: u8) -> Self {
        IntConfigMag {
            interrupt_xaxis: match (reg_value & CfgBitmasks::XIEN) >> 7 {
                1 => Flag::Enabled,
                _ => Flag::Disabled,
            },
            interrupt_yaxis: match (reg_value & CfgBitmasks::YIEN) >> 6 {
                1 => Flag::Enabled,
                _ => Flag::Disabled,
            },
            interrupt_zaxis: match (reg_value & CfgBitmasks::ZIEN) >> 5 {
                1 => Flag::Enabled,
                _ => Flag::Disabled,
            },
            active_high_or_low: match (reg_value & CfgBitmasks::IEA) >> 2 {
                1 => IntActive::High,
                _ => IntActive::Low,
            },
            interrupt_latching: match (reg_value & CfgBitmasks::IEL) >> 1 {
                1 => IntLatch::NotLatched,
                _ => IntLatch::Latched, // NOTE: it's reversed, 0 is latched
            },
            enable_interrupt: match reg_value & CfgBitmasks::IEN {
                1 => Flag::Enabled,
                _ => Flag::Disabled,
            },
        }
    }
}

/// Bitmasks for interrupt-related settings in INT_SRC_M register
pub struct InterruptBitmasks;

#[allow(dead_code)]
impl InterruptBitmasks {
    pub const PTH_X: u8 = 0b1000_0000;
    pub const PTH_Y: u8 = 0b0100_0000;
    pub const PTH_Z: u8 = 0b0010_0000;
    pub const NTH_X: u8 = 0b0001_0000;
    pub const NTH_Y: u8 = 0b0000_1000;
    pub const NTH_Z: u8 = 0b0000_0100;
    pub const MROI: u8 = 0b0000_0010;
    pub const INT: u8 = 0b0000_0001;
}

/// Bitmasks for interrupt-related settings in INT_CFG_M register
pub struct CfgBitmasks;

#[allow(dead_code)]
impl CfgBitmasks {
    pub const XIEN: u8 = 0b1000_0000;
    pub const YIEN: u8 = 0b0100_0000;
    pub const ZIEN: u8 = 0b0010_0000;
    pub const IEA: u8 = 0b0000_0100;
    pub const IEL: u8 = 0b0000_0010;
    pub const IEN: u8 = 0b0000_0001;
}

#[derive(Debug)]
/// Contents of the INT_SRC_M register (interrupt active and threshold excess events Flags)
pub struct IntStatusMag {
    pub xaxis_exceeds_thresh_pos: bool,
    pub yaxis_exceeds_thresh_pos: bool,
    pub zaxis_exceeds_thresh_pos: bool,
    pub xaxis_exceeds_thresh_neg: bool,
    pub yaxis_exceeds_thresh_neg: bool,
    pub zaxis_exceeds_thresh_neg: bool,
    pub measurement_range_overflow: bool,
    pub interrupt_occurs: bool,
}

#[test]
fn configure_mag_int() {
    let config = IntConfigMag::default();
    assert_eq!(config.int_cfg_m(), 0b0000_0010);

    let config = IntConfigMag {
        interrupt_xaxis: Flag::Enabled,
        interrupt_yaxis: Flag::Enabled,
        interrupt_zaxis: Flag::Enabled,
        active_high_or_low: IntActive::High,
        interrupt_latching: IntLatch::Latched,
        enable_interrupt: Flag::Enabled,
    };
    assert_eq!(config.int_cfg_m(), 0b1110_0101);
}
