#![no_std]
#![no_main]

use embedded_io::Write;
use kendryte_hal::pwm::pad::IntoPwmOut; // for mapping pad to PWM output
use kendryte_hal::pwm::{Pwm, SetDutyCycle};
use kendryte_hal::uart::{BlockingUart, Config};
use kendryte_rt::{Clocks, Peripherals, entry};
use panic_halt as _;

#[entry]
fn main(p: Peripherals, c: Clocks) -> ! {
    // Init UART0 (io38: TX, io39: RX) for simple logging so we can see it's working
    let mut uart0 = BlockingUart::new(
        p.uart0,
        Some(p.iomux.io38),
        Some(p.iomux.io39),
        Config::new(),
        c,
    );
    writeln!(uart0, "pwm-demo: UART initialized.").ok();

    let mut pwm = Pwm::new(p.pwm0);

    // 直接使用开发板蜂鸣器所在的 IO43 (PWM1 输出, sel=2 在宏里已处理)
    let _pwm1_pad = p.iomux.io43.into_pwm_out();

    // Basic config
    pwm.reset_config();
    pwm.set_scale(0); // no prescale
    pwm.set_period(1023); // 10-bit resolution
    pwm.start();

    // Dynamic frequency sweep to make audible change; ensure we use channel 1 (comparator 1) for PWM1.
    const PWM_CLK_HZ: u32 = 100_000_000; // assumed source
    const FREQ_TABLE: &[u32] = &[400, 523, 660, 784, 1000, 1500, 800, 600];

    fn pick(scale_limit: u8, target: u32, fclk: u32) -> (u8, u16, u32) {
        let mut best: Option<(u8, u16, u32)> = None;
        for scale in 0..=scale_limit {
            // small search first
            let div = 1u32 << scale;
            let counts = fclk / (target * div); // (top+1)
            if counts <= 1 {
                continue;
            }
            let top = counts - 1;
            if top > u16::MAX as u32 || top < 128 {
                continue;
            }
            let actual = fclk / (div * (top + 1));
            let diff = if actual > target {
                actual - target
            } else {
                target - actual
            };
            match best {
                None => best = Some((scale, top as u16, diff)),
                Some((_, _, bd)) if diff < bd => best = Some((scale, top as u16, diff)),
                _ => {}
            }
        }
        best.unwrap_or((5, 3124, 0))
    }

    // Initialize first tone
    let mut idx = 0usize;
    let (mut scale, mut top, _d) = pick(10, FREQ_TABLE[idx], PWM_CLK_HZ);
    pwm.set_scale(scale);
    pwm.set_period(top);
    let (mut ch1, _c2, _c3) = pwm.split();
    let mut duty = (top as u32 + 1) / 2; // 50%
    let _ = ch1.set_duty_cycle(duty as u16);
    let mut current_freq = PWM_CLK_HZ / ((1u32 << scale) * (top as u32 + 1));
    writeln!(
        uart0,
        "Start sweep: freq={}Hz scale={} top={} duty={} (50%)",
        current_freq, scale, top, duty
    )
    .ok();

    let mut ms: u32 = 0;
    loop {
        riscv::asm::delay(100_000); // ~1ms coarse
        ms = ms.wrapping_add(1);
        // change every 800ms
        if ms % 800 == 0 {
            idx = (idx + 1) % FREQ_TABLE.len();
            let target = FREQ_TABLE[idx];
            let (s, t, _diff) = pick(12, target, PWM_CLK_HZ);
            pwm.set_scale(s);
            pwm.set_period(t);
            let (mut temp_ch1, _a2, _a3) = pwm.split();
            duty = (t as u32 + 1) / 2;
            let _ = temp_ch1.set_duty_cycle(duty as u16);
            ch1 = temp_ch1; // still keep handle; assignment now meaningful for subsequent writes
            scale = s;
            top = t;
            current_freq = PWM_CLK_HZ / ((1u32 << scale) * (top as u32 + 1));
            writeln!(
                uart0,
                "[sweep] t={}ms target={}Hz actual={}Hz scale={} top={} duty={}",
                ms, target, current_freq, scale, top, duty
            )
            .ok();
        } else if ms % 200 == 0 {
            // intermediate debug
            writeln!(
                uart0,
                "[debug] t={}ms freq={}Hz scale={} top={} duty={}",
                ms, current_freq, scale, top, duty
            )
            .ok();
        }
    }
}
