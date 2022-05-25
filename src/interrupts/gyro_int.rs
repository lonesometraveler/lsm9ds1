/// Functions related to gyroscope-specific interrupts
///
/// TO DO:
/// - complete gyroscope threshold setting for X, Y and Z axis (INT_GEN_THS_X/Y/Z_G)
/// - ORIENT_CFG_G settings (user orientation selection (???)) -> to be done in gyro.rs
///
use super::*;

/// Gyroscope interrupt generator settings
#[derive(Debug)]
pub struct IntConfigGyro {
    /// Combination of gyroscope interrupt events
    pub events_combination: COMBINATION,
    /// Latch interrupt request
    pub latch_interrupts: INT_LATCH,
    /// Enable interrupt generation on Z-axis (yaw) high event
    pub interrupt_high_zaxis: FLAG,
    /// Enable interrupt generation on Z-axis (yaw) low event
    pub interrupt_low_zaxis: FLAG,
    /// Enable interrupt generation on Y-axis (roll) high event
    pub interrupt_high_yaxis: FLAG,
    /// Enable interrupt generation on Y-axis (roll) low event
    pub interrupt_low_yaxis: FLAG,
    /// Enable interrupt generation on X-axis (pitch) high event
    pub interrupt_high_xaxis: FLAG,
    /// Enable interrupt generation on X-axis (pitch) low event
    pub interrupt_low_xaxis: FLAG,
}

impl Default for IntConfigGyro {
    fn default() -> Self {
        IntConfigGyro {
            events_combination: COMBINATION::OR,
            latch_interrupts: INT_LATCH::NotLatched,
            interrupt_high_zaxis: FLAG::Disabled,
            interrupt_low_zaxis: FLAG::Disabled,
            interrupt_high_yaxis: FLAG::Disabled,
            interrupt_low_yaxis: FLAG::Disabled,
            interrupt_high_xaxis: FLAG::Disabled,
            interrupt_low_xaxis: FLAG::Disabled,
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

#[allow(non_camel_case_types)]
pub struct G_CFG_Bitmasks;
#[allow(dead_code)]
/// Bitmasks for interrupt-related settings in INT_GEN_CFG_G register
impl G_CFG_Bitmasks {
    pub(crate) const AOI_G: u8 = 0b1000_0000;
    pub(crate) const LIR_G: u8 = 0b0100_0000;
    pub(crate) const ZHIE_G: u8 = 0b0010_0000;
    pub(crate) const ZLIE_G: u8 = 0b0001_0000;
    pub(crate) const YHIE_G: u8 = 0b0000_1000;
    pub(crate) const YLIE_G: u8 = 0b0000_0100;
    pub(crate) const XHIE_G: u8 = 0b0000_0010;
    pub(crate) const XLIE_G: u8 = 0b0000_0001;
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
    /// Enable and configure interrupts for gyroscope
    pub fn configure_interrupts_gyro(&mut self, config: IntConfigGyro) -> Result<(), T::Error> {
        self.interface.write(
            Sensor::Gyro,
            register::AG::INT_GEN_CFG_G.addr(),
            config.int_gen_cfg_g(),
        )?;
        Ok(())
    }

    /// Get the current gyroscope interrupts configuration
    pub fn get_gyro_int_config(&mut self) -> Result<IntConfigGyro, T::Error> {
        let reg_value = self.read_register(Sensor::Gyro, register::AG::INT_GEN_CFG_G.addr())?;

        let config = IntConfigGyro {
            events_combination: match (reg_value & G_CFG_Bitmasks::AOI_G) >> 7 {
                1 => COMBINATION::AND,
                _ => COMBINATION::OR,
            },
            latch_interrupts: match (reg_value & G_CFG_Bitmasks::LIR_G) >> 6 {
                1 => INT_LATCH::Latched,
                _ => INT_LATCH::NotLatched,
            },
            interrupt_high_zaxis: match (reg_value & G_CFG_Bitmasks::ZHIE_G) >> 5 {
                1 => FLAG::Enabled,
                _ => FLAG::Disabled,
            },
            interrupt_low_zaxis: match (reg_value & G_CFG_Bitmasks::ZLIE_G) >> 4 {
                1 => FLAG::Enabled,
                _ => FLAG::Disabled,
            },
            interrupt_high_yaxis: match (reg_value & G_CFG_Bitmasks::YHIE_G) >> 3 {
                1 => FLAG::Enabled,
                _ => FLAG::Disabled,
            },
            interrupt_low_yaxis: match (reg_value & G_CFG_Bitmasks::YLIE_G) >> 2 {
                1 => FLAG::Enabled,
                _ => FLAG::Disabled,
            },
            interrupt_high_xaxis: match (reg_value & G_CFG_Bitmasks::XHIE_G) >> 1 {
                1 => FLAG::Enabled,
                _ => FLAG::Disabled,
            },
            interrupt_low_xaxis: match reg_value & G_CFG_Bitmasks::XLIE_G {
                1 => FLAG::Enabled,
                _ => FLAG::Disabled,
            },
        };
        Ok(config)
    }

    // === SINGLE SETTERS ===

    /// Set AND/OR combination of the gyroscope interrupt events
    pub fn set_gyro_events_combination(&mut self, setting: COMBINATION) -> Result<(), T::Error> {
        let reg_value = self.read_register(Sensor::Gyro, register::AG::INT_GEN_CFG_XL.addr())?;

        let mut data: u8 = reg_value & !G_CFG_Bitmasks::AOI_G; // clear the specific bit

        data = match setting {
            COMBINATION::AND => data | (1 << 7), // if Enabled, set bit
            COMBINATION::OR => data,             // if Disabled, bit is cleared
        };

        self.interface
            .write(Sensor::Gyro, register::AG::INT_GEN_CFG_XL.addr(), data)?;

        Ok(())
    }

    /// Latch gyroscope interrupt request
    pub fn set_gyro_int_latching(&mut self, setting: INT_LATCH) -> Result<(), T::Error> {
        let reg_value = self.read_register(Sensor::Gyro, register::AG::INT_GEN_CFG_G.addr())?;

        let mut data: u8 = reg_value & !G_CFG_Bitmasks::LIR_G; // clear the specific bit

        data = match setting {
            INT_LATCH::Latched => data | (1 << 6), // if Enabled, set bit
            INT_LATCH::NotLatched => data,         // if Disabled, bit is cleared
        };

        self.interface
            .write(Sensor::Gyro, register::AG::INT_GEN_CFG_G.addr(), data)?;

        Ok(())
    }

    /// Enable interrupt generation on gyroscope’s Z-axis high event
    pub fn set_gyro_interrupt_high_zaxis(&mut self, setting: FLAG) -> Result<(), T::Error> {
        let reg_value = self.read_register(Sensor::Gyro, register::AG::INT_GEN_CFG_G.addr())?;

        let mut data: u8 = reg_value & !G_CFG_Bitmasks::ZHIE_G; // clear the specific bit

        data = match setting {
            FLAG::Enabled => data | (1 << 5), // if Enabled, set bit
            FLAG::Disabled => data,           // if Disabled, bit is cleared
        };

        self.interface
            .write(Sensor::Gyro, register::AG::INT_GEN_CFG_G.addr(), data)?;

        Ok(())
    }

    /// Enable interrupt generation on gyroscope’s Z-axis high event
    pub fn set_gyro_interrupt_low_zaxis(&mut self, setting: FLAG) -> Result<(), T::Error> {
        let reg_value = self.read_register(Sensor::Gyro, register::AG::INT_GEN_CFG_G.addr())?;

        let mut data: u8 = reg_value & !G_CFG_Bitmasks::ZLIE_G; // clear the specific bit

        data = match setting {
            FLAG::Enabled => data | (1 << 4), // if Enabled, set bit
            FLAG::Disabled => data,           // if Disabled, bit is cleared
        };

        self.interface
            .write(Sensor::Gyro, register::AG::INT_GEN_CFG_G.addr(), data)?;

        Ok(())
    }

    /// Enable interrupt generation on gyroscope’s Y-axis high event
    pub fn set_gyro_interrupt_high_yaxis(&mut self, setting: FLAG) -> Result<(), T::Error> {
        let reg_value = self.read_register(Sensor::Gyro, register::AG::INT_GEN_CFG_G.addr())?;

        let mut data: u8 = reg_value & !G_CFG_Bitmasks::YHIE_G; // clear the specific bit

        data = match setting {
            FLAG::Enabled => data | (1 << 3), // if Enabled, set bit
            FLAG::Disabled => data,           // if Disabled, bit is cleared
        };

        self.interface
            .write(Sensor::Gyro, register::AG::INT_GEN_CFG_G.addr(), data)?;

        Ok(())
    }

    /// Enable interrupt generation on gyroscope’s Y-axis high event
    pub fn set_gyro_interrupt_low_yaxis(&mut self, setting: FLAG) -> Result<(), T::Error> {
        let reg_value = self.read_register(Sensor::Gyro, register::AG::INT_GEN_CFG_G.addr())?;

        let mut data: u8 = reg_value & !G_CFG_Bitmasks::YLIE_G; // clear the specific bit

        data = match setting {
            FLAG::Enabled => data | (1 << 2), // if Enabled, set bit
            FLAG::Disabled => data,           // if Disabled, bit is cleared
        };

        self.interface
            .write(Sensor::Gyro, register::AG::INT_GEN_CFG_G.addr(), data)?;

        Ok(())
    }

    /// Enable interrupt generation on gyroscope’s X-axis high event
    pub fn set_gyro_interrupt_high_xaxis(&mut self, setting: FLAG) -> Result<(), T::Error> {
        let reg_value = self.read_register(Sensor::Gyro, register::AG::INT_GEN_CFG_G.addr())?;

        let mut data: u8 = reg_value & !G_CFG_Bitmasks::XHIE_G; // clear the specific bit

        data = match setting {
            FLAG::Enabled => data | (1 << 1), // if Enabled, set bit
            FLAG::Disabled => data,           // if Disabled, bit is cleared
        };

        self.interface
            .write(Sensor::Gyro, register::AG::INT_GEN_CFG_G.addr(), data)?;

        Ok(())
    }

    /// Enable interrupt generation on gyroscope’s X-axis high event
    pub fn set_gyro_interrupt_low_xaxis(&mut self, setting: FLAG) -> Result<(), T::Error> {
        let reg_value = self.read_register(Sensor::Gyro, register::AG::INT_GEN_CFG_G.addr())?;

        let mut data: u8 = reg_value & !G_CFG_Bitmasks::XLIE_G; // clear the specific bit

        data = match setting {
            FLAG::Enabled => data | 1, // if Enabled, set bit
            FLAG::Disabled => data,    // if Disabled, bit is cleared
        };

        self.interface
            .write(Sensor::Gyro, register::AG::INT_GEN_CFG_G.addr(), data)?;

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
        self.interface
            .write(Sensor::Gyro, register::AG::REFERENCE_G.addr(), value)?;
        Ok(())
    }

    /// Get gyroscope reference value for digital high-pass filter.
    pub fn read_hipass_ref(&mut self) -> Result<u8, T::Error> {
        let data: u8 = self.read_register(Sensor::Gyro, register::AG::REFERENCE_G.addr())?;
        Ok(data)
    }

    /// Gyroscope interrupt duration
    /// Enable/disable wait function and define duration (for how many samples to wait before exiting interrupt)    
    pub fn gyro_int_duration(&mut self, wait: FLAG, duration: u8) -> Result<(), T::Error> {
        // read the current value of the register

        let mut reg_value = self.read_register(Sensor::Gyro, register::AG::INT_GEN_DUR_G.addr())?;

        match wait {
            FLAG::Enabled => reg_value & !0b1000_0000 | 0b1000_0000, // set bit
            FLAG::Disabled => reg_value & !0b1000_0000,              // clear bit
        };

        let duration: u8 = match duration {
            // clamp duration to 7 bit values
            0..=127 => duration,
            _ => 127,
        };

        reg_value &= !0b0111_1111; // clear the lowest 7 bits

        reg_value |= duration;

        self.interface
            .write(Sensor::Gyro, register::AG::INT_GEN_DUR_G.addr(), reg_value)?;

        Ok(())
    }

    /// Set gyroscope mode during inactivity, activation threshold and inactivity duration
    pub fn set_gyro_inactivity(
        &mut self,
        setting: FLAG,
        threshold: u8,
        duration: u8,
    ) -> Result<(), T::Error> {
        let mut data: u8 = 0;

        match setting {
            FLAG::Enabled => data | (1 << 7),
            FLAG::Disabled => data,
        };

        match threshold {
            0..=127 => data | threshold,
            _ => data | 127,
        };

        self.interface
            .write(Sensor::Gyro, register::AG::ACT_THS.addr(), data)?;

        self.interface
            .write(Sensor::Gyro, register::AG::ACT_DUR.addr(), duration)?;

        Ok(())
    }

    // == COMPLETE THIS FUNCTION ==

    /// Set threshold in ?
    pub fn set_gyro_threshold(
        &mut self,
        //threshold: f32
        x_ths: u16,
        y_ths: u16,
        z_ths: u16,
    ) -> Result<(), T::Error> {
        // let sensitivity = self.mag.scale.sensitivity();
        // let mut data = threshold / sensitivity;

        let mut x_data = x_ths;

        // get the current content of the INT_GEN_THS_XH_G (to keep the DCRM_G bit value)
        let reg_x_high =
            self.read_register(Sensor::Accelerometer, register::AG::INT_GEN_THS_XH_G.addr())?;

        // make sure it's not more than 15 bits, and it must be a positive value
        if x_data >= 32767 {
            x_data = 32767;
        } else if x_data < 0 {
            x_data = 0;
        }

        // THIS SHOULD BE DONE IN TWO STEPS MAYBE? FIRST ZERO THE UPPER BITS

        let x_data_low = x_data & 255;

        let x_data_low = x_data_low as u8;

        let mut x_data_high = reg_x_high & !0b1000_0000; // keep the highest bit

        x_data_high |= ((x_data as u16) >> 8) as u8;

        self.interface.write(
            Sensor::Gyro,
            register::AG::INT_GEN_THS_XH_G.addr(),
            x_data_high,
        )?;
        self.interface.write(
            Sensor::Gyro,
            register::AG::INT_GEN_THS_XL_G.addr(),
            x_data_low,
        )?;

        let mut y_data = y_ths;

        // make sure it's not more than 15 bits, and it must be a positive value
        if y_data >= 32767 {
            y_data = 32767;
        } else if y_data < 0 {
            y_data = 0;
        }

        // THIS SHOULD BE DONE IN TWO STEPS MAYBE? FIRST ZERO THE UPPER BITS

        let y_data_low = y_data & 255;

        let y_data_low = y_data_low as u8;

        // let mut y_data_high = y_reg_high & !0b1000_0000; // keep the highest bit

        let y_data_high = (y_data >> 8) as u8;

        self.interface.write(
            Sensor::Gyro,
            register::AG::INT_GEN_THS_YH_G.addr(),
            y_data_high,
        )?;
        self.interface.write(
            Sensor::Gyro,
            register::AG::INT_GEN_THS_YL_G.addr(),
            y_data_low,
        )?;

        let mut z_data = z_ths;

        // make sure it's not more than 15 bits, and it must be a positive value
        if z_data >= 32767 {
            z_data = 32767;
        } else if z_data < 0 {
            z_data = 0;
        }

        // THIS SHOULD BE DONE IN TWO STEPS MAYBE? FIRST ZERO THE UPPER BITS

        let z_data_low = z_data & 255;

        let z_data_low = z_data_low as u8;

        // let mut y_data_high = y_reg_high & !0b1000_0000; // keep the highest bit

        let z_data_high = (z_data >> 8) as u8;

        self.interface.write(
            Sensor::Gyro,
            register::AG::INT_GEN_THS_ZH_G.addr(),
            z_data_high,
        )?;
        self.interface.write(
            Sensor::Gyro,
            register::AG::INT_GEN_THS_ZL_G.addr(),
            z_data_low,
        )?;

        Ok(())
    }

    pub fn get_gyro_threshold(&mut self) -> Result<(u16, u16, u16), T::Error> {
        // let sensitivity = self.mag.scale.sensitivity();

        let mut data = [0u8; 6];

        self.interface.read(
            Sensor::Gyro,
            register::AG::INT_GEN_THS_XH_G.addr(),
            &mut data,
        )?;

        let x: u16 = ((data[0] & !0b0111_1111) as u16) << 8 | data[1] as u16;
        let y: u16 = ((data[2] & !0b0111_1111) as u16) << 8 | data[3] as u16;
        let z: u16 = ((data[4] & !0b0111_1111) as u16) << 8 | data[5] as u16;

        Ok((x, y, z))
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
