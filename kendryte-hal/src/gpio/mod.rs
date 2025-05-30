mod input;
mod output;
mod register;

pub use embedded_hal::digital::{InputPin, OutputPin, PinState, StatefulOutputPin};
pub use input::Input;
pub use output::Output;
pub use register::*;

use crate::instance::SharedInstance;

#[repr(transparent)]
pub struct Instance<const N: usize> {
    inner: RegisterBlock,
}

impl<const N: usize> SharedInstance for Instance<N> {
    type Target = RegisterBlock;
}
