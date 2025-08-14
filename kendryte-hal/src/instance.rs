pub trait Instance<'i> {
    type R;
    fn inner(self) -> Self::R;
}

pub trait Numbered<'i, const N: usize>: Instance<'i> {}
