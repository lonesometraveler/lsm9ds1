#![allow(dead_code, non_camel_case_types)]

/// Accelerometer settings
#[derive(Debug)]
pub struct AccelSettings {
    pub enable_x: bool,
    pub enable_y: bool,
    pub enable_z: bool,
    pub scale: AccelScale,
    pub sample_rate: AccelODR,
    pub bandwidth_selection: AccelBandwidthSelection,
    pub bandwidth: AccelBandwidth,
    pub high_res_bandwidth: HighRes,
}

impl Default for AccelSettings {
    fn default() -> Self {
        AccelSettings {
            enable_x: true,
            enable_y: true,
            enable_z: true,
            scale: AccelScale::LA_FS_2G,
            sample_rate: AccelODR::ODR_119,
            bandwidth_selection: AccelBandwidthSelection::ByODR,
            bandwidth: AccelBandwidth::BW_408,
            high_res_bandwidth: HighRes::Disabled,
        }
    }
}

impl AccelSettings {
    /// Create a setting
    pub fn new() -> AccelSettings {
        Default::default()
    }

    ///    CTRL_REG5_XL (0x1F) (Default value: 0x38)
    ///    [DEC_1][DEC_0][Zen_XL][Yen_XL][Zen_XL][0][0][0]
    ///    DEC[0:1] - Decimation of accel data on OUT REG and FIFO.
    ///    00: None, 01: 2 samples, 10: 4 samples 11: 8 samples
    ///    Zen_XL - Z-axis output enabled
    ///    Yen_XL - Y-axis output enabled
    ///    Xen_XL - X-axis output enabled
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

/// Accelerometer full-scale selection. Default value: 00. See table 67.
#[derive(Debug, Clone, Copy)]
pub enum AccelScale {
    /// 2g
    LA_FS_2G = 0b00,
    /// 16g
    LA_FS_16G = 0b01,
    /// 4g
    LA_FS_4G = 0b10,
    /// 8g
    LA_FS_8G = 0b11,
}

impl AccelScale {
    pub fn value(self) -> u8 {
        (self as u8) << 3
    }

    pub fn sensitivity(self) -> f32 {
        match self {
            AccelScale::LA_FS_2G => 0.000_061,
            AccelScale::LA_FS_4G => 0.000_122,
            AccelScale::LA_FS_8G => 0.000_244,
            AccelScale::LA_FS_16G => 0.000_732,
        }
    }
}

/// Output data rate and power mode selection (ODR_XL). default value: 000 (see Table 68)
#[derive(Debug, Clone, Copy)]
pub enum AccelODR {
    /// Power-down mode
    POWER_DOWN = 0b000,
    /// 10 Hz
    ODR_10 = 0b001,
    /// 50 Hz
    ODR_50 = 0b010,
    /// 119 Hz
    ODR_119 = 0b011,
    /// 238 Hz
    ODR_238 = 0b100,
    /// 476 Hz
    ODR_476 = 0b101,
    /// 952 Hz
    ODR_952 = 0b110,
}

impl AccelODR {
    pub fn value(self) -> u8 {
        (self as u8) << 5
    }
}

#[derive(Debug)]
pub enum AccelBandwidthSelection {
    ByODR,
    ByBW,
}

impl AccelBandwidthSelection {
    pub fn value(&self) -> u8 {
        match self {
            AccelBandwidthSelection::ByODR => 0 << 2,
            AccelBandwidthSelection::ByBW => 1 << 2,
        }
    }
}

/// Anti-aliasing filter bandwidth selection (BW_XL). Default value: 00. See table 67
#[derive(Debug, Clone, Copy)]
pub enum AccelBandwidth {
    /// 408 Hz
    BW_408 = 0b00,
    /// 211 Hz
    BW_211 = 0b01,
    /// 105 Hz
    BW_105 = 0b10,
    ///  50 Hz
    BW_50 = 0b11,
}

impl AccelBandwidth {
    pub fn value(self) -> u8 {
        self as u8
    }
}

/// Accelerometer digital filter (high pass and low pass) cutoff frequency selection:
/// the band- width of the high-pass filter depends on the selected ODR. Refer to Table 71
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

#[derive(Debug)]
pub struct GyroSettings {
    pub enabled: bool,
    pub enable_x: bool,
    pub enable_y: bool,
    pub enable_z: bool,
    pub flip_x: bool,
    pub flip_y: bool,
    pub flip_z: bool,
    pub scale: GyroScale,
    pub sample_rate: GyroODR,
    pub bandwidth: GyroBandwidth,
    pub int_selection: GyroIntSelection,
    pub out_selection: GyroOutSelection,
    pub low_power_mode: LowPowerMode,
    pub hpf_mode: HpFilter,
    pub hpf_cutoff: HpFilterCutoff,
    pub latch_interrupt: LatchInterrupt,
    pub orientation: u8,
}

impl Default for GyroSettings {
    fn default() -> Self {
        GyroSettings {
            enabled: true,
            enable_x: true,
            enable_y: true,
            enable_z: true,
            flip_x: false,
            flip_y: false,
            flip_z: false,
            scale: GyroScale::G_FS_245DPS,
            sample_rate: GyroODR::ODR_952,
            bandwidth: GyroBandwidth::LPF_0,
            int_selection: GyroIntSelection::SEL_0,
            out_selection: GyroOutSelection::SEL_0,
            low_power_mode: LowPowerMode::Disabled,
            hpf_mode: HpFilter::Disabled,
            hpf_cutoff: HpFilterCutoff::HPCF_1,
            latch_interrupt: LatchInterrupt::Disabled,
            orientation: 0,
        }
    }
}

impl GyroSettings {
    pub fn new() -> GyroSettings {
        Default::default()
    }

    ///    CTRL_REG1_G (Default value: 0x00), page 45
    ///    [ODR_G2][ODR_G1][ODR_G0][FS_G1][FS_G0][0][BW_G1][BW_G0]
    ///    ODR_G[2:0] - Output data rate selection
    ///    FS_G[1:0] - Gyroscope full-scale selection
    ///    BW_G[1:0] - Gyroscope bandwidth selection
    pub fn crtl_reg1_g(&self) -> u8 {
        self.sample_rate.value()
            | self.scale.value()
            | self.bandwidth.value()
    }

    // CTRL_REG2_G (Default value: 0x00)
	// [0][0][0][0][INT_SEL1][INT_SEL0][OUT_SEL1][OUT_SEL0]
	// INT_SEL[1:0] - INT selection configuration
	// OUT_SEL[1:0] - Out selection configuration
	pub fn crtl_reg2_g(&self) -> u8 {
        self.int_selection.value() | self.out_selection.value()
    }

    ///    CTRL_REG3_G (Default value: 0x00). see page 47
	///    [LP_mode][HP_EN][0][0][HPCF3_G][HPCF2_G][HPCF1_G][HPCF0_G]
	///    LP_mode - Low-power mode enable (0: disabled, 1: enabled)
	///    HP_EN - HPF enable (0:disabled, 1: enabled)
	///    HPCF_G[3:0] - HPF cutoff frequency
    pub fn crtl_reg3_g(&self) -> u8 {
        self.low_power_mode.value() | self.hpf_mode.value() | self.hpf_cutoff.value()
    }

    /// CTRL_REG4 (Default value: 0x38). see page 50
	/// [0][0][Zen_G][Yen_G][Xen_G][0][LIR_XL1][4D_XL1]
	/// Zen_G - Z-axis output enable (0:disable, 1:enable)
	/// Yen_G - Y-axis output enable (0:disable, 1:enable)
	/// Xen_G - X-axis output enable (0:disable, 1:enable)
	/// LIR_XL1 - Latched interrupt (0:not latched, 1:latched)
    /// 4D_XL1 - 4D option on interrupt (0:6D used, 1:4D used) // TODO:
    pub fn ctrl_reg4(&self) -> u8 {
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
        result | self.latch_interrupt.value()
    }

    /// ORIENT_CFG_G (Default value: 0x00)
	/// [0][0][SignX_G][SignY_G][SignZ_G][Orient_2][Orient_1][Orient_0]
	/// SignX_G - Pitch axis (X) angular rate sign (0: positive, 1: negative)
	/// Orient [2:0] - Directional user orientation selection // TODO:
    pub fn orient_cfg_g(&self) -> u8 {
        let mut result = 0_u8;
        if self.flip_x {
            result |= 1 << 5;
        }
        if self.flip_y {
            result |= 1 << 4;
        }
        if self.flip_z {
            result |= 1 << 3;
        }
        result
    }
}

/// gyro_scale defines the possible full-scale ranges of the gyroscope:
#[derive(Debug, Clone, Copy)]
pub enum GyroScale {
    /// 245 degrees per second
    G_FS_245DPS = 0b00,
    /// 500 dps
    G_FS_500DPS = 0b01,
    /// 2000 dps
    G_FS_2000DPS = 0b11,
}

impl GyroScale {
    pub fn value(self) -> u8 {
        (self as u8) << 3
    }

    pub fn sensitivity(&self) -> f32 {
        match self {
            GyroScale::G_FS_245DPS => 0.00875,
            GyroScale::G_FS_500DPS => 0.0175,
            GyroScale::G_FS_2000DPS => 0.07,
        }
    }
}

/// Gyroscope operating modes. See table 9.
#[derive(Debug, Clone, Copy)]
pub enum GyroODR {
    /// Power down (0)
    ODR_PD = 0b000,
    /// 14.9 Hz (1)
    ODR_149 = 0b001,
    /// 59.5 Hz (2)
    ODR_595 = 0b010,
    /// 119 Hz (3)
    ODR_119 = 0b011,
    /// 238 Hz (4)
    ODR_238 = 0b100,
    /// 476 Hz (5)
    ODR_476 = 0b101,
    /// 952 Hz (6)
    ODR_952 = 0b110,
}

impl GyroODR {
    pub fn value(self) -> u8 {
        (self as u8) << 5
    }
}

/// Gyroscope bandwidth selection. Default value: 00 see table 47
#[derive(Debug, Clone, Copy)]
pub enum GyroBandwidth {
    /// 00
    LPF_0 = 0b00,
    /// 01
    LPF_1 = 0b01,
    /// 10
    LPF_2 = 0b10,
    /// 11
    LPF_3 = 0b11,
}

impl GyroBandwidth {
    pub fn value(self) -> u8 {
        self as u8
    }
}

/// Gyroscope bandwidth selection. Default value: 00 see table 47
#[derive(Debug, Clone, Copy)]
pub enum GyroIntSelection {
    /// 00
    SEL_0 = 0b00,
    /// 01
    SEL_1 = 0b01,
    /// 10
    SEL_2 = 0b10,
    /// 11
    SEL_3 = 0b11,
}

impl GyroIntSelection {
    pub fn value(self) -> u8 {
        (self as u8) << 2
    }
}

/// Gyroscope bandwidth selection. Default value: 00 see table 47
#[derive(Debug, Clone, Copy)]
pub enum GyroOutSelection {
    /// 00
    SEL_0 = 0b00,
    /// 01
    SEL_1 = 0b01,
    /// 10
    SEL_2 = 0b10,
    /// 11
    SEL_3 = 0b11,
}

impl GyroOutSelection {
    pub fn value(self) -> u8 {
        self as u8
    }
}

/// Low-power mode enable. Default value: Disabled. see table 51
#[derive(Debug, Clone, Copy)]
pub enum LowPowerMode {
    Disabled = 0,
    Enabled = 1,
}

impl LowPowerMode {
    pub fn value(self) -> u8 {
        (self as u8) << 7
    }
}

/// High-pass filter enable. Default value: Disabled. see table 51
#[derive(Debug, Clone, Copy)]
pub enum HpFilter {
    Disabled = 0,
    Enabled = 1,
}

impl HpFilter {
    pub fn value(self) -> u8 {
        (self as u8) << 6
    }
}

/// Low-power mode enable. Default value: Disabled. see table 51
#[derive(Debug, Clone, Copy)]
pub enum LatchInterrupt {
    Disabled = 0,
    Enabled = 1,
}

impl LatchInterrupt {
    pub fn value(self) -> u8 {
        (self as u8) << 1
    }
}

/// Gyroscope high-pass filter cutoff frequency selection. Default value: 0000. see table 52
#[derive(Debug, Clone, Copy)]
pub enum HpFilterCutoff {
    HPCF_1 = 0b0000,
    HPCF_2 = 0b0001,
    HPCF_3 = 0b0010,
    HPCF_4 = 0b0011,
    HPCF_5 = 0b0100,
    HPCF_6 = 0b0101,
    HPCF_7 = 0b0110,
    HPCF_8 = 0b0111,
    HPCF_9 = 0b1000,
    HPCF_10 = 0b1001,
}

impl HpFilterCutoff {
    pub fn value(self) -> u8 {
        self as u8
    }
}

/// LSM9DS1 Accel/Gyro (XL/G) Registers
#[allow(non_camel_case_types)]
#[derive(Clone, Copy)]
pub enum Register {
    /// Activity threshold register.
    ACT_THS = 0x04,
    /// Inactivity duration register.
    ACT_DUR = 0x05,
    /// Linear acceleration sensor interrupt generator configuration register.
    INT_GEN_CFG_XL = 0x06,
    /// Linear acceleration sensor interrupt threshold register.
    INT_GEN_THS_X_XL = 0x07,
    /// Linear acceleration sensor interrupt threshold register.
    INT_GEN_THS_Y_XL = 0x08,
    /// Linear acceleration sensor interrupt threshold register.
    INT_GEN_THS_Z_XL = 0x09,
    /// Linear acceleration sensor interrupt duration register.
    INT_GEN_DUR_XL = 0x0A,
    /// Angular rate sensor reference value register for digital high-pass filter (r/w).
    REFERENCE_G = 0x0B,
    /// INT1_A/G pin control register.
    INT1_CTRL = 0x0C,
    /// INT2_A/G pin control register.
    INT2_CTRL = 0x0D,
    /// Who_AM_I register.
    WHO_AM_I = 0x0F,
    /// Angular rate sensor Control Register 1.
    CTRL_REG1_G = 0x10,
    /// Angular rate sensor Control Register 2.
    CTRL_REG2_G = 0x11,
    /// Angular rate sensor Control Register 3.
    CTRL_REG3_G = 0x12,
    /// Angular rate sensor sign and orientation register.
    ORIENT_CFG_G = 0x13,
    /// Angular rate sensor interrupt source register.
    INT_GEN_SRC_G = 0x14,
    /// Temperature data output register. L and H registers together express a 16-bit word in two’s complement right-justified.
    OUT_TEMP_L = 0x15,
    OUT_TEMP_H = 0x16,
    /// Status register.
    STATUS_REG_0 = 0x17,
    /// Angular rate sensor pitch axis (X) angular rate output register. The value is expressed as a 16-bit word in two’s complement.
    OUT_X_L_G = 0x18,
    OUT_X_H_G = 0x19,
    /// Angular rate sensor roll axis (Y) angular rate output register. The value is expressed as a 16-bit word in two’s complement.
    OUT_Y_L_G = 0x1A,
    OUT_Y_H_G = 0x1B,
    /// Angular rate sensor Yaw axis (Z) angular rate output register. The value is expressed as a 16-bit word in two’s complement.
    OUT_Z_L_G = 0x1C,
    OUT_Z_H_G = 0x1D,
    /// Control register 4.
    CTRL_REG4 = 0x1E,
    /// Linear acceleration sensor Control Register 5.
    CTRL_REG5_XL = 0x1F,
    /// Linear acceleration sensor Control Register 6.
    CTRL_REG6_XL = 0x20,
    /// Linear acceleration sensor Control Register 7.
    CTRL_REG7_XL = 0x21,
    /// Control register 8.
    CTRL_REG8 = 0x22,
    /// Control register 9.
    CTRL_REG9 = 0x23,
    /// Control register 10.
    CTRL_REG10 = 0x24,
    /// Linear acceleration sensor interrupt source register.
    INT_GEN_SRC_XL = 0x26,
    /// Status register.
    STATUS_REG_1 = 0x27,
    /// Linear acceleration sensor X-axis output register. The value is expressed as a 16-bit word in two’s complement.
    OUT_X_L_XL = 0x28,
    OUT_X_H_XL = 0x29,
    /// Linear acceleration sensor Y-axis output register. The value is expressed as a 16-bit word in two’s complement.
    OUT_Y_L_XL = 0x2A,
    OUT_Y_H_XL = 0x2B,
    /// Linear acceleration sensor Z-axis output register. The value is expressed as a 16-bit word in two’s complement.
    OUT_Z_L_XL = 0x2C,
    OUT_Z_H_XL = 0x2D,
    /// FIFO control register.
    FIFO_CTRL = 0x2E,
    /// FIFO status control register.
    FIFO_SRC = 0x2F,
    /// Angular rate sensor interrupt generator configuration register.
    INT_GEN_CFG_G = 0x30,
    /// Angular rate sensor interrupt generator threshold registers. The value is expressed as a 15- bit word in two’s complement.
    INT_GEN_THS_XH_G = 0x31,
    INT_GEN_THS_XL_G = 0x32,
    /// Angular rate sensor interrupt generator threshold registers. The value is expressed as a 15-bit word in two’s complement.
    INT_GEN_THS_YH_G = 0x33,
    INT_GEN_THS_YL_G = 0x34,
    /// Angular rate sensor interrupt generator threshold registers. The value is expressed as a 15-bit word in two’s complement.
    INT_GEN_THS_ZH_G = 0x35,
    INT_GEN_THS_ZL_G = 0x36,
    /// Angular rate sensor interrupt generator duration register.
    INT_GEN_DUR_G = 0x37,
}

impl Register {
    pub fn addr(self) -> u8 {
        self as u8
    }
}
