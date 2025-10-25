use crate::soc::k230::pads::Pad;
use crate::soc::k230::{UART0, UART1, UART2, UART3, UART4};
use arbitrary_int::u3;
use kendryte_hal::instance::{Instance, Numbered};
use kendryte_hal::iomux::ops::PadOps;
use kendryte_hal::iomux::{FlexPad, IntoFlexPad};
use kendryte_hal::uart::MmioRegisterBlock;
use kendryte_hal::uart::pad::{
    IntoUartCts, IntoUartDe, IntoUartRe, IntoUartRts, IntoUartSin, IntoUartSout,
};

macro_rules! uart {
    (
        $(
            ($UARTx:ty, $n:literal)
        ),+ $(,)?
    ) => {
        $(
            impl Instance<'static> for $UARTx {
                type R = MmioRegisterBlock<'static>;

                #[inline]
                fn inner(self) -> Self::R {
                    unsafe { <$UARTx>::mmio_register_block() }
                }
            }

            impl Numbered<'static, $n> for $UARTx {}

            impl<'i> Instance<'i> for &'i mut $UARTx {
                type R = MmioRegisterBlock<'static>;

                #[inline]
                fn inner(self) -> Self::R {
                    unsafe { <$UARTx>::mmio_register_block() }
                }
            }

            impl<'i> Numbered<'i, $n> for &'i mut $UARTx {}
        )+
    };
}

uart! {
    (UART0, 0),
    (UART1, 1),
    (UART2, 2),
    (UART3, 3),
    (UART4, 4),
}

macro_rules! pad_uart_sout {
 (
        $(
           ($pad_num:expr, $function_select:expr,$uart_num:expr)
        ),+ $(,)?
    )=> {
      $(
        impl IntoUartSout<'static,$uart_num> for Pad<$pad_num> {
            fn into_uart_sout(self) -> FlexPad<'static> {
                let mut flex_pad = self.into_flex_pad();
                flex_pad.set_output()
                    .set_function_select(u3::new($function_select));
                flex_pad
            }
        }
        impl<'p> IntoUartSout<'p,$uart_num> for & 'p mut Pad<$pad_num> {
            fn into_uart_sout(self) -> FlexPad<'p> {
                let mut flex_pad = self.into_flex_pad();
                flex_pad.set_output()
                    .set_function_select(u3::new($function_select));
                flex_pad
            }
        }
      )+
    };
}

pad_uart_sout! {
    (38, 1, 0),

    (40, 1, 1),
    (9, 2, 1),
    (3, 3, 1),

    (44, 1, 2),
    (11, 2, 2),
    (5, 3, 2),

    (50, 1, 3),
    (28, 2, 3),
    (32, 3, 3),

    (48, 1, 4),
    (36, 3, 4),
}

macro_rules! pad_uart_sin {
    (
        $(
            ($pad_num:expr, $function_select:expr, $uart_num:expr)
        ),+ $(,)?
    ) => {
        $(
            impl IntoUartSin<'static, $uart_num> for Pad<$pad_num> {
                fn into_uart_sin(self) -> FlexPad<'static> {
                    let mut flex_pad = self.into_flex_pad();
                    flex_pad.set_output()
                        .set_function_select(u3::new($function_select));
                    flex_pad
                }
            }

            impl<'p> IntoUartSin<'p, $uart_num> for &'p mut Pad<$pad_num> {
                fn into_uart_sin(self) -> FlexPad<'p> {
                    let mut flex_pad = self.into_flex_pad();
                    flex_pad.set_output()
                        .set_function_select(u3::new($function_select));
                    flex_pad
                }
            }
        )+
    };
}

pad_uart_sin! {
    (39, 1, 0),

    (41, 1, 1),
    (10, 2, 1),
    (4, 3, 1),

    (45, 1, 2),
    (12, 2, 2),
    (6, 3, 2),

    (51, 1, 3),
    (29, 2, 3),
    (33, 3, 3),

    (49, 1, 4),
    (37, 3, 4)
}

macro_rules! pad_uart_rts {
    (
        $(
            ($pad_num:expr, $function_select:expr, $uart_num:expr)
        ),+ $(,)?
    ) => {
        $(
            impl IntoUartRts<'static, $uart_num> for Pad<$pad_num> {
                fn into_uart_rts(self) -> FlexPad<'static> {
                    let mut flex_pad = self.into_flex_pad();
                    flex_pad.set_output()
                        .set_function_select(u3::new($function_select));
                    flex_pad
                }
            }

            impl<'p> IntoUartRts<'p, $uart_num> for &'p mut Pad<$pad_num> {
                fn into_uart_rts(self) -> FlexPad<'p> {
                    let mut flex_pad = self.into_flex_pad();
                    flex_pad.set_output()
                        .set_function_select(u3::new($function_select));
                    flex_pad
                }
            }
        )+
    };
}

pad_uart_rts! {
    (42, 1, 1),

    (46, 1, 2),

    (52, 1, 3),
    (30, 2, 3),
    (34, 3, 3)
}

macro_rules! pad_uart_cts {
    (
        $(
            ($pad_num:expr, $function_select:expr, $uart_num:expr)
        ),+ $(,)?
    ) => {
        $(
            impl IntoUartCts<'static, $uart_num> for Pad<$pad_num> {
                fn into_uart_cts(self) -> FlexPad<'static> {
                    let mut flex_pad = self.into_flex_pad();
                    flex_pad.set_output()
                        .set_function_select(u3::new($function_select));
                    flex_pad
                }
            }

            impl<'p> IntoUartCts<'p, $uart_num> for &'p mut Pad<$pad_num> {
                fn into_uart_cts(self) -> FlexPad<'p> {
                    let mut flex_pad = self.into_flex_pad();
                    flex_pad.set_output()
                        .set_function_select(u3::new($function_select));
                    flex_pad
                }
            }
        )+
    };
}

pad_uart_cts! {
    (43, 1, 1),

    (47, 1, 2),

    (53, 1, 3),
    (31, 2, 3),
    (35, 1, 3)
}

macro_rules! pad_uart_de {
    (
        $(
            ($pad_num:expr, $function_select:expr, $uart_num:expr)
        ),+ $(,)?
    ) => {
        $(
            impl IntoUartDe<'static, $uart_num> for Pad<$pad_num> {
                fn into_uart_de(self) -> FlexPad<'static> {
                    let mut flex_pad = self.into_flex_pad();
                    flex_pad.set_output()
                        .set_function_select(u3::new($function_select));
                    flex_pad
                }
            }

            impl<'p> IntoUartDe<'p, $uart_num> for &'p mut Pad<$pad_num> {
                fn into_uart_de(self) -> FlexPad<'p> {
                    let mut flex_pad = self.into_flex_pad();
                    flex_pad.set_output()
                        .set_function_select(u3::new($function_select));
                    flex_pad
                }
            }
        )+
    };
}

pad_uart_de! {
    (62, 2, 3)
}

macro_rules! pad_uart_re {
    (
        $(
            ($pad_num:expr, $function_select:expr, $uart_num:expr)
        ),+ $(,)?
    ) => {
        $(
            impl IntoUartRe<'static, $uart_num> for Pad<$pad_num> {
                fn into_uart_re(self) -> FlexPad<'static> {
                    let mut flex_pad = self.into_flex_pad();
                    flex_pad.set_output()
                        .set_function_select(u3::new($function_select));
                    flex_pad
                }
            }

            impl<'p> IntoUartRe<'p, $uart_num> for &'p mut Pad<$pad_num> {
                fn into_uart_re(self) -> FlexPad<'p> {
                    let mut flex_pad = self.into_flex_pad();
                    flex_pad.set_output()
                        .set_function_select(u3::new($function_select));
                    flex_pad
                }
            }
        )+
    };
}

pad_uart_re! {
    (63, 2, 3)
}
