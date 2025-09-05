//! Dynamic GPIO pin implementation
//!
//! This module provides the [`Dynamic`] type for GPIO pins that can switch
//! between input and output modes at runtime. This is useful when the pin
//! direction needs to change during program execution.

use crate::gpio::blocking::unconfigured::Unconfigured;
use crate::gpio::blocking::{PinCommon, PinInfo};
use crate::gpio::config::Pull;
use crate::gpio::{DriveStrength, GpioError, GpioPort, IntoGpio};
use embedded_hal::digital::{ErrorType, InputPin, OutputPin, PinState, StatefulOutputPin};

/// GPIO pin mode enumeration.
///
/// Tracks the current configuration mode of a dynamic GPIO pin.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PinMode {
    /// Pin configured as input.
    Input,
    /// Pin configured as output.
    Output,
    /// Pin not yet configured.
    Unconfigured,
}

/// Dynamic GPIO pin.
///
/// A GPIO pin that can be reconfigured at runtime between input and output modes.
/// Mode changes are tracked and operations are validated against the current mode.
pub struct Dynamic<'i, 'p> {
    pub(crate) common: PinCommon<'i, 'p>,
    pub(crate) mode: PinMode,
}

/// Implement PinInfo trait for Dynamic pins.
impl<'i, 'p> PinInfo for Dynamic<'i, 'p> {
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

impl<'i, 'p> Dynamic<'i, 'p> {
    /// Create a new dynamic pin from unconfigured state.
    ///
    /// The pin starts in unconfigured mode and must be configured before use.
    pub fn new<const N: usize, P: IntoGpio<'p, N>>(
        instance: impl crate::instance::Numbered<'i, N, R = crate::gpio::MmioRegisterBlock<'static>>,
        pad: P,
    ) -> Self {
        Unconfigured::new(instance, pad).into_dynamic()
    }

    /// Create a dynamic pin configured as input.
    ///
    /// Creates and immediately configures the pin for input operations.
    pub fn new_input<const N: usize, P: IntoGpio<'p, N>>(
        instance: impl crate::instance::Numbered<'i, N, R = crate::gpio::MmioRegisterBlock<'static>>,
        pad: P,
        pull: Pull,
    ) -> Self {
        let mut pin = Self::new(instance, pad);
        pin.configure_as_input(pull);
        pin
    }

    /// Create a dynamic pin configured as output.
    ///
    /// Creates and immediately configures the pin for output operations.
    pub fn new_output<const N: usize, P: IntoGpio<'p, N>>(
        instance: impl crate::instance::Numbered<'i, N, R = crate::gpio::MmioRegisterBlock<'static>>,
        pad: P,
        state: PinState,
        drive_strength: DriveStrength,
    ) -> Self {
        let mut pin = Self::new(instance, pad);
        pin.configure_as_output(state, drive_strength);
        pin
    }

    /// Convenience method: create floating input dynamic pin.
    ///
    /// Creates a dynamic pin configured as floating input (no pull resistors).
    pub fn new_floating_input<const N: usize, P: IntoGpio<'p, N>>(
        instance: impl crate::instance::Numbered<'i, N, R = crate::gpio::MmioRegisterBlock<'static>>,
        pad: P,
    ) -> Self {
        Self::new_input(instance, pad, Pull::None)
    }

    /// Convenience method: create pull-up input dynamic pin.
    ///
    /// Creates a dynamic pin configured as input with pull-up resistor.
    pub fn new_pull_up_input<const N: usize, P: IntoGpio<'p, N>>(
        instance: impl crate::instance::Numbered<'i, N, R = crate::gpio::MmioRegisterBlock<'static>>,
        pad: P,
    ) -> Self {
        Self::new_input(instance, pad, Pull::Up)
    }

    /// Convenience method: create pull-down input dynamic pin.
    ///
    /// Creates a dynamic pin configured as input with pull-down resistor.
    pub fn new_pull_down_input<const N: usize, P: IntoGpio<'p, N>>(
        instance: impl crate::instance::Numbered<'i, N, R = crate::gpio::MmioRegisterBlock<'static>>,
        pad: P,
    ) -> Self {
        Self::new_input(instance, pad, Pull::Down)
    }

    /// Convenience method: create high output dynamic pin.
    ///
    /// Creates a dynamic pin configured as output with initial high state.
    pub fn new_output_high<const N: usize, P: IntoGpio<'p, N>>(
        instance: impl crate::instance::Numbered<'i, N, R = crate::gpio::MmioRegisterBlock<'static>>,
        pad: P,
    ) -> Self {
        Self::new_output(instance, pad, PinState::High, DriveStrength::Medium)
    }

    /// Convenience method: create low output dynamic pin.
    ///
    /// Creates a dynamic pin configured as output with initial low state.
    pub fn new_output_low<const N: usize, P: IntoGpio<'p, N>>(
        instance: impl crate::instance::Numbered<'i, N, R = crate::gpio::MmioRegisterBlock<'static>>,
        pad: P,
    ) -> Self {
        Self::new_output(instance, pad, PinState::Low, DriveStrength::Medium)
    }

    /// Get current pin mode.
    ///
    /// Returns the current configuration mode of the dynamic pin.
    pub fn mode(&self) -> PinMode {
        self.mode
    }

    /// Configure as input mode.
    ///
    /// Switches the pin to input mode with the specified pull resistor setting.
    pub fn configure_as_input(&mut self, pull: Pull) {
        self.common.configure_as_input();
        self.common.set_pull(pull);
        self.mode = PinMode::Input;
    }

    /// Configure as output mode.
    ///
    /// Switches the pin to output mode with specified state and drive strength.
    pub fn configure_as_output(&mut self, state: PinState, drive_strength: DriveStrength) {
        self.common.set_drive_strength(drive_strength);
        self.common.configure_as_output(state);
        self.mode = PinMode::Output;
    }

    /// Read pin state (when configured as input).
    ///
    /// Returns an error if the pin is not in input mode.
    pub fn read_input_state(&self) -> Result<PinState, GpioError> {
        if self.mode != PinMode::Input {
            return Err(GpioError::IncompatibleMode);
        }
        Ok(self.common.read_input_state())
    }

    /// Set output state (when configured as output).
    ///
    /// Returns an error if the pin is not in output mode.
    pub fn set_output_state(&mut self, state: PinState) -> Result<(), GpioError> {
        if self.mode != PinMode::Output {
            return Err(GpioError::IncompatibleMode);
        }
        self.common.set_output_state(state);
        Ok(())
    }

    /// Read output register state (when configured as output).
    ///
    /// Returns an error if the pin is not in output mode.
    pub fn output_state(&self) -> Result<PinState, GpioError> {
        if self.mode != PinMode::Output {
            return Err(GpioError::IncompatibleMode);
        }
        Ok(self.common.output_state())
    }

    /// Configure pull resistor.
    ///
    /// Sets the pull resistor configuration for the pin.
    pub fn set_pull(&mut self, pull: Pull) {
        self.common.set_pull(pull);
    }

    /// Get current pull resistor configuration.
    pub fn pull(&self) -> Result<Pull, GpioError> {
        self.common.pull()
    }

    /// Set drive strength.
    ///
    /// Configures the output drive strength for the pin.
    pub fn set_drive_strength(&mut self, drive_strength: DriveStrength) {
        self.common.set_drive_strength(drive_strength);
    }

    /// Get current drive strength setting.
    pub fn drive_strength(&self) -> DriveStrength {
        self.common.drive_strength()
    }

    /// Convert to dedicated input pin.
    ///
    /// Returns a type-safe input pin that cannot be reconfigured.
    pub fn into_input(mut self, pull: Pull) -> super::Input<'i, 'p> {
        self.common.configure_as_input();
        self.common.set_pull(pull);
        super::Input {
            common: self.common,
        }
    }

    /// Convert to dedicated output pin.
    ///
    /// Returns a type-safe output pin that cannot be reconfigured.
    pub fn into_output(
        mut self,
        state: PinState,
        drive_strength: DriveStrength,
    ) -> super::Output<'i, 'p> {
        self.common.set_drive_strength(drive_strength);
        self.common.configure_as_output(state);
        super::Output {
            common: self.common,
        }
    }

    /// Convert to unconfigured pin.
    ///
    /// Returns the pin to an unconfigured state.
    pub fn into_unconfigured(self) -> Unconfigured<'i, 'p> {
        Unconfigured {
            common: self.common,
        }
    }
}

/// Implement embedded-hal ErrorType trait.
impl<'i, 'p> ErrorType for Dynamic<'i, 'p> {
    type Error = GpioError;
}

/// Implement embedded-hal InputPin trait.
///
/// Note: These operations only succeed when the pin is in input mode.
impl<'i, 'p> InputPin for Dynamic<'i, 'p> {
    fn is_high(&mut self) -> Result<bool, Self::Error> {
        let state = self.read_input_state()?;
        Ok(state == PinState::High)
    }

    fn is_low(&mut self) -> Result<bool, Self::Error> {
        let state = self.read_input_state()?;
        Ok(state == PinState::Low)
    }
}

/// Implement embedded-hal OutputPin trait.
///
/// Note: These operations only succeed when the pin is in output mode.
impl<'i, 'p> OutputPin for Dynamic<'i, 'p> {
    fn set_low(&mut self) -> Result<(), Self::Error> {
        self.set_output_state(PinState::Low)
    }

    fn set_high(&mut self) -> Result<(), Self::Error> {
        self.set_output_state(PinState::High)
    }
}

/// Implement embedded-hal StatefulOutputPin trait.
///
/// Note: These operations only succeed when the pin is in output mode.
impl<'i, 'p> StatefulOutputPin for Dynamic<'i, 'p> {
    fn is_set_high(&mut self) -> Result<bool, Self::Error> {
        let state = self.output_state()?;
        Ok(state == PinState::High)
    }

    fn is_set_low(&mut self) -> Result<bool, Self::Error> {
        let state = self.output_state()?;
        Ok(state == PinState::Low)
    }
}
