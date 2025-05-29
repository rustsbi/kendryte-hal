use volatile_register::RW;

pub mod function;
pub mod pad_ops;
pub mod pads;

pub use pad_ops::*;
pub use pads::*;

/// Pad represents a strongly-typed IO pad with a compile-time index N.
/// It provides type safety for pin assignments.
#[repr(transparent)]
pub struct Pad<const N: usize> {
    inner: RW<crate::iomux::Pad>,
}

/// FlexPad is a type-erased IO pad that can be used for dynamic pin operations.
/// It allows flexible manipulation of any pad at runtime.
#[repr(transparent)]
pub struct FlexPad {
    inner: RW<crate::iomux::Pad>,
}

impl<const N: usize> Pad<N> {
    /// Convert this Pad into a mutable reference to FlexPad.
    /// This enables dynamic operations on the pad.
    pub fn as_flexible_mut(&mut self) -> &mut FlexPad {
        unsafe { core::mem::transmute(self) }
    }
}

impl<const N: usize> PadOps for Pad<N> {
    /// Get a reference to the underlying pad register.
    fn inner(&self) -> &RW<crate::iomux::Pad> {
        &self.inner
    }
}

impl PadOps for FlexPad {
    /// Get a reference to the underlying pad register.
    fn inner(&self) -> &RW<crate::iomux::Pad> {
        &self.inner
    }
}
