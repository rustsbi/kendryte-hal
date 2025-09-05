//! GPIO error types and error handling.
//!
//! This module defines error types that can occur during GPIO operations,
//! with support for embedded-hal error traits.

/// GPIO operation error types.
///
/// These errors can occur during GPIO pin configuration and operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GpioError {
    /// Pin configuration failed during setup.
    ConfigurationFailed,
    /// Hardware register access error.
    HardwareError,
    /// Pin mode incompatible with requested operation.
    IncompatibleMode,
    /// Operation timed out waiting for condition.
    Timeout,
}

impl core::fmt::Display for GpioError {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::ConfigurationFailed => write!(f, "Pin configuration failed"),
            Self::HardwareError => write!(f, "Hardware access error"),
            Self::IncompatibleMode => write!(f, "Pin mode not compatible with operation"),
            Self::Timeout => write!(f, "Operation timeout"),
        }
    }
}

impl embedded_hal::digital::Error for GpioError {
    fn kind(&self) -> embedded_hal::digital::ErrorKind {
        match self {
            _ => embedded_hal::digital::ErrorKind::Other,
        }
    }
}
