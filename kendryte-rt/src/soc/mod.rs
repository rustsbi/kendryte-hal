pub mod k230;

unsafe extern "Rust" {
    fn main() -> !;
}

pub struct PeripheralWrapper<const ADDR: usize, T> {
    _marker: core::marker::PhantomData<T>,
}

impl<const ADDR: usize, T> AsRef<T> for PeripheralWrapper<ADDR, T> {
    fn as_ref(&self) -> &T {
        unsafe { &*(ADDR as *const T) }
    }
}

impl<const ADDR: usize, T> core::ops::Deref for PeripheralWrapper<ADDR, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*(ADDR as *const T) }
    }
}
