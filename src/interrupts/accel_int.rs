/// Functions related to accelerometer-specific interrupts
/// 
/// TO DO:
/// - add acceleration threshold setting for X, Y and Z axis (INT_GEN_THS_X/Y/Z_XL)
/// - LIR_XL1 and 4D_XL1 bits of CTRL_REG4 => should they be incorporated in the Config struct? what's the relation between 4D_XL1 and _6D?
/// 


#[allow(non_camel_case_types)]

use super::*;

/// Accelerometer interrupt generation settings
#[derive(Debug)]
pub struct IntConfigAccel {
    /// Combination of accelerometer's interrupt events
    pub events_combination: COMBINATION,
    /// Enable 6-direction detection
    pub enable_6d: FLAG,
    /// Enable interrupt generation on Z-axis high event
    pub interrupt_high_zaxis: FLAG,
    /// Enable interrupt generation on Z-axis low event
    pub interrupt_low_zaxis: FLAG,
    /// Enable interrupt generation on Y-axis high event
    pub interrupt_high_yaxis: FLAG,    
    /// Enable interrupt generation on Y-axis low event
    pub interrupt_low_yaxis: FLAG,
    /// Enable interrupt generation on X-axis high event
    pub interrupt_high_xaxis: FLAG,
    /// Enable interrupt generation on X-axis low event
    pub interrupt_low_xaxis: FLAG,    
    
}
impl Default for IntConfigAccel {
    fn default() -> Self {
        IntConfigAccel {
            events_combination: COMBINATION::OR,
            enable_6d: FLAG::Disabled,
            interrupt_high_zaxis: FLAG::Disabled,
            interrupt_low_zaxis: FLAG::Disabled,                        
            interrupt_high_yaxis: FLAG::Disabled,
            interrupt_low_yaxis: FLAG::Disabled,
            interrupt_high_xaxis: FLAG::Disabled,
            interrupt_low_xaxis: FLAG::Disabled,
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

#[allow(non_camel_case_types)]
pub struct XL_CFG_Bitmasks;
#[allow(dead_code)]
/// Bitmasks for interrupt-related settings in INT_GEN_CFG_XL register
impl XL_CFG_Bitmasks {
    pub(crate) const AOI_XL: u8 = 0b1000_0000;
    pub(crate) const _6D: u8 = 0b0100_0000;
    pub(crate) const ZHIE_XL: u8 = 0b0010_0000;
    pub(crate) const ZLIE_XL: u8 = 0b0001_0000;
    pub(crate) const YHIE_XL: u8 = 0b0000_1000;
    pub(crate) const YLIE_XL: u8 = 0b0000_0100;
    pub(crate) const XHIE_XL: u8 = 0b0000_0010;
    pub(crate) const XLIE_XL: u8 = 0b0000_0001;

    pub(crate) const LIR_XL1: u8 = 0b0000_0010;
    pub(crate) const _4D_XL1: u8 = 0b0000_0001;
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

    /// Get the current accelerometer interrupts configuration
    pub fn get_accel_int_config(&mut self) -> Result<IntConfigAccel, T::Error> {
        
        let reg_value: u8 = self.read_register(Sensor::Accelerometer, 
                                              register::AG::INT_GEN_CFG_XL.addr())?;
        
        let config = IntConfigAccel {
                    events_combination: match (reg_value & XL_CFG_Bitmasks::AOI_XL) >> 7 {
                        1 => COMBINATION::AND,
                        _ => COMBINATION::OR,
                    },
                    enable_6d: match (reg_value & XL_CFG_Bitmasks::_6D) >> 6 {
                        1 => FLAG::Enabled,
                        _ => FLAG::Disabled,
                    },
                    interrupt_high_zaxis: match (reg_value & XL_CFG_Bitmasks::ZHIE_XL) >> 5 {
                        1 => FLAG::Enabled,
                        _ => FLAG::Disabled,
                    },
                    interrupt_low_zaxis: match (reg_value & XL_CFG_Bitmasks::ZLIE_XL) >> 4 {
                        1 => FLAG::Enabled,
                        _ => FLAG::Disabled,
                    },
                    interrupt_high_yaxis: match (reg_value & XL_CFG_Bitmasks::YHIE_XL) >> 3 {
                        1 => FLAG::Enabled,
                        _ => FLAG::Disabled,
                    },
                    interrupt_low_yaxis: match (reg_value & XL_CFG_Bitmasks::XLIE_XL) >> 2 {
                        1 => FLAG::Enabled,
                        _ => FLAG::Disabled,
                    },                 
                    interrupt_high_xaxis: match (reg_value & XL_CFG_Bitmasks::XHIE_XL) >> 1 {
                        1 => FLAG::Enabled,
                        _ => FLAG::Disabled,
                    },
                    interrupt_low_xaxis: match reg_value & XL_CFG_Bitmasks::XLIE_XL {
                        1 => FLAG::Enabled,
                        _ => FLAG::Disabled,
                    },                   
                };
            Ok(config)
        }

    // === SINGLE SETTERS ===

    /// Set AND/OR combination of the accelerometer's interrupt events
    pub fn accel_int_events_combination (&mut self, setting: COMBINATION) -> Result<(), T::Error> {

        let reg_value = self.read_register(Sensor::Accelerometer, register::AG::INT_GEN_CFG_XL.addr())?;
    
        let mut data: u8  = reg_value &! XL_CFG_Bitmasks::AOI_XL; // clear the specific bit
    
        data = match setting {
            COMBINATION::AND => data | (1 << 7),       // if Enabled, set bit
            COMBINATION::OR => data,                 // if Disabled, bit is cleared
        };
    
        self.interface.write(Sensor::Accelerometer, register::AG::INT_GEN_CFG_XL.addr(), data)?;
    
        Ok(())
    
    }

    /// Enable/disable 6-direction detection for interrupt
    pub fn accel_int_enable_6d (&mut self, setting: FLAG) -> Result<(), T::Error> {

        let reg_value = self.read_register(Sensor::Accelerometer, register::AG::INT_GEN_CFG_XL.addr())?;
    
        let mut data: u8  = reg_value &! XL_CFG_Bitmasks::_6D; // clear the specific bit
    
        data = match setting {
            FLAG::Enabled => data | (1 << 6),       // if Enabled, set bit
            FLAG::Disabled => data,                 // if Disabled, bit is cleared
        };

        self.interface.write(Sensor::Accelerometer, register::AG::INT_GEN_CFG_XL.addr(), data)?;
    
        Ok(())
    }

    /// Enable interrupt generation on accelerometer’s Z-axis high event
    pub fn accel_int_interrupt_high_zaxis (&mut self, setting: FLAG) -> Result<(), T::Error> {

        let reg_value = self.read_register(Sensor::Accelerometer, register::AG::INT_GEN_CFG_XL.addr())?;
    
        let mut data: u8  = reg_value &! XL_CFG_Bitmasks::ZHIE_XL; // clear the specific bit
    
        data = match setting {
            FLAG::Enabled => data | (1 << 5),       // if Enabled, set bit
            FLAG::Disabled => data,                 // if Disabled, bit is cleared
        };

        self.interface.write(Sensor::Accelerometer, register::AG::INT_GEN_CFG_XL.addr(), data)?;
    
        Ok(())
    }

    /// Enable interrupt generation on accelerometer’s Z-axis high event
    pub fn accel_int_interrupt_low_zaxis (&mut self, setting: FLAG) -> Result<(), T::Error> {

        let reg_value = self.read_register(Sensor::Accelerometer, register::AG::INT_GEN_CFG_XL.addr())?;
    
        let mut data: u8  = reg_value &! XL_CFG_Bitmasks::ZLIE_XL; // clear the specific bit
    
        data = match setting {
            FLAG::Enabled => data | (1 << 4),       // if Enabled, set bit
            FLAG::Disabled => data,                 // if Disabled, bit is cleared
        };

        self.interface.write(Sensor::Accelerometer, register::AG::INT_GEN_CFG_XL.addr(), data)?;
    
        Ok(())
    }

    /// Enable interrupt generation on accelerometer’s Y-axis high event
    pub fn accel_int_interrupt_high_yaxis (&mut self, setting: FLAG) -> Result<(), T::Error> {

        let reg_value = self.read_register(Sensor::Accelerometer, register::AG::INT_GEN_CFG_XL.addr())?;
    
        let mut data: u8  = reg_value &! XL_CFG_Bitmasks::YHIE_XL; // clear the specific bit
    
        data = match setting {
            FLAG::Enabled => data | (1 << 3),       // if Enabled, set bit
            FLAG::Disabled => data,                 // if Disabled, bit is cleared
        };

        self.interface.write(Sensor::Accelerometer, register::AG::INT_GEN_CFG_XL.addr(), data)?;
    
        Ok(())
    }

    /// Enable interrupt generation on accelerometer’s Y-axis high event
    pub fn accel_int_interrupt_low_yaxis (&mut self, setting: FLAG) -> Result<(), T::Error> {

        let reg_value = self.read_register(Sensor::Accelerometer, register::AG::INT_GEN_CFG_XL.addr())?;
    
        let mut data: u8  = reg_value &! XL_CFG_Bitmasks::YLIE_XL; // clear the specific bit
    
        data = match setting {
            FLAG::Enabled => data | (1 << 2),       // if Enabled, set bit
            FLAG::Disabled => data,                 // if Disabled, bit is cleared
        };

        self.interface.write(Sensor::Accelerometer, register::AG::INT_GEN_CFG_XL.addr(), data)?;
    
        Ok(())
    }

    /// Enable interrupt generation on accelerometer’s X-axis high event
    pub fn accel_int_interrupt_high_xaxis (&mut self, setting: FLAG) -> Result<(), T::Error> {

        let reg_value = self.read_register(Sensor::Accelerometer, register::AG::INT_GEN_CFG_XL.addr())?;
    
        let mut data: u8  = reg_value &! XL_CFG_Bitmasks::XHIE_XL; // clear the specific bit
    
        data = match setting {
            FLAG::Enabled => data | (1 << 1),       // if Enabled, set bit
            FLAG::Disabled => data,                 // if Disabled, bit is cleared
        };

        self.interface.write(Sensor::Accelerometer, register::AG::INT_GEN_CFG_XL.addr(), data)?;
    
        Ok(())
    }

    /// Enable interrupt generation on accelerometer’s X-axis high event
    pub fn accel_int_interrupt_low_xaxis (&mut self, setting: FLAG) -> Result<(), T::Error> {

        let reg_value = self.read_register(Sensor::Accelerometer, register::AG::INT_GEN_CFG_XL.addr())?;
    
        let mut data: u8  = reg_value &! XL_CFG_Bitmasks::XLIE_XL; // clear the specific bit
    
        data = match setting {
            FLAG::Enabled => data | 1,       // if Enabled, set bit
            FLAG::Disabled => data,                 // if Disabled, bit is cleared
        };

        self.interface.write(Sensor::Accelerometer, register::AG::INT_GEN_CFG_XL.addr(), data)?;
    
        Ok(())
    }



    /// Latch accelerometer interrupt request
    pub fn accel_int_latching (&mut self, setting: INT_LATCH) -> Result<(), T::Error> {

        let reg_value = self.read_register(Sensor::Accelerometer, register::AG::CTRL_REG4.addr())?;
    
        let mut data: u8  = reg_value &! XL_CFG_Bitmasks::LIR_XL1; // clear the specific bit
    
        data = match setting {
            INT_LATCH::Latched => data | (1 << 1),          // if Enabled, set bit
            INT_LATCH::NotLatched => data,                  // if Disabled, bit is cleared
        };

        self.interface.write(Sensor::Gyro, register::AG::CTRL_REG4.addr(), data)?;
    
        Ok(())
    }

    /// Position recognition setting for the interrupt generator (use 4D or 6D)
    pub fn accel_int_pos_recog (&mut self, setting: POS_RECOG) -> Result<(), T::Error> {

        let reg_value = self.read_register(Sensor::Accelerometer, register::AG::CTRL_REG4.addr())?;
    
        let mut data: u8  = reg_value &! XL_CFG_Bitmasks::_4D_XL1; // clear the specific bit
    
        data = match setting {
            POS_RECOG::_4D => data | 1,                 // set bit if 4D used
            POS_RECOG::_6D => data,                     // leave bit cleared if 6D used (default setting)
        };

        self.interface.write(Sensor::Gyro, register::AG::CTRL_REG4.addr(), data)?;
    
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

    /// Accelerometer interrupt duration
    /// Enable/disable wait function and define for how many samples to wait before exiting interrupt    
    pub fn accel_int_duration(&mut self, wait: FLAG, duration: u8) -> Result<(), T::Error> {
                
        let mut reg_value = self.read_register(Sensor::Accelerometer, register::AG::INT_GEN_DUR_XL.addr())?;

        match wait {
            FLAG::Enabled => reg_value & !0b1000_0000 | 0b1000_0000, // set bit
            FLAG::Disabled => reg_value & !0b1000_0000, // clear bit
        };

        let duration: u8 = match duration { // clamp duration to 7 bit values
            0..=127 => duration,
            _ => 127,
        };

        reg_value &= !0b0111_1111; // clear the lowest 7 bits

        reg_value |= duration; 

        self.interface.write(Sensor::Accelerometer, register::AG::INT_GEN_DUR_XL.addr(), reg_value)?;

        Ok(())
    }

    /// Set accelerometer interrupt threshold for X, Y and Z axes
    /// 
    /// TO DO: use actual values as input (mG)?
    /// 
    /// 
    pub fn accel_int_int_thresholds(&mut self, x_ths: u8, y_ths: u8, z_ths: u8) -> Result<(), T::Error> {
        
        self.interface.write(Sensor::Accelerometer, register::AG::INT_GEN_THS_X_XL.addr(), x_ths)?;
        self.interface.write(Sensor::Accelerometer, register::AG::INT_GEN_THS_Y_XL.addr(), y_ths)?;
        self.interface.write(Sensor::Accelerometer, register::AG::INT_GEN_THS_Z_XL.addr(), z_ths)?;

        Ok(())

    }

    /// Get accelerometer interrupt thresholds for X, Y and Z axes as a tuple
    /// 
    /// TO DO: get these as actual values? (mG)
    /// 
    pub fn get_accel_int_thresholds(&mut self) -> Result<(u8, u8, u8), T::Error> {
        
        let mut data = [0u8;3];

        self.interface.read(Sensor::Accelerometer, register::AG::INT_GEN_THS_X_XL.addr(), &mut data)?;
        
        Ok((data[0], data[1], data[2]))

    }


}

#[test]
fn configure_accel_int() {
    let config = IntConfigAccel::default();//IntConfigAccel {..Default::default()};
    assert_eq!(config.int_gen_cfg_xl(), 0b0000_0000);

    let config = IntConfigAccel {
                    events_combination: COMBINATION::AND,
                    enable_6d: FLAG::Enabled,
                    interrupt_high_zaxis: FLAG::Enabled,
                    interrupt_low_zaxis: FLAG::Enabled,
                    interrupt_high_yaxis: FLAG::Enabled,
                    interrupt_low_yaxis: FLAG::Enabled,
                    interrupt_high_xaxis: FLAG::Enabled,
                    interrupt_low_xaxis: FLAG::Enabled,
                };
    assert_eq!(config.int_gen_cfg_xl(), 0b1111_1111);

    let config = IntConfigAccel {
        interrupt_high_zaxis: FLAG::Enabled,
        interrupt_low_xaxis: FLAG::Enabled,
        ..Default::default()
    };
    assert_eq!(config.int_gen_cfg_xl(), 0b0010_0001);

}





