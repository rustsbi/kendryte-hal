mod blocking;
mod config;
mod error;
pub mod pad;
mod register;

pub use blocking::BlockingUart;
pub use config::{Config, ParityMode};
pub use error::UartError;
pub use register::*;
