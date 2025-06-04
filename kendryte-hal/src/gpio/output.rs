use crate::gpio::{Direction, Instance, RegisterBlock};
use crate::instance::SharedInstance;
use crate::iomux::Strength;
use crate::pad::function::gpio::{GpioFunction, Port};
use crate::pad::pad_ops::PadOps;
use crate::pad::{FlexPad, Pad};
use arbitrary_int::u4;
use core::convert::Infallible;
use embedded_hal::digital::{ErrorType, OutputPin, PinState, StatefulOutputPin};

/// Represents a GPIO output pin.
pub struct Output<'pad> {
    inner: &'static RegisterBlock,
    pad: &'pad mut FlexPad,
    port: Port,
    pin_num: usize,
}

impl<'pad> Output<'pad> {
    /// Creates a new Output instance for a specific pad and GPIO port.
    pub fn new<const PAD_NUM: usize, const GPIO_NUM: usize>(
        instance: &Instance<GPIO_NUM>,
        pad: &'pad mut Pad<PAD_NUM>,
        pin_state: PinState,
        drive_strength: Strength,
    ) -> Self
    where
        Pad<PAD_NUM>: GpioFunction<GPIO_NUM>,
    {
        pad.set_gpio_function();
        pad.set_drive_strength(drive_strength);
        let port = <Pad<PAD_NUM> as GpioFunction<GPIO_NUM>>::PORT;
        let pin_num = <Pad<PAD_NUM> as GpioFunction<GPIO_NUM>>::PIN_NUM;

        match port {
            Port::A => unsafe {
                instance
                    .inner()
                    .swporta_ddr
                    .modify(|r| r.with_direction(pin_num, Direction::Output));
                instance
                    .inner()
                    .swporta_dr
                    .modify(|r| r.with_pin_state(pin_num, pin_state.into()))
            },
            Port::B => unsafe {
                instance
                    .inner()
                    .swportb_ddr
                    .modify(|r| r.with_direction(pin_num, Direction::Output));
                instance
                    .inner()
                    .swportb_dr
                    .modify(|r| r.with_pin_state(pin_num, pin_state.into()))
            },
        }

        Self {
            inner: instance.inner(),
            pad: pad.as_flexible_mut(),
            port,
            pin_num,
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

impl<'pad> ErrorType for Output<'pad> {
    type Error = Infallible;
}

impl<'pad> OutputPin for Output<'pad> {
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

impl<'pad> StatefulOutputPin for Output<'pad> {
    fn is_set_high(&mut self) -> Result<bool, Self::Error> {
        Ok(self.pin_state() == PinState::High)
    }

    fn is_set_low(&mut self) -> Result<bool, Self::Error> {
        Ok(self.pin_state() == PinState::Low)
    }
}
