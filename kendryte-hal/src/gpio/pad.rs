//! GPIO pad and port definitions.
//!
//! This module defines GPIO port types and traits for converting
//! hardware pads into GPIO pins.

use crate::iomux::FlexPad;

/// GPIO port enumeration.
///
/// Kendryte K230 has two GPIO ports (A and B), each supporting multiple pins.
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum GpioPort {
    /// GPIO Port A - supports pins 0-31.
    A,
    /// GPIO Port B - supports pins 0-31.
    B,
}

/// Trait for converting hardware pads into GPIO pins.
///
/// This trait provides the necessary information and functionality
/// to convert a hardware pad into a usable GPIO pin.
pub trait IntoGpio<'p, const N: usize> {
    /// The GPIO port this pad belongs to.
    const PORT: GpioPort;
    /// The pin number within the port (0-31).
    const PIN_NUM: usize;
    /// Convert this pad into a flexible GPIO pad.
    fn into_gpio(self) -> FlexPad<'p>;
}
