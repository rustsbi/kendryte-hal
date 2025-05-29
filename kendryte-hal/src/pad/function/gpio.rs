use crate::pad::Pad;
use crate::pad::pad_ops::PadOps;
use arbitrary_int::u3;

pub enum Port {
    A,
    B,
}

pub trait GpioFunction<const N: usize> {
    const PORT: Port;
    const PIN_NUM: usize;

    fn set_gpio_function(&mut self) -> &mut Self;
}

macro_rules! impl_pad_gpio {
    ($pad_num:expr, $function_select:expr,$gpio_num:expr,$port:expr,$pin_num:expr) => {
        impl GpioFunction<$gpio_num> for Pad<$pad_num> {
            const PORT: Port = $port;
            const PIN_NUM: usize = $pin_num;

            fn set_gpio_function(&mut self) -> &mut Self {
                self.set_bidirectional()
                    .set_function_select(u3::new($function_select));
                self
            }
        }
    };
}

// GPIO Group 0
impl_pad_gpio!(0, 1, 0, Port::A, 0);
impl_pad_gpio!(1, 1, 0, Port::A, 1);
impl_pad_gpio!(2, 1, 0, Port::A, 2);
impl_pad_gpio!(3, 1, 0, Port::A, 3);
impl_pad_gpio!(4, 1, 0, Port::A, 4);
impl_pad_gpio!(5, 1, 0, Port::A, 5);
impl_pad_gpio!(6, 1, 0, Port::A, 6);
impl_pad_gpio!(7, 1, 0, Port::A, 7);
impl_pad_gpio!(8, 1, 0, Port::A, 8);
impl_pad_gpio!(9, 1, 0, Port::A, 9);
impl_pad_gpio!(10, 1, 0, Port::A, 10);
impl_pad_gpio!(11, 1, 0, Port::A, 11);
impl_pad_gpio!(12, 1, 0, Port::A, 12);
impl_pad_gpio!(13, 1, 0, Port::A, 13);
impl_pad_gpio!(14, 2, 0, Port::A, 14);
impl_pad_gpio!(15, 2, 0, Port::A, 15);
impl_pad_gpio!(16, 1, 0, Port::A, 16);
impl_pad_gpio!(17, 1, 0, Port::A, 17);
impl_pad_gpio!(18, 0, 0, Port::A, 18);
impl_pad_gpio!(19, 0, 0, Port::A, 19);
impl_pad_gpio!(20, 0, 0, Port::A, 20);
impl_pad_gpio!(21, 0, 0, Port::A, 21);
impl_pad_gpio!(22, 0, 0, Port::A, 22);
impl_pad_gpio!(23, 0, 0, Port::A, 23);
impl_pad_gpio!(24, 0, 0, Port::A, 24);
impl_pad_gpio!(25, 0, 0, Port::A, 25);
impl_pad_gpio!(26, 0, 0, Port::A, 26);
impl_pad_gpio!(27, 0, 0, Port::A, 27);
impl_pad_gpio!(28, 0, 0, Port::A, 28);
impl_pad_gpio!(29, 0, 0, Port::A, 29);
impl_pad_gpio!(30, 0, 0, Port::A, 30);
impl_pad_gpio!(31, 0, 0, Port::A, 31);

// GPIO Group 1
impl_pad_gpio!(32, 1, 1, Port::A, 0);
impl_pad_gpio!(33, 1, 1, Port::A, 1);
impl_pad_gpio!(34, 1, 1, Port::A, 2);
impl_pad_gpio!(35, 1, 1, Port::A, 3);
impl_pad_gpio!(36, 1, 1, Port::A, 4);
impl_pad_gpio!(37, 1, 1, Port::A, 5);
impl_pad_gpio!(38, 1, 1, Port::A, 6);
impl_pad_gpio!(39, 1, 1, Port::A, 7);
impl_pad_gpio!(40, 1, 1, Port::A, 8);
impl_pad_gpio!(41, 1, 1, Port::A, 9);
impl_pad_gpio!(42, 1, 1, Port::A, 10);
impl_pad_gpio!(43, 1, 1, Port::A, 11);
impl_pad_gpio!(44, 1, 1, Port::A, 12);
impl_pad_gpio!(45, 1, 1, Port::A, 13);
impl_pad_gpio!(46, 2, 1, Port::A, 14);
impl_pad_gpio!(47, 2, 1, Port::A, 15);
impl_pad_gpio!(48, 1, 1, Port::A, 16);
impl_pad_gpio!(49, 1, 1, Port::A, 17);
impl_pad_gpio!(50, 0, 1, Port::A, 18);
impl_pad_gpio!(51, 0, 1, Port::A, 19);
impl_pad_gpio!(52, 0, 1, Port::A, 20);
impl_pad_gpio!(53, 0, 1, Port::A, 21);
impl_pad_gpio!(54, 0, 1, Port::A, 22);
impl_pad_gpio!(55, 0, 1, Port::A, 23);
impl_pad_gpio!(56, 0, 1, Port::A, 24);
impl_pad_gpio!(57, 0, 1, Port::A, 25);
impl_pad_gpio!(58, 0, 1, Port::A, 26);
impl_pad_gpio!(59, 0, 1, Port::A, 27);
impl_pad_gpio!(60, 0, 1, Port::A, 28);
impl_pad_gpio!(61, 0, 1, Port::A, 29);
impl_pad_gpio!(62, 0, 1, Port::A, 30);
impl_pad_gpio!(63, 0, 1, Port::A, 31);
