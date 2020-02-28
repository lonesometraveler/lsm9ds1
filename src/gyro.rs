#![allow(dead_code, non_camel_case_types)]

#[derive(Debug)]
pub struct GyroSettings {
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

    /// CTRL_REG1_G (Default value: 0x00), page 45
    /// [ODR_G2][ODR_G1][ODR_G0][FS_G1][FS_G0][0][BW_G1][BW_G0]
    /// ODR_G[2:0] - Output data rate selection
    /// FS_G[1:0] - Gyroscope full-scale selection
    /// BW_G[1:0] - Gyroscope bandwidth selection
    pub fn ctrl_reg1_g(&self) -> u8 {
        self.sample_rate.value() | self.scale.value() | self.bandwidth.value()
    }

    // CTRL_REG2_G (Default value: 0x00)
    // [0][0][0][0][INT_SEL1][INT_SEL0][OUT_SEL1][OUT_SEL0]
    // INT_SEL[1:0] - INT selection configuration
    // OUT_SEL[1:0] - Out selection configuration
    pub fn ctrl_reg2_g(&self) -> u8 {
        self.int_selection.value() | self.out_selection.value()
    }

    /// CTRL_REG3_G (Default value: 0x00). see page 47
    /// [LP_mode][HP_EN][0][0][HPCF3_G][HPCF2_G][HPCF1_G][HPCF0_G]
    /// LP_mode - Low-power mode enable (0: disabled, 1: enabled)
    /// HP_EN - HPF enable (0:disabled, 1: enabled)
    /// HPCF_G[3:0] - HPF cutoff frequency
    pub fn ctrl_reg3_g(&self) -> u8 {
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

    pub fn sensitivity(self) -> f32 {
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
