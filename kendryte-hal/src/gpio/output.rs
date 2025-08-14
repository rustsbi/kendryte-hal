use crate::gpio::pad::{IntoGpio, Port};
use crate::gpio::{Direction, Input, MmioRegisterBlock};
use crate::instance::Numbered;
use crate::iomux::FlexPad;
use crate::iomux::ops::{PadOps, Pull};
use crate::iomux::pad::Strength;
use arbitrary_int::u4;
use core::convert::Infallible;
use core::marker::PhantomData;
use embedded_hal::digital::{ErrorType, OutputPin, PinState, StatefulOutputPin};

/// Represents a GPIO output pin.
pub struct Output<'i, 'p> {
    pub(crate) inner: MmioRegisterBlock<'static>,
    pub(crate) pad: FlexPad<'p>,
    pub(crate) port: Port,
    pub(crate) pin_num: usize,
    pub(crate) _marker: PhantomData<&'i ()>,
}

impl<'i, 'p> Output<'i, 'p> {
    /// Creates a new Output instance for a specific pad and GPIO port.
    pub fn new<const N: usize, P>(
        instance: impl Numbered<'i, N, R = MmioRegisterBlock<'static>>,
        pad: P,
        pin_state: PinState,
        drive_strength: Strength,
    ) -> Self
    where
        P: IntoGpio<'p, N>,
    {
        let mut pad = pad.into_gpio();
        pad.set_drive_strength(drive_strength);
        let port = <P as IntoGpio<N>>::PORT;
        let pin_num = <P as IntoGpio<N>>::PIN_NUM;
        let mut inner = instance.inner();

        match port {
            Port::A => unsafe {
                inner.modify_swporta_ddr(|r| r.with_direction(pin_num, Direction::Output));
                inner.modify_swporta_dr(|r| r.with_pin_state(pin_num, pin_state.into()))
            },
            Port::B => unsafe {
                inner.modify_swporta_ddr(|r| r.with_direction(pin_num, Direction::Output));
                inner.modify_swporta_dr(|r| r.with_pin_state(pin_num, pin_state.into()))
            },
        }

        Self {
            inner,
            pad,
            port,
            pin_num,
            _marker: PhantomData,
        }
    }

    /// Reads the current output state of the pin.
    pub fn pin_state(&mut self) -> PinState {
        match self.port {
            Port::A => self.inner.read_swporta_dr().pin_state(self.pin_num).into(),
            Port::B => self.inner.read_swportb_dr().pin_state(self.pin_num).into(),
        }
    }
    /// Converts the pin into an input pin with specified pull configuration.
    pub fn into_input(mut self, pull: Pull) -> Input<'i, 'p> {
        self.pad.set_pull(pull);
        unsafe {
            match self.port {
                Port::A => self
                    .inner
                    .modify_swporta_ddr(|r| r.with_direction(self.pin_num, Direction::Input)),
                Port::B => self
                    .inner
                    .modify_swportb_ddr(|r| r.with_direction(self.pin_num, Direction::Input)),
            }
        }

        Input {
            inner: self.inner,
            pad: self.pad,
            port: self.port,
            pin_num: self.pin_num,
            _marker: PhantomData,
        }
    }

    /// Converts the pin into an input pin with pull-up resistor enabled.
    pub fn into_pull_up_input(self) -> Input<'i, 'p> {
        self.into_input(Pull::Up)
    }

    /// Converts the pin into an input pin with pull-down resistor enabled.
    pub fn into_pull_down_input(self) -> Input<'i, 'p> {
        self.into_input(Pull::Down)
    }

    /// Converts the pin into a floating input pin with no pull resistors.
    pub fn into_floating_input(self) -> Input<'i, 'p> {
        self.into_input(Pull::None)
    }
}

impl<'i, 'p> ErrorType for Output<'i, 'p> {
    type Error = Infallible;
}

impl<'i, 'p> OutputPin for Output<'i, 'p> {
    fn set_low(&mut self) -> Result<(), Self::Error> {
        match self.port {
            Port::A => unsafe {
                self.inner
                    .modify_swporta_dr(|r| r.with_pin_state(self.pin_num, PinState::Low.into()));
            },
            Port::B => unsafe {
                self.inner
                    .modify_swportb_dr(|r| r.with_pin_state(self.pin_num, PinState::Low.into()));
            },
        }
        Ok(())
    }

    fn set_high(&mut self) -> Result<(), Self::Error> {
        match self.port {
            Port::A => unsafe {
                self.inner
                    .modify_swporta_dr(|r| r.with_pin_state(self.pin_num, PinState::High.into()));
            },
            Port::B => unsafe {
                self.inner
                    .modify_swportb_dr(|r| r.with_pin_state(self.pin_num, PinState::High.into()));
            },
        }
        Ok(())
    }
}

impl<'i, 'p> StatefulOutputPin for Output<'i, 'p> {
    fn is_set_high(&mut self) -> Result<bool, Self::Error> {
        Ok(self.pin_state() == PinState::High)
    }

    fn is_set_low(&mut self) -> Result<bool, Self::Error> {
        Ok(self.pin_state() == PinState::Low)
    }
}
