use crate::gpio::pad::{IntoGpio, Port};
use crate::gpio::{Direction, RegisterBlock};
use crate::instance::Numbered;
use crate::iomux::FlexPad;
use crate::iomux::ops::{PadOps, Pull};
use core::convert::Infallible;
use core::marker::PhantomData;
use embedded_hal::digital::{ErrorType, InputPin, PinState};

/// Represents a GPIO input pin.
pub struct Input<'i, 'p> {
    inner: &'static RegisterBlock,
    pad: FlexPad<'p>,
    port: Port,
    pin_num: usize,
    _marker: PhantomData<&'i ()>,
}

impl<'i, 'p> Input<'i, 'p> {
    /// Creates a new Input instance for a specific pad and GPIO port.
    pub fn new<const N: usize, P>(
        instance: impl Numbered<'i, N, R = RegisterBlock>,
        pad: P,
        pull: Pull,
    ) -> Self
    where
        P: PadOps + IntoGpio<'p, N>,
    {
        let mut pad = pad.into_gpio();
        pad.set_pull(pull);
        let port = <P as IntoGpio<N>>::PORT;
        let pin_num = <P as IntoGpio<N>>::PIN_NUM;
        let inner = instance.inner();

        unsafe {
            match port {
                Port::A => inner
                    .swporta_ddr
                    .modify(|r| r.with_direction(pin_num, Direction::Input)),
                Port::B => inner
                    .swportb_ddr
                    .modify(|r| r.with_direction(pin_num, Direction::Input)),
            }
        }

        Self {
            inner,
            pad,
            port,
            pin_num,
            _marker: PhantomData,
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

impl<'i, 'p> ErrorType for Input<'i, 'p> {
    type Error = Infallible;
}

impl<'i, 'p> InputPin for Input<'i, 'p> {
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
