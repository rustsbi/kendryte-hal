pub(crate) use crate::iomux::FlexPad;

pub trait IntoUartSout<'p, const N: usize> {
    fn into_uart_sout(self) -> FlexPad<'p>;
}

pub trait IntoUartSin<'p, const N: usize> {
    fn into_uart_sin(self) -> FlexPad<'p>;
}

pub trait IntoUartRts<'p, const N: usize> {
    fn into_uart_rts(self) -> FlexPad<'p>;
}

pub trait IntoUartCts<'p, const N: usize> {
    fn into_uart_cts(self) -> FlexPad<'p>;
}

pub trait IntoUartDe<'p, const N: usize> {
    fn into_uart_de(self) -> FlexPad<'p>;
}

pub trait IntoUartRe<'p, const N: usize> {
    fn into_uart_re(self) -> FlexPad<'p>;
}
