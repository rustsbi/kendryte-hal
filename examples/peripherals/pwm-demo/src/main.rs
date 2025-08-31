#![no_std]
#![no_main]

use kendryte_hal::pwm::{Pwm, SetDutyCycle};
use kendryte_rt::{Clocks, Peripherals, entry};
use panic_halt as _;

#[entry]
fn main(p: Peripherals, _c: Clocks) -> ! {
    let mut pwm = Pwm::new(p.pwm0);

    // Basic config
    pwm.reset_config();
    pwm.set_scale(0); // no prescale
    pwm.set_period(1023); // 10-bit resolution
    pwm.start();

    // Split channels
    let (mut ch1, _ch2, _ch3) = pwm.split();

    // Simple fade
    loop {
        for d in 0..=1023u16 {
            let _ = ch1.set_duty_cycle(d);
            riscv::asm::delay(20_000);
        }
        for d in (0..=1023u16).rev() {
            let _ = ch1.set_duty_cycle(d);
            riscv::asm::delay(20_000);
        }
    }
}
