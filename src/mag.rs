//! Magnetometer settings, types
#![allow(dead_code, non_camel_case_types)]

use crate::{configuration::CustomConfiguration, interface::Sensor, register};

/// Magnetometer settings. Use this struct to configure the sensor.
#[derive(Debug)]
pub struct MagSettings {
    /// Output data rate selection
    pub sample_rate: ODR,
    /// Temperature compensation
    pub temp_compensation: TempComp,
    /// X & Y axes op mode selection
    pub x_y_performance: OpModeXY,
    /// Full-scale configuration
    pub scale: Scale,
    /// Enable/Disable I2C interace
    pub i2c_mode: I2cMode,
    /// Operating mode. See page 64.
    pub system_op: SysOpMode,
    /// Low-power mode cofiguration. See page 64.
    pub low_power: LowPowerMode,
    /// SPI mode selection
    pub spi_mode: SpiMode,
    /// Z-axis operative mode selection
    pub z_performance: OpModeZ,
}

impl Default for MagSettings {
    fn default() -> Self {
        MagSettings {
            temp_compensation: TempComp::Disabled,
            x_y_performance: OpModeXY::Low,
            sample_rate: ODR::_10Hz,
            scale: Scale::_4G,
            i2c_mode: I2cMode::Enabled,
            system_op: SysOpMode::Continuous,
            low_power: LowPowerMode::Disabled,
            spi_mode: SpiMode::RW,
            z_performance: OpModeZ::Low,
        }
    }
}

impl MagSettings {
    /// Returns `u8` to write to CTRL_REG1_M. See page 63.
    /// # CTRL_REG1_M: [TEMP_COMP][OM1][OM0][DO2][DO1][DO0][0][ST]
    /// - TEMP_COMP - Temperature compensation
    /// - OM[1:0] - X & Y axes op mode selection
    ///     - 00:low-power
    ///     - 01:medium performance
    ///     - 10: high performance
    ///     - 11:ultra-high performance
    /// - DO[2:0] - Output data rate selection
    /// - ST - Self-test enable // TODO
    pub fn ctrl_reg1_m(&self) -> u8 {
        self.temp_compensation.value() | self.x_y_performance.value() | self.sample_rate.value()
    }

    pub fn ctrl_reg1_m_config(&self) -> CustomConfiguration {
        CustomConfiguration {
            value: self.ctrl_reg1_m(),
            sensor: Sensor::Magnetometer,
            register: register::Mag::CTRL_REG1_M.addr(),
        }
    }

    /// Returns `u8` to write to CTRL_REG2_M. See page 64.
    /// # CTRL_REG2_M: [0][FS1][FS0][0][REBOOT][SOFT_RST][0][0]
    /// - FS[1:0] - Full-scale configuration
    /// - REBOOT - Reboot memory content (0:normal, 1:reboot) // TODO
    /// - SOFT_RST - Reset config and user registers (0:default, 1:reset) // TODO
    pub fn ctrl_reg2_m(&self) -> u8 {
        self.scale.value()
    }

    pub fn ctrl_reg2_m_config(&self) -> CustomConfiguration {
        CustomConfiguration {
            value: self.ctrl_reg2_m(),
            sensor: Sensor::Magnetometer,
            register: register::Mag::CTRL_REG2_M.addr(),
        }
    }

    /// Returns `u8` to write to CTRL_REG3_M. See page 64.
    /// # CTRL_REG3_M: [I2C_DISABLE][0][LP][0][0][SIM][MD1][MD0]
    /// - I2C_DISABLE - Disable I2C interace (0:enable, 1:disable)
    /// - LP - Low-power mode cofiguration (1:enable)
    /// - SIM - SPI mode selection (0:read/write enable, 1:write-only)
    /// - MD[1:0] - Operating mode
    ///     - 00:continuous conversion
    ///     - 01:single-conversion,
    ///     - 10,11: Power-down
    pub fn ctrl_reg3_m(&self) -> u8 {
        self.i2c_mode.value()
            | self.low_power.value()
            | self.spi_mode.value()
            | self.system_op.value()
    }

    pub fn ctrl_reg3_m_config(&self) -> CustomConfiguration {
        CustomConfiguration {
            value: self.ctrl_reg3_m(),
            sensor: Sensor::Magnetometer,
            register: register::Mag::CTRL_REG3_M.addr(),
        }
    }

    /// Returns `u8` to write to CTRL_REG4_M. See page 65.
    /// # CTRL_REG4_M: [0][0][0][0][OMZ1][OMZ0][BLE][0]
    /// - OMZ[1:0] - Z-axis operative mode selection
    ///     - 00:low-power mode
    ///     - 01:medium performance
    ///     - 10:high performance
    ///     - 10:ultra-high performance
    /// - BLE - Big/little endian data // TODO
    pub fn ctrl_reg4_m(&self) -> u8 {
        self.z_performance.value()
    }

    pub fn ctrl_reg4_m_config(&self) -> CustomConfiguration {
        CustomConfiguration {
            value: self.ctrl_reg4_m(),
            sensor: Sensor::Magnetometer,
            register: register::Mag::CTRL_REG4_M.addr(),
        }
    }

    /// Returns `u8` to write to CTRL_REG5_M. See page 65.
    /// # CTRL_REG5_M: [0][BDU][0][0][0][0][0][0]
    /// - BDU - Block data update for magnetic data // TODO
    ///     - 0:continuous
    ///     - 1:not updated until MSB/LSB are read
    pub fn ctrl_reg5_m(&self) -> u8 {
        0x00 // TODO
    }

    pub fn ctrl_reg5_m_config(&self) -> CustomConfiguration {
        CustomConfiguration {
            value: self.ctrl_reg5_m(),
            sensor: Sensor::Magnetometer,
            register: register::Mag::CTRL_REG5_M.addr(),
        }
    }
}

/// Temperature compensation enable. (Refer to Table 109)
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

/// X and Y axes operative mode selection. (Refer to Table 110)
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

/// Z axe operative mode selection. (Refer to Table 110)
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

/// Output data rate selection. (Refer to Table 111)
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

/// Full-scale selection. (Refer to Table 114)
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

    /// Returns Magnetic sensitivity depending on scale. (Refer to Page 12)
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

/// I2C Interface mode selection. Disable I2C interface. (0: I2C enable; 1: I2C disable) (Refer to table 116)
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

/// SPI Serial Interface mode selection. (Refer to Table 115 -> table 115 is wrong. W and RW should be the other way.)
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

/// Operating mode selection. (Refer to Table 117)
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
    let settings = MagSettings::default();
    assert_eq!(settings.ctrl_reg1_m(), 0b0001_0000); // [TEMP_COMP][OM1][OM0][DO2][DO1][DO0][0][ST]
    assert_eq!(settings.ctrl_reg2_m(), 0b0000_0000); // [0][FS1][FS0][0][REBOOT][SOFT_RST][0][0]
    assert_eq!(settings.ctrl_reg3_m(), 0b0000_0000); // [I2C_DISABLE][0][LP][0][0][SIM][MD1][MD0]
    assert_eq!(settings.ctrl_reg4_m(), 0b0000_0000); // [0][0][0][0][OMZ1][OMZ0][BLE][0]
    assert_eq!(settings.ctrl_reg5_m(), 0b0000_0000); // [0][BDU][0][0][0][0][0][0]
}

#[test]
fn mag_set_scale() {
    use Scale::*;
    let mask = 0b0110_0000;

    let mag = MagSettings {
        scale: _4G,
        ..Default::default()
    };
    assert_eq!(mag.ctrl_reg2_m() & mask, 0b0000_0000);

    let mag = MagSettings {
        scale: _8G,
        ..Default::default()
    };
    assert_eq!(mag.ctrl_reg2_m() & mask, 0b0010_0000);

    let mag = MagSettings {
        scale: _12G,
        ..Default::default()
    };
    assert_eq!(mag.ctrl_reg2_m() & mask, 0b0100_0000);

    let mag = MagSettings {
        scale: _16G,
        ..Default::default()
    };
    assert_eq!(mag.ctrl_reg2_m() & mask, 0b0110_0000);
}

#[test]
fn mag_set_odr() {
    use ODR::*;
    let mask = 0b0001_1100;

    let mag = MagSettings {
        sample_rate: _0_625Hz,
        ..Default::default()
    };
    assert_eq!(mag.ctrl_reg1_m() & mask, 0b0000_0000);

    let mag = MagSettings {
        sample_rate: _1_25Hz,
        ..Default::default()
    };
    assert_eq!(mag.ctrl_reg1_m() & mask, 0b0000_0100);

    let mag = MagSettings {
        sample_rate: _2_5Hz,
        ..Default::default()
    };
    assert_eq!(mag.ctrl_reg1_m() & mask, 0b0000_1000);

    let mag = MagSettings {
        sample_rate: _5Hz,
        ..Default::default()
    };
    assert_eq!(mag.ctrl_reg1_m() & mask, 0b0000_1100);

    let mag = MagSettings {
        sample_rate: _10Hz,
        ..Default::default()
    };
    assert_eq!(mag.ctrl_reg1_m() & mask, 0b0001_0000);

    let mag = MagSettings {
        sample_rate: _20Hz,
        ..Default::default()
    };
    assert_eq!(mag.ctrl_reg1_m() & mask, 0b0001_0100);

    let mag = MagSettings {
        sample_rate: _40Hz,
        ..Default::default()
    };
    assert_eq!(mag.ctrl_reg1_m() & mask, 0b0001_1000);

    let mag = MagSettings {
        sample_rate: _80Hz,
        ..Default::default()
    };
    assert_eq!(mag.ctrl_reg1_m() & mask, 0b0001_1100);
}
