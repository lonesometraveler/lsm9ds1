#[allow(non_camel_case_types)]
pub struct M_INT_Bitmasks;

#[allow(dead_code)]
/// Bitmasks for interrupt-related settings in INT_SRC_M register
impl M_INT_Bitmasks {
    pub (crate) const PTH_X: u8 = 0b1000_0000;
    pub (crate) const PTH_Y: u8 = 0b0100_0000;
    pub (crate) const PTH_Z: u8 = 0b0010_0000;
    pub (crate) const NTH_X: u8 = 0b0001_0000;
    pub (crate) const NTH_Y: u8 = 0b0000_1000;
    pub (crate) const NTH_Z: u8 = 0b0000_0100;
    pub (crate) const MROI: u8 = 0b0000_0010;
    pub (crate) const INT: u8 = 0b0000_0001;
}


#[derive(Debug)]
/// Contents of the INT_SRC_M register (interrupt active and threshold excess events flags)
pub struct IntStatusMag {
    pub xaxis_exceeds_thresh_pos: bool,
    pub yaxis_exceeds_thresh_pos: bool,
    pub zaxis_exceeds_thresh_pos: bool,
    pub xaxis_exceeds_thresh_neg: bool,
    pub yaxis_exceeds_thresh_neg: bool,
    pub zaxis_exceeds_thresh_neg: bool,
    pub measurement_range_overflow: bool,
    pub interrupt_occurs: bool,     
}
