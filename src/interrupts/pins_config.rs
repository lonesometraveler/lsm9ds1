//! Functions related to INT1_AG and INT2_AG interrupt pins configuration
///
use super::*;

/// Accelerometer/gyroscope interrupt pins common settings
#[derive(Debug)]
pub struct PinConfig {
    // --- CTRL_REG8 REGISTER ---
    /// Interrupt pin active level (default 0: active high)
    pub active_level: IntActive,
    /// Interrupt pin push-pull or open-drain configuration (default 0: push-pull)
    pub pin_mode: IntPin,
}

impl Default for PinConfig {
    fn default() -> Self {
        PinConfig {
            active_level: IntActive::High,
            pin_mode: IntPin::PushPull,
        }
    }
}

impl PinConfig {
    /// Returns values to be written to CTRL_REG8 register
    pub(crate) fn ctrl_reg8(&self) -> u8 {
        let mut data: u8 = 0;

        data |= self.active_level.value() << 5;
        data |= self.pin_mode.value() << 4;

        data
    }
}

/// Accelerometer/gyroscope interrupt pin (INT1_A/G) settings
#[derive(Debug)]
pub struct IntConfigAG1 {
    // --- INT1_CTRL REGISTER ---
    /// Enable gyroscope interrupt generation on pin INT1_A/G
    pub enable_gyro_int: Flag,
    /// Enable accelerometer interrupt generation on pin INT1_A/G
    pub enable_accel_int: Flag,
    /// Enable FSS5 interrupt on on pin INT1_A/G
    pub enable_fss5: Flag,
    /// Enable overrun interrupt on on pin INT1_A/G
    pub enable_overrun: Flag,
    /// Enable FIFO threshold interrupt on on pin INT1_A/G
    pub enable_fth: Flag,
    /// Enable boot status interrupt on on pin INT1_A/G
    pub enable_boot_status: Flag,
    /// Enable gyroscope data ready interrupt on on pin INT1_A/G
    pub enable_gyro_dataready: Flag,
    /// Enable accelerometer data ready interrupt on on pin INT1_A/G
    pub enable_accel_dataready: Flag,
}

impl Default for IntConfigAG1 {
    fn default() -> Self {
        IntConfigAG1 {
            enable_gyro_int: Flag::Disabled,
            enable_accel_int: Flag::Disabled,
            enable_fss5: Flag::Disabled,
            enable_overrun: Flag::Disabled,
            enable_fth: Flag::Disabled,
            enable_boot_status: Flag::Disabled,
            enable_gyro_dataready: Flag::Disabled,
            enable_accel_dataready: Flag::Disabled,
        }
    }
}

impl IntConfigAG1 {
    /// Returns values to be written to INT1_CTRL register
    pub(crate) fn int1_ctrl(&self) -> u8 {
        let mut data: u8 = 0;

        data |= self.enable_gyro_int.value() << 7;
        data |= self.enable_accel_int.value() << 6;
        data |= self.enable_fss5.value() << 5;
        data |= self.enable_overrun.value() << 4;
        data |= self.enable_fth.value() << 3;
        data |= self.enable_boot_status.value() << 2;
        data |= self.enable_gyro_dataready.value() << 1;
        data |= self.enable_accel_dataready.value();

        data
    }
}

/// Accelerometer/gyroscope interrupt pin (INT2_A/G) settings
#[derive(Debug)]
pub struct IntConfigAG2 {
    // --- INT2_CTRL REGISTER ---
    /// Enable FSS5 interrupt on on pin INT1_A/G
    pub enable_fss5: Flag,
    /// Enable overrun interrupt on on pin INT2_A/G
    pub enable_overrun: Flag,
    /// Enable FIFO threshold interrupt on on pin INT2_A/G
    pub enable_fth: Flag,
    /// Enable temperature data ready interrupt on on pin INT2_A/G
    pub enable_temp_dataready: Flag,
    /// Enable gyroscope data ready interrupt on on pin INT2_A/G
    pub enable_gyro_dataready: Flag,
    /// Enable accelerometer data ready interrupt on on pin INT2_A/G
    pub enable_accel_dataready: Flag,
}

impl Default for IntConfigAG2 {
    fn default() -> Self {
        IntConfigAG2 {
            enable_fss5: Flag::Disabled,
            enable_overrun: Flag::Disabled,
            enable_fth: Flag::Disabled,
            enable_temp_dataready: Flag::Disabled,
            enable_gyro_dataready: Flag::Disabled,
            enable_accel_dataready: Flag::Disabled,
        }
    }
}

impl IntConfigAG2 {
    /// Returns values to be written to INT2_CTRL register
    pub(crate) fn int2_ctrl(&self) -> u8 {
        let mut data: u8 = 0;

        data |= self.enable_fss5.value() << 5;
        data |= self.enable_overrun.value() << 4;
        data |= self.enable_fth.value() << 3;
        data |= self.enable_temp_dataready.value() << 2;
        data |= self.enable_gyro_dataready.value() << 1;
        data |= self.enable_accel_dataready.value();

        data
    }
}

#[test]
fn configure_ag1() {
    let config = IntConfigAG1::default();
    assert_eq!(config.int1_ctrl(), 0b0000_0000);

    let config = IntConfigAG1 {
        enable_gyro_int: Flag::Enabled,
        enable_accel_int: Flag::Enabled,
        enable_fss5: Flag::Enabled,
        enable_overrun: Flag::Enabled,
        enable_fth: Flag::Enabled,
        enable_boot_status: Flag::Enabled,
        enable_gyro_dataready: Flag::Enabled,
        enable_accel_dataready: Flag::Enabled,
    };
    assert_eq!(config.int1_ctrl(), 0b1111_1111);
}

#[test]
fn configure_ag2() {
    let config = IntConfigAG2::default();
    assert_eq!(config.int2_ctrl(), 0b0000_0000);

    let config = IntConfigAG2 {
        enable_fss5: Flag::Enabled,
        enable_overrun: Flag::Enabled,
        enable_fth: Flag::Enabled,
        enable_temp_dataready: Flag::Enabled,
        enable_gyro_dataready: Flag::Enabled,
        enable_accel_dataready: Flag::Enabled,
    };
    assert_eq!(config.int2_ctrl(), 0b0011_1111);
}

#[test]
fn configure_pins() {
    let config = PinConfig::default();
    assert_eq!(config.ctrl_reg8(), 0b0000_0000);

    let config = PinConfig {
        active_level: IntActive::Low,
        pin_mode: IntPin::OpenDrain,
    };
    assert_eq!(config.ctrl_reg8(), 0b0011_0000);
}
