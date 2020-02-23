/// LSM9DS1 Magnetometer Registers
#[allow(non_camel_case_types)]
#[derive(Clone, Copy)]
pub enum Register {
    /// This register is a 16-bit register and represents the X offset used to compensate environmental effects.
    OFFSET_X_REG_L_M = 0x05,
    OFFSET_X_REG_H_M = 0x06,
    /// This register is a 16-bit register and represents the Y offset used to compensate environmental effects.
    OFFSET_Y_REG_L_M = 0x07,
    OFFSET_Y_REG_H_M = 0x08,
    /// This register is a 16-bit register and represents the Z offset used to compensate environmental effects.
    OFFSET_Z_REG_L_M = 0x09,
    OFFSET_Z_REG_H_M = 0x0A,
    /// Device identification register.
    WHO_AM_I = 0x0F,
    CTRL_REG1_M = 0x20,
    CTRL_REG2_M = 0x21,
    CTRL_REG3_M = 0x22,
    CTRL_REG4_M = 0x23,
    CTRL_REG5_M = 0x24,
    STATUS_REG_M = 0x27,
    /// Magnetometer X-axis data output. The value of the magnetic field is expressed as two’s complement.
    OUT_X_L_M = 0x28,
    OUT_X_H_M = 0x29,
    /// Magnetometer Y-axis data output. The value of the magnetic field is expressed as two’s complement.
    OUT_Y_L_M = 0x2A,
    OUT_Y_H_M = 0x2B,
    /// Magnetometer Z-axis data output. The value of the magnetic field is expressed as two’s complement.
    OUT_Z_L_M = 0x2C,
    OUT_Z_H_M = 0x2D,
    INT_CFG_M = 0x30,
    INT_SRC_M = 0x31,
    INT_THS_L_M = 0x32,
    INT_THS_H_M = 0x33,
}

impl Register {
    pub fn addr(self) -> u8 {
        self as u8
    }
}
