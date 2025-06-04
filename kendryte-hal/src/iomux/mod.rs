mod register;
use crate::instance::ExclusiveInstance;
pub use register::*;

#[repr(transparent)]
pub struct Instance {
    inner: RegisterBlock,
}

impl ExclusiveInstance for Instance {
    type Target = RegisterBlock;
}
