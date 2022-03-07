use super::*;

/// Magnetometer interrupt pin (INT_M) settings
#[derive(Debug)]
pub struct IntConfigMag {
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

impl Default for IntConfigMag {
    fn default() -> Self {
        IntConfigMag {                    
            interrupt_xaxis: FLAG::Disabled,            
            interrupt_yaxis: FLAG::Disabled,            
            interrupt_zaxis: FLAG::Disabled,            
            active_high_or_low: INT_ACTIVE::Low,            
            interrupt_latching: FLAG::Enabled,            
            enable_interrupt: FLAG::Disabled,            
        }
    }
}

impl IntConfigMag {
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

