/// Functions related to interrupt pins configuration
/// 
/// TO DO: 
/// - add functions to set these configurations (write to registers)
/// - add getters as well?
use super::*;

// --- A/G PINS CONFIGURATION ---

/// Accelerometer/gyroscope interrupt pin (INT1_A/G) settings
#[derive(Debug)]
pub struct IntConfigAG1 {
    // --- INT1_CTRL REGISTER ---
    /// Enable gyroscope interrupt generation on pin INT1_A/G
    pub enable_gyro_int: FLAG,
    /// Enable accelerometer interrupt generation on pin INT1_A/G
    pub enable_accel_int: FLAG,
    /// Enable FSS5 interrupt on on pin INT1_A/G
    pub enable_fss5: FLAG,
    /// Enable overrun interrupt on on pin INT1_A/G
    pub enable_overrun: FLAG,
    /// Enable FIFO threshold interrupt on on pin INT1_A/G
    pub enable_fth: FLAG,
    /// Enable boot status interrupt on on pin INT1_A/G
    pub enable_boot_status: FLAG,
    /// Enable gyroscope data ready interrupt on on pin INT1_A/G
    pub enable_gyro_dataready: FLAG,
    /// Enable accelerometer data ready interrupt on on pin INT1_A/G
    pub enable_accel_dataready: FLAG,
}

impl Default for IntConfigAG1 {
    fn default() -> Self {
        IntConfigAG1 {
            enable_gyro_int: FLAG::Disabled,
            enable_accel_int: FLAG::Disabled,
            enable_fss5: FLAG::Disabled,
            enable_overrun: FLAG::Disabled,
            enable_fth: FLAG::Disabled,
            enable_boot_status: FLAG::Disabled,
            enable_gyro_dataready: FLAG::Disabled,
            enable_accel_dataready: FLAG::Disabled,
        }
    }
}

impl IntConfigAG1 {
    /// Returns values to be written to INT1_CTRL register
    fn int1_ctrl(&self) -> u8 {
        let mut data: u8 = 0;
        if self.enable_gyro_int.status() {
            data |= 1 << 7;
        }
        if self.enable_accel_int.status() {
            data |= 1 << 6;
        }
        if self.enable_fss5.status() {
            data |= 1 << 5;
        }
        if self.enable_overrun.status() {
            data |= 1 << 4;
        }
        if self.enable_fth.status() {
            data |= 1 << 3;
        }
        if self.enable_boot_status.status() {
            data |= 1 << 2;
        }
        if self.enable_gyro_dataready.status() {
            data |= 1 << 1;
        }
        if self.enable_accel_dataready.status() {
            data |= 1;
        }
        data
    }
}

/// Accelerometer/gyroscope interrupt pin (INT2_A/G) settings
#[derive(Debug)]
pub struct IntConfigAG2 {
    // --- INT2_CTRL REGISTER ---
    /// Enable FSS5 interrupt on on pin INT1_A/G
    pub enable_fss5: FLAG,
    /// Enable overrun interrupt on on pin INT2_A/G
    pub enable_overrun: FLAG,
    /// Enable FIFO threshold interrupt on on pin INT2_A/G
    pub enable_fth: FLAG,
    /// Enable temperature data ready interrupt on on pin INT2_A/G
    pub enable_temp_dataready: FLAG,
    /// Enable gyroscope data ready interrupt on on pin INT2_A/G
    pub enable_gyro_dataready: FLAG,
    /// Enable accelerometer data ready interrupt on on pin INT2_A/G
    pub enable_accel_dataready: FLAG,
}

impl Default for IntConfigAG2 {
    fn default() -> Self {
        IntConfigAG2 {
            enable_fss5: FLAG::Disabled,
            enable_overrun: FLAG::Disabled,
            enable_fth: FLAG::Disabled,
            enable_temp_dataready: FLAG::Disabled,
            enable_gyro_dataready: FLAG::Disabled,
            enable_accel_dataready: FLAG::Disabled,
        }
    }
}

impl IntConfigAG2 {
    /// Returns values to be written to INT2_CTRL register
    fn int2_ctrl(&self) -> u8 {
        let mut data: u8 = 0;

        if self.enable_fss5.status() {
            data |= 1 << 5;
        }
        if self.enable_overrun.status() {
            data |= 1 << 4;
        }
        if self.enable_fth.status() {
            data |= 1 << 3;
        }
        if self.enable_temp_dataready.status() {
            data |= 1 << 2;
        }
        if self.enable_gyro_dataready.status() {
            data |= 1 << 1;
        }
        if self.enable_accel_dataready.status() {
            data |= 1;
        }
        data
    }
}

/// Magnetometer interrupt pin (INT_M) settings
#[derive(Debug)]
pub struct IntConfigM {
    /// Enable interrupt generation on X-axis
    pub interrupt_xaxis: FLAG,
    /// Enable interrupt generation on Y-axis
    pub interrupt_yaxis: FLAG,
    /// Enable interrupt generation on Z-axis
    pub interrupt_zaxis: FLAG,
    /// Configure interrupt pin INT_M as active high or active low
    pub active_high_or_low: INT_ACTIVE,
    /// Latch interrupt request (Once latched, the INT_M pin remains in the same state until INT_SRC_M is read)
    pub interrupt_latching: FLAG,
    /// Interrupt enable on the INT_M pin
    pub enable_interrupt: FLAG,
}

impl Default for IntConfigM {
    fn default() -> Self {
        IntConfigM {
            interrupt_xaxis: FLAG::Disabled,
            interrupt_yaxis: FLAG::Disabled,
            interrupt_zaxis: FLAG::Disabled,
            active_high_or_low: INT_ACTIVE::Low,
            interrupt_latching: FLAG::Enabled,
            enable_interrupt: FLAG::Disabled,
        }
    }
}

impl IntConfigM {
    /// Returns values to be written to INT_CFG_M:    
    fn int_cfg_m(&self) -> u8 {
        let mut data = 0u8;
        if self.interrupt_xaxis.status() {
            data |= 1 << 7;
        }
        if self.interrupt_yaxis.status() {
            data |= 1 << 6;
        }
        if self.interrupt_zaxis.status() {
            data |= 1 << 5;
        }
        if self.active_high_or_low.status() {
            data |= 1 << 2;
        }
        if self.interrupt_latching.status() {
            data |= 1 << 1;
        }
        if self.enable_interrupt.status() {
            data |= 1;
        }
        data
    }
}

/*
   // WHICH SENSOR SHOULD I USE HERE? IT'S BOTH ACCEL AND GYRO!

    /// Enable interrupts for accelerometer/gyroscope and configure the INT1_A/G interrupt pin
    pub fn configure_interrupts_ag1(&mut self, config: IntConfigAG1) -> Result<(), T::Error> {
        self.interface.write(Sensor::Accelerometer, register::AG::INT1_CTRL.addr(), config.int1_ctrl())?;
        Ok(())
    }



    // WHICH SENSOR SHOULD I USE HERE? IT'S BOTH ACCEL AND GYRO!

    /// Enable interrupts for accelerometer/gyroscope and configure the INT1_A/G interrupt pin
    pub fn configure_interrupts_ag2(&mut self, config: IntConfigAG2) -> Result<(), T::Error> {

        let reg_data = self.read_register(Sensor::Accelerometer, register::AG::INT2_CTRL.addr())?;

        let mut data: u8 = reg_data & !0b1100_0000;

        data |= config.int2_ctrl();

        self.interface.write(Sensor::Accelerometer, register::AG::INT2_CTRL.addr(), data)?;
        Ok(())
    }

*/
