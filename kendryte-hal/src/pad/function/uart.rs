use crate::pad::Pad;
use crate::pad::pad_ops::PadOps;
use arbitrary_int::u3;
pub trait UartSoutFunction<const N: usize> {
    fn set_uart_sout_function(&mut self) -> &mut Self;
}

pub trait UartSinFunction<const N: usize> {
    fn set_uart_sin_function(&mut self) -> &mut Self;
}

pub trait UartRtsFunction<const N: usize> {
    fn set_uart_rts_function(&mut self) -> &mut Self;
}

pub trait UartCtsFunction<const N: usize> {
    fn set_uart_cts_function(&mut self) -> &mut Self;
}

pub trait UartDeFunction<const N: usize> {
    fn set_uart_de_function(&mut self) -> &mut Self;
}

pub trait UartReFunction<const N: usize> {
    fn set_uart_re_function(&mut self) -> &mut Self;
}

macro_rules! impl_pad_uart_sout {
    ($pad_num:expr, $function_select:expr,$uart_num:expr) => {
        impl UartSoutFunction<$uart_num> for Pad<$pad_num> {
            fn set_uart_sout_function(&mut self) -> &mut Self {
                self.set_output()
                    .set_function_select(u3::new($function_select));
                self
            }
        }
    };
}

impl_pad_uart_sout!(38, 1, 0);

impl_pad_uart_sout!(40, 1, 1);
impl_pad_uart_sout!(9, 2, 1);
impl_pad_uart_sout!(3, 3, 1);

impl_pad_uart_sout!(44, 1, 2);
impl_pad_uart_sout!(11, 2, 2);
impl_pad_uart_sout!(5, 3, 2);

impl_pad_uart_sout!(50, 1, 3);
impl_pad_uart_sout!(28, 2, 3);
impl_pad_uart_sout!(32, 3, 3);

impl_pad_uart_sout!(48, 1, 4);
impl_pad_uart_sout!(36, 3, 4);

macro_rules! impl_pad_uart_sin {
    ($pad_num:expr, $function_select:expr,$uart_num:expr) => {
        impl UartSinFunction<$uart_num> for Pad<$pad_num> {
            fn set_uart_sin_function(&mut self) -> &mut Self {
                self.set_input()
                    .set_function_select(u3::new($function_select));
                self
            }
        }
    };
}

impl_pad_uart_sin!(39, 1, 0);

impl_pad_uart_sin!(41, 1, 1);
impl_pad_uart_sin!(10, 2, 1);
impl_pad_uart_sin!(4, 3, 1);

impl_pad_uart_sin!(45, 1, 2);
impl_pad_uart_sin!(12, 2, 2);
impl_pad_uart_sin!(6, 3, 2);

impl_pad_uart_sin!(51, 1, 3);
impl_pad_uart_sin!(29, 2, 3);
impl_pad_uart_sin!(33, 3, 3);

impl_pad_uart_sin!(49, 1, 4);
impl_pad_uart_sin!(37, 3, 4);

macro_rules! impl_pad_uart_rts {
    ($pad_num:expr, $function_select:expr,$uart_num:expr) => {
        impl UartRtsFunction<$uart_num> for Pad<$pad_num> {
            fn set_uart_rts_function(&mut self) -> &mut Self {
                self.set_output()
                    .set_function_select(u3::new($function_select));
                self
            }
        }
    };
}

impl_pad_uart_rts!(42, 1, 1);

impl_pad_uart_rts!(46, 1, 2);

impl_pad_uart_rts!(52, 1, 3);
impl_pad_uart_rts!(30, 2, 3);
impl_pad_uart_rts!(34, 3, 3);

macro_rules! impl_pad_uart_cts {
    ($pad_num:expr, $function_select:expr,$uart_num:expr) => {
        impl UartCtsFunction<$uart_num> for Pad<$pad_num> {
            fn set_uart_cts_function(&mut self) -> &mut Self {
                self.set_input()
                    .set_function_select(u3::new($function_select));
                self
            }
        }
    };
}

impl_pad_uart_cts!(43, 1, 1);

impl_pad_uart_cts!(47, 1, 2);

impl_pad_uart_cts!(53, 1, 3);
impl_pad_uart_cts!(31, 2, 3);
impl_pad_uart_cts!(35, 1, 3);

macro_rules! impl_pad_uart_de {
    ($pad_num:expr, $function_select:expr,$uart_num:expr) => {
        impl UartDeFunction<$uart_num> for Pad<$pad_num> {
            fn set_uart_de_function(&mut self) -> &mut Self {
                self.set_output()
                    .set_function_select(u3::new($function_select));
                self
            }
        }
    };
}

impl_pad_uart_de!(62, 2, 3);

macro_rules! impl_pad_uart_re {
    ($pad_num:expr, $function_select:expr,$uart_num:expr) => {
        impl UartReFunction<$uart_num> for Pad<$pad_num> {
            fn set_uart_re_function(&mut self) -> &mut Self {
                self.set_output()
                    .set_function_select(u3::new($function_select));
                self
            }
        }
    };
}

impl_pad_uart_re!(63, 2, 3);
