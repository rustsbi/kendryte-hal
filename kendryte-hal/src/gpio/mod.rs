//! GPIO (General Purpose Input/Output) module for Kendryte K230.
//!
//! This module provides a high-level GPIO API that is compatible with embedded-hal traits.
//! It supports both input and output operations with configurable pull resistors and drive strength.
//!
//! # Features
//! - Input pins with configurable pull-up/pull-down resistors.
//! - Output pins with configurable drive strength.
//! - Dynamic pins that can switch between input and output modes.
//! - Blocking operations for edge detection and state changes.
//! - Full embedded-hal compatibility.
//!
//! # Example
//! ```rust
//! use kendryte_hal::gpio::{Input, Output, Pull, DriveStrength};
//! use kendryte_hal::pac::GPIO0;
//! use embedded_hal::digital::{InputPin, OutputPin};
//!
//! // Create an input pin with pull-up resistor.
//! let input_pin = Input::new_pull_up(gpio0, pad_a0);
//!
//! // Create an output pin starting with high state.
//! let mut output_pin = Output::new_high(gpio0, pad_a1);
//!
//! // Use embedded-hal traits.
//! if input_pin.is_high()? {
//!     output_pin.set_low()?;
//! }
//! ```

pub mod blocking;
pub mod config;
pub mod error;
pub mod pad;
pub mod register;

// Re-export core types for convenient access
pub use blocking::{Dynamic, Input, Output, PinCommon, PinInfo, PinMode, Unconfigured};
pub use config::DriveStrength;
pub use error::GpioError;
pub use pad::{GpioPort, IntoGpio};
pub use register::*;

// Re-export embedded-hal traits for convenience
pub use embedded_hal::digital::*;
