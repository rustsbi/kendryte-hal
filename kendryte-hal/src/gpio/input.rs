use crate::gpio::{Direction, Instance, RegisterBlock};
use crate::instance::SharedInstance;
use crate::pad::function::gpio::{GpioFunction, Port};
use crate::pad::pad_ops::{PadOps, Pull};
use crate::pad::{FlexPad, Pad};
use core::convert::Infallible;
use embedded_hal::digital::{ErrorType, InputPin, PinState};

/// Represents a GPIO input pin.
pub struct Input<'pad> {
    inner: &'static RegisterBlock,
    pad: &'pad mut FlexPad,
    port: Port,
    pin_num: usize,
}

impl<'pad> Input<'pad> {
    /// Creates a new Input instance for a specific pad and GPIO port.
    pub fn new<const PAD_NUM: usize, const GPIO_NUM: usize>(
        instance: &Instance<GPIO_NUM>,
        pad: &'pad mut Pad<PAD_NUM>,
        pull: Pull,
    ) -> Self
    where
        Pad<PAD_NUM>: GpioFunction<GPIO_NUM>,
    {
        pad.set_gpio_function();
        pad.set_pull(pull);
        let port = <Pad<PAD_NUM> as GpioFunction<GPIO_NUM>>::PORT;
        let pin_num = <Pad<PAD_NUM> as GpioFunction<GPIO_NUM>>::PIN_NUM;

        unsafe {
            match port {
                Port::A => instance
                    .inner()
                    .swporta_ddr
                    .modify(|r| r.with_direction(pin_num, Direction::Input)),
                Port::B => instance
                    .inner()
                    .swportb_ddr
                    .modify(|r| r.with_direction(pin_num, Direction::Input)),
            }
        }

        Self {
            inner: instance.inner(),
            pad: pad.as_flexible_mut(),
            port,
            pin_num,
        }
    }

    /// Reads the current state of the input pin.
    pub fn pin_state(&mut self) -> PinState {
        match self.port {
            Port::A => self
                .inner
                .ext_porta
                .read()
                .external_pin_state(self.pin_num)
                .into(),
            Port::B => self
                .inner
                .ext_portb
                .read()
                .external_pin_state(self.pin_num)
                .into(),
        }
    }
}

impl<'pad> ErrorType for Input<'pad> {
    type Error = Infallible;
}

impl<'pad> InputPin for Input<'pad> {
    fn is_high(&mut self) -> Result<bool, Self::Error> {
        match self.port {
            Port::A => {
                let pin_state: PinState = self
                    .inner
                    .ext_porta
                    .read()
                    .external_pin_state(self.pin_num)
                    .into();
                Ok(pin_state == PinState::High)
            }
            Port::B => {
                let pin_state: PinState = self
                    .inner
                    .ext_portb
                    .read()
                    .external_pin_state(self.pin_num)
                    .into();
                Ok(pin_state == PinState::High)
            }
        }
    }

    fn is_low(&mut self) -> Result<bool, Self::Error> {
        match self.port {
            Port::A => {
                let pin_state: PinState = self
                    .inner
                    .ext_porta
                    .read()
                    .external_pin_state(self.pin_num)
                    .into();
                Ok(pin_state == PinState::Low)
            }
            Port::B => {
                let pin_state: PinState = self
                    .inner
                    .ext_portb
                    .read()
                    .external_pin_state(self.pin_num)
                    .into();
                Ok(pin_state == PinState::Low)
            }
        }
    }
}
