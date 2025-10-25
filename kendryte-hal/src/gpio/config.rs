//! GPIO configuration types and enums.
//!
//! This module defines configuration options for GPIO pins, including
//! drive strength levels and pull resistor settings.

pub use crate::iomux::ops::Pull;

use crate::iomux::pad::Strength;

/// GPIO pin drive strength configuration.
///
/// Controls the output current capability of GPIO pins. Higher drive strength
/// allows faster switching and driving heavier loads, but increases power consumption.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DriveStrength {
    /// Low drive strength - suitable for light loads and low-speed signals.
    #[default]
    Low,
    /// Medium drive strength - balanced performance and power consumption.
    Medium,
    /// High drive strength - for heavier loads and faster switching.
    High,
    /// Maximum drive strength - highest current capability.
    Maximum,
}

impl Into<Strength> for DriveStrength {
    fn into(self) -> Strength {
        match self {
            DriveStrength::Low => Strength::_1,
            DriveStrength::Medium => Strength::_4,
            DriveStrength::High => Strength::_7,
            DriveStrength::Maximum => Strength::_15,
        }
    }
}

impl From<Strength> for DriveStrength {
    fn from(strength: Strength) -> Self {
        match strength {
            // Low range: 0-2 -> Low
            Strength::_0 | Strength::_1 | Strength::_2 => DriveStrength::Low,
            // Medium range: 3-5 -> Medium
            Strength::_3 | Strength::_4 | Strength::_5 => DriveStrength::Medium,
            // High range: 6-10 -> High
            Strength::_6 | Strength::_7 | Strength::_8 | Strength::_9 | Strength::_10 => {
                DriveStrength::High
            }
            // Maximum range: 11-15 -> Maximum
            Strength::_11 | Strength::_12 | Strength::_13 | Strength::_14 | Strength::_15 => {
                DriveStrength::Maximum
            }
        }
    }
}
