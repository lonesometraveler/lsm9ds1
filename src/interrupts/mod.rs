/// Interrupt active setting for the INT_DRDY pin: active high (default) or active low
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy)]
pub enum IntActive {
    /// Active high
    High,
    /// Active low
    Low,
}

impl IntActive {
    pub fn status(self) -> u8 {
        match self {
            IntActive::High => 0,
            IntActive::Low => 1,
        }
    }
}

/// Interrupt pad setting for INT_DRDY pin: push-pull (default) or open-drain.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy)]
pub enum IntPin {
    /// Push-pull
    PushPull,
    /// Open drain
    OpenDrain,
}

impl IntPin {
    pub fn status(self) -> u8 {
        match self {
            IntPin::PushPull => 0,
            IntPin::OpenDrain => 1,
        }
    }
}

/// Interrupt latching setting (interrupt request latched or not latched)
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy)]
pub enum IntLatch {
    /// Interrupt request latched
    Latched,
    /// Interrupt request not latched
    NotLatched,
}

impl IntLatch {
    pub fn status(self) -> u8 {
        match self {
            IntLatch::Latched => 1,
            IntLatch::NotLatched => 0,
        }
    }
}

/// 6D or 4D used by interrupt generator for for position recognition
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy)]
pub enum PosRecog {
    /// 4D option used for position recognition
    _4D,
    /// 4D option used for position recognition
    _6D,
}

impl PosRecog {
    pub fn status(self) -> u8 {
        match self {
            PosRecog::_4D => 1,
            PosRecog::_6D => 0,
        }
    }
}

/// Decrement or reset counter mode selection.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy)]
pub enum Counter {
    /// Decrement counter (see pages 58-61)
    Decrement,
    /// Reset counter
    Reset,
}

impl Counter {
    pub fn status(self) -> u8 {
        match self {
            Counter::Decrement => 1,
            Counter::Reset => 0,
        }
    }
}
