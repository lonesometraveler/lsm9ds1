//! Accelerometer settings, types
#![allow(dead_code, non_camel_case_types)]

/// Accelerometer settings
#[derive(Debug)]
pub struct AccelSettings {
    pub enable_x: bool,
    pub enable_y: bool,
    pub enable_z: bool,
    pub sample_rate: ODR,
    pub scale: Scale,
    pub bandwidth_selection: BandwidthSelection,
    pub bandwidth: Bandwidth,
    pub high_res_bandwidth: HighRes,
}

impl Default for AccelSettings {
    fn default() -> Self {
        AccelSettings {
            enable_x: true,
            enable_y: true,
            enable_z: true,
            sample_rate: ODR::_119Hz,
            scale: Scale::_2G,
            bandwidth_selection: BandwidthSelection::ByODR,
            bandwidth: Bandwidth::_408Hz,
            high_res_bandwidth: HighRes::Disabled,
        }
    }
}

impl AccelSettings {
    /// return the default setting
    pub fn new() -> AccelSettings {
        Default::default()
    }

    /// CTRL_REG5_XL (0x1F) (Default value: 0x38)
    /// [DEC_1][DEC_0][Zen_XL][Yen_XL][Xen_XL][0][0][0]
    /// DEC[0:1] - Decimation of accel data on OUT REG and FIFO.
    /// 00: None, 01: 2 samples, 10: 4 samples 11: 8 samples
    /// Zen_XL - Z-axis output enabled
    /// Yen_XL - Y-axis output enabled
    /// Xen_XL - X-axis output enabled
    pub fn ctrl_reg5_xl(&self) -> u8 {
        let mut result = 0_u8;
        if self.enable_z {
            result |= 1 << 5;
        }
        if self.enable_y {
            result |= 1 << 4;
        }
        if self.enable_x {
            result |= 1 << 3;
        }
        result
    }

    /// CTRL_REG6_XL (0x20) (Default value: 0x00)
    /// [ODR_XL2][ODR_XL1][ODR_XL0][FS1_XL][FS0_XL][BW_SCAL_ODR][BW_XL1][BW_XL0]
    /// ODR_XL[2:0] - Output data rate & power mode selection
    /// FS_XL[1:0] - Full-scale selection
    /// BW_SCAL_ODR - Bandwidth selection
    /// BW_XL[1:0] - Anti-aliasing filter bandwidth selection
    pub fn ctrl_reg6_xl(&self) -> u8 {
        self.sample_rate.value()
            | self.scale.value()
            | self.bandwidth_selection.value()
            | self.bandwidth.value()
    }

    /// CTRL_REG7_XL (0x21) (Default value: 0x00)
    /// [HR][DCF1][DCF0][0][0][FDS][0][HPIS1]
    /// HR - High resolution mode (0: disable, 1: enable)
    /// DCF[1:0] - Digital filter cutoff frequency
    /// FDS - Filtered data selection
    /// HPIS1 - HPF enabled for interrupt function
    pub fn ctrl_reg7_xl(&self) -> u8 {
        self.high_res_bandwidth.value()
    }
}

/// Accelerometer full-scale selection. Default value: 00. (Refer to Table 67)
#[derive(Debug, Clone, Copy)]
pub enum Scale {
    /// 2g
    _2G = 0b00,
    /// 16g
    _16G = 0b01,
    /// 4g
    _4G = 0b10,
    /// 8g
    _8G = 0b11,
}

impl Scale {
    pub fn value(self) -> u8 {
        (self as u8) << 3
    }

    /// return Linear acceleration sensitivity depending on scale. (Refer to Page 12)
    pub fn sensitivity(self) -> f32 {
        match self {
            Scale::_2G => 0.000_061,
            Scale::_4G => 0.000_122,
            Scale::_8G => 0.000_244,
            Scale::_16G => 0.000_732,
        }
    }
}

/// Output data rate and power mode selection (ODR_XL). default value: 000 (Refer to Table 68)
#[derive(Debug, Clone, Copy)]
pub enum ODR {
    /// Power-down mode
    PowerDown = 0b000,
    /// 10 Hz
    _10Hz = 0b001,
    /// 50 Hz
    _50Hz = 0b010,
    /// 119 Hz
    _119Hz = 0b011,
    /// 238 Hz
    _238Hz = 0b100,
    /// 476 Hz
    _476Hz = 0b101,
    /// 952 Hz
    _952Hz = 0b110,
}

impl ODR {
    pub fn value(self) -> u8 {
        (self as u8) << 5
    }
}

/// Bandwidth selection. Default value: 0. (Refer to Table 67)
#[derive(Debug)]
pub enum BandwidthSelection {
    ByODR,
    ByBW,
}

impl BandwidthSelection {
    pub fn value(&self) -> u8 {
        match self {
            BandwidthSelection::ByODR => 0 << 2,
            BandwidthSelection::ByBW => 1 << 2,
        }
    }
}

/// Anti-aliasing filter bandwidth selection (BW_XL). Default value: 00. (Refer to Table 67)
#[derive(Debug, Clone, Copy)]
pub enum Bandwidth {
    /// 408 Hz
    _408Hz = 0b00,
    /// 211 Hz
    _211Hz = 0b01,
    /// 105 Hz
    _105Hz = 0b10,
    ///  50 Hz
    _50Hz = 0b11,
}

impl Bandwidth {
    pub fn value(self) -> u8 {
        self as u8
    }
}

/// Accelerometer digital filter (high pass and low pass) cutoff frequency selection:
/// the band- width of the high-pass filter depends on the selected ODR. (Refer to Table 71)
#[derive(Debug, Clone, Copy)]
pub enum HighRes {
    Disabled = 0b000,
    ODR_50 = 0b100,
    ODR_100 = 0b101,
    ODR_9 = 0b110,
    ODR_400 = 0b111,
}

impl HighRes {
    pub fn value(self) -> u8 {
        (self as u8) << 5
    }
}

#[test]
fn accel_init_values() {
    let settings = AccelSettings::new();
    assert_eq!(settings.ctrl_reg5_xl(), 0b0011_1000); // [DEC_1][DEC_0][Zen_XL][Yen_XL][Zen_XL][0][0][0]
    assert_eq!(settings.ctrl_reg6_xl(), 0b0110_0000); // [ODR_XL2][ODR_XL1][ODR_XL0][FS1_XL][FS0_XL][BW_SCAL_ODR][BW_XL1][BW_XL0]
    assert_eq!(settings.ctrl_reg7_xl(), 0b0000_0000); // [HR][DCF1][DCF0][0][0][FDS][0][HPIS1]
}