//! Various functions related to FIFO
 
use super::*;

/// CTRL_REG9 bit 4 FIFO_TEMP_EN
const FIFO_TEMP_EN: u8 = 0b0001_0000;
/// CTRL_REG9 bit 1 FIFO_EN
const FIFO_EN: u8 = 0b0000_0010;
/// CTRL_REG9 bit 0 STOP_ON_FTH
const STOP_ON_FTH: u8 = 0b0000_0001; 
/// FIFO_SRC bit 7 FTH
const FTH: u8 = 0b1000_0000;
/// FIFO_SRC bit 6 OVRN
const OVRN: u8 = 0b0100_0000;
/// FIFO_SRC bits 5:0 FSS
const FSS: u8 = 0b0011_1111;

/// FIFO settings
#[derive(Debug)]
pub struct FIFOConfig {    
    /// FIFO memory enable
    pub fifo_enable: bool, 
    /// Select FIFO operation mode (see Table 84 for details)        
    pub fifo_mode: FIFO_MODE, // default Bypass    
    /// Enable threshold level use
    pub fifo_use_threshold: bool, 
    /// Set the threshold level
    pub fifo_threshold: u8, // default 32
    /// Store temperature data in FIFO
    fifo_temperature_enable: bool, 
    }
 
impl Default for FIFOConfig {
    fn default() -> Self {
        FIFOConfig {                      
            fifo_enable: false, // disabled
            fifo_mode: FIFO_MODE::Bypass, // Bypass mode            
            fifo_use_threshold: false, // FIFO depth not limited
            fifo_threshold: 32u8, // set the threshold level      
            fifo_temperature_enable: false, // temperature data not stored in FIFO
            }
        }
    }

impl FIFOConfig {
    /// Returns values to be written to CTRL_REG9 and FIFO_CTRL:   
    fn f_fifo_ctrl(&self) -> u8 {
        let mut data = 0u8;
        data |= self.fifo_mode.value();
        data |= self.fifo_threshold;        
        data
    }
    fn f_ctrl_reg9(&self) -> u8 {
        let mut data = 0u8;
        if self.fifo_temperature_enable {data |= 1 << 4;}        
        if self.fifo_enable { data |= 1 << 1;}
        if self.fifo_use_threshold { data |= 1;}        
        data
    }   
}

#[derive(Debug)]
/// Contents of the FIFO_STATUS register (threshold reached, overrun, empty, stored data level)
pub struct FifoStatus {
    /// FIFO threshold status. True if FIFO filling is equal or higher than    threshold level
    pub fifo_thresh_reached: bool,
    /// True is FIFO is completely filled and at least one samples has been overwritten
    pub fifo_overrun: bool,
    /// True if FIFO is empty (level = 0)
    pub fifo_empty: bool,
    /// Number of unread samples stored into FIFO
    pub fifo_level: u8,    
}
 
impl<T> LSM9DS1<T>
where
    T: Interface,
    {
    
    /// Enable and configure FIFO
    pub fn configure_fifo(&mut self, config: FIFOConfig) -> Result<(), T::Error> {        
        // set/clear the three bits of CTRL_REG9 register
        match config.fifo_enable {
            true => self.set_register_bit_flag(Sensor::Accelerometer, Registers::CTRL_REG9.addr(), FIFO_EN),
            false => self.clear_register_bit_flag(Sensor::Accelerometer, Registers::CTRL_REG9.addr(), FIFO_EN),
        }?;
        match config.fifo_temperature_enable {
            true => self.set_register_bit_flag(Sensor::Accelerometer, Registers::CTRL_REG9.addr(), FIFO_TEMP_EN),
            false => self.clear_register_bit_flag(Sensor::Accelerometer, Registers::CTRL_REG9.addr(), FIFO_TEMP_EN),
        }?;
        match config.fifo_use_threshold {
            true => self.set_register_bit_flag(Sensor::Accelerometer, Registers::CTRL_REG9.addr(), STOP_ON_FTH),
            false => self.clear_register_bit_flag(Sensor::Accelerometer, Registers::CTRL_REG9.addr(), STOP_ON_FTH),
        }?;
                
        // set the entire FIFO_CTRL register
        self.interface.write(Registers::FIFO_CTRL.addr(), config.f_fifo_ctrl())?;
        Ok(())
    }
 
 
    /// Get flags and FIFO level from the FIFO_STATUS register
    pub fn get_fifo_status(&mut self) -> Result<FifoStatus, T::Error> {         
        let mut data = [0u8];
        self.interface.read(Sensor::Accelerometer, Registers::FIFO_SRC.addr(), &mut data)?;                       
        let fifo_level_value = data[0] & FSS; 
 
        let status = FifoStatus {
            /// Is FIFO filling equal or higher than the threshold?
            fifo_thresh_reached: match data & FTH {
                0 => false,
                _ => true,
            },
            /// Is FIFO full and at least one sample has been overwritten?
            fifo_overrun: match data & OVRN {
                0 => false,
                _ => true,
            },                        
            /// Is FIFO empty (no unread samples)?
            fifo_empty: match fifo_level_value {
                0 => true,
                _ => false,
            },
            /// Read FIFO stored data level
            fifo_level: fifo_level_value,
        };
        Ok(status)
    }
}

/// FIFO mode selection. (Refer to datasheets)
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy)]
pub enum FIFO_MODE {
    /// Bypass mode (he FIFO is not operational and it remains empty).
    Bypass = 0b000,
    /// FIFO mode (data from the output channels are stored in the FIFO until it is overwritten).
    FIFO = 0b001,    
    /// Continuous-to-FIFO mode (continuous mode until trigger is deasserted, then FIFO mode).
    Continuous_to_FIFO = 0b011,
    /// Bypass-to-Continuous mode (Bypass mode until trigger is deasserted, then Continuous mode).
    Bypass_to_continuous = 0b100,    
    /// Continuous mode. If the FIFO is full, the new sample overwrites the older sample.
    Continuous = 0b110,     
}
 
impl FIFO_MODE {
     pub fn value(self) -> u8 {
        (self as u8) << 5 // shifted into the right position, can be used directly
    }
}

