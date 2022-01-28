// STATUS_REG (17h) - contains inactivity signal flag, accel and gyro interrupt generated flag
// CTRL_REG4 (1Eh) - has the LIR (interrupt latching) and 4D/6D switch 
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
//! 
//! TO DO:
//! - CHECK IF ALL FUNCTIONS ARE IMPLEMENTED
//! - MAKE SURE REGISTERS ARE NOT OVERWRITTEN BY MISTAKE
//! 
//! FUNCTIONS MISSING:
//! - accelerometer interrupt threshold setting (should it be a struct?)
//! - accelerometer interrupt threshold reading
//! - IG_XL & IG_G reading (if necessary)
//! - CTRL_REG4 LIR_XL1 and 4D_XL1 setting/reading
//! - gyroscope interrupt threshold setting (should it be a struct?)
//! - gyroscope interrupt threshold reading


use super::*;
 
#[allow(non_camel_case_types)]
pub struct M_INT_Bitmasks;

#[allow(dead_code)]
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

// Interrupt active setting for the INT_DRDY pin: active high (default) or active low
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy)]
pub enum INT_ACTIVE {
    /// Active high
    High,
    /// Active low
    Low,
}

impl INT_ACTIVE {
    pub fn status(self) -> bool {
        let status = match self {
            INT_ACTIVE::High => false,
            INT_ACTIVE::Low => true,
        };
        status
    }
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
     




    // --- ACCELEROMETER INTERRUPTS FUNCTIONS START HERE --- 

    // ACCELEROMETER INTERRUPT GENERATION CONFIGURATION

    /// Accelerometer interrupt generation settings
    #[derive(Debug)]
    pub struct IntConfigAccel {
        /// Combination of accelerometer's interrupt events
        pub events_combination: COMBINATION
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
            if self.zaxis_high_event.status() {
                data |= 1 << 5;
            }
            if self.zaxis_low_event.status() {
                data |= 1 << 4;
            }
            if self.yaxis_high_event.status() {
                data |= 1 << 3;
            }
            if self.yaxis_low_event.status() {
                data |= 1 << 2;
            }
            if self.xaxis_high_event.status() {
                data |= 1 << 1;
            }
            if self.xaxis_low_event.status() {
                data |= 1;
            }
        }

    /// Enable and configure interrupts for accelrometer
    pub fn configure_interrupts_accel(&mut self, config: IntConfigAccel) -> Result<(), T::Error> {
        self.interface.write(Sensor::Accelerometer, register::AG::INT_GEN_CFG_XL.addr(), config.int_gen_cfg_xl)?;                
        Ok(())
    }
    


    // interrupt threshold: 3 8-bit values, could be a tuple or a struct
    // registers are INT_GEN_THS_X_XL, INT_GEN_THS_Y_XL, INT_GEN_THS_Z_XL,
    // and they are r/w so the struct could be used for both reading and setting
    pub struct AccelIntThresh {
        x: u8,
        y: u8, 
        z: u8
        }

    impl Default for AccelIntThresh {
        fn default() -> Self {
            AccelIntThresh {                    
                x: 0u8,
                y: 0u8,
                z: 0u8,
                }
            }
        }


    /// accelerometer interrupt duration
    // set in INT_GEN_DUR_XL register
    pub fn accel_int_duration(&mut self, wait: FLAG, duration: u8) -> Result<(), T::Error()> {
        // read the current value of the register
        
        let mut reg_value = self.read_register(Sensor::Accelerometer, register::AG::INT_GEN_DUR_XL.addr())?;

        match wait {
            FLAG::Enabled => reg_value & !0b1000_0000 | 0b1000_0000, // set bit
            FLAG::Disabled => reg_value & !0b1000_0000, // clear bit
        }

        let duration = duration & !0b1000_0000;

        reg_value =& !0b0111_1111;

        reg_value |= duration; // need to make sure duration is 7 bit only!

        self.interface.write(Sensor::Accelerometer, register::AG::INT_GEN_DUR_XL.addr(), reg_value);

    }


    // ACCELEROMETER INTERRUPT STATUS
    
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

    /// Get all the flags from the INT_GEN_SRC_XL register
    pub fn accel_int_status(&mut self) -> Result<IntStatusAccel, T::Error> {        
            
        let reg_data: u8 = self.read_register(Sensor::Accelerometer, register::AG::INT_GEN_SRC_XL.addr())?;

        let status = IntStatusAccel {            
            /// This bit signals whether one or more interrupt events occured.
            interrupt_active: match reg_data & G_INT_Bitmasks::IA_XL {
                0 => false,
                _ => true,
            },
            /// X-axis high event has occurred
            xaxis_high_event: match reg_data & G_INT_Bitmasks::XH_XL {
                0 => false,
                _ => true,
            },
            /// X-axis low event has occurred
            xaxis_low_event: match reg_data & G_INT_Bitmasks::XL_XL {
                0 => false,
                _ => true,
            },
            /// Y-axis high event has occurred
            yaxis_high_event: match reg_data & G_INT_Bitmasks::YH_XL {
                0 => false,
                _ => true,
            },
            /// Y-axis low event has occurred
            yaxis_low_event: match reg_data & G_INT_Bitmasks::YL_XL {
                0 => false,
                _ => true,
            },
            /// Z-axis high event has occurred
            zaxis_high_event: match reg_data & G_INT_Bitmasks::ZH_XL {
                0 => false,
                _ => true,
            },
            /// X-axis low event has occurred
            zaxis_low_event: match reg_data & G_INT_Bitmasks::ZL_XL {
                0 => false,
                _ => true,
            },                
        };
        Ok(status)
    }



    // --- GYROSCOPE INTERRUPTS FUNCTIONS START HERE --- 

    // Angular rate sensor interrupt generator configuration register.
    //
    // AND/OR combination of gyroscope’s interrupt events.
    // Latch Gyroscope interrupt request.
    // enable interrupt generation for high / low events on X, Y, Z axis

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
            if self.zaxis_high_event.status() {
                data |= 1 << 5;
            }
            if self.zaxis_low_event.status() {
                data |= 1 << 4;
            }
            if self.yaxis_high_event.status() {
                data |= 1 << 3;
            }
            if self.yaxis_low_event.status() {
                data |= 1 << 2;
            }
            if self.xaxis_high_event.status() {
                data |= 1 << 1;
            }
            if self.xaxis_low_event.status() {
                data |= 1;
            }
        }

    /// Enable and configure interrupts for gyroscope
    pub fn configure_interrupts_gyro(&mut self, config: IntConfigGyro) -> Result<(), T::Error> {
        self.interface.write(Sensor::Gyroscope, register::AG::INT_GEN_CFG_G.addr(), config.int_gen_cfg_g)?;                
        Ok(())
    }





    /// Set gyroscope reference value for digital high-pass filter.
    pub fn set_hipass_ref(&mut self, value: u8) -> Result<(), T::Error> {
        self.interface.write(Sensor::Gyro, register::AG::REFERENCE_G.addr(), value)?;
        Ok(())
    }

    /// Set gyroscope reference value for digital high-pass filter.
    pub fn read_hipass_ref(&mut self) -> Result<u8, T::Error> {
        let data: u8 = self.interface.read(Sensor::Gyro, register::AG::REFERENCE_G.addr())?;
        Ok(data)
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

    /// gyroscope interrupt duration
    // set in INT_GEN_DUR_G register
    pub fn gyro_int_duration(&mut self, wait: FLAG, duration: u8) -> Result<(), T::Error()> {
        // read the current value of the register
        
        let mut reg_value = self.read_register(Sensor::Gyro, register::AG::INT_GEN_DUR_G.addr())?;

        match wait {
            FLAG::Enabled => reg_value & !0b1000_0000 | 0b1000_0000, // set bit
            FLAG::Disabled => reg_value & !0b1000_0000, // clear bit
        }

        let duration = duration & !0b1000_0000;

        reg_value =& !0b0111_1111;

        reg_value |= duration; // need to make sure duration is 7 bit only!

        self.interface.write(Sensor::Gyro, register::AG::INT_GEN_DUR_G.addr(), reg_value);

    }
     
   


    // STATUS_REG HAS IG_XL and IG_G BITS - ARE THEY THE SAME AS IA_XL AND IA_G? AWAITING ANSWERS ON ST MICRO FORUM




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

    // WHICH SENSOR SHOULD I USE HERE? IT'S BOTH ACCEL AND GYRO!

    /// Enable interrupts for accelerometer/gyroscope and configure the INT1_A/G interrupt pin     
    pub fn configure_interrupts_ag1(&mut self, config: IntConfigAG1) -> Result<(), T::Error> {
        self.interface.write(Sensor::Accelerometer, register::AG::INT1_CTRL.addr(), config.int1_ctrl())?;                
        Ok(())
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

    // WHICH SENSOR SHOULD I USE HERE? IT'S BOTH ACCEL AND GYRO!

    /// Enable interrupts for accelerometer/gyroscope and configure the INT1_A/G interrupt pin     
    pub fn configure_interrupts_ag2(&mut self, config: IntConfigAG2) -> Result<(), T::Error> {
        
        let reg_data = self.read_register(Sensor::Accelerometer, register::AG::INT2_CTRL.addr())?;
        
        let mut data: u8 = reg_data & !0b1100_0000;

        data |= config.int2_ctrl();

        self.interface.write(Sensor::Accelerometer, register::AG::INT2_CTRL.addr(), data)?;                
        Ok(())
    }



}



