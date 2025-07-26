mod input;
mod output;
pub mod pad;
mod register;

pub use embedded_hal::digital::{InputPin, OutputPin, PinState, StatefulOutputPin};
pub use input::Input;
pub use output::Output;
pub use register::*;
