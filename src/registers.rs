use bitfield::bitfield;

/// LM3549 Registers
pub enum Register {
    BankSel = 0x00,
    Ir0Lsb = 0x01,
    Ir0Msb = 0x02,
    Ig0Lsb = 0x03,
    Ig0Msb = 0x04,
    Ib0Lsb = 0x05,
    Ib0Msb = 0x06,
    Ir1Lsb = 0x07,
    Ir1Msb = 0x08,
    Ig1Lsb = 0x09,
    Ig1Msb = 0x0A,
    Ib1Lsb = 0x0B,
    Ib1Msb = 0x0C,
    Ir2Lsb = 0x0D,
    Ir2Msb = 0x0E,
    Ig2Lsb = 0x0F,
    Ig2Msb = 0x10,
    Ib2Lsb = 0x11,
    Ib2Msb = 0x12,
    Fader = 0x13,
    Ctrl = 0x14,
    Ilimit = 0x15,
    FaultMask = 0x16,
    Fault = 0x17,
    User1 = 0x19,
    User2 = 0x1A,
    EepromCtrl = 0x40,
}

/// Selects bank of current settings
#[derive(Copy, Clone, Debug)]
pub enum Bank {
    /// Bank 0
    B0 = 0x01,
    /// Bank 1
    B1 = 0x07,
    /// Bank 2
    B2 = 0x0D,
}

/// Buck-boost converter positive current limit
#[derive(Copy, Clone, Debug)]
pub enum PosLimit {
    /// 500 mA
    MA500 = 0,
    /// 1000 mA
    MA1000 = 1,
    /// 1500 mA
    MA1500 = 2,
    /// 2000 mA
    MA2000 = 3,
}

impl From<PosLimit> for u8 {
    fn from(p: PosLimit) -> Self {
        p as u8
    }
}

impl From<u8> for PosLimit {
    fn from(x: u8) -> Self {
        match x {
            0 => PosLimit::MA500,
            1 => PosLimit::MA1000,
            2 => PosLimit::MA1500,
            _ => PosLimit::MA2000,
        }
    }
}

/// Buck-boost converter negative current limit
#[derive(Copy, Clone, Debug)]
pub enum NegLimit {
    /// 550 mA
    MA550 = 0,
    /// 1000 mA
    MA1100 = 1,
    /// 1650 mA
    MA1650 = 2,
    /// 2200 mA
    MA2200 = 3,
}

impl From<NegLimit> for u8 {
    fn from(n: NegLimit) -> Self {
        n as u8
    }
}

impl From<u8> for NegLimit {
    fn from(x: u8) -> Self {
        match x {
            0 => NegLimit::MA550,
            1 => NegLimit::MA1100,
            2 => NegLimit::MA1650,
            _ => NegLimit::MA2200,
        }
    }
}

bitfield! {
  /// Current limit register
  pub struct Ilimit(u8);
  impl Debug;
  /// Positive limit
  pub u8, from into PosLimit, softstart, set_softstart: 5,4;
  /// Negative limit
  pub u8, from into NegLimit, timeout, set_timeout: 1, 0;
}

impl Default for Ilimit {
    /// Default current limit of positive 1000 mA and negative 1100 mA
    fn default() -> Self {
        Ilimit(0x11)
    }
}

/// Source of open/short fault
#[derive(Copy, Clone, Debug)]
pub enum OpenShort {
    /// No fault
    None = 0,
    /// Red driver
    Red = 1,
    /// Green driver
    Green = 2,
    /// Blue driver
    Blue = 3,
}

impl From<u8> for OpenShort {
    fn from(x: u8) -> Self {
        match x {
            1 => OpenShort::Red,
            2 => OpenShort::Green,
            3 => OpenShort::Blue,
            _ => OpenShort::None,
        }
    }
}

/// Selects how long device stays in active mode after all enable pins have gone low.
#[derive(Copy, Clone, Debug)]
pub enum Timeout {
    /// 125 ms
    MS125 = 0,
    /// 250 ms
    MS250 = 1,
    /// 500 ms
    MS500 = 2,
    /// 1000 ms (1 s)
    MS1000 = 3,
}

impl From<u8> for Timeout {
    fn from(x: u8) -> Self {
        match x {
            0 => Timeout::MS125,
            1 => Timeout::MS250,
            2 => Timeout::MS500,
            _ => Timeout::MS1000,
        }
    }
}

impl From<Timeout> for u8 {
    fn from(t: Timeout) -> Self {
        t as u8
    }
}

/// Selects soft start time
#[derive(Copy, Clone, Debug)]
pub enum SoftStart {
    /// No soft start
    None = 0,
    /// 0.5 s
    MS500 = 1,
    /// 1 s
    MS1000 = 2,
    /// 2 s
    MS2000 = 3,
}

impl From<u8> for SoftStart {
    fn from(x: u8) -> Self {
        match x {
            0 => SoftStart::None,
            1 => SoftStart::MS500,
            2 => SoftStart::MS1000,
            _ => SoftStart::MS2000,
        }
    }
}

impl From<SoftStart> for u8 {
    fn from(s: SoftStart) -> Self {
        s as u8
    }
}

bitfield! {
  /// Control register
  pub struct Ctrl(u8);
  impl Debug;
  /// Soft start control
  pub u8, from into SoftStart, softstart, set_softstart: 5,4;
  /// Timeout control
  pub u8, from into Timeout, timeout, set_timeout: 3, 2;
  /// Enable fade control from Fader register
  pub mfe, set_mfe: 1;
  /// Enable fade control from PWM input (overrides mfe).
  pub pwm, set_pwm: 0;
}

impl Default for Ctrl {
    /// Default control register value, no fader control, no soft start, 125 ms timeout.
    fn default() -> Self {
        Ctrl(0x00)
    }
}

bitfield! {
  /// Fault register
  pub struct Fault(u8);
  impl Debug;
  /// Shorted drivers
  pub u8, into OpenShort, short, _: 6,5;
  /// Open drivers
  pub u8, into OpenShort, open, _: 4,3;
  /// Under voltage lock-out
  pub uvlo, _: 2;
  /// Temperature shutdown
  pub tsd, _: 1;
  /// Overcurrent
  pub ocp, _: 0;
}

impl Fault {
    /// No flags are active
    pub fn is_ok(&self) -> bool {
        !self.is_err()
    }

    /// One or more fault flags are active
    pub fn is_err(&self) -> bool {
        self.0 == 0x00
    }
}

bitfield! {
  /// Enable faults to drive the FAULT open-drain output
  pub struct FaultMask(u8);
  impl Debug;
  /// Shorted drivers
  pub short, _: 4;
  /// Open drivers
  pub open, _: 3;
  /// Under voltage lock-out
  pub uvlo, _: 2;
  /// Temperature shutdown
  pub tsd, _: 1;
  /// Overcurrent
  pub ocp, _: 0;
}

impl Default for FaultMask {
    /// No faults enabled
    fn default() -> Self {
        FaultMask(0x00)
    }
}
