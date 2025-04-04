//! Bare-metal ROM runtime for Cannan Kendryte chips.
#![feature(naked_functions)]
#![no_std]
pub mod arch;
pub mod soc;

pub use kendryte_rt_macros::entry;

cfg_if::cfg_if! {
    if #[cfg(feature = "k230")] {
        pub use soc::k230::{Peripherals, Clocks};
        #[doc(hidden)]
        pub use soc::k230::__rom_init_params;
    }
}
