#![no_std]
#![no_main]

use core::arch::{asm, naked_asm};
use core::sync::atomic::{AtomicU32, Ordering};
use embedded_io::Write;
use kendryte_hal::uart::*;
use kendryte_rt::{Clocks, Peripherals, entry};
use panic_halt as _;
use riscv::asm::delay; // for slowing down print rate

// Hart1 reset vector & control register (simplified single-attempt bring-up).
const CPU1_RSTVEC: usize = 0x9110_2104; // cpu1_hart_rstvec
const CPU_CTRL: usize = 0x9110_100c; // control: done/reset bits

// Approximate core frequency hint for debug delays (adjust to actual clock if known).
// Used only for coarse 5s startup delay observation.
const APPROX_CYCLES_PER_SEC: u32 = 50_000_000; // adjust if output cadence is off
const STARTUP_DELAY_SECS: u32 = 5; // user requested ~5s observation window

// Provide a small separate stack for the 2nd hart.
#[unsafe(no_mangle)]
#[unsafe(link_section = ".bss.uninit")]
static mut HART1_STACK: [u8; 4 * 1024] = [0; 4 * 1024];

// Assembly trampoline for hart1. We set its reset vector to `hart1_entry`.
#[unsafe(naked)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn hart1_reset_trap() -> ! {
    // Set up stack then jump into Rust function.
    naked_asm!(
        r#"
        .option push
        .option norvc
        la      sp, HART1_STACK
        li      t0, 4096
        add     sp, sp, t0
        j       hart1_main
        .option pop
    "#
    );
}

// Shared mailbox state (now atomic instead of raw static mut)
#[unsafe(link_section = ".bss.uninit")]
static HART1_FLAG: AtomicU32 = AtomicU32::new(0);
#[unsafe(link_section = ".bss.uninit")]
static HART1_TICKS: AtomicU32 = AtomicU32::new(0);
// Magic values sequence used to prove hart1 is actively updating (cycles every step).
const HART1_MAGIC: [u32; 4] = [0xCAFE_BABE, 0xDEAD_BEEF, 0x1234_5678, 0x0BAD_F00D];

#[unsafe(no_mangle)]
unsafe extern "C" fn hart1_main() -> ! {
    // Initialize with first magic so launcher detection still works.
    HART1_FLAG.store(HART1_MAGIC[0], Ordering::Release);
    let mut tick: u32 = 0;
    // (Removed optional hart1 direct UART banner output.)
    loop {
        tick = tick.wrapping_add(1);
        if tick % 50_000 == 0 {
            // Publish tick count.
            HART1_TICKS.store(tick, Ordering::Relaxed);
            // Advance magic index based on how many periods elapsed.
            let step = ((tick / 50_000) & 3) as usize;
            HART1_FLAG.store(HART1_MAGIC[step], Ordering::Release);
            // (Removed optional hart1 direct UART heartbeat output.)
        }
        unsafe {
            asm!("nop");
        }
    }
}

#[inline(always)]
fn write_reg(addr: usize, val: u32) {
    unsafe { (addr as *mut u32).write_volatile(val) }
}

fn start_hart1(entry_addr: usize) {
    // Program reset vector (low 32 bits enough for this demo) then perform reset sequence.
    write_reg(CPU1_RSTVEC, entry_addr as u32);
    unsafe {
        asm!("fence.i");
        asm!("fence iorw, iorw");
    }
    write_reg(CPU_CTRL, 0x1000_1000); // clear done bit
    write_reg(CPU_CTRL, 0x0001_0001); // assert reset
    write_reg(CPU_CTRL, 0x0001_0000); // deassert / release
    unsafe {
        asm!("fence.i");
        asm!("fence iorw, iorw");
    }
}

#[entry]
fn main(p: Peripherals, c: Clocks) -> ! {
    // Init two UARTs for logging: uart0 and uart3
    let mut uart0 = BlockingUart::new(
        p.uart0,
        Some(p.iomux.io38),
        Some(p.iomux.io39),
        Config::new(),
        c,
    );
    writeln!(uart0, "=== multicore-demo (K230) ===").ok();
    writeln!(uart0, "hart0: starting bring-up sequence").ok();
    // Pre-launch diagnostics: read current mailbox state (may be uninitialized random value).
    let pre_flag = HART1_FLAG.load(Ordering::Acquire);
    let pre_ticks = HART1_TICKS.load(Ordering::Acquire);
    writeln!(
        uart0,
        "pre-start mailbox: hart1_flag=0x{:08x} hart1_ticks={}",
        pre_flag, pre_ticks
    )
    .ok();
    writeln!(
        uart0,
        "observing {}s before launching hart1 (progress 0%->100%)",
        STARTUP_DELAY_SECS
    )
    .ok();
    const BAR_WIDTH: usize = 20;
    for sec in 1..=STARTUP_DELAY_SECS {
        riscv::asm::delay(APPROX_CYCLES_PER_SEC);
        let filled = (sec as usize * BAR_WIDTH + (STARTUP_DELAY_SECS as usize - 1))
            / STARTUP_DELAY_SECS as usize;
        let mut bar = [b'.'; BAR_WIDTH];
        for i in 0..filled.min(BAR_WIDTH) {
            bar[i] = b'#';
        }
        let pct = (sec * 100) / STARTUP_DELAY_SECS;
        let bar_str = core::str::from_utf8(&bar).unwrap_or("????????????????????");
        writeln!(
            uart0,
            "[{:02}/{:02}s] [{}] {:3}%",
            sec, STARTUP_DELAY_SECS, bar_str, pct
        )
        .ok();
    }
    writeln!(uart0, "startup window complete -> launching hart1").ok();

    // Launch hart1 (single, simple sequence) and sample first flag shortly after.
    let entry = hart1_reset_trap as usize;
    writeln!(uart0, "launching hart1 rstvec=0x{:08x}", entry as u32).ok();
    start_hart1(entry);
    for _ in 0..300_000 {
        unsafe {
            asm!("nop");
        }
    }
    let first_flag = HART1_FLAG.load(Ordering::Acquire);
    writeln!(uart0, "hart1 initial flag=0x{:08x}", first_flag).ok();

    let mut counter: u32 = 0;
    let mut last_flag: u32 = 0;
    let mut last_ticks: u32 = 0;
    loop {
        counter = counter.wrapping_add(1);
        // Print far less frequently to avoid flooding the UART.
        if counter % 100_000 == 0 {
            let flag_now = HART1_FLAG.load(Ordering::Acquire);
            let ticks_now = HART1_TICKS.load(Ordering::Acquire);
            let flag_changed = if flag_now != last_flag { "*" } else { "" };
            let tick_changed = if ticks_now != last_ticks { "*" } else { "" };
            writeln!(
                uart0,
                "hart0 cnt={} hart1_flag=0x{:08x}{} hart1_ticks={}{}",
                counter, flag_now, flag_changed, ticks_now, tick_changed
            )
            .ok();
            last_flag = flag_now;
            last_ticks = ticks_now;
            // Insert a busy wait delay (~tunable) to further slow down output.
            delay(5_000_000);
        }
        unsafe {
            asm!("nop");
        }
    }
}
