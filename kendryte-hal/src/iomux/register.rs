use arbitrary_int::{u1, u3};
use bitbybit::{bitenum, bitfield};
use volatile_register::RW;

/// IOMUX Register Block.
///
/// This structure represents the memory-mapped registers of a IOMUX peripheral.
/// Each field corresponds to a specific register or group of registers.
#[repr(C)]
pub struct RegisterBlock {
    pub(crate) pads: [RW<Pad>; 64],
}

/// SlewRate controls the speed of the output signal transition.
/// Fast means a faster transition, while Slow means a slower transition.
#[bitenum(u1, exhaustive = true)]
#[derive(Debug, PartialEq, Eq)]
pub enum SlewRate {
    /// Fast transition speed.
    Fast = 0b0,
    /// Slow transition speed.
    Slow = 0b1,
}

/// Strength sets the drive strength of the IO pad.
/// The value ranges from 0 (weakest) to 15 (strongest).
#[bitenum(u4, exhaustive = true)]
#[derive(Debug, PartialEq, Eq)]
pub enum Strength {
    _0 = 0b0000,
    _1 = 0b0001,
    _2 = 0b0010,
    _3 = 0b0011,
    _4 = 0b0100,
    _5 = 0b0101,
    _6 = 0b0110,
    _7 = 0b0111,
    _8 = 0b1000,
    _9 = 0b1001,
    _10 = 0b1010,
    _11 = 0b1011,
    _12 = 0b1100,
    _13 = 0b1101,
    _14 = 0b1110,
    _15 = 0b1111,
}

/// Pad represents the configuration of a single IO pad.
/// Each field controls a specific aspect of the pad's behavior.
#[bitfield(u32)]
pub struct Pad {
    /// Input data from outside.
    #[bit(31, r)]
    pub data_input: u1,

    /// IO function select, determines the function of the pad.
    #[bits(11..=13, rw)]
    pub function_select: u3,

    /// Slew rate control, sets the output transition speed.
    #[bit(10, rw)]
    pub slew_rate: SlewRate,

    /// Input enable, allows the pad to receive input.
    #[bit(8, rw)]
    pub input_enable: bool,

    /// Output enable, allows the pad to drive output.
    #[bit(7, rw)]
    pub output_enable: bool,

    /// Pull up enable, enables the internal pull-up resistor.
    #[bit(6, rw)]
    pub pull_up_enable: bool,

    /// Pull down enable, enables the internal pull-down resistor.
    #[bit(5, rw)]
    pub pull_down_enable: bool,

    /// Drive strength control, sets the output drive strength.
    #[bits(1..=4, rw)]
    pub drive_strength: Strength,

    /// Schmitt trigger enable, enables the Schmitt trigger for input.
    #[bit(0, rw)]
    pub schmitt_trigger_enable: bool,
}
