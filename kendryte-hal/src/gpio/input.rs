use crate::gpio::pad::{IntoGpio, Port};
use crate::gpio::{Direction, Output, RegisterBlock};
use crate::instance::Numbered;
use crate::iomux::FlexPad;
use crate::iomux::ops::{PadOps, Pull};
use crate::iomux::pad::Strength;
use core::convert::Infallible;
use core::marker::PhantomData;
use embedded_hal::digital::{ErrorType, InputPin, PinState};

/// Represents a GPIO input pin.
pub struct Input<'i, 'p> {
    pub(crate) inner: &'static RegisterBlock,
    pub(crate) pad: FlexPad<'p>,
    pub(crate) port: Port,
    pub(crate) pin_num: usize,
    pub(crate) _marker: PhantomData<&'i ()>,
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

    /// Converts the pin into an output pin with specified state and drive strength.
    pub fn into_output(self, pin_state: PinState, drive_strength: Strength) -> Output<'i, 'p> {
        self.pad.set_drive_strength(drive_strength);
        match self.port {
            Port::A => unsafe {
                self.inner
                    .swporta_ddr
                    .modify(|r| r.with_direction(self.pin_num, Direction::Output));
                self.inner
                    .swporta_dr
                    .modify(|r| r.with_pin_state(self.pin_num, pin_state.into()))
            },
            Port::B => unsafe {
                self.inner
                    .swportb_ddr
                    .modify(|r| r.with_direction(self.pin_num, Direction::Output));
                self.inner
                    .swportb_dr
                    .modify(|r| r.with_pin_state(self.pin_num, pin_state.into()))
            },
        }

        Output {
            inner: self.inner,
            pad: self.pad,
            port: self.port,
            pin_num: self.pin_num,
            _marker: PhantomData,
        }
    }

    /// Converts the pin into an output pin with initial high state.
    pub fn into_output_with_high(self) -> Output<'i, 'p> {
        self.into_output(PinState::High, Strength::_7)
    }

    /// Converts the pin into an output pin with initial low state.
    pub fn into_output_with_low(self) -> Output<'i, 'p> {
        self.into_output(PinState::Low, Strength::_7)
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
