//! Various settings related to FIFO functionality of the sensors
#[allow(non_camel_case_types)]
pub struct FIFOBitmasks;

#[allow(dead_code)]
/// Bitmasks for FIFO-related settings in CTRL_REG9 and CTRL_REG5_XL registers
impl FIFOBitmasks {
    pub(crate) const FTH: u8 = 0b1000_0000;
    /// FIFO_SRC bit 6 OVRN
    pub(crate) const OVRN: u8 = 0b0100_0000;
    /// FIFO_SRC bits 5:0 FSS
    pub(crate) const FSS: u8 = 0b0011_1111;
    /// CTRL_REG9 FIFO-related settings
    pub(crate) const CTRL_REG9_FIFO: u8 = 0b0001_0011;
    /// Decimation setting in CTRL_REG5_XL
    pub(crate) const DEC: u8 = 0b1100_0000;
}

/// FIFO settings
#[derive(Debug)]
pub struct FIFOConfig {
    /// FIFO memory enable
    pub fifo_enable: bool,
    /// Select FIFO operation mode (see Table 84 for details)        
    pub fifo_mode: FIFOMode, // default Bypass
    /// Enable threshold level use
    pub fifo_use_threshold: bool,
    /// Set the threshold level
    pub fifo_threshold: u8,
    /// Store temperature data in FIFO
    pub fifo_temperature_enable: bool,
}

impl Default for FIFOConfig {
    fn default() -> Self {
        FIFOConfig {
            fifo_mode: FIFOMode::FIFO,      // FIFO mode
            fifo_threshold: 32u8,           // set the default threshold level
            fifo_enable: true,              // FIFO enabled
            fifo_temperature_enable: false, // temperature data not stored in FIFO
            fifo_use_threshold: true,       // FIFO depth limited to set threshold
        }
    }
}

impl FIFOConfig {
    /// Returns `u8` to be written to FIFO_CTRL.  
    pub(crate) fn f_fifo_ctrl(&self) -> u8 {
        let mut data = 0u8;

        // clamp the inserted threshold value to minimum 1 and maximum 32.
        // threshold value must be set as t-1, i.e. to set it to 32,
        // value 0b11111 must be written to the register
        let threshold_data = self.fifo_threshold.clamp(1, 32) - 1;

        data |= self.fifo_mode.value();
        data |= threshold_data;
        data
    }

    /// Returns `u8` to be written to CTRL_REG9.
    pub(crate) fn f_ctrl_reg9(&self) -> u8 {
        let mut data = 0u8;
        if self.fifo_temperature_enable {
            data |= 1 << 4;
        }
        if self.fifo_enable {
            data |= 1 << 1;
        }
        if self.fifo_use_threshold {
            data |= 1;
        }
        data
    }
}

#[derive(Debug)]
/// Contents of the FIFO_STATUS register (threshold reached, overrun, empty, stored data level)
pub struct FIFOStatus {
    /// FIFO threshold status. True if FIFO filling is equal or higher than    threshold level
    pub fifo_thresh_reached: bool,
    /// True is FIFO is completely filled and at least one samples has been overwritten
    pub fifo_overrun: bool,
    /// True if FIFO is empty (level = 0)
    pub fifo_empty: bool,
    /// Number of unread samples stored into FIFO
    pub fifo_level: u8,
}

impl From<u8> for FIFOStatus {
    fn from(value: u8) -> Self {
        let fifo_level_value = value & FIFOBitmasks::FSS;
        FIFOStatus {
            /// Is FIFO filling equal or higher than the threshold?
            fifo_thresh_reached: value & FIFOBitmasks::FTH != 0,
            /// Is FIFO full and at least one sample has been overwritten?
            fifo_overrun: value & FIFOBitmasks::OVRN != 0,
            /// Is FIFO empty (no unread samples)?
            fifo_empty: fifo_level_value == 0,
            /// Read FIFO stored data level
            fifo_level: fifo_level_value,
        }
    }
}

/// FIFO mode selection. (Refer to datasheets)
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy)]
pub enum FIFOMode {
    /// Bypass mode (he FIFO is not operational and it remains empty).
    Bypass = 0b000,
    /// FIFO mode (data from the output channels are stored in the FIFO until it is overwritten).
    FIFO = 0b001,
    /// Continuous-to-FIFO mode (continuous mode until trigger is deasserted, then FIFO mode).
    ContinuousToFIFO = 0b011,
    /// Bypass-to-Continuous mode (Bypass mode until trigger is deasserted, then Continuous mode).
    BypassToContinuous = 0b100,
    /// Continuous mode. If the FIFO is full, the new sample overwrites the older sample.
    Continuous = 0b110,
}

impl FIFOMode {
    pub fn value(self) -> u8 {
        (self as u8) << 5 // shifted into the right position, can be used directly
    }
}

/// Decimation of acceleration data on OUT REG and FIFO (Refer to table 65)
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy)]
pub enum Decimate {
    /// No decimation
    NoDecimation = 0b00,
    /// update every 2 samples;
    _2samples = 0b01,
    /// update every 4 samples;
    _4samples = 0b10,
    /// update every 8 samples;
    _8samples = 0b11,
}

impl Decimate {
    pub fn value(self) -> u8 {
        (self as u8) << 6 // shifted to bits [7:6], can be used directly
    }
}
