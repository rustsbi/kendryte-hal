#![no_std]
#![no_main]

use core::arch::asm;
use kendryte_rt::{Clocks, Peripherals, entry};
use panic_halt as _;
use uart16550;
use uart16550::RbrThrDll;

pub struct UART0;
impl UART0 {
    pub const fn ptr() -> *mut uart16550::RegisterBlock {
        0x9140_0000 as *mut _
    }

    pub const fn instance() -> &'static mut uart16550::RegisterBlock {
        unsafe { &mut *(Self::ptr()) }
    }
}

#[derive(Debug)]
pub struct Console;

impl core::fmt::Write for Console {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for ch in s.as_bytes() {
            loop {
                if UART0::instance().lsr.read().is_transmitter_empty() {
                    let thr = RbrThrDll::default().set_transmitter_data(*ch);
                    unsafe {
                        UART0::instance().rbr_thr_dll.write(thr);
                    }
                    break;
                }
                core::hint::spin_loop();
            }
        }

        Ok(())
    }
}

#[macro_export]
macro_rules! println {
    ($($arg:tt)*) => {
        {
            use core::fmt::Write;
            writeln!(&mut $crate::Console, $($arg)*).unwrap();
        }
    };
    () => {
        {
            use core::fmt::Write;
            writeln!(&mut $crate::Console, "").unwrap();
        }
    };
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {
        {
            use core::fmt::Write;
            write!(&mut $crate::Console, $($arg)*).unwrap();
        }
    };
}

#[entry]
fn main(p: Peripherals, _c: Clocks) -> ! {
    loop {
        println!("Hello, World!");
        println!("Hello, Rust!");
        for _ in 0..10000000 {
            unsafe { asm!("nop") }
        }
    }
}
