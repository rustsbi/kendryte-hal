mod register;
use crate::instance::SharedInstance;
pub use register::*;

#[repr(transparent)]
pub struct Instance<const N: usize> {
    inner: RegisterBlock,
}

impl<const N: usize> SharedInstance for Instance<N> {
    type Target = RegisterBlock;
}
