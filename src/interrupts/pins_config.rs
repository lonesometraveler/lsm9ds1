/// Functions related to INT1_AG and INT2_AG interrupt pins configuration
///
/// TO DO:
/// -
///
use super::*;

/// Accelerometer/gyroscope interrupt pins common settings
#[derive(Debug)]
pub struct PinConfig {
    // --- CTRL_REG8 REGISTER ---
    /// Interrupt pin active level (default 0: active high)
    pub active_level: INT_ACTIVE,
    /// Interrupt pin push-pull or open-drain configuration (default 0: push-pull)
    pub pin_mode: INT_PIN,
}

impl Default for PinConfig {
    fn default() -> Self {
        PinConfig {
            active_level: INT_ACTIVE::High,
            pin_mode: INT_PIN::PushPull,
        }
    }
}

impl PinConfig {
    /// Returns values to be written to CTRL_REG8 register
    fn ctrl_reg8(&self) -> u8 {
        let mut data: u8 = 0;

        if self.active_level.status() {
            data |= 1 << 5;
        }
        if self.pin_mode.status() {
            data |= 1 << 4;
        }

        data
    }
}

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

impl<T> LSM9DS1<T>
where
    T: Interface,
{
    /// Enable interrupts for accelerometer/gyroscope and configure the INT1_A/G interrupt pin
    pub fn configure_interrupts_ag1(&mut self, config: IntConfigAG1) -> Result<(), T::Error> {
        self.interface.write(
            Sensor::Accelerometer,
            register::AG::INT1_CTRL.addr(),
            config.int1_ctrl(),
        )?;
        Ok(())
    }

    /// Enable interrupts for accelerometer/gyroscope and configure the INT1_A/G interrupt pin
    pub fn configure_interrupts_ag2(&mut self, config: IntConfigAG2) -> Result<(), T::Error> {
        let reg_data = self.read_register(Sensor::Accelerometer, register::AG::INT2_CTRL.addr())?;

        let mut data: u8 = reg_data & !0b1100_0000;

        data |= config.int2_ctrl();

        self.interface
            .write(Sensor::Accelerometer, register::AG::INT2_CTRL.addr(), data)?;
        Ok(())
    }

    /// Interrupt pins electrical configuration
    pub fn configure_interrupts_pins(&mut self, config: PinConfig) -> Result<(), T::Error> {
        let reg_data = self.read_register(Sensor::Accelerometer, register::AG::CTRL_REG8.addr())?;

        let mut data: u8 = reg_data & !0b0011_0000;

        //data |= config.int2_ctrl();

        self.interface
            .write(Sensor::Accelerometer, register::AG::CTRL_REG8.addr(), data)?;
        Ok(())
    }

    /// Get the current A/G1 pin configuration
    pub fn get_ag1_config(&mut self) -> Result<IntConfigAG1, T::Error> {
        let reg_value: u8 =
            self.read_register(Sensor::Accelerometer, register::AG::INT1_CTRL.addr())?;

        let config = IntConfigAG1 {
            enable_gyro_int: match (reg_value & 0b1000_0000) >> 7 {
                1 => FLAG::Enabled,
                _ => FLAG::Disabled,
            },
            enable_accel_int: match (reg_value & 0b0100_0000) >> 6 {
                1 => FLAG::Enabled,
                _ => FLAG::Disabled,
            },
            enable_fss5: match (reg_value & 0b0010_0000) >> 5 {
                1 => FLAG::Enabled,
                _ => FLAG::Disabled,
            },
            enable_overrun: match (reg_value & 0b0001_0000) >> 4 {
                1 => FLAG::Enabled,
                _ => FLAG::Disabled,
            },
            enable_fth: match (reg_value & 0b0000_1000) >> 3 {
                1 => FLAG::Enabled,
                _ => FLAG::Disabled,
            },
            enable_boot_status: match reg_value & 0b0000_0100 >> 2 {
                1 => FLAG::Enabled,
                _ => FLAG::Disabled,
            },
            enable_gyro_dataready: match reg_value & 0b0000_0010 >> 1 {
                1 => FLAG::Enabled,
                _ => FLAG::Disabled,
            },
            enable_accel_dataready: match reg_value & 0b0000_0001 {
                1 => FLAG::Enabled,
                _ => FLAG::Disabled,
            },
        };

        Ok(config)
    }

    /// Get the current A/G2 pin configuration
    pub fn get_ag2_config(&mut self) -> Result<IntConfigAG2, T::Error> {
        let reg_value: u8 =
            self.read_register(Sensor::Accelerometer, register::AG::INT2_CTRL.addr())?;

        let config = IntConfigAG2 {
            enable_fss5: match (reg_value & 0b0010_0000) >> 5 {
                1 => FLAG::Enabled,
                _ => FLAG::Disabled,
            },
            enable_overrun: match (reg_value & 0b0001_0000) >> 4 {
                1 => FLAG::Enabled,
                _ => FLAG::Disabled,
            },
            enable_fth: match (reg_value & 0b0000_1000) >> 3 {
                1 => FLAG::Enabled,
                _ => FLAG::Disabled,
            },
            enable_temp_dataready: match reg_value & 0b0000_0100 >> 2 {
                1 => FLAG::Enabled,
                _ => FLAG::Disabled,
            },
            enable_gyro_dataready: match reg_value & 0b0000_0010 >> 1 {
                1 => FLAG::Enabled,
                _ => FLAG::Disabled,
            },
            enable_accel_dataready: match reg_value & 0b0000_0001 {
                1 => FLAG::Enabled,
                _ => FLAG::Disabled,
            },
        };

        Ok(config)
    }

    /// Get the current common pins configuration
    pub fn get_pins_config(&mut self) -> Result<PinConfig, T::Error> {
        let reg_value: u8 =
            self.read_register(Sensor::Accelerometer, register::AG::CTRL_REG8.addr())?;

        let config = PinConfig {
            active_level: match (reg_value & 0b0100_0000) >> 5 {
                1 => INT_ACTIVE::Low,
                _ => INT_ACTIVE::High,
            },
            pin_mode: match (reg_value & 0b0010_0000) >> 4 {
                1 => INT_PIN::OpenDrain,
                _ => INT_PIN::PushPull,
            },
        };

        Ok(config)
    }
}

#[test]
fn configure_ag1() {
    let config = IntConfigAG1::default();
    assert_eq!(config.int1_ctrl(), 0b0000_0000);

    let config = IntConfigAG1 {
        enable_gyro_int: FLAG::Enabled,
        enable_accel_int: FLAG::Enabled,
        enable_fss5: FLAG::Enabled,
        enable_overrun: FLAG::Enabled,
        enable_fth: FLAG::Enabled,
        enable_boot_status: FLAG::Enabled,
        enable_gyro_dataready: FLAG::Enabled,
        enable_accel_dataready: FLAG::Enabled,
    };
    assert_eq!(config.int1_ctrl(), 0b1111_1111);
}

#[test]
fn configure_ag2() {
    let config = IntConfigAG2::default();
    assert_eq!(config.int2_ctrl(), 0b0000_0000);

    let config = IntConfigAG2 {
        enable_fss5: FLAG::Enabled,
        enable_overrun: FLAG::Enabled,
        enable_fth: FLAG::Enabled,
        enable_temp_dataready: FLAG::Enabled,
        enable_gyro_dataready: FLAG::Enabled,
        enable_accel_dataready: FLAG::Enabled,
    };
    assert_eq!(config.int2_ctrl(), 0b0011_1111);
}

#[test]
fn configure_pins() {
    let config = PinConfig::default();
    assert_eq!(config.ctrl_reg8(), 0b0000_0000);

    let config = PinConfig {
        active_level: INT_ACTIVE::Low,
        pin_mode: INT_PIN::OpenDrain,
    };
    assert_eq!(config.ctrl_reg8(), 0b0011_0000);
}
