//! GPIO input pin implementation
//!
//! This module provides the [`Input`] type for GPIO pins configured as inputs.
//! Input pins can read digital states and wait for edge transitions.

use crate::gpio::blocking::unconfigured::Unconfigured;
use crate::gpio::blocking::{PinCommon, PinInfo};
use crate::gpio::{config::*, error::*, pad::*};
use crate::instance::Numbered;
use embedded_hal::digital::{ErrorType, InputPin, PinState};

/// GPIO input pin.
///
/// Represents a GPIO pin configured for input operations. Supports reading
/// digital states, configuring pull resistors, and waiting for edge transitions.
pub struct Input<'i, 'p> {
    pub(crate) common: PinCommon<'i, 'p>,
}

/// Implement PinInfo trait for Input pins.
impl<'i, 'p> PinInfo for Input<'i, 'p> {
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

impl<'i, 'p> Input<'i, 'p> {
    /// Create a new input pin.
    ///
    /// # Arguments
    /// * `instance` - GPIO peripheral instance
    /// * `pad` - Hardware pad to use for this pin
    /// * `pull` - Pull resistor configuration
    pub fn new<const N: usize, P: IntoGpio<'p, N>>(
        instance: impl Numbered<'i, N, R = crate::gpio::MmioRegisterBlock<'static>>,
        pad: P,
        pull: Pull,
    ) -> Self {
        Unconfigured::new(instance, pad).into_input(pull)
    }

    /// Read the current pin state.
    ///
    /// Returns the electrical state of the pin (High or Low).
    pub fn read_state(&self) -> PinState {
        self.common.read_input_state()
    }

    /// Configure pull resistor.
    ///
    /// Changes the pull resistor configuration for this input pin.
    pub fn set_pull(&mut self, pull: Pull) {
        self.common.set_pull(pull);
    }

    /// Get current pull resistor configuration.
    ///
    /// Returns the current pull resistor setting.
    pub fn pull(&self) -> Result<Pull, GpioError> {
        self.common.pull()
    }

    /// Convert to output pin.
    ///
    /// Reconfigures this pin as an output with the specified initial state and drive strength.
    pub fn into_output(
        self,
        state: PinState,
        drive_strength: DriveStrength,
    ) -> super::Output<'i, 'p> {
        self.into_unconfigured().into_output(state, drive_strength)
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
            mode: super::dynamic::PinMode::Input,
        }
    }

    /// Convenience constructor: create floating input pin.
    ///
    /// Creates an input pin with no pull resistors (floating/high-impedance).
    pub fn new_floating<const N: usize, P: IntoGpio<'p, N>>(
        instance: impl Numbered<'i, N, R = crate::gpio::MmioRegisterBlock<'static>>,
        pad: P,
    ) -> Self {
        Self::new(instance, pad, Pull::None)
    }

    /// Convenience constructor: create pull-up input pin.
    ///
    /// Creates an input pin with internal pull-up resistor enabled.
    pub fn new_pull_up<const N: usize, P: IntoGpio<'p, N>>(
        instance: impl Numbered<'i, N, R = crate::gpio::MmioRegisterBlock<'static>>,
        pad: P,
    ) -> Self {
        Self::new(instance, pad, Pull::Up)
    }

    /// Convenience constructor: create pull-down input pin.
    ///
    /// Creates an input pin with internal pull-down resistor enabled.
    pub fn new_pull_down<const N: usize, P: IntoGpio<'p, N>>(
        instance: impl Numbered<'i, N, R = crate::gpio::MmioRegisterBlock<'static>>,
        pad: P,
    ) -> Self {
        Self::new(instance, pad, Pull::Down)
    }

    /// Block until pin goes high.
    ///
    /// Continuously polls the pin state until it reads as High.
    /// Warning: This is a blocking operation that will spin-wait.
    pub fn wait_for_high(&mut self) {
        while self.read_state() == PinState::Low {
            core::hint::spin_loop();
        }
    }

    /// Block until pin goes low.
    ///
    /// Continuously polls the pin state until it reads as Low.
    /// Warning: This is a blocking operation that will spin-wait.
    pub fn wait_for_low(&mut self) {
        while self.read_state() == PinState::High {
            core::hint::spin_loop();
        }
    }

    /// Block until rising edge (Low -> High).
    ///
    /// Waits for the pin to go low first, then waits for it to go high.
    /// Warning: This is a blocking operation that will spin-wait.
    pub fn wait_for_rising_edge(&mut self) {
        // Wait for pin to go low first
        self.wait_for_low();
        // Then wait for it to go high
        self.wait_for_high();
    }

    /// Block until falling edge (High -> Low).
    ///
    /// Waits for the pin to go high first, then waits for it to go low.
    /// Warning: This is a blocking operation that will spin-wait.
    pub fn wait_for_falling_edge(&mut self) {
        // Wait for pin to go high first
        self.wait_for_high();
        // Then wait for it to go low
        self.wait_for_low();
    }

    /// Block until any edge transition.
    ///
    /// Waits for the pin state to change from its current value.
    /// Warning: This is a blocking operation that will spin-wait.
    pub fn wait_for_any_edge(&mut self) {
        let initial_state = self.read_state();
        while self.read_state() == initial_state {
            core::hint::spin_loop();
        }
    }

    /// Wait for pin to go high with timeout.
    ///
    /// # Arguments
    /// * `max_iterations` - Maximum polling iterations before timing out
    ///
    /// # Returns
    /// * `Ok(())` if pin went high within timeout
    /// * `Err(GpioError::Timeout)` if timeout exceeded
    pub fn wait_for_high_timeout(&mut self, max_iterations: u32) -> Result<(), GpioError> {
        for _ in 0..max_iterations {
            if self.read_state() == PinState::High {
                return Ok(());
            }
            core::hint::spin_loop();
        }
        Err(GpioError::Timeout)
    }

    /// Wait for pin to go low with timeout.
    ///
    /// # Arguments
    /// * `max_iterations` - Maximum polling iterations before timing out
    ///
    /// # Returns
    /// * `Ok(())` if pin went low within timeout
    /// * `Err(GpioError::Timeout)` if timeout exceeded
    pub fn wait_for_low_timeout(&mut self, max_iterations: u32) -> Result<(), GpioError> {
        for _ in 0..max_iterations {
            if self.read_state() == PinState::Low {
                return Ok(());
            }
            core::hint::spin_loop();
        }
        Err(GpioError::Timeout)
    }

    /// Wait for rising edge with timeout.
    ///
    /// Waits for Low->High transition with a maximum iteration limit.
    pub fn wait_for_rising_edge_timeout(&mut self, max_iterations: u32) -> Result<(), GpioError> {
        self.wait_for_low_timeout(max_iterations / 2)?;
        self.wait_for_high_timeout(max_iterations / 2)
    }

    /// Wait for falling edge with timeout.
    ///
    /// Waits for High->Low transition with a maximum iteration limit.
    pub fn wait_for_falling_edge_timeout(&mut self, max_iterations: u32) -> Result<(), GpioError> {
        self.wait_for_high_timeout(max_iterations / 2)?;
        self.wait_for_low_timeout(max_iterations / 2)
    }

    /// Wait for any edge transition with timeout.
    ///
    /// Waits for any state change with a maximum iteration limit.
    pub fn wait_for_any_edge_timeout(&mut self, max_iterations: u32) -> Result<(), GpioError> {
        let initial_state = self.read_state();
        for _ in 0..max_iterations {
            if self.read_state() != initial_state {
                return Ok(());
            }
            core::hint::spin_loop();
        }
        Err(GpioError::Timeout)
    }
}

/// Implement embedded-hal ErrorType trait.
impl<'i, 'p> ErrorType for Input<'i, 'p> {
    type Error = GpioError;
}

/// Implement embedded-hal InputPin trait.
impl<'i, 'p> InputPin for Input<'i, 'p> {
    fn is_high(&mut self) -> Result<bool, Self::Error> {
        Ok(self.read_state() == PinState::High)
    }

    fn is_low(&mut self) -> Result<bool, Self::Error> {
        Ok(self.read_state() == PinState::Low)
    }
}
