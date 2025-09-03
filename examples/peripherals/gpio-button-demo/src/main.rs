#![no_std]
#![no_main]

use kendryte_hal::gpio::{DriveStrength, Input, Output, OutputPin, PinState};
use kendryte_hal::iomux::ops::Pull;
use kendryte_rt::{Clocks, Peripherals, entry};
use panic_halt as _;

#[entry]
fn main(p: Peripherals, _c: Clocks) -> ! {
    let mut led = Output::new(
        &p.gpio0,
        p.iomux.io19,
        PinState::High,
        DriveStrength::Medium,
    );
    let mut button = Input::new(&p.gpio0, p.iomux.io20, Pull::Down);
    loop {
        match button.read_state() {
            PinState::High => led.set_high().ok(),
            PinState::Low => led.set_low().ok(),
        };
        riscv::asm::delay(100_000);
    }
}
