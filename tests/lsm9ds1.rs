use lsm9ds1::LSM9DS1;
use lsm9ds1::accel;
use lsm9ds1::gyro;
use lsm9ds1::mag;
use lsm9ds1::interface;

#[test]
fn accel_init_values() {
    let interface = interface::FakeInterface::new();
    let imu = LSM9DS1::from_interface(interface);
    assert_eq!(imu.accel.ctrl_reg5_xl(), 0b0011_1000); // [DEC_1][DEC_0][Zen_XL][Yen_XL][Zen_XL][0][0][0]
    assert_eq!(imu.accel.ctrl_reg6_xl(), 0b0110_0000); // [ODR_XL2][ODR_XL1][ODR_XL0][FS1_XL][FS0_XL][BW_SCAL_ODR][BW_XL1][BW_XL0]
    assert_eq!(imu.accel.ctrl_reg7_xl(), 0b0000_0000); // [HR][DCF1][DCF0][0][0][FDS][0][HPIS1]
}

#[test]
fn gyro_init_values() {
    let interface = interface::FakeInterface::new();
    let imu = LSM9DS1::from_interface(interface);
    assert_eq!(imu.gyro.ctrl_reg1_g(), 0b1100_0000); // [ODR_G2][ODR_G1][ODR_G0][FS_G1][FS_G0][0][BW_G1][BW_G0]
    assert_eq!(imu.gyro.ctrl_reg2_g(), 0b0000_0000); // [0][0][0][0][INT_SEL1][INT_SEL0][OUT_SEL1][OUT_SEL0]
    assert_eq!(imu.gyro.ctrl_reg3_g(), 0b0000_0000); // [LP_mode][HP_EN][0][0][HPCF3_G][HPCF2_G][HPCF1_G][HPCF0_G]
    assert_eq!(imu.gyro.ctrl_reg4(), 0b0011_1000); // [0][0][Zen_G][Yen_G][Xen_G][0][LIR_XL1][4D_XL1]
}

#[test]
fn mag_init_values() {
    let interface = interface::FakeInterface::new();
    let imu = LSM9DS1::from_interface(interface);
    assert_eq!(imu.mag.ctrl_reg1_m(), 0b0101_0000); // [TEMP_COMP][OM1][OM0][DO2][DO1][DO0][0][ST]
    assert_eq!(imu.mag.ctrl_reg2_m(), 0b0000_0000); // [0][FS1][FS0][0][REBOOT][SOFT_RST][0][0]
    assert_eq!(imu.mag.ctrl_reg3_m(), 0b0000_0000); // [I2C_DISABLE][0][LP][0][0][SIM][MD1][MD0]
    assert_eq!(imu.mag.ctrl_reg4_m(), 0b0000_1000); // [0][0][0][0][OMZ1][OMZ0][BLE][0]
    assert_eq!(imu.mag.ctrl_reg5_m(), 0b0000_0000); // [0][BDU][0][0][0][0][0][0]
}

#[test]
fn accel_set_scale() {
    let mask = 0b0001_1000;
    let interface = interface::FakeInterface::new();
    let mut imu = LSM9DS1::from_interface(interface);
    imu.set_accel_scale(accel::Scale::_16G).unwrap();
    assert_eq!(imu.accel.ctrl_reg6_xl() & mask, 0b0000_1000);
    imu.set_accel_scale(accel::Scale::_4G).unwrap();
    assert_eq!(imu.accel.ctrl_reg6_xl() & mask, 0b0001_0000);
    imu.set_accel_scale(accel::Scale::_8G).unwrap();
    assert_eq!(imu.accel.ctrl_reg6_xl() & mask, 0b0001_1000);
    imu.set_accel_scale(accel::Scale::_2G).unwrap();
    assert_eq!(imu.accel.ctrl_reg6_xl() & mask, 0b0000_0000);
}

#[test]
fn gyro_set_scale() {
    let mask = 0b0001_1000;
    let interface = interface::FakeInterface::new();
    let mut imu = LSM9DS1::from_interface(interface);
    imu.set_gyro_scale(gyro::Scale::_500DPS).unwrap();
    assert_eq!(imu.gyro.ctrl_reg1_g() & mask, 0b0000_1000);
    imu.set_gyro_scale(gyro::Scale::_2000DPS).unwrap();
    assert_eq!(imu.gyro.ctrl_reg1_g() & mask, 0b0001_1000);
    imu.set_gyro_scale(gyro::Scale::_245DPS).unwrap();
    assert_eq!(imu.gyro.ctrl_reg1_g() & mask, 0b0000_0000);
}

#[test]
fn mag_set_scale() {
    let mask = 0b0110_0000;
    let interface = interface::FakeInterface::new();
    let mut imu = LSM9DS1::from_interface(interface);
    imu.set_mag_scale(mag::Scale::_8G).unwrap();
    assert_eq!(imu.mag.ctrl_reg2_m() & mask, 0b0010_0000);
    imu.set_mag_scale(mag::Scale::_12G).unwrap();
    assert_eq!(imu.mag.ctrl_reg2_m() & mask, 0b0100_0000);
    imu.set_mag_scale(mag::Scale::_16G).unwrap();
    assert_eq!(imu.mag.ctrl_reg2_m() & mask, 0b0110_0000);
    imu.set_mag_scale(mag::Scale::_4G).unwrap();
    assert_eq!(imu.mag.ctrl_reg2_m() & mask, 0b0000_0000);
}

#[test]
fn accel_set_odr() {
    let mask = 0b1110_0000;
    let interface = interface::FakeInterface::new();
    let mut imu = LSM9DS1::from_interface(interface);
    imu.set_accel_odr(accel::ODR::PowerDown).unwrap();
    assert_eq!(imu.accel.ctrl_reg6_xl() & mask, 0b0000_0000);
    imu.set_accel_odr(accel::ODR::_10Hz).unwrap();
    assert_eq!(imu.accel.ctrl_reg6_xl() & mask, 0b0010_0000);
    imu.set_accel_odr(accel::ODR::_50Hz).unwrap();
    assert_eq!(imu.accel.ctrl_reg6_xl() & mask, 0b0100_0000);
    imu.set_accel_odr(accel::ODR::_119Hz).unwrap();
    assert_eq!(imu.accel.ctrl_reg6_xl() & mask, 0b0110_0000);
    imu.set_accel_odr(accel::ODR::_238Hz).unwrap();
    assert_eq!(imu.accel.ctrl_reg6_xl() & mask, 0b1000_0000);
    imu.set_accel_odr(accel::ODR::_476Hz).unwrap();
    assert_eq!(imu.accel.ctrl_reg6_xl() & mask, 0b1010_0000);
    imu.set_accel_odr(accel::ODR::_952Hz).unwrap();
    assert_eq!(imu.accel.ctrl_reg6_xl() & mask, 0b1100_0000);
}

#[test]
fn gyro_set_odr() {
    let mask = 0b1110_0000;
    let interface = interface::FakeInterface::new();
    let mut imu = LSM9DS1::from_interface(interface);
    imu.set_gyro_odr(gyro::ODR::PowerDown).unwrap();
    assert_eq!(imu.gyro.ctrl_reg1_g() & mask, 0b0000_0000);
    imu.set_gyro_odr(gyro::ODR::_14_9Hz).unwrap();
    assert_eq!(imu.gyro.ctrl_reg1_g() & mask, 0b0010_0000);
    imu.set_gyro_odr(gyro::ODR::_59_5Hz).unwrap();
    assert_eq!(imu.gyro.ctrl_reg1_g() & mask, 0b0100_0000);
    imu.set_gyro_odr(gyro::ODR::_119Hz).unwrap();
    assert_eq!(imu.gyro.ctrl_reg1_g() & mask, 0b0110_0000);
    imu.set_gyro_odr(gyro::ODR::_238Hz).unwrap();
    assert_eq!(imu.gyro.ctrl_reg1_g() & mask, 0b1000_0000);
    imu.set_gyro_odr(gyro::ODR::_476Hz).unwrap();
    assert_eq!(imu.gyro.ctrl_reg1_g() & mask, 0b1010_0000);
    imu.set_gyro_odr(gyro::ODR::_952Hz).unwrap();
    assert_eq!(imu.gyro.ctrl_reg1_g() & mask, 0b1100_0000);
}

#[test]
fn mag_set_odr() {
    let mask = 0b0001_1100;
    let interface = interface::FakeInterface::new();
    let mut imu = LSM9DS1::from_interface(interface);
    imu.set_mag_odr(mag::ODR::_0_625Hz).unwrap();
    assert_eq!(imu.mag.ctrl_reg1_m() & mask, 0b0000_0000);
    imu.set_mag_odr(mag::ODR::_1_25Hz).unwrap();
    assert_eq!(imu.mag.ctrl_reg1_m() & mask, 0b0000_0100);
    imu.set_mag_odr(mag::ODR::_2_5Hz).unwrap();
    assert_eq!(imu.mag.ctrl_reg1_m() & mask, 0b0000_1000);
    imu.set_mag_odr(mag::ODR::_5Hz).unwrap();
    assert_eq!(imu.mag.ctrl_reg1_m() & mask, 0b0000_1100);
    imu.set_mag_odr(mag::ODR::_10Hz).unwrap();
    assert_eq!(imu.mag.ctrl_reg1_m() & mask, 0b0001_0000);
    imu.set_mag_odr(mag::ODR::_20Hz).unwrap();
    assert_eq!(imu.mag.ctrl_reg1_m() & mask, 0b0001_0100);
    imu.set_mag_odr(mag::ODR::_40Hz).unwrap();
    assert_eq!(imu.mag.ctrl_reg1_m() & mask, 0b0001_1000);
    imu.set_mag_odr(mag::ODR::_80Hz).unwrap();
    assert_eq!(imu.mag.ctrl_reg1_m() & mask, 0b0001_1100);
}