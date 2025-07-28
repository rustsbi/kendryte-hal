pub mod ops;
pub mod pad;
mod register;

use crate::iomux::ops::PadOps;
use core::marker::PhantomData;
pub use register::*;

pub struct FlexPad<'p> {
    inner: &'static pad::RegisterBlock,
    _marker: PhantomData<&'p ()>,
}

impl<'p> PadOps for FlexPad<'p> {
    fn inner(&self) -> &'static pad::RegisterBlock {
        self.inner
    }
}

impl<'p> FlexPad<'p> {
    pub fn new(inner: &'static pad::RegisterBlock) -> Self {
        Self {
            inner,
            _marker: PhantomData,
        }
    }
}

pub trait IntoFlexPad<'p> {
    fn into_flex_pad(self) -> FlexPad<'p>;
}
