#![allow(dead_code, non_camel_case_types)]

/// Accelerometer settings
pub struct AccelSettings {
    enable_x: bool,
    enable_y: bool,
    enable_z: bool,
    scale: AccelScale,
    sample_rate: AccelODR,
    bandwidth_selection: AccelBandwidthSelection,
    bandwidth: AccelBandwidth,
    high_res_bandwidth: HighRes,
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
#[derive(Clone, Copy)]
enum AccelScale {
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
}

/// Output data rate and power mode selection (ODR_XL). default value: 000 (see Table 68)
#[derive(Clone, Copy)]
enum AccelODR {
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

enum AccelBandwidthSelection {
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
#[derive(Clone, Copy)]
enum AccelBandwidth {
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
#[derive(Clone, Copy)]
enum HighRes {
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

// gyro_scale defines the possible full-scale ranges of the gyroscope:
enum GyroScale {
    /// 245 degrees per second
    G_FS_245DPS = 0b00,
    /// 500 dps
    G_FS_500DPS = 0b01,
    /// 2000 dps
    G_FS_2000DPS = 0b11,
}

/// Gyroscope operating modes. See table 9.
enum GyroOdr {
    /// Power down (0)
    ODR_PD,
    /// 14.9 Hz (1)
    ODR_149,
    /// 59.5 Hz (2)
    ODR_595,
    /// 119 Hz (3)
    ODR_119,
    /// 238 Hz (4)
    ODR_238,
    /// 476 Hz (5)
    ODR_476,
    /// 952 Hz (6)
    ODR_952,
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
