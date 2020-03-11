//! Magnetometer settings, types
#![allow(dead_code, non_camel_case_types)]

/// Magnetometer settings
#[derive(Debug)]
pub struct MagSettings {
    pub enabled: bool,
    pub sample_rate: ODR,
    pub temp_compensation: TempComp,
    pub x_y_performance: OpModeXY,
    pub scale: Scale,
    pub i2c_mode: I2cMode,
    pub system_op: SysOpMode,
    pub low_power: LowPowerMode,
    pub spi_mode: SpiMode,
    pub z_performance: OpModeZ,
}

impl Default for MagSettings {
    fn default() -> Self {
        MagSettings {
            enabled: true,
            temp_compensation: TempComp::Disabled,
            x_y_performance: OpModeXY::High,
            sample_rate: ODR::_10Hz,
            scale: Scale::_4G,
            i2c_mode: I2cMode::Enabled,
            system_op: SysOpMode::Continuous,
            low_power: LowPowerMode::Disabled,
            spi_mode: SpiMode::RW,
            z_performance: OpModeZ::High,
        }
    }
}

impl MagSettings {
    /// return the default setting
    pub fn new() -> MagSettings {
        Default::default()
    }

    /// CTRL_REG1_M (Default value: 0x10)
    /// [TEMP_COMP][OM1][OM0][DO2][DO1][DO0][0][ST]
    /// TEMP_COMP - Temperature compensation
    /// OM[1:0] - X & Y axes op mode selection
    /// 00:low-power, 01:medium performance
    /// 10: high performance, 11:ultra-high performance
    /// DO[2:0] - Output data rate selection
    /// ST - Self-test enable // TODO
    pub fn ctrl_reg1_m(&self) -> u8 {
        self.temp_compensation.value() | self.x_y_performance.value() | self.sample_rate.value()
    }

    /// CTRL_REG2_M (Default value 0x00)
    /// [0][FS1][FS0][0][REBOOT][SOFT_RST][0][0]
    /// FS[1:0] - Full-scale configuration
    /// REBOOT - Reboot memory content (0:normal, 1:reboot) // TODO
    /// SOFT_RST - Reset config and user registers (0:default, 1:reset) // TODO
    pub fn ctrl_reg2_m(&self) -> u8 {
        self.scale.value()
    }

    /// CTRL_REG3_M (Default value: 0x03)
    /// [I2C_DISABLE][0][LP][0][0][SIM][MD1][MD0]
    /// I2C_DISABLE - Disable I2C interace (0:enable, 1:disable)
    /// LP - Low-power mode cofiguration (1:enable)
    /// SIM - SPI mode selection (0:read/write enable, 1:write-only)
    /// MD[1:0] - Operating mode
    /// 00:continuous conversion, 01:single-conversion,
    /// 10,11: Power-down
    pub fn ctrl_reg3_m(&self) -> u8 {
        self.i2c_mode.value()
            | self.low_power.value()
            | self.spi_mode.value()
            | self.system_op.value()
    }

    /// CTRL_REG4_M (Default value: 0x00)
    /// [0][0][0][0][OMZ1][OMZ0][BLE][0]
    /// OMZ[1:0] - Z-axis operative mode selection
    /// 00:low-power mode, 01:medium performance
    /// 10:high performance, 10:ultra-high performance
    /// BLE - Big/little endian data // TODO
    pub fn ctrl_reg4_m(&self) -> u8 {
        self.z_performance.value()
    }

    /// CTRL_REG5_M (Default value: 0x00)
    /// [0][BDU][0][0][0][0][0][0]
    /// BDU - Block data update for magnetic data // TODO
    /// 0:continuous, 1:not updated until MSB/LSB are read
    pub fn ctrl_reg5_m(&self) -> u8 {
        0x00 // TODO
    }
}

/// Temperature compensation enable. Default value: 0 (Refer to Table 109)
#[derive(Debug, Clone, Copy)]
pub enum TempComp {
    Disabled = 0,
    Enabled = 1,
}

impl TempComp {
    pub fn value(self) -> u8 {
        (self as u8) << 7
    }
}

/// X and Y axes operative mode selection. Default value: 00 (Refer to Table 110)
#[derive(Debug, Clone, Copy)]
pub enum OpModeXY {
    Low = 0b00,
    Medium = 0b01,
    High = 0b10,
    UltraHigh = 0b11,
}

impl OpModeXY {
    pub fn value(self) -> u8 {
        (self as u8) << 5
    }
}

/// Z axe operative mode selection. Default value: 00 (Refer to Table 110)
#[derive(Debug, Clone, Copy)]
pub enum OpModeZ {
    Low = 0b00,
    Medium = 0b01,
    High = 0b10,
    UltraHigh = 0b11,
}

impl OpModeZ {
    pub fn value(self) -> u8 {
        (self as u8) << 2
    }
}

/// Output data rate selection. Default value: 100 (Refer to Table 111)
#[derive(Debug, Clone, Copy)]
pub enum ODR {
    _0_625Hz = 0b000,
    _1_25Hz = 0b001,
    _2_5Hz = 0b010,
    _5Hz = 0b011,
    _10Hz = 0b100,
    _20Hz = 0b101,
    _40Hz = 0b110,
    _80Hz = 0b111,
}

impl ODR {
    pub fn value(self) -> u8 {
        (self as u8) << 2
    }
}

/// Full-scale selection. Default value: 00. (Refer to Table 114)
#[derive(Debug, Clone, Copy)]
pub enum Scale {
    /// ± 4 gauss
    _4G = 0b00,
    /// ± 8 gauss
    _8G = 0b01,
    /// ± 12 gauss
    _12G = 0b10,
    /// ± 16 gauss
    _16G = 0b11,
}

impl Scale {
    pub fn value(self) -> u8 {
        (self as u8) << 5
    }

    /// return Magnetic sensitivity depending on scale. (Refer to Page 12)
    pub fn sensitivity(self) -> f32 {
        use Scale::*;
        match self {
            _4G => 0.14,
            _8G => 0.29,
            _12G => 0.43,
            _16G => 0.58,
        }
    }
}

/// I2C Interface mode selection. Disable I2C interface. Default value 0. (0: I2C enable; 1: I2C disable) (Refer to table 116)
#[derive(Debug, Clone, Copy)]
pub enum I2cMode {
    Enabled = 0,
    Disabled = 1,
}

impl I2cMode {
    pub fn value(self) -> u8 {
        (self as u8) << 7
    }
}

/// Low Power Mode
#[derive(Debug, Clone, Copy)]
pub enum LowPowerMode {
    Disabled = 0,
    Enabled = 1,
}

impl LowPowerMode {
    pub fn value(self) -> u8 {
        (self as u8) << 5
    }
}

/// SPI Serial Interface mode selection. Default value: 0 (Refer to Table 115 -> table 115 is wrong. W and RW should be the other way.)
#[derive(Debug, Clone, Copy)]
pub enum SpiMode {
    W = 1,
    RW = 0,
}

impl SpiMode {
    pub fn value(self) -> u8 {
        (self as u8) << 2
    }
}

/// Operating mode selection. Default value: 11 (Refer to Table 117)
#[derive(Debug, Clone, Copy)]
pub enum SysOpMode {
    Continuous = 0b00,
    Single = 0b01,
    PowerDown = 0b11,
}

impl SysOpMode {
    pub fn value(self) -> u8 {
        self as u8
    }
}

#[test]
fn mag_init_values() {
    let settings = MagSettings::new();
    assert_eq!(settings.ctrl_reg1_m(), 0b0101_0000); // [TEMP_COMP][OM1][OM0][DO2][DO1][DO0][0][ST]
    assert_eq!(settings.ctrl_reg2_m(), 0b0000_0000); // [0][FS1][FS0][0][REBOOT][SOFT_RST][0][0]
    assert_eq!(settings.ctrl_reg3_m(), 0b0000_0000); // [I2C_DISABLE][0][LP][0][0][SIM][MD1][MD0]
    assert_eq!(settings.ctrl_reg4_m(), 0b0000_1000); // [0][0][0][0][OMZ1][OMZ0][BLE][0]
    assert_eq!(settings.ctrl_reg5_m(), 0b0000_0000); // [0][BDU][0][0][0][0][0][0]
}
