pub trait SharedInstance {
    type Target;
    unsafe fn transmute_at(addr: usize) -> &'static Self
    where
        Self: Sized,
        Self::Target: Sized,
    {
        unsafe { &*(addr as *const Self::Target as *const Self) }
    }

    fn inner(&self) -> &'static Self::Target
    where
        Self: Sized,
        Self::Target: Sized,
    {
        unsafe { &*(self as *const Self as *const Self::Target) }
    }
}

pub trait ExclusiveInstance {
    type Target;
    unsafe fn transmute_at(addr: usize) -> &'static mut Self
    where
        Self: Sized,
        Self::Target: Sized,
    {
        unsafe { &mut *(addr as *mut Self::Target as *mut Self) }
    }

    fn inner(&self) -> &'static Self::Target
    where
        Self: Sized,
        Self::Target: Sized,
    {
        unsafe { &*(self as *const Self as *const Self::Target) }
    }

    fn inner_mut(&mut self) -> &'static mut Self::Target
    where
        Self: Sized,
        Self::Target: Sized,
    {
        unsafe { &mut *(self as *mut Self as *mut Self::Target) }
    }
}
