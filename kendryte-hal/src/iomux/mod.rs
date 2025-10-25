pub mod ops;
pub mod pad;
mod register;

use crate::iomux::ops::PadOps;
use core::marker::PhantomData;
pub use register::*;

pub struct FlexPad<'p> {
    inner: pad::MmioRegisterBlock<'static>,
    _marker: PhantomData<&'p ()>,
}

impl<'p> PadOps for FlexPad<'p> {
    fn inner(&self) -> &pad::MmioRegisterBlock<'static> {
        &self.inner
    }

    fn inner_mut(&mut self) -> &mut pad::MmioRegisterBlock<'static> {
        &mut self.inner
    }
}

impl<'p> FlexPad<'p> {
    pub fn new(inner: pad::MmioRegisterBlock<'static>) -> Self {
        Self {
            inner,
            _marker: PhantomData,
        }
    }
}

pub trait IntoFlexPad<'p> {
    fn into_flex_pad(self) -> FlexPad<'p>;
}
