/// Functions related to accelerometer-specific interrupts

#[allow(non_camel_case_types)]

use super::*;

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
    pub(crate) const IA_XL: u8 = 0b0100_0000;
    pub(crate) const ZH_XL: u8 = 0b0010_0000;
    pub(crate) const ZL_XL: u8 = 0b0001_0000;
    pub(crate) const YH_XL: u8 = 0b0000_1000;
    pub(crate) const YL_XL: u8 = 0b0000_0100;
    pub(crate) const XH_XL: u8 = 0b0000_0010;
    pub(crate) const XL_XL: u8 = 0b0000_0001;
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

impl<T> LSM9DS1<T>
where
    T: Interface,
{
    /// Enable and configure interrupts for accelerometer
    pub fn configure_interrupts_accel(&mut self, config: IntConfigAccel) -> Result<(), T::Error> {
        self.interface.write(
            Sensor::Accelerometer,
            register::AG::INT_GEN_CFG_XL.addr(),
            config.int_gen_cfg_xl(),
        )?;
        Ok(())
    }

    /// Get all the flags from the INT_GEN_SRC_XL register
    pub fn accel_int_status(&mut self) -> Result<IntStatusAccel, T::Error> {
        let reg_data: u8 =
            self.read_register(Sensor::Accelerometer, register::AG::INT_GEN_SRC_XL.addr())?;

        let status = IntStatusAccel {
            /// This bit signals whether one or more interrupt events occured.
            interrupt_active: match reg_data & XL_INT_Bitmasks::IA_XL {
                0 => false,
                _ => true,
            },
            /// X-axis high event has occurred
            xaxis_high_event: match reg_data & XL_INT_Bitmasks::XH_XL {
                0 => false,
                _ => true,
            },
            /// X-axis low event has occurred
            xaxis_low_event: match reg_data & XL_INT_Bitmasks::XL_XL {
                0 => false,
                _ => true,
            },
            /// Y-axis high event has occurred
            yaxis_high_event: match reg_data & XL_INT_Bitmasks::YH_XL {
                0 => false,
                _ => true,
            },
            /// Y-axis low event has occurred
            yaxis_low_event: match reg_data & XL_INT_Bitmasks::YL_XL {
                0 => false,
                _ => true,
            },
            /// Z-axis high event has occurred
            zaxis_high_event: match reg_data & XL_INT_Bitmasks::ZH_XL {
                0 => false,
                _ => true,
            },
            /// X-axis low event has occurred
            zaxis_low_event: match reg_data & XL_INT_Bitmasks::ZL_XL {
                0 => false,
                _ => true,
            },
        };
        Ok(status)
    }

    /// accelerometer interrupt duration
    // set in INT_GEN_DUR_XL register
    pub fn accel_int_duration(&mut self, wait: FLAG, duration: u8) -> Result<(), T::Error> {
        // read the current value of the register
        
        let mut reg_value = self.read_register(Sensor::Accelerometer, register::AG::INT_GEN_DUR_XL.addr())?;

        match wait {
            FLAG::Enabled => reg_value & !0b1000_0000 | 0b1000_0000, // set bit
            FLAG::Disabled => reg_value & !0b1000_0000, // clear bit
        };

        let duration = duration & !0b1000_0000;

        reg_value &= !0b0111_1111;

        reg_value |= duration; // need to make sure duration is 7 bit only!

        self.interface.write(Sensor::Accelerometer, register::AG::INT_GEN_DUR_XL.addr(), reg_value)?;

        Ok(())
    }

    /// Get the current accelerometer interrupts configuration
    pub fn get_accel_int_config(&self) -> IntConfigAccel {
        let reg_value = self.read_register(Sensor::Accelerometer, 
                                              register::AG::INT_GEN_CFG_XL)?;
        
        let config = IntConfigAccel {
                    events_combination: match (reg_value & 0b1000_0000) >> 7 {
                        1 => COMBINATION::AND,
                        _ => COMBINATION::OR,
                    },
                    enable_6d: match (reg_value & 0b0100_0000) >> 6 {
                        1 => FLAG::Enabled,
                        _ => FLAG::Disabled,
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
                    interrupt_low_zaxis: match (reg_value & 0b0000_0001) {
                        1 => FLAG::Enabled,
                        _ => FLAG::Disabled,
                    }
                };
            Ok(config)
        }

}

#[test]
fn configure_accel_int() {
    let config = IntConfigAccel::default();//IntConfigAccel {..Default::default()};
    assert_eq!(config.int_gen_cfg_xl(), 0b0000_0000);

    let config = IntConfigAccel {
                    events_combination: COMBINATION::AND,
                    enable_6d: FLAG::Enabled,
                    interrupt_high_xaxis: FLAG::Enabled,
                    interrupt_low_xaxis: FLAG::Enabled,
                    interrupt_high_yaxis: FLAG::Enabled,
                    interrupt_low_yaxis: FLAG::Enabled,
                    interrupt_high_zaxis: FLAG::Enabled,
                    interrupt_low_zaxis: FLAG::Enabled,
                };
    assert_eq!(config.int_gen_cfg_xl(), 0b1111_1111);


}