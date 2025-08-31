#![no_std]
#![no_main]
use embedded_hal::spi::{SpiBus, MODE_0};
use embedded_io::Write as _;
use kendryte_hal::spi::{Config as SpiConfig, Spi};
use kendryte_hal::uart::{BlockingUart, Config as UartConfig};
use kendryte_rt::{entry, Clocks, Peripherals};
use panic_halt as _;

#[entry]
fn main(p: Peripherals, c: Clocks) -> ! {
    // UART for printing
    let mut uart = BlockingUart::new(
        p.uart3,
        Some(p.iomux.io50),
        Some(p.iomux.io51),
        UartConfig::new(),
        c,
    );

    // Use pads-based API (pad numbers are subject to board routing; adjust as needed)
    let mut spi = Spi::with_pads(
        p.spi0,
        (p.iomux.io40, p.iomux.io41, p.iomux.io39, p.iomux.io38), // SCLK, MOSI, MISO, CS
        SpiConfig {
            frequency: 10_000_000,
            mode: MODE_0,
            data_bits: 8,
            ss_index: 0,
        },
        c,
    );

    // JEDEC RDID (0x9F) expects 3-4 bytes back
    // Compose command + dummy bytes and do an in-place transfer to keep CS asserted
    let mut buf = [0x9F, 0, 0, 0];
    spi.transfer_in_place(&mut buf).ok();
    let id = &buf[1..4];
    writeln!(uart, "JEDEC ID: {:02X} {:02X} {:02X}", id[0], id[1], id[2]).ok();

    loop {
        riscv::asm::delay(50_000_000);
    }
}
