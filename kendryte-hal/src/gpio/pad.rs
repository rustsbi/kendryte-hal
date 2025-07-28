use crate::iomux::FlexPad;

pub enum Port {
    A,
    B,
}

pub trait IntoGpio<'p, const N: usize> {
    const PORT: Port;
    const PIN_NUM: usize;
    fn into_gpio(self) -> FlexPad<'p>;
}
