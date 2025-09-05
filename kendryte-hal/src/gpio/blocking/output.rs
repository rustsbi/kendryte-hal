//! GPIO output pin implementation
//!
//! This module provides the [`Output`] type for GPIO pins configured as outputs.
//! Output pins can drive digital signals and control external devices.

use crate::gpio::blocking::unconfigured::Unconfigured;
use crate::gpio::blocking::{PinCommon, PinInfo};
use crate::gpio::{MmioRegisterBlock, config::*, error::*, pad::*};
use crate::instance::Numbered;
use embedded_hal::digital::{ErrorType, OutputPin, PinState, StatefulOutputPin};

/// GPIO output pin.
///
/// Represents a GPIO pin configured for output operations. Supports setting
/// digital states, configuring drive strength, and reading back output state.
pub struct Output<'i, 'p> {
    pub(crate) common: PinCommon<'i, 'p>,
}

/// Implement PinInfo trait for Output pins.
impl<'i, 'p> PinInfo for Output<'i, 'p> {
    fn port(&self) -> GpioPort {
        self.common.port()
    }

    fn pin_number(&self) -> usize {
        self.common.pin_number()
    }

    fn instance_number(&self) -> usize {
        self.common.instance_number()
    }
}

impl<'i, 'p> Output<'i, 'p> {
    /// Default drive strength for new output pins.
    pub const DEFAULT_DRIVE_STRENGTH: DriveStrength = DriveStrength::Medium;

    /// Create a new output pin.
    ///
    /// # Arguments
    /// * `instance` - GPIO peripheral instance
    /// * `pad` - Hardware pad to use for this pin
    /// * `state` - Initial output state (High or Low)
    /// * `drive_strength` - Output drive strength setting
    pub fn new<const N: usize, P: IntoGpio<'p, N>>(
        instance: impl Numbered<'i, N, R = MmioRegisterBlock<'static>>,
        pad: P,
        state: PinState,
        drive_strength: DriveStrength,
    ) -> Self {
        Unconfigured::new(instance, pad).into_output(state, drive_strength)
    }

    /// Set the output pin state.
    ///
    /// Changes the output state of the pin to High or Low.
    pub fn set_state(&mut self, state: PinState) -> Result<(), GpioError> {
        self.common.set_output_state(state);
        Ok(())
    }

    /// Read the current output register state.
    ///
    /// Returns the state stored in the output data register.
    pub fn state(&self) -> PinState {
        self.common.output_state()
    }

    /// Set output drive strength.
    ///
    /// Configures the output drive capability of this pin.
    pub fn set_drive_strength(&mut self, strength: DriveStrength) {
        self.common.set_drive_strength(strength);
    }

    /// Get current drive strength setting.
    ///
    /// Returns the current output drive strength configuration.
    pub fn drive_strength(&self) -> DriveStrength {
        self.common.drive_strength()
    }

    /// Convert to input pin.
    ///
    /// Reconfigures this pin as an input with the specified pull resistor setting.
    pub fn into_input(self, pull: Pull) -> super::Input<'i, 'p> {
        self.into_unconfigured().into_input(pull)
    }

    /// Convert to unconfigured pin.
    ///
    /// Returns the pin to an unconfigured state for reconfiguration.
    pub fn into_unconfigured(self) -> Unconfigured<'i, 'p> {
        Unconfigured {
            common: self.common,
        }
    }

    /// Convert to dynamic pin.
    ///
    /// Creates a dynamic pin that can switch between input and output modes at runtime.
    pub fn into_dynamic(self) -> super::Dynamic<'i, 'p> {
        super::Dynamic {
            common: self.common,
            mode: super::dynamic::PinMode::Output,
        }
    }

    /// Convenience constructor: create high output pin.
    ///
    /// Creates an output pin with initial High state and default drive strength.
    pub fn new_high<const N: usize, P: IntoGpio<'p, N>>(
        instance: impl Numbered<'i, N, R = MmioRegisterBlock<'static>>,
        pad: P,
    ) -> Self {
        Self::new(instance, pad, PinState::High, Self::DEFAULT_DRIVE_STRENGTH)
    }

    /// Convenience constructor: create low output pin.
    ///
    /// Creates an output pin with initial Low state and default drive strength.
    pub fn new_low<const N: usize, P: IntoGpio<'p, N>>(
        instance: impl Numbered<'i, N, R = MmioRegisterBlock<'static>>,
        pad: P,
    ) -> Self {
        Self::new(instance, pad, PinState::Low, Self::DEFAULT_DRIVE_STRENGTH)
    }

    /// Convenience constructor: create output pin with specified state.
    ///
    /// Creates an output pin with the given initial state and default drive strength.
    pub fn new_with_state<const N: usize, P: IntoGpio<'p, N>>(
        instance: impl Numbered<'i, N, R = MmioRegisterBlock<'static>>,
        pad: P,
        state: PinState,
    ) -> Self {
        Self::new(instance, pad, state, Self::DEFAULT_DRIVE_STRENGTH)
    }
}

/// Implement embedded-hal ErrorType trait.
impl<'i, 'p> ErrorType for Output<'i, 'p> {
    type Error = GpioError;
}

/// Implement embedded-hal OutputPin trait.
impl<'i, 'p> OutputPin for Output<'i, 'p> {
    fn set_low(&mut self) -> Result<(), Self::Error> {
        self.set_state(PinState::Low)
    }

    fn set_high(&mut self) -> Result<(), Self::Error> {
        self.set_state(PinState::High)
    }
}

/// Implement embedded-hal StatefulOutputPin trait.
impl<'i, 'p> StatefulOutputPin for Output<'i, 'p> {
    fn is_set_high(&mut self) -> Result<bool, Self::Error> {
        Ok(self.state() == PinState::High)
    }

    fn is_set_low(&mut self) -> Result<bool, Self::Error> {
        Ok(self.state() == PinState::Low)
    }
}
