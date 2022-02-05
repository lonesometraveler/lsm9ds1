/// Accelerometer interrupt generation settings
#[derive(Debug)]
pub struct IntConfigAccel {
    /// Combination of accelerometer's interrupt events
    pub events_combination: COMBINATION,
    /// Enable 6-direction detection 
    pub enable_6d: FLAG,
    /// Enable interrupt generation on X-axis high event
    pub interrupt_high_xaxis: FLAG,
    /// Enable interrupt generation on Y-axis high event
    pub interrupt_high_yaxis: FLAG,
    /// Enable interrupt generation on Z-axis high event
    pub interrupt_high_zaxis: FLAG,
    /// Enable interrupt generation on X-axis low event
    pub interrupt_low_xaxis: FLAG,
    /// Enable interrupt generation on Y-axis low event
    pub interrupt_low_yaxis: FLAG,
    /// Enable interrupt generation on Z-axis low event
    pub interrupt_low_zaxis: FLAG,        
    
}
impl Default for IntConfigAccel {
    fn default() -> Self {
        IntConfigAccel {         
            events_combination: COMBINATION::OR,
            enable_6d: FLAG::Disabled,
            interrupt_high_xaxis: FLAG::Disabled,            
            interrupt_high_yaxis: FLAG::Disabled,
            interrupt_high_zaxis: FLAG::Disabled,
            interrupt_low_xaxis: FLAG::Disabled,
            interrupt_low_yaxis: FLAG::Disabled,
            interrupt_low_zaxis: FLAG::Disabled,
        }
    }
}

impl IntConfigAccel {
    /// Returns values to be written to INT_GEN_CFG_XL:    
    fn int_gen_cfg_xl(&self) -> u8 {
        let mut data = 0u8;
        if self.events_combination.status() {
            data |= 1 << 7;
        }
        if self.enable_6d.status() {
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


#[allow(non_camel_case_types)]
    pub struct XL_INT_Bitmasks;

    #[allow(dead_code)]
    /// Bitmasks for interrupt-related settings in INT_GEN_SRC_XL register
    impl XL_INT_Bitmasks {    
        pub (crate) const IA_XL: u8 = 0b0100_0000;
        pub (crate) const ZH_XL: u8 = 0b0010_0000;
        pub (crate) const ZL_XL: u8 = 0b0001_0000;
        pub (crate) const YH_XL: u8 = 0b0000_1000;
        pub (crate) const YL_XL: u8 = 0b0000_0100;
        pub (crate) const XH_XL: u8 = 0b0000_0010;
        pub (crate) const XL_XL: u8 = 0b0000_0001;
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
