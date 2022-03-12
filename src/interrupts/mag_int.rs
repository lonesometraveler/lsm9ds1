/// Functions related to magnetometer-specific interrupts

use super::*;

pub struct M_INT_Bitmasks;

/// Magnetometer interrupt pin (INT_M) settings
#[derive(Debug)]
pub struct IntConfigMag {
    /// Enable interrupt generation on X-axis, default 0
    pub interrupt_xaxis: FLAG,
    /// Enable interrupt generation on Y-axis, default 0
    pub interrupt_yaxis: FLAG,
    /// Enable interrupt generation on Z-axis, default 0
    pub interrupt_zaxis: FLAG,
    /// Configure interrupt pin INT_M as active high or active low, default 0 (low) 
    pub active_high_or_low: INT_ACTIVE,
    /// Latch interrupt request (Once latched, the INT_M pin remains in the same state until INT_SRC_M is read), default 0 (latched)
    pub interrupt_latching: INT_LATCH,
    /// Interrupt enable on the INT_M pin, default 0
    pub enable_interrupt: FLAG,
}

impl Default for IntConfigMag {
    fn default() -> Self {
        IntConfigMag {                    
            interrupt_xaxis: FLAG::Disabled,            
            interrupt_yaxis: FLAG::Disabled,            
            interrupt_zaxis: FLAG::Disabled,            
            active_high_or_low: INT_ACTIVE::Low,                  // reversed!           
            interrupt_latching: INT_LATCH::NotLatched,            // NOTE: it's reversed, 0 is latched (default)
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
        if !self.active_high_or_low.status() {
            data |= 1 << 2;
        }
        if !self.interrupt_latching.status() {                  // NOTE: it's reversed, 0 is latched
            data |= 1 << 1;
        }
        if self.enable_interrupt.status() {
            data |= 1;
        }
        data        
    }    
}

#[allow(non_camel_case_types)]
/// Bitmasks for interrupt-related settings in INT_SRC_M register
impl M_INT_Bitmasks {
    pub (crate) const PTH_X: u8 = 0b1000_0000;
    pub (crate) const PTH_Y: u8 = 0b0100_0000;
    pub (crate) const PTH_Z: u8 = 0b0010_0000;
    pub (crate) const NTH_X: u8 = 0b0001_0000;
    pub (crate) const NTH_Y: u8 = 0b0000_1000;
    pub (crate) const NTH_Z: u8 = 0b0000_0100;
    pub (crate) const MROI: u8 = 0b0000_0010;
    pub (crate) const INT: u8 = 0b0000_0001;
}

#[allow(non_camel_case_types)]
pub struct M_CFG_Bitmasks;
#[allow(dead_code)]
/// Bitmasks for interrupt-related settings in INT_CFG_M register
impl M_CFG_Bitmasks {
    pub(crate) const XIEN: u8 = 0b1000_0000;
    pub(crate) const YIEN: u8 = 0b0100_0000;
    pub(crate) const ZIEN: u8 = 0b0010_0000;
    pub(crate) const IEA: u8 = 0b0001_0000;
    pub(crate) const IEL: u8 = 0b0000_1000;
    pub(crate) const IEN: u8 = 0b0000_0100;
    
}



#[derive(Debug)]
/// Contents of the INT_SRC_M register (interrupt active and threshold excess events flags)
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


impl<T> LSM9DS1<T>
where
    T: Interface,
    {
    /// Enable interrupts for magnetometer and configure the INT_M interrupt pin     
    pub fn configure_interrupts_mag(&mut self, config: IntConfigMag) -> Result<(), T::Error> {
        self.interface.write(Sensor::Magnetometer, register::Mag::INT_CFG_M.addr(), config.int_cfg_m())?;                
        Ok(())
    }

    /// Get the current magnetometer interrupts configuration
    pub fn get_mag_int_config(&mut self) -> Result<IntConfigMag, T::Error> {
        
        let reg_value: u8 = self.read_register(Sensor::Magnetometer, 
                                              register::Mag::INT_CFG_M.addr())?;
        
        let config = IntConfigMag {
                            interrupt_xaxis: match (reg_value & M_CFG_Bitmasks::XIEN) >> 7 {
                                1 => FLAG::Enabled,
                                _ => FLAG::Disabled,
                            },
                            interrupt_yaxis: match (reg_value & M_CFG_Bitmasks::YIEN) >> 6 {
                                1 => FLAG::Enabled,
                                _ => FLAG::Disabled,
                            },
                            interrupt_zaxis: match (reg_value & M_CFG_Bitmasks::ZIEN) >> 5 {
                                1 => FLAG::Enabled,
                                _ => FLAG::Disabled,
                            },
                            active_high_or_low: match (reg_value & M_CFG_Bitmasks::IEA) >> 2 {
                                1 => INT_ACTIVE::High,
                                _ => INT_ACTIVE::Low,
                            },
                            interrupt_latching: match (reg_value & M_CFG_Bitmasks::IEL) >> 1 {
                                1 => INT_LATCH::NotLatched,     
                                _ => INT_LATCH::Latched,            // NOTE: it's reversed, 0 is latched
                            },
                            enable_interrupt: match reg_value & M_CFG_Bitmasks::IEN {
                                1 => FLAG::Enabled,     
                                _ => FLAG::Disabled,
                            },
                        };
                        Ok(config)
            }

    // == SINGLE SETTERS ==
    
    /// Enable interrupt generation on magnetometer’s X-axis
    pub fn mag_int_xaxis (&mut self, setting: FLAG) -> Result<(), T::Error> {

        let reg_value = self.read_register(Sensor::Magnetometer, register::Mag::INT_CFG_M.addr())?;
    
        let mut data: u8  = reg_value &! M_CFG_Bitmasks::XIEN; // clear the specific bit
    
        data = match setting {
            FLAG::Enabled => data | (1 << 7),       // if Enabled, set bit
            FLAG::Disabled => data,                 // if Disabled, bit is cleared
        };
    
        self.interface.write(Sensor::Magnetometer, register::Mag::INT_CFG_M.addr(), data)?;
    
        Ok(())
    
    }

    /// Enable interrupt generation on magnetometer’s Y-axis
    pub fn mag_int_yaxis (&mut self, setting: FLAG) -> Result<(), T::Error> {

        let reg_value = self.read_register(Sensor::Magnetometer, register::Mag::INT_CFG_M.addr())?;
    
        let mut data: u8  = reg_value &! M_CFG_Bitmasks::YIEN; // clear the specific bit
    
        data = match setting {
            FLAG::Enabled => data | (1 << 6),       // if Enabled, set bit
            FLAG::Disabled => data,                 // if Disabled, bit is cleared
        };
    
        self.interface.write(Sensor::Magnetometer, register::Mag::INT_CFG_M.addr(), data)?;
    
        Ok(())
    
    }

    /// Enable interrupt generation on magnetometer’s Z-axis
    pub fn mag_int_zaxis (&mut self, setting: FLAG) -> Result<(), T::Error> {

        let reg_value = self.read_register(Sensor::Magnetometer, register::Mag::INT_CFG_M.addr())?;

        let mut data: u8  = reg_value &! M_CFG_Bitmasks::ZIEN; // clear the specific bit

        data = match setting {
            FLAG::Enabled => data | (1 << 5),       // if Enabled, set bit
            FLAG::Disabled => data,                 // if Disabled, bit is cleared
        };

        self.interface.write(Sensor::Magnetometer, register::Mag::INT_CFG_M.addr(), data)?;

        Ok(())

    }


    /// Interrupt active setting for the INT_MAG pin: active high (default) or active low
    pub fn mag_int_pin_active (&mut self, setting: INT_ACTIVE) -> Result<(), T::Error> {

        let reg_value = self.read_register(Sensor::Magnetometer, register::Mag::INT_CFG_M.addr())?;

        let mut data: u8  = reg_value &! M_CFG_Bitmasks::IEA; // clear the specific bit

        data = match setting {
            INT_ACTIVE::High => data | (1 << 2),       // if Enabled, set bit
            INT_ACTIVE::Low => data,                 // if Disabled, bit is cleared
        };

        self.interface.write(Sensor::Magnetometer, register::Mag::INT_CFG_M.addr(), data)?;

        Ok(())

    }

    /// Latch interrupt request. Once latched, the INT_M pin remains in the same state until interrupt status is read.
    pub fn mag_int_latching (&mut self, setting: INT_LATCH) -> Result<(), T::Error> {

        let reg_value = self.read_register(Sensor::Magnetometer, register::Mag::INT_CFG_M.addr())?;

        let mut data: u8  = reg_value &! M_CFG_Bitmasks::IEL; // clear the specific bit

        data = match setting {
            INT_LATCH::NotLatched => data | (1 << 1),       // if Enabled, set bit
            INT_LATCH::Latched => data,                 // if Disabled, bit is cleared
        };

        self.interface.write(Sensor::Magnetometer, register::Mag::INT_CFG_M.addr(), data)?;

        Ok(())

    }

    /// Interrupt enable on the INT_M pin
    pub fn mag_int_enable (&mut self, setting: FLAG) -> Result<(), T::Error> {

        let reg_value = self.read_register(Sensor::Magnetometer, register::Mag::INT_CFG_M.addr())?;

        let mut data: u8  = reg_value &! M_CFG_Bitmasks::IEN; // clear the specific bit

        data = match setting {
            FLAG::Enabled => data | 1,       // if Enabled, set bit
            FLAG::Disabled => data,                 // if Disabled, bit is cleared
        };

        self.interface.write(Sensor::Magnetometer, register::Mag::INT_CFG_M.addr(), data)?;

        Ok(())

    }


    /// Get all the flags from the INT_SRC_M register
    pub fn mag_int_status(&mut self) -> Result<IntStatusMag, T::Error> {        
        
        let reg_data: u8 = self.read_register(Sensor::Magnetometer, register::Mag::INT_SRC_M.addr())?;

        let status = IntStatusMag {            
            /// Does value on X-axis exceed the threshold on the positive side?
            xaxis_exceeds_thresh_pos: match reg_data & M_INT_Bitmasks::PTH_X {
                0 => false,
                _ => true,
            },
            /// Does value on Y-axis exceed the threshold on the positive side?
            yaxis_exceeds_thresh_pos: match reg_data & M_INT_Bitmasks::PTH_Y {
                0 => false,
                _ => true,
            },
            /// Does value on Z-axis exceed the threshold on the positive side?
            zaxis_exceeds_thresh_pos: match reg_data & M_INT_Bitmasks::PTH_Z {
                0 => false,
                _ => true,
            },
            /// Does value on X-axis exceed the threshold on the negative side?
            xaxis_exceeds_thresh_neg: match reg_data & M_INT_Bitmasks::NTH_X {
                0 => false,
                _ => true,
            },
            /// Does value on Y-axis exceed the threshold on the negative side?
            yaxis_exceeds_thresh_neg: match reg_data & M_INT_Bitmasks::NTH_Y {
                0 => false,
                _ => true,
            },
            /// Does value on Z-axis exceed the threshold on the negative side?
            zaxis_exceeds_thresh_neg: match reg_data & M_INT_Bitmasks::NTH_Z {
                0 => false,
                _ => true,
            },
            /// Did internal measurement range overflow on magnetic value?
            measurement_range_overflow: match reg_data & M_INT_Bitmasks::MROI {
                0 => false,
                _ => true,
            },
            /// This bit signals when the interrupt event occurs.
            interrupt_occurs: match reg_data & M_INT_Bitmasks::INT {
                0 => false,
                _ => true,
            },
        };
        Ok(status)
    }


    // == MAKE SURE THE ORDER IS CORRECT!!! == 

    /// Set threshold in miligauss
    pub fn set_mag_threshold(&mut self, threshold: f32) -> Result<(), T::Error> {
        let sensitivity = self.mag.scale.sensitivity();
        let mut data = threshold / sensitivity;
        // make sure it's not more than 15 bits, and it must be a positive value
        if data >= 32767.0 {
            data = 32767.0;
        } else if data < 0.0 {
            data = 0.0;
        }
        //data = data as u16;

        let data_low: u8 = data as u8;
        let data_high: u8 = ((data as u16) >> 8) as u8;

        self.interface.write(Sensor::Magnetometer, register::Mag::INT_THS_H_M.addr(), data_high)?;
        self.interface.write(Sensor::Magnetometer, register::Mag::INT_THS_L_M.addr(), data_low)?;

        Ok(())
    }

    /// Read the magnetometer threshold setting (value in miligauss)
    pub fn get_mag_threshold(&mut self) -> Result<f32, T::Error> {
        let sensitivity = self.mag.scale.sensitivity();
        
        let mut buffer = [0u8;2];
        self.interface.read(Sensor::Magnetometer, register::Mag::INT_THS_L_M.addr(), &mut buffer)?;
        
        let t: u16 = (buffer[1] as u16) << 8 | buffer[0] as u16; // threshold is a 15bit unsigned value
        
        Ok(t as f32 * sensitivity)
        
    }

    /// Set offset for all three axes of the magnetometer in miligauss
    pub fn set_mag_offset(&mut self, offset: (f32, f32, f32)) -> Result<(), T::Error> {
        let (mut x, mut y, mut z) = offset;
        let sensitivity = self.mag.scale.sensitivity();
        x = x / sensitivity;
        y = y / sensitivity;
        z = z / sensitivity;

        let x_low: u8 = x as u8;
        let x_high: u8 = ((x as i16) >> 8) as u8;
        self.interface.write(Sensor::Magnetometer, register::Mag::OFFSET_X_REG_L_M.addr(), x_low)?;
        self.interface.write(Sensor::Magnetometer, register::Mag::OFFSET_X_REG_H_M.addr(), x_high)?;
        
        let y_low: u8 = y as u8;
        let y_high: u8 = ((y as i16) >> 8) as u8;
        self.interface.write(Sensor::Magnetometer, register::Mag::OFFSET_Y_REG_L_M.addr(), y_low)?;
        self.interface.write(Sensor::Magnetometer, register::Mag::OFFSET_Y_REG_H_M.addr(), y_high)?;

        let z_low: u8 = z as u8;
        let z_high: u8 = ((z as i16) >> 8) as u8;
        self.interface.write(Sensor::Magnetometer, register::Mag::OFFSET_Z_REG_L_M.addr(), z_low)?;
        self.interface.write(Sensor::Magnetometer, register::Mag::OFFSET_Z_REG_H_M.addr(), z_high)?;

        Ok(())
    }

    /// Read the offset settings for all three axes of the magnetometer (values in miligauss)
    pub fn get_mag_offset(&mut self) -> Result<(f32, f32, f32), T::Error> {
        let sensitivity = self.mag.scale.sensitivity();
        
        let mut buffer = [0u8;6];
        self.interface.read(Sensor::Magnetometer, register::Mag::OFFSET_X_REG_L_M.addr(), &mut buffer)?;
        
        let x: i16 = (buffer[1] as i16) << 8 | buffer[0] as i16;
        let y: i16 = (buffer[3] as i16) << 8 | buffer[2] as i16;
        let z: i16 = (buffer[5] as i16) << 8 | buffer[4] as i16;

        Ok((x as f32 * sensitivity, y as f32 * sensitivity, z as f32 * sensitivity))
        
    }     

    
}

#[test]
fn configure_mag_int() {
        
    let config = IntConfigMag::default();
    assert_eq!(config.int_cfg_m(), 0b0000_0010);
        
    let config = IntConfigMag {
                interrupt_xaxis: FLAG::Enabled,            
                interrupt_yaxis: FLAG::Enabled,            
                interrupt_zaxis: FLAG::Enabled,            
                active_high_or_low: INT_ACTIVE::High,            
                interrupt_latching: INT_LATCH::Latched,            
                enable_interrupt: FLAG::Enabled,     
                };
    assert_eq!(config.int_cfg_m(), 0b1110_0101);    

}