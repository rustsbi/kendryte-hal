#![no_std]
#![no_main]

use kendryte_hal::gpio::{DriveStrength, Output, PinState, StatefulOutputPin};
use kendryte_rt::{Clocks, Peripherals, entry};
use panic_halt as _;

#[entry]
fn main(p: Peripherals, _c: Clocks) -> ! {
    let mut led = Output::new(p.gpio0, p.iomux.io19, PinState::High, DriveStrength::Medium);
    loop {
        led.toggle().ok();
        riscv::asm::delay(10_000_000);
    }
}
