use super::*;

    /// Gyroscope interrupt generator settings
    #[derive(Debug)]
    pub struct IntConfigGyro {
        /// Combination of gyroscope interrupt events
        pub events_combination: COMBINATION,        
        /// Latch interrupt request
        pub latch_interrupts: FLAG,
        /// Enable interrupt generation on X-axis (pitch) high event
        pub interrupt_high_xaxis: FLAG,
        /// Enable interrupt generation on X-axis (pitch) low event
        pub interrupt_low_xaxis: FLAG,
        /// Enable interrupt generation on Y-axis (roll) high event
        pub interrupt_high_yaxis: FLAG,
        /// Enable interrupt generation on Y-axis (roll) low event
        pub interrupt_low_yaxis: FLAG,
        /// Enable interrupt generation on Z-axis (yaw) high event
        pub interrupt_high_zaxis: FLAG,
        /// Enable interrupt generation on Z-axis (yaw) low event
        pub interrupt_low_zaxis: FLAG,
    }

    impl Default for IntConfigGyro {
        fn default() -> Self {
            IntConfigGyro {         
                events_combination: COMBINATION::OR,
                latch_interrupts: FLAG::Disabled,
                interrupt_high_xaxis: FLAG::Disabled,            
                interrupt_high_yaxis: FLAG::Disabled,
                interrupt_high_zaxis: FLAG::Disabled,
                interrupt_low_xaxis: FLAG::Disabled,
                interrupt_low_yaxis: FLAG::Disabled,
                interrupt_low_zaxis: FLAG::Disabled,
            }
        }
    }
    
    impl IntConfigGyro {
        /// Returns values to be written to INT_GEN_CFG_G:    
        fn int_gen_cfg_g(&self) -> u8 {
            let mut data = 0u8;
            if self.events_combination.status() {
                data |= 1 << 7;
            }
            if self.latch_interrupts.status() {
                data |= 1 << 6;
            }
            if self.interrupt_high_zaxis.status() {
                data |= 1 << 5;
            }
            if self.interrupt_low_zaxis.status() {
                data |= 1 << 4;
            }
            if self.interrupt_high_yaxis.status() {
                data |= 1 << 3;
            }
            if self.interrupt_low_yaxis.status() {
                data |= 1 << 2;
            }
            if self.interrupt_high_xaxis.status() {
                data |= 1 << 1;
            }
            if self.interrupt_low_xaxis.status() {
                data |= 1;
            }
            data
        }
    }

// --- GYROSCOPE INTERRUPT STATUS
    // INT_GEN_SRC_G
    // needs a struct with 7 fields

    #[allow(non_camel_case_types)]
    pub struct G_INT_Bitmasks;

    #[allow(dead_code)]
    /// Bitmasks for interrupt-related settings in INT_GEN_SRC_G register
    impl G_INT_Bitmasks {    
        pub (crate) const IA_G: u8 = 0b0100_0000;
        pub (crate) const ZH_G: u8 = 0b0010_0000;
        pub (crate) const ZL_G: u8 = 0b0001_0000;
        pub (crate) const YH_G: u8 = 0b0000_1000;
        pub (crate) const YL_G: u8 = 0b0000_0100;
        pub (crate) const XH_G: u8 = 0b0000_0010;
        pub (crate) const XL_G: u8 = 0b0000_0001;
    }

    #[derive(Debug)]
    /// Contents of the INT_GEN_SRC_G register (interrupt active and differential pressure events flags)
    pub struct IntStatusGyro {
        pub interrupt_active: bool,
        pub xaxis_high_event: bool,
        pub xaxis_low_event: bool,
        pub yaxis_high_event: bool,
        pub yaxis_low_event: bool,
        pub zaxis_high_event: bool,
        pub zaxis_low_event: bool, 
    }
