//! Blocking GPIO pin implementations.
//!
//! This module provides blocking GPIO pin types that implement embedded-hal traits.
//! All operations are synchronous and will block until completion.
//!
//! # Pin Types
//! - [`Input`] - Input pins with configurable pull resistors.
//! - [`Output`] - Output pins with configurable drive strength.
//! - [`Dynamic`] - Pins that can switch between input and output modes.
//! - [`Unconfigured`] - Unconfigured pins that can be converted to any mode.
//!
//! # Common Functionality
//! All pin types share common functionality through the [`PinCommon`] structure
//! and [`PinInfo`] trait, including drive strength control and pin state reading.

mod dynamic;
mod input;
mod output;
mod unconfigured;

use core::marker::PhantomData;
pub use dynamic::{Dynamic, PinMode};
pub use input::Input;
pub use output::Output;
pub use unconfigured::Unconfigured;
// Re-export embedded-hal traits for convenience
pub use embedded_hal::digital::{ErrorType, InputPin, OutputPin, PinState, StatefulOutputPin};

use crate::gpio::config::Pull;
use crate::gpio::{Direction, DriveStrength, GpioError, GpioPort, MmioRegisterBlock};
use crate::iomux::FlexPad;
use crate::iomux::ops::PadOps;

/// Common pin information trait.
///
/// Provides access to basic pin information shared across all pin types.
/// This trait helps reduce code duplication by providing a common interface.
pub trait PinInfo {
    /// Get the GPIO port this pin belongs to.
    fn port(&self) -> GpioPort;

    /// Get the pin number within the port (0-31).
    fn pin_number(&self) -> usize;

    /// Get the GPIO instance number.
    fn instance_number(&self) -> usize;
}

/// Common GPIO pin structure.
///
/// Contains the hardware registers and pad configuration shared by all pin types.
/// This structure provides the low-level hardware interface for GPIO operations.
pub struct PinCommon<'i, 'p> {
    pub(crate) inner: MmioRegisterBlock<'static>,
    pub(crate) pad: FlexPad<'p>,
    pub(crate) numbered: usize,
    pub(crate) port: GpioPort,
    pub(crate) pin_num: usize,
    pub(crate) _marker: PhantomData<&'i ()>,
}

/// Implement PinInfo trait for PinCommon.
impl<'i, 'p> PinInfo for PinCommon<'i, 'p> {
    /// Get the GPIO port this pin belongs to.
    fn port(&self) -> GpioPort {
        self.port
    }

    /// Get the pin number within the port.
    fn pin_number(&self) -> usize {
        self.pin_num
    }

    /// Get the GPIO instance number.
    fn instance_number(&self) -> usize {
        self.numbered
    }
}

/// Common methods available on all pin types.
///
/// These methods provide hardware-level access to GPIO functionality.
/// and are used by higher-level pin type implementations.
impl<'i, 'p> PinCommon<'i, 'p> {
    /// Read the current pin input state.
    ///
    /// This method reads the actual electrical state of the pin from the hardware,
    /// regardless of whether it's configured as input or output.
    pub fn read_input_state(&self) -> PinState {
        match self.port {
            GpioPort::A => self
                .inner
                .read_ext_porta()
                .external_pin_state(self.pin_num)
                .into(),
            GpioPort::B => self
                .inner
                .read_ext_portb()
                .external_pin_state(self.pin_num)
                .into(),
        }
    }

    /// Read the output register state.
    ///
    /// Returns the state set in the output data register, which may differ
    /// from the actual pin state if the pin is not configured as output.
    pub fn output_state(&self) -> PinState {
        match self.port {
            GpioPort::A => self.inner.read_swporta_dr().pin_state(self.pin_num).into(),
            GpioPort::B => self.inner.read_swportb_dr().pin_state(self.pin_num).into(),
        }
    }

    /// Set the output register state.
    ///
    /// This method updates the output data register. The pin must be configured
    /// as output for this to have any effect on the actual pin state.
    pub fn set_output_state(&mut self, state: PinState) {
        match self.port {
            GpioPort::A => unsafe {
                self.inner
                    .modify_swporta_dr(|r| r.with_pin_state(self.pin_num, state.into()));
            },
            GpioPort::B => unsafe {
                self.inner
                    .modify_swportb_dr(|r| r.with_pin_state(self.pin_num, state.into()));
            },
        }
    }

    /// Configure pull resistor setting.
    ///
    /// Sets the internal pull resistor configuration for this pin.
    pub fn set_pull(&mut self, pull: Pull) {
        self.pad.set_pull(pull);
    }

    /// Get current pull resistor configuration.
    ///
    /// Returns the current pull resistor setting, or an error if unable to read.
    pub fn pull(&self) -> Result<Pull, GpioError> {
        self.pad.pull().ok_or(GpioError::HardwareError)
    }

    /// Set output drive strength.
    ///
    /// Configures the output drive strength, affecting current capability and switching speed.
    pub fn set_drive_strength(&mut self, drive_strength: DriveStrength) {
        self.pad.set_drive_strength(drive_strength.into());
    }

    /// Get current drive strength setting.
    ///
    /// Returns the current output drive strength configuration.
    pub fn drive_strength(&self) -> DriveStrength {
        self.pad.drive_strength().into()
    }

    /// Internal method: configure pin as input.
    ///
    /// Sets the data direction register to configure this pin as an input.
    pub(crate) fn configure_as_input(&mut self) {
        unsafe {
            match self.port {
                GpioPort::A => self
                    .inner
                    .modify_swporta_ddr(|r| r.with_direction(self.pin_num, Direction::Input)),
                GpioPort::B => self
                    .inner
                    .modify_swportb_ddr(|r| r.with_direction(self.pin_num, Direction::Input)),
            }
        }
    }

    /// Internal method: configure pin as output.
    ///
    /// Sets the data direction register to output mode and sets initial state.
    pub(crate) fn configure_as_output(&mut self, pin_state: PinState) {
        unsafe {
            match self.port {
                GpioPort::A => {
                    self.inner
                        .modify_swporta_ddr(|r| r.with_direction(self.pin_num, Direction::Output));
                    self.inner
                        .modify_swporta_dr(|r| r.with_pin_state(self.pin_num, pin_state.into()))
                }
                GpioPort::B => {
                    self.inner
                        .modify_swportb_ddr(|r| r.with_direction(self.pin_num, Direction::Output));
                    self.inner
                        .modify_swportb_dr(|r| r.with_pin_state(self.pin_num, pin_state.into()))
                }
            }
        }
    }
}
