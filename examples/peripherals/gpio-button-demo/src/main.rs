#![no_std]
#![no_main]

use kendryte_hal::gpio::{Input, Output, OutputPin, PinState};
use kendryte_hal::iomux::Strength;
use kendryte_hal::pad::Pull;
use kendryte_rt::{Clocks, Peripherals, entry};
use panic_halt as _;

#[entry]
fn main(p: Peripherals, _c: Clocks) -> ! {
    let mut led = Output::new(p.gpio0, p.iomux.io19, PinState::High, Strength::_7);
    let mut button = Input::new(p.gpio0, p.iomux.io20, Pull::Down);
    loop {
        match button.pin_state() {
            PinState::High => led.set_high().ok(),
            PinState::Low => led.set_low().ok(),
        };
        riscv::asm::delay(100_000);
    }
}
