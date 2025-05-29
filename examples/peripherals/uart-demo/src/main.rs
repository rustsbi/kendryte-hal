#![no_std]
#![no_main]
use embedded_io::Write;
use kendryte_hal::uart::*;
use kendryte_rt::{Clocks, Peripherals, entry};
use panic_halt as _;

#[entry]
fn main(p: Peripherals, c: Clocks) -> ! {
    let mut serial0 = BlockingUart::new(
        p.uart0,
        Some(p.iomux.io38),
        Some(p.iomux.io39),
        Config::new(),
        c,
    );
    let mut serial3 = BlockingUart::new(
        p.uart3,
        Some(p.iomux.io50),
        Some(p.iomux.io51),
        Config::new(),
        c,
    );
    loop {
        writeln!(serial0, "Welcome to use kendryte-hal🦀!").ok();
        writeln!(serial3, "Welcome to use kendryte-hal🦀!").ok();
        riscv::asm::delay(10_000_000);
    }
}
