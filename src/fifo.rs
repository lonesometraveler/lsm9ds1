//! Various functions related to FIFO
//! 
//! TO DO:
//! - CHECK IF ALL FUNCTIONS ARE IMPLEMENTED
//! - MAKE SURE REGISTERS ARE NOT OVERWRITTEN BY MISTAKE

use super::*;

#[allow(non_camel_case_types)]
pub struct FIFO_Bitmasks;

#[allow(dead_code)]
/// Bitmasks for FIFO-related settings in CTRL_REG9 and CTRL_REG5_XL registers
impl FIFO_Bitmasks {
    pub (crate) const FTH: u8 = 0b1000_0000;
    /// FIFO_SRC bit 6 OVRN
    pub (crate) const OVRN: u8 = 0b0100_0000;
    /// FIFO_SRC bits 5:0 FSS
    pub (crate) const FSS: u8 = 0b0011_1111;
    /// CTRL_REG9 FIFO-related settings 
    pub (crate) const CTRL_REG9_FIFO: u8 = 0b0001_0011;
    /// Decimation setting in CTRL_REG5_XL
    pub (crate) const DEC: u8 = 0b1100_0000;
}

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
        
        // write values to the FIFO_CTRL register
        self.interface.write(Sensor::Accelerometer, register::AG::FIFO_CTRL.addr(), config.f_fifo_ctrl())?;         

        // write values to specific bits of the CTRL_REG9 register        
        let ctrl_reg9: u8 = self.read_register(Sensor::Accelerometer, register::AG::CTRL_REG9.addr())?; 
        //let mask: u8 = 0b00010011;
        let data: u8 = config.f_ctrl_reg9();
        let mut payload: u8 = ctrl_reg9 & !FIFO_Bitmasks::CTRL_REG9_FIFO;       
        payload |= data;        
        self.interface.write(Sensor::Accelerometer, register::AG::CTRL_REG9.addr(), payload)?;        
        
        Ok(())
    }
 
    /// Get flags and FIFO level from the FIFO_STATUS register
    pub fn get_fifo_status(&mut self) -> Result<FifoStatus, T::Error> {                 
        let fifo_src = self.read_register(Sensor::Accelerometer, register::AG::FIFO_SRC.addr())?;        
        let fifo_level_value = fifo_src & FIFO_Bitmasks::FSS;  
        let status = FifoStatus {
            /// Is FIFO filling equal or higher than the threshold?
            fifo_thresh_reached: match fifo_src & FIFO_Bitmasks::FTH {
                0 => false,
                _ => true,
            },
            /// Is FIFO full and at least one sample has been overwritten?
            fifo_overrun: match fifo_src & FIFO_Bitmasks::OVRN {
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

    pub fn set_decimation(&mut self, decimation: DECIMATE) -> Result<(), T::Error> {
        let data: u8 = self.read_register(Sensor::Accelerometer, register::AG::CTRL_REG5_XL.addr())?; // read current content of the register
        let mut payload: u8 = data & !FIFO_Bitmasks::DEC; // use bitmask to affect only bits [7:6]
        payload |= decimation.value(); // set the selected decimation value
        self.interface.write(Sensor::Accelerometer, register::AG::CTRL_REG5_XL.addr(), payload)?;
        Ok(())
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

/// Decimation of acceleration data on OUT REG and FIFO (Refer to table 65)
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy)]
pub enum DECIMATE {
    /// No decimation
    NoDecimation = 0b00,
    /// update every 2 samples;
    _2samples = 0b01,
    /// update every 4 samples;
    _4samples = 0b10,
    /// update every 8 samples;
    _8samples = 0b11,
}

impl DECIMATE {
    pub fn value(self) -> u8 {
        (self as u8) << 6 // shifted to bits [7:6], can be used directly
    }
}
