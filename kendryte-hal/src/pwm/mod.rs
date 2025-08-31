mod channel;
mod driver;
pub mod pad;
mod register;

pub use channel::{Ch1, Ch2, Ch3};
pub use driver::Pwm;
pub use embedded_hal::pwm::SetDutyCycle;
pub use register::*;
