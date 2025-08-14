use crate::soc::k230::pads::Pad;
use crate::soc::k230::{GPIO0, GPIO1};
use arbitrary_int::u3;
use kendryte_hal::gpio::MmioRegisterBlock;
use kendryte_hal::gpio::pad::{IntoGpio, Port};
use kendryte_hal::instance::{Instance, Numbered};
use kendryte_hal::iomux::ops::PadOps;
use kendryte_hal::iomux::{FlexPad, IntoFlexPad};

macro_rules! gpio {
    (
        $(
            ($GPIOx:ty, $n:literal)
        ),+ $(,)?
    ) => {
        $(
            impl Instance<'static> for $GPIOx {
                type R = MmioRegisterBlock<'static>;

                #[inline]
                fn inner(self) -> Self::R {
                    unsafe { <$GPIOx>::mmio_register_block() }
                }
            }

            impl Numbered<'static, $n> for $GPIOx {}

            impl<'i> Instance<'i> for &'i $GPIOx {
                type R = MmioRegisterBlock<'static>;

                #[inline]
                fn inner(self) -> Self::R {
                    unsafe { <$GPIOx>::mmio_register_block() }
                }
            }

            impl<'i> Numbered<'i, $n> for &'i $GPIOx {}

            impl<'i> Instance<'i> for &'i mut $GPIOx {
                type R = MmioRegisterBlock<'static>;

                #[inline]
                fn inner(self) -> Self::R {
                    unsafe { <$GPIOx>::mmio_register_block() }
                }
            }

            impl<'i> Numbered<'i, $n> for &'i mut $GPIOx {}
        )+
    };
}

gpio! {
    (GPIO0, 0),
    (GPIO1, 1),
}

macro_rules! pad_gpio {
    (
        $(
           ($pad_num:expr, $function_select:expr, $gpio_num:expr, $port:expr, $pin_num:expr)
        ),+ $(,)?
    ) => {
        $(
            impl IntoGpio<'static, $gpio_num> for Pad<$pad_num> {
                const PORT: Port = $port;
                const PIN_NUM: usize = $pin_num;

                #[inline]
                fn into_gpio(mut self) -> FlexPad<'static> {
                    let mut flex_pad = self.into_flex_pad();
                    flex_pad.set_bidirectional()
                        .set_function_select(u3::new($function_select));
                    flex_pad
                }
            }

            impl<'p> IntoGpio<'p, $gpio_num> for &'p Pad<$pad_num> {
                const PORT: Port = $port;
                const PIN_NUM: usize = $pin_num;

                #[inline]
                fn into_gpio(mut self) -> FlexPad<'p> {
                    let mut flex_pad = self.into_flex_pad();
                    flex_pad.set_bidirectional()
                        .set_function_select(u3::new($function_select));
                    flex_pad
                }
            }

            impl<'p> IntoGpio<'p, $gpio_num> for &'p mut Pad<$pad_num> {
                const PORT: Port = $port;
                const PIN_NUM: usize = $pin_num;

                #[inline]
                fn into_gpio(mut self) -> FlexPad<'p> {
                    let mut flex_pad = self.into_flex_pad();
                    flex_pad.set_bidirectional()
                        .set_function_select(u3::new($function_select));
                    flex_pad
                }
            }
        )+
    };
}

pad_gpio! {
    // GPIO Group 0
    (0, 1, 0, Port::A, 0),
    (1, 1, 0, Port::A, 1),
    (2, 1, 0, Port::A, 2),
    (3, 1, 0, Port::A, 3),
    (4, 1, 0, Port::A, 4),
    (5, 1, 0, Port::A, 5),
    (6, 1, 0, Port::A, 6),
    (7, 1, 0, Port::A, 7),
    (8, 1, 0, Port::A, 8),
    (9, 1, 0, Port::A, 9),
    (10, 1, 0, Port::A, 10),
    (11, 1, 0, Port::A, 11),
    (12, 1, 0, Port::A, 12),
    (13, 1, 0, Port::A, 13),
    (14, 2, 0, Port::A, 14),
    (15, 2, 0, Port::A, 15),
    (16, 1, 0, Port::A, 16),
    (17, 1, 0, Port::A, 17),
    (18, 0, 0, Port::A, 18),
    (19, 0, 0, Port::A, 19),
    (20, 0, 0, Port::A, 20),
    (21, 0, 0, Port::A, 21),
    (22, 0, 0, Port::A, 22),
    (23, 0, 0, Port::A, 23),
    (24, 0, 0, Port::A, 24),
    (25, 0, 0, Port::A, 25),
    (26, 0, 0, Port::A, 26),
    (27, 0, 0, Port::A, 27),
    (28, 0, 0, Port::A, 28),
    (29, 0, 0, Port::A, 29),
    (30, 0, 0, Port::A, 30),
    (31, 0, 0, Port::A, 31),

    // GPIO Group 1
    (32, 1, 1, Port::A, 0),
    (33, 1, 1, Port::A, 1),
    (34, 1, 1, Port::A, 2),
    (35, 1, 1, Port::A, 3),
    (36, 1, 1, Port::A, 4),
    (37, 1, 1, Port::A, 5),
    (38, 1, 1, Port::A, 6),
    (39, 1, 1, Port::A, 7),
    (40, 1, 1, Port::A, 8),
    (41, 1, 1, Port::A, 9),
    (42, 1, 1, Port::A, 10),
    (43, 1, 1, Port::A, 11),
    (44, 1, 1, Port::A, 12),
    (45, 1, 1, Port::A, 13),
    (46, 2, 1, Port::A, 14),
    (47, 2, 1, Port::A, 15),
    (48, 1, 1, Port::A, 16),
    (49, 1, 1, Port::A, 17),
    (50, 0, 1, Port::A, 18),
    (51, 0, 1, Port::A, 19),
    (52, 0, 1, Port::A, 20),
    (53, 0, 1, Port::A, 21),
    (54, 0, 1, Port::A, 22),
    (55, 0, 1, Port::A, 23),
    (56, 0, 1, Port::A, 24),
    (57, 0, 1, Port::A, 25),
    (58, 0, 1, Port::A, 26),
    (59, 0, 1, Port::A, 27),
    (60, 0, 1, Port::A, 28),
    (61, 0, 1, Port::A, 29),
    (62, 0, 1, Port::A, 30),
    (63, 0, 1, Port::A, 31),
}
