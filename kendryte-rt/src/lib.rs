//! Bare-metal ROM runtime for Cannan Kendryte chips.
#![no_std]
#![allow(unused)]

#[macro_use]
mod macros;

pub mod arch;
pub mod interrupt;
pub mod soc;

pub use kendryte_rt_macros::{entry, exception, interrupt};

// Simple println-like macro for UART tx that implements `core::fmt::Write`.
// Usage: uprintln!(tx, "Hello {}", 123);
#[macro_export]
macro_rules! uprintln {
    ($tx:expr) => {
        let _ = core::fmt::Write::write_str(&mut *$tx, "\r\n");
    };
    ($tx:expr, $($arg:tt)*) => {
        let _ = core::fmt::Write::write_fmt(&mut *$tx, format_args!("{}\r\n", format_args!($($arg)*)));
    };
}

cfg_if::cfg_if! {
    if #[cfg(feature = "k230")] {
        pub use soc::k230::{Peripherals, STACK, STACK_SIZE};
        pub use kendryte_hal::clocks::Clocks;
        #[doc(hidden)]
        pub use soc::k230::__rom_init_params;
    } else {
        #[doc(hidden)]
        pub static STACK: [u8; 0] = [];
        #[doc(hidden)]
        pub const STACK_SIZE: usize = 0;
    }
}

unsafe extern "Rust" {
    fn main() -> !;
}
