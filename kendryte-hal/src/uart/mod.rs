mod register;
use crate::instance::ExclusiveInstance;
pub use register::*;

#[repr(transparent)]
pub struct Instance<const N: usize> {
    inner: RegisterBlock,
}

impl<const N: usize> ExclusiveInstance for Instance<N> {
    type Target = RegisterBlock;
}
