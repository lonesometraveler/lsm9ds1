// INT_GEN_CFG_XL (06h) Linear acceleration sensor interrupt generator configuration register.
// enable interrupt generation for high/low event for X/Y/Z axis
// AND/OR combination for interrupts
// 6D detection for interrupt
// INT_GEN_THS_X_XL (07h) Linear acceleration sensor interrupt threshold register for axis X, then also Y and Z
// INT_GEN_DUR_XL (0Ah) Linear acceleration sensor interrupt duration register. - wait or not and for how long before exiting interrupt
// INT1_CTRL (0Ch) INT1_A/G pin control register. (for pin 1)
// gyroscope interrupt enable, accelerometer interrupt enable, 
// inactivity, FSS, overrun, FIFO threshold, temperature data ready, accel data ready, gyro data ready signals, boot status
// INT2_CTRL (0Dh) INT2_A/G pin control register. (for pin 2)
//
// INT_GEN_SRC_G (14h) - Angular rate sensor interrupt source register. (interrupt events flags for gyroscope)
// INT_GEN_SRC_XL (26h) - Linear acceleration sensor interrupt source register. (interrupt events flags for gyroscope)
//
// STATUS_REG (17h) - contains inactivity signal flag, accel and gyro interrupt generated flag
// CTRL_REG4 (1Eh) - has the LIR (interrupt latching) and 4D/6D switch 
// STATUS_REG (27h)  ??? identical to STATUS_REG (17h)
// INT_GEN_CFG_G (30h) Angular rate sensor interrupt generator configuration register. (AND/OR, LIR, enable generation on Low/High on X/Y/Z)
// INT_GEN_THS_X_G (31h - 32h) Angular rate sensor interrupt generator threshold registers. The value is expressed as a 15- bit word in two’s complement. 
// INT_GEN_THS_X_G contains also the reset/decrement switch for the counter
// INT_GEN_THS_Y_G (33h - 34h) and INT_GEN_THS_Z_G (35h - 36h) 
// INT_GEN_DUR_G (37h) - duration
//
//
//
// Magnetometer has its own registers for interrupts:
// INT_CFG_M (30h) - enable/disable generation on X/Y/Z axis, interrupt active high/low for the INT_M pin, latching request, and interrupt enable on INT_M pin
// INT_SRC_M (31h) - value exceeds threshold on positive/negative side, internal measurement range overflow, interrupt event occurs flag
// INT_THS_L(32h), INT_THS_H(33h) Interrupt threshold. Default value: 0. The value is expressed in 15-bit unsigned. Even if the threshold is expressed in absolute


//! Various functions related to interrupts
 
use super::*;
 

const PTH_X: u8 = 0b1000_0000;
const PTH_Y: u8 = 0b0100_0000;
const PTH_Z: u8 = 0b0010_0000;
const NTH_X: u8 = 0b0001_0000;
const NTH_Y: u8 = 0b0000_1000;
const NTH_Z: u8 = 0b0000_0100;
const MROI: u8 = 0b0000_0010;
const INT: u8 = 0b0000_0001;

/// Magnetometer interrupt pin (INT_M) settings
#[derive(Debug)]
pub struct IntConfigMag {
    /// Enable interrupt generation on X-axis
    pub interrupt_Xaxis: FLAG,
    /// Enable interrupt generation on Y-axis
    pub interrupt_Yaxis: FLAG,
    /// Enable interrupt generation on Z-axis
    pub interrupt_Zaxis: FLAG,
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
            interrupt_Xaxis: FLAG::Disabled,            
            interrupt_Yaxis: FLAG::Disabled,            
            interrupt_Zaxis: FLAG::Disabled,            
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
        if self.interrupt_Xaxis.status() {
            data |= 1 << 7;
        }
        if self.interrupt_Yaxis.status() {
            data |= 1 << 6;
        }
        if self.interrupt_Zaxis.status() {
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

#[derive(Debug)]
/// Contents of the INT_SOURCE register (interrupt active and differential pressure events flags)
pub struct IntStatusMag {
    pub Xaxis_exceeds_thresh_pos: bool,
    pub Yaxis_exceeds_thresh_pos: bool,
    pub Zaxis_exceeds_thresh_pos: bool,
    pub Xaxis_exceeds_thresh_neg: bool,
    pub Yaxis_exceeds_thresh_neg: bool,
    pub Zaxis_exceeds_thresh_neg: bool,
    pub measurement_range_overflow: bool,
    pub interrupt_occurs: bool,     
}


impl<T, E> LSM9DS1<T>
where
    T: Interface,
    {
    /// Enable interrupts for magnetometer and configure the INT_M interrupt pin     
    pub fn configure_interrupts_mag(&mut self, config: IntConfigMag) -> Result<(), T::Error> {
        self.interface.write(Sensor::Mag, Registers::INT_CFG_M.addr(), config.int_cfg_m())?;                
        Ok(())
    }

    /// Get all the flags from the INT_SRC_M register
    pub fn get_int_status(&mut self) -> Result<IntStatusMag, T::Error> {        
        
        let mut reg_data = [0u8];
        self.interface.read(Sensor::Mag, Registers::INT_SRC_M.addr(), &mut reg_data)?;      

        let status = IntStatusMag {
            
            /// Does value on X-axis exceed the threshold on the positive side?
            Xaxis_exceeds_thresh_pos: match reg_value & PTH_X {
                0 => false,
                _ => true,
            },
            /// Does value on Y-axis exceed the threshold on the positive side?
            Yaxis_exceeds_thresh_pos: match reg_value & PTH_Y {
                0 => false,
                _ => true,
            },
            /// Does value on Z-axis exceed the threshold on the positive side?
            Zaxis_exceeds_thresh_pos: match reg_value & PTH_Z {
                0 => false,
                _ => true,
            },
            /// Does value on X-axis exceed the threshold on the negative side?
            Xaxis_exceeds_thresh_neg: match reg_value & NTH_X {
                0 => false,
                _ => true,
            },
            /// Does value on Y-axis exceed the threshold on the negative side?
            Yaxis_exceeds_thresh_neg: match reg_value & NTH_Y {
                0 => false,
                _ => true,
            },
            /// Does value on Z-axis exceed the threshold on the negative side?
            Zaxis_exceeds_thresh_neg: match reg_value & NTH_Z {
                0 => false,
                _ => true,
            },
            /// Did internal measurement range overflow on magnetic value?
            measurement_range_overflow: match reg_value & MROI {
                0 => false,
                _ => true,
            },
            /// This bit signals when the interrupt event occurs.
            interrupt_occurs: match reg_value & INT {
                0 => false,
                _ => true,
            },
        };
        Ok(status)
    }
}
