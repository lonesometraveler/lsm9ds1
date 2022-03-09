/// Functions related to gyroscope-specific interrupts
use super::*;

/// Gyroscope interrupt generator settings
#[derive(Debug)]
pub struct IntConfigGyro {
    /// Combination of gyroscope interrupt events
    pub events_combination: COMBINATION,
    /// Latch interrupt request
    pub latch_interrupts: INT_LATCH,
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
            latch_interrupts: INT_LATCH::NotLatched,
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
    pub(crate) const IA_G: u8 = 0b0100_0000;
    pub(crate) const ZH_G: u8 = 0b0010_0000;
    pub(crate) const ZL_G: u8 = 0b0001_0000;
    pub(crate) const YH_G: u8 = 0b0000_1000;
    pub(crate) const YL_G: u8 = 0b0000_0100;
    pub(crate) const XH_G: u8 = 0b0000_0010;
    pub(crate) const XL_G: u8 = 0b0000_0001;
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


impl<T> LSM9DS1<T>
where
    T: Interface,
    {        
    // Angular rate sensor interrupt generator configuration register.
    //
    // AND/OR combination of gyroscope’s interrupt events.
    // Latch Gyroscope interrupt request.
    // enable interrupt generation for high / low events on X, Y, Z axis

    /// Enable and configure interrupts for gyroscope
    pub fn configure_interrupts_gyro(&mut self, config: IntConfigGyro) -> Result<(), T::Error> {
        self.interface.write(Sensor::Gyro, register::AG::INT_GEN_CFG_G.addr(), config.int_gen_cfg_g())?;                
        Ok(())
    }

    /// Get all the flags from the INT_GEN_SRC_G register
    pub fn gyro_int_status(&mut self) -> Result<IntStatusGyro, T::Error> {        
            
        let reg_data: u8 = self.read_register(Sensor::Gyro, register::AG::INT_GEN_SRC_G.addr())?;

        let status = IntStatusGyro {            
            /// This bit signals whether one or more interrupt events occured.
            interrupt_active: match reg_data & G_INT_Bitmasks::IA_G {
                0 => false,
                _ => true,
            },
            /// Pitch (X-axis) high event has occurred
            xaxis_high_event: match reg_data & G_INT_Bitmasks::XH_G {
                0 => false,
                _ => true,
            },
            /// Pitch (X-axis) low event has occurred
            xaxis_low_event: match reg_data & G_INT_Bitmasks::XL_G {
                0 => false,
                _ => true,
            },
            /// Roll (Y-axis) high event has occurred
            yaxis_high_event: match reg_data & G_INT_Bitmasks::YH_G {
                0 => false,
                _ => true,
            },
            /// Roll (Y-axis) low event has occurred
            yaxis_low_event: match reg_data & G_INT_Bitmasks::YL_G {
                0 => false,
                _ => true,
            },
            /// Yaw (Z-axis) high event has occurred
            zaxis_high_event: match reg_data & G_INT_Bitmasks::ZH_G {
                0 => false,
                _ => true,
            },
            /// Yaw (Z-axis) low event has occurred
            zaxis_low_event: match reg_data & G_INT_Bitmasks::ZL_G {
                0 => false,
                _ => true,
            },                
        };
        Ok(status)
    }

    /// Set gyroscope reference value for digital high-pass filter.
    pub fn set_hipass_ref(&mut self, value: u8) -> Result<(), T::Error> {
        self.interface.write(Sensor::Gyro, register::AG::REFERENCE_G.addr(), value)?;
        Ok(())
    }

    /// Set gyroscope reference value for digital high-pass filter.
    pub fn read_hipass_ref(&mut self) -> Result<u8, T::Error> {
        let data: u8 = self.read_register(Sensor::Gyro, register::AG::REFERENCE_G.addr())?;
        Ok(data)
    }

    /// gyroscope interrupt duration
    // set in INT_GEN_DUR_G register
    pub fn gyro_int_duration(&mut self, wait: FLAG, duration: u8) -> Result<(), T::Error> {
        // read the current value of the register
        
        let mut reg_value = self.read_register(Sensor::Gyro, register::AG::INT_GEN_DUR_G.addr())?;

        match wait {
            FLAG::Enabled => reg_value & !0b1000_0000 | 0b1000_0000, // set bit
            FLAG::Disabled => reg_value & !0b1000_0000, // clear bit
        };

        let duration = duration & !0b1000_0000;

        reg_value &= !0b0111_1111;

        reg_value |= duration; // need to make sure duration is 7 bit only!

        self.interface.write(Sensor::Gyro, register::AG::INT_GEN_DUR_G.addr(), reg_value)?;

        Ok(())

    }
       

    /// Get the current gyroscope interrupts configuration
    pub fn get_gyro_int_config(&mut self) -> Result<IntConfigGyro, T::Error> {
        let reg_value = self.read_register(Sensor::Gyro, 
                                              register::AG::INT_GEN_CFG_G.addr())?;
        
        let config = IntConfigGyro {
                    events_combination: match (reg_value & 0b1000_0000) >> 7 {
                        1 => COMBINATION::AND,
                        _ => COMBINATION::OR,
                    },
                    latch_interrupts: match (reg_value & 0b0100_0000) >> 6 {
                        1 => INT_LATCH::Latched,
                        _ => INT_LATCH::NotLatched,
                    },
                    interrupt_high_xaxis: match (reg_value & 0b0010_0000) >> 5 {
                        1 => FLAG::Enabled,
                        _ => FLAG::Disabled,
                    },
                    interrupt_low_xaxis: match (reg_value & 0b0001_0000) >> 4 {
                        1 => FLAG::Enabled,
                        _ => FLAG::Disabled,
                    },
                    interrupt_high_yaxis: match (reg_value & 0b0000_1000) >> 3 {
                        1 => FLAG::Enabled,
                        _ => FLAG::Disabled,
                    },
                    interrupt_low_yaxis: match (reg_value & 0b0000_0100) >> 2 {
                        1 => FLAG::Enabled,
                        _ => FLAG::Disabled,
                    },
                    interrupt_high_zaxis: match (reg_value & 0b0000_0010) >> 1 {
                        1 => FLAG::Enabled,
                        _ => FLAG::Disabled,
                    },
                    interrupt_low_zaxis: match reg_value & 0b0000_0001 {
                        1 => FLAG::Enabled,
                        _ => FLAG::Disabled,
                    }
                };
            Ok(config)
        }

}

#[test]
fn configure_gyro_int() {
    let config = IntConfigGyro::default();
    assert_eq!(config.int_gen_cfg_g(), 0b0000_0000);

    let config = IntConfigGyro {
                events_combination: COMBINATION::AND,
                latch_interrupts: INT_LATCH::Latched,
                interrupt_high_xaxis: FLAG::Enabled,
                interrupt_high_yaxis: FLAG::Enabled,
                interrupt_high_zaxis: FLAG::Enabled,
                interrupt_low_xaxis: FLAG::Enabled,
                interrupt_low_yaxis: FLAG::Enabled,
                interrupt_low_zaxis: FLAG::Enabled,
                };
    assert_eq!(config.int_gen_cfg_g(), 0b1111_1111);

}