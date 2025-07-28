use crate::gpio::pad::{IntoGpio, Port};
use crate::gpio::{Direction, RegisterBlock};
use crate::instance::Numbered;
use crate::iomux::FlexPad;
use crate::iomux::ops::PadOps;
use crate::iomux::pad::Strength;
use arbitrary_int::u4;
use core::convert::Infallible;
use core::marker::PhantomData;
use embedded_hal::digital::{ErrorType, OutputPin, PinState, StatefulOutputPin};

/// Represents a GPIO output pin.
pub struct Output<'i, 'p> {
    inner: &'static RegisterBlock,
    pad: FlexPad<'p>,
    port: Port,
    pin_num: usize,
    _marker: PhantomData<&'i ()>,
}

impl<'i, 'p> Output<'i, 'p> {
    /// Creates a new Output instance for a specific pad and GPIO port.
    pub fn new<const N: usize, P>(
        instance: impl Numbered<'i, N, R = RegisterBlock>,
        pad: P,
        pin_state: PinState,
        drive_strength: Strength,
    ) -> Self
    where
        P: PadOps + IntoGpio<'p, N>,
    {
        let mut pad = pad.into_gpio();
        pad.set_drive_strength(drive_strength);
        let port = <P as IntoGpio<N>>::PORT;
        let pin_num = <P as IntoGpio<N>>::PIN_NUM;
        let inner = instance.inner();

        match port {
            Port::A => unsafe {
                inner
                    .swporta_ddr
                    .modify(|r| r.with_direction(pin_num, Direction::Output));
                inner
                    .swporta_dr
                    .modify(|r| r.with_pin_state(pin_num, pin_state.into()))
            },
            Port::B => unsafe {
                inner
                    .swportb_ddr
                    .modify(|r| r.with_direction(pin_num, Direction::Output));
                inner
                    .swportb_dr
                    .modify(|r| r.with_pin_state(pin_num, pin_state.into()))
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
            Port::A => self.inner.swporta_dr.read().pin_state(self.pin_num).into(),
            Port::B => self.inner.swportb_dr.read().pin_state(self.pin_num).into(),
        }
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
                    .swporta_dr
                    .modify(|r| r.with_pin_state(self.pin_num, PinState::Low.into()));
            },
            Port::B => unsafe {
                self.inner
                    .swportb_dr
                    .modify(|r| r.with_pin_state(self.pin_num, PinState::Low.into()));
            },
        }
        Ok(())
    }

    fn set_high(&mut self) -> Result<(), Self::Error> {
        match self.port {
            Port::A => unsafe {
                self.inner
                    .swporta_dr
                    .modify(|r| r.with_pin_state(self.pin_num, PinState::High.into()));
            },
            Port::B => unsafe {
                self.inner
                    .swportb_dr
                    .modify(|r| r.with_pin_state(self.pin_num, PinState::High.into()));
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
