//! Unconfigured GPIO pin implementation
//!
//! This module provides the [`Unconfigured`] type for GPIO pins that haven't
//! been configured yet. These pins can be converted to any other pin type.

use crate::gpio::blocking::{PinCommon, PinInfo};
use crate::gpio::config::Pull;
use crate::gpio::{DriveStrength, Dynamic, GpioPort, IntoGpio, MmioRegisterBlock};
use crate::instance::{Instance, Numbered};
use core::marker::PhantomData;
use embedded_hal::digital::PinState;

/// Unconfigured GPIO pin.
///
/// Represents a GPIO pin that has not been configured for any specific mode.
/// Can be converted to input, output, or dynamic pins.
pub struct Unconfigured<'i, 'p> {
    pub(crate) common: PinCommon<'i, 'p>,
}

/// Implement PinInfo trait for Unconfigured pins.
impl<'i, 'p> PinInfo for Unconfigured<'i, 'p> {
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

/// Construction and conversion methods for unconfigured pins.
impl<'i, 'p> Unconfigured<'i, 'p> {
    /// Create a new unconfigured pin.
    ///
    /// # Arguments
    /// * `instance` - GPIO peripheral instance
    /// * `pad` - Hardware pad to use for this pin
    pub fn new<const N: usize, P: IntoGpio<'p, N>>(
        instance: impl Numbered<'i, N, R = MmioRegisterBlock<'static>>,
        pad: P,
    ) -> Self {
        let pad = pad.into_gpio();
        let numbered = N;
        let port = P::PORT;
        let pin_num = P::PIN_NUM;
        let inner = instance.inner();

        let common = PinCommon {
            inner,
            pad,
            numbered,
            port,
            pin_num,
            _marker: PhantomData,
        };

        Self { common }
    }

    /// Convert to input pin.
    ///
    /// Configures the pin for input operations with the specified pull resistor.
    pub fn into_input(mut self, pull: Pull) -> super::Input<'i, 'p> {
        // Configure as input mode
        self.common.configure_as_input();

        // Set pull resistor configuration
        self.common.set_pull(pull);

        super::Input {
            common: self.common,
        }
    }

    /// Convert to output pin.
    ///
    /// Configures the pin for output operations with specified initial state and drive strength.
    pub fn into_output(
        mut self,
        state: PinState,
        drive_strength: DriveStrength,
    ) -> super::Output<'i, 'p> {
        // Set drive strength
        self.common.set_drive_strength(drive_strength);

        // Configure as output mode and set initial state
        self.common.configure_as_output(state);

        super::Output {
            common: self.common,
        }
    }

    /// Convert to dynamic pin.
    ///
    /// Creates a dynamic pin that can be reconfigured at runtime.
    pub fn into_dynamic(self) -> Dynamic<'i, 'p> {
        Dynamic {
            common: self.common,
            mode: super::dynamic::PinMode::Unconfigured,
        }
    }
}
