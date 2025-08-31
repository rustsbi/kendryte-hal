use crate::soc::k230::SPI0;
use kendryte_hal::instance::{Instance, Numbered};
use kendryte_hal::spi::RegisterBlock;

macro_rules! spi {
    (
        $(
            ($SPIx:ty, $n:literal)
        ),+ $(,)?
    ) => {
        $(
            impl Instance<'static> for $SPIx {
                type R = RegisterBlock;

                #[inline]
                fn inner(self) -> &'static Self::R {
                    unsafe { &*<$SPIx>::ptr() }
                }
            }

            impl Numbered<'static, $n> for $SPIx {}

            impl<'i> Instance<'i> for &'i mut $SPIx {
                type R = RegisterBlock;

                #[inline]
                fn inner(self) -> &'static Self::R {
                    unsafe { &*<$SPIx>::ptr() }
                }
            }

            impl<'i> Numbered<'i, $n> for &'i mut $SPIx {}
        )+
    };
}

spi! {
    (SPI0, 0),
}
