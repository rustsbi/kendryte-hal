pub trait Instance<'i> {
    type R;
    fn inner(self) -> &'static Self::R;
}

pub trait Numbered<'i, const N: usize>: Instance<'i> {}
