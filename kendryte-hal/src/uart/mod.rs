mod blocking;
mod config;
mod error;
mod register;

pub use blocking::BlockingUart;
pub use config::{Config, ParityMode};
pub use register::*;

use crate::instance::ExclusiveInstance;

#[repr(transparent)]
pub struct Instance<const N: usize> {
    inner: RegisterBlock,
}

impl<const N: usize> ExclusiveInstance for Instance<N> {
    type Target = RegisterBlock;
}
