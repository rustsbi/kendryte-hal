pub trait SharedInstance {
    type Target;
    unsafe fn transmute_at(addr: usize) -> &'static Self
    where
        Self: Sized,
        Self::Target: Sized,
    {
        let register_block: &'static Self::Target = unsafe { &*(addr as *mut Self::Target) };
        unsafe { core::mem::transmute(register_block) }
    }

    fn inner(&self) -> &'static Self::Target
    where
        Self: Sized,
        Self::Target: Sized,
    {
        unsafe { core::mem::transmute(self) }
    }
}

pub trait ExclusiveInstance {
    type Target;
    unsafe fn transmute_at(addr: usize) -> &'static mut Self
    where
        Self: Sized,
        Self::Target: Sized,
    {
        let register_block: &'static mut Self::Target =
            unsafe { &mut *(addr as *mut Self::Target) };
        unsafe { core::mem::transmute(register_block) }
    }

    fn inner(&self) -> &'static Self::Target
    where
        Self: Sized,
        Self::Target: Sized,
    {
        unsafe { core::mem::transmute(self) }
    }

    fn inner_mut(&mut self) -> &'static mut Self::Target
    where
        Self: Sized,
        Self::Target: Sized,
    {
        unsafe { core::mem::transmute(self) }
    }
}
