//! Gyroscope settings, types
#![allow(dead_code, non_camel_case_types)]

/// Gyro settings. Use this struct to configure the sensor.
#[derive(Debug)]
pub struct GyroSettings {
    /// - Xen_G - X-axis output enable (false :disable, true :enable)
    pub enable_x: bool,
    /// - Yen_G - Y-axis output enable (false :disable, true :enable)
    pub enable_y: bool,
    /// - Zen_G - Z-axis output enable (false :disable, true :enable)
    pub enable_z: bool,
    /// Pitch axis (X) angular rate sign (false: positive, true: negative)
    pub flip_x: bool,
    /// - SignY_G - Roll axis (Y) angular rate sign (false: positive, true: negative)
    pub flip_y: bool,
    /// - SignZ_G - Yaw axis (Z) angular rate sign (false: positive, true: negative)
    pub flip_z: bool,
    /// Gyroscope full-scale selection
    pub scale: Scale,
    /// Output data rate selection
    pub sample_rate: ODR,
    /// Bandwidth selection
    pub bandwidth: Bandwidth,
    /// INT selection configuration. See page 47
    pub int_selection: GyroIntSelection,
    /// Out selection configuration. See page 47
    pub out_selection: GyroOutSelection,
    /// Low-power mode enable. See page 47
    pub low_power_mode: LowPowerMode,
    /// HPF enable. See page 47
    pub hpf_mode: HpFilter,
    /// HPF cutoff frequency. See page 47
    pub hpf_cutoff: HpFilterCutoff,
    /// Latched interrupt. See page 50
    pub latch_interrupt: LatchInterrupt,
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
            scale: Scale::_245DPS,
            sample_rate: ODR::_952Hz,
            bandwidth: Bandwidth::LPF_0,
            int_selection: GyroIntSelection::SEL_0,
            out_selection: GyroOutSelection::SEL_0,
            low_power_mode: LowPowerMode::Disabled,
            hpf_mode: HpFilter::Disabled,
            hpf_cutoff: HpFilterCutoff::HPCF_1,
            latch_interrupt: LatchInterrupt::Disabled,
        }
    }
}

impl GyroSettings {
    /// Returns `u8` to write to CTRL_REG1_G. See page 45
    /// # CTRL_REG1_G: [ODR_G2][ODR_G1][ODR_G0][FS_G1][FS_G0][0][BW_G1][BW_G0]
    /// - ODR_G[2:0] - Output data rate selection
    /// - FS_G[1:0] - Gyroscope full-scale selection
    /// - BW_G[1:0] - Gyroscope bandwidth selection
    pub fn ctrl_reg1_g(&self) -> u8 {
        self.sample_rate.value() | self.scale.value() | self.bandwidth.value()
    }

    /// Returns `u8` to write to CTRL_REG2_G. See page 47
    /// # CTRL_REG2_G: [0][0][0][0][INT_SEL1][INT_SEL0][OUT_SEL1][OUT_SEL0]
    /// - INT_SEL[1:0] - INT selection configuration
    /// - OUT_SEL[1:0] - Out selection configuration
    pub fn ctrl_reg2_g(&self) -> u8 {
        self.int_selection.value() | self.out_selection.value()
    }

    /// Returns `u8` to write to CTRL_REG3_G. See page 47
    /// # CTRL_REG3_G: [LP_mode][HP_EN][0][0][HPCF3_G][HPCF2_G][HPCF1_G][HPCF0_G]
    /// - LP_mode - Low-power mode enable (0: disabled, 1: enabled)
    /// - HP_EN - HPF enable (0:disabled, 1: enabled)
    /// - HPCF_G[3:0] - HPF cutoff frequency
    pub fn ctrl_reg3_g(&self) -> u8 {
        self.low_power_mode.value() | self.hpf_mode.value() | self.hpf_cutoff.value()
    }

    /// Returns `u8` to write to CTRL_REG4. See page 50
    /// # CTRL_REG4: [0][0][Zen_G][Yen_G][Xen_G][0][LIR_XL1][4D_XL1]
    /// - Zen_G - Z-axis output enable (false :disable, true :enable)
    /// - Yen_G - Y-axis output enable (false :disable, true :enable)
    /// - Xen_G - X-axis output enable (false :disable, true :enable)
    /// - LIR_XL1 - Latched interrupt (0:not latched, 1:latched) // TODO:
    /// - 4D_XL1 - 4D option on interrupt (0:6D used, 1:4D used) // TODO:
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

    /// Returns `u8` to write to ORIENT_CFG_G
    /// # ORIENT_CFG_G: [0][0][SignX_G][SignY_G][SignZ_G][Orient_2][Orient_1][Orient_0]
    /// - SignX_G - Pitch axis (X) angular rate sign (false: positive, true: negative)
    /// - SignY_G - Roll axis (Y) angular rate sign (false: positive, true: negative)
    /// - SignZ_G - Yaw axis (Z) angular rate sign (false: positive, true: negative)
    /// - Orient [2:0] - Directional user orientation selection // TODO:
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
pub enum Scale {
    /// 245 degrees per second
    _245DPS = 0b00,
    /// 500 dps
    _500DPS = 0b01,
    /// 2000 dps
    _2000DPS = 0b11,
}

impl Scale {
    pub fn value(self) -> u8 {
        (self as u8) << 3
    }

    /// Returns Angular rate sensitivity depending on scale. (Refer to Page 12)
    pub fn sensitivity(self) -> f32 {
        use Scale::*;
        match self {
            _245DPS => 0.00875,
            _500DPS => 0.0175,
            _2000DPS => 0.07,
        }
    }
}

/// Gyroscope operating modes. (Refer to Table 9)
#[derive(Debug, Clone, Copy)]
pub enum ODR {
    /// Power down (0)
    PowerDown = 0b000,
    /// 14.9 Hz (1)
    _14_9Hz = 0b001,
    /// 59.5 Hz (2)
    _59_5Hz = 0b010,
    /// 119 Hz (3)
    _119Hz = 0b011,
    /// 238 Hz (4)
    _238Hz = 0b100,
    /// 476 Hz (5)
    _476Hz = 0b101,
    /// 952 Hz (6)
    _952Hz = 0b110,
}

impl ODR {
    pub fn value(self) -> u8 {
        (self as u8) << 5
    }
}

/// Gyroscope bandwidth selection. (Refer to Table 47)
#[derive(Debug, Clone, Copy)]
pub enum Bandwidth {
    /// 00
    LPF_0 = 0b00,
    /// 01
    LPF_1 = 0b01,
    /// 10
    LPF_2 = 0b10,
    /// 11
    LPF_3 = 0b11,
}

impl Bandwidth {
    pub fn value(self) -> u8 {
        self as u8
    }
}

/// INT selection configuration. (Refer to table 49)
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

/// Out selection configuration. (Refer to table 49)
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

/// Low-power mode enable. Default value: Disabled. (Refer to Table 51)
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

/// High-pass filter enable. (Refer to Table 51)
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

/// Low-power mode enable. (Refer to Table 51)
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

/// Gyroscope high-pass filter cutoff frequency selection. (Refer to Table 52)
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

#[test]
fn gyro_init_values() {
    let settings = GyroSettings::default();
    assert_eq!(settings.ctrl_reg1_g(), 0b1100_0000); // [ODR_G2][ODR_G1][ODR_G0][FS_G1][FS_G0][0][BW_G1][BW_G0]
    assert_eq!(settings.ctrl_reg2_g(), 0b0000_0000); // [0][0][0][0][INT_SEL1][INT_SEL0][OUT_SEL1][OUT_SEL0]
    assert_eq!(settings.ctrl_reg3_g(), 0b0000_0000); // [LP_mode][HP_EN][0][0][HPCF3_G][HPCF2_G][HPCF1_G][HPCF0_G]
    assert_eq!(settings.ctrl_reg4(), 0b0011_1000); // [0][0][Zen_G][Yen_G][Xen_G][0][LIR_XL1][4D_XL1]
}

#[test]
fn gyro_set_scale() {
    use Scale::*;
    let mask = 0b0001_1000;

    let gyro = GyroSettings {
        scale: _245DPS,
        ..Default::default()
    };
    assert_eq!(gyro.ctrl_reg1_g() & mask, 0b0000_0000);

    let gyro = GyroSettings {
        scale: _500DPS,
        ..Default::default()
    };
    assert_eq!(gyro.ctrl_reg1_g() & mask, 0b0000_1000);

    let gyro = GyroSettings {
        scale: _2000DPS,
        ..Default::default()
    };
    assert_eq!(gyro.ctrl_reg1_g() & mask, 0b0001_1000);
}

#[test]
fn gyro_set_odr() {
    use ODR::*;
    let mask = 0b1110_0000;

    let gyro = GyroSettings {
        sample_rate: PowerDown,
        ..Default::default()
    };
    assert_eq!(gyro.ctrl_reg1_g() & mask, 0b0000_0000);

    let gyro = GyroSettings {
        sample_rate: _14_9Hz,
        ..Default::default()
    };
    assert_eq!(gyro.ctrl_reg1_g() & mask, 0b0010_0000);

    let gyro = GyroSettings {
        sample_rate: _59_5Hz,
        ..Default::default()
    };
    assert_eq!(gyro.ctrl_reg1_g() & mask, 0b0100_0000);

    let gyro = GyroSettings {
        sample_rate: _119Hz,
        ..Default::default()
    };
    assert_eq!(gyro.ctrl_reg1_g() & mask, 0b0110_0000);

    let gyro = GyroSettings {
        sample_rate: _238Hz,
        ..Default::default()
    };
    assert_eq!(gyro.ctrl_reg1_g() & mask, 0b1000_0000);

    let gyro = GyroSettings {
        sample_rate: _476Hz,
        ..Default::default()
    };
    assert_eq!(gyro.ctrl_reg1_g() & mask, 0b1010_0000);

    let gyro = GyroSettings {
        sample_rate: _952Hz,
        ..Default::default()
    };
    assert_eq!(gyro.ctrl_reg1_g() & mask, 0b1100_0000);
}

#[test]
fn set_gyro_bandwidth() {
    use Bandwidth::*;
    let mask = 0b0000_0011;

    let gyro = GyroSettings {
        bandwidth: LPF_0,
        ..Default::default()
    };
    assert_eq!(gyro.ctrl_reg1_g() & mask, 0b0000_0000);

    let gyro = GyroSettings {
        bandwidth: LPF_1,
        ..Default::default()
    };
    assert_eq!(gyro.ctrl_reg1_g() & mask, 0b0000_0001);

    let gyro = GyroSettings {
        bandwidth: LPF_2,
        ..Default::default()
    };
    assert_eq!(gyro.ctrl_reg1_g() & mask, 0b0000_0010);

    let gyro = GyroSettings {
        bandwidth: LPF_3,
        ..Default::default()
    };
    assert_eq!(gyro.ctrl_reg1_g() & mask, 0b0000_0011);
}
