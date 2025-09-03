use crate::soc::k230::pads::Pad;
use crate::soc::k230::{GPIO0, GPIO1};
use arbitrary_int::u3;
use kendryte_hal::gpio::pad::IntoGpio;
use kendryte_hal::gpio::{GpioPort, MmioRegisterBlock};
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
                const PORT: GpioPort = $port;
                const PIN_NUM: usize = $pin_num;

                #[inline]
                fn into_gpio(self) -> FlexPad<'static> {
                    let mut flex_pad = self.into_flex_pad();
                    flex_pad.set_bidirectional()
                        .set_function_select(u3::new($function_select));
                    flex_pad
                }
            }

            impl<'p> IntoGpio<'p, $gpio_num> for &'p Pad<$pad_num> {
                const PORT: GpioPort = $port;
                const PIN_NUM: usize = $pin_num;

                #[inline]
                fn into_gpio(self) -> FlexPad<'p> {
                    let mut flex_pad = self.into_flex_pad();
                    flex_pad.set_bidirectional()
                        .set_function_select(u3::new($function_select));
                    flex_pad
                }
            }

            impl<'p> IntoGpio<'p, $gpio_num> for &'p mut Pad<$pad_num> {
                const PORT: GpioPort = $port;
                const PIN_NUM: usize = $pin_num;

                #[inline]
                fn into_gpio(self) -> FlexPad<'p> {
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
    (0, 1, 0, GpioPort::A, 0),
    (1, 1, 0, GpioPort::A, 1),
    (2, 1, 0, GpioPort::A, 2),
    (3, 1, 0, GpioPort::A, 3),
    (4, 1, 0, GpioPort::A, 4),
    (5, 1, 0, GpioPort::A, 5),
    (6, 1, 0, GpioPort::A, 6),
    (7, 1, 0, GpioPort::A, 7),
    (8, 1, 0, GpioPort::A, 8),
    (9, 1, 0, GpioPort::A, 9),
    (10, 1, 0, GpioPort::A, 10),
    (11, 1, 0, GpioPort::A, 11),
    (12, 1, 0, GpioPort::A, 12),
    (13, 1, 0, GpioPort::A, 13),
    (14, 2, 0, GpioPort::A, 14),
    (15, 2, 0, GpioPort::A, 15),
    (16, 1, 0, GpioPort::A, 16),
    (17, 1, 0, GpioPort::A, 17),
    (18, 0, 0, GpioPort::A, 18),
    (19, 0, 0, GpioPort::A, 19),
    (20, 0, 0, GpioPort::A, 20),
    (21, 0, 0, GpioPort::A, 21),
    (22, 0, 0, GpioPort::A, 22),
    (23, 0, 0, GpioPort::A, 23),
    (24, 0, 0, GpioPort::A, 24),
    (25, 0, 0, GpioPort::A, 25),
    (26, 0, 0, GpioPort::A, 26),
    (27, 0, 0, GpioPort::A, 27),
    (28, 0, 0, GpioPort::A, 28),
    (29, 0, 0, GpioPort::A, 29),
    (30, 0, 0, GpioPort::A, 30),
    (31, 0, 0, GpioPort::A, 31),

    // GPIO Group 1
    (32, 1, 1, GpioPort::A, 0),
    (33, 1, 1, GpioPort::A, 1),
    (34, 1, 1, GpioPort::A, 2),
    (35, 1, 1, GpioPort::A, 3),
    (36, 1, 1, GpioPort::A, 4),
    (37, 1, 1, GpioPort::A, 5),
    (38, 1, 1, GpioPort::A, 6),
    (39, 1, 1, GpioPort::A, 7),
    (40, 1, 1, GpioPort::A, 8),
    (41, 1, 1, GpioPort::A, 9),
    (42, 1, 1, GpioPort::A, 10),
    (43, 1, 1, GpioPort::A, 11),
    (44, 1, 1, GpioPort::A, 12),
    (45, 1, 1, GpioPort::A, 13),
    (46, 2, 1, GpioPort::A, 14),
    (47, 2, 1, GpioPort::A, 15),
    (48, 1, 1, GpioPort::A, 16),
    (49, 1, 1, GpioPort::A, 17),
    (50, 0, 1, GpioPort::A, 18),
    (51, 0, 1, GpioPort::A, 19),
    (52, 0, 1, GpioPort::A, 20),
    (53, 0, 1, GpioPort::A, 21),
    (54, 0, 1, GpioPort::A, 22),
    (55, 0, 1, GpioPort::A, 23),
    (56, 0, 1, GpioPort::A, 24),
    (57, 0, 1, GpioPort::A, 25),
    (58, 0, 1, GpioPort::A, 26),
    (59, 0, 1, GpioPort::A, 27),
    (60, 0, 1, GpioPort::A, 28),
    (61, 0, 1, GpioPort::A, 29),
    (62, 0, 1, GpioPort::A, 30),
    (63, 0, 1, GpioPort::A, 31),
}
