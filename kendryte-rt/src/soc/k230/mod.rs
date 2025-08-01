//! Kendryte K230 AIoT chip.

mod pads;
mod peripheral;

use crate::arch::rvi::Stack;
use kendryte_hal::{clocks::Clocks, gpio, iomux, uart};
use pads::Pads;

/// Platform stack size.
pub const STACK_SIZE: usize = 32 * 1024;

/// Stack for current platform.
#[cfg(any(doc, feature = "k230"))]
#[unsafe(link_section = ".bss.uninit")]
pub static mut STACK: Stack<STACK_SIZE> = Stack([0; STACK_SIZE]);

soc! {
    /// Input/Output Multiplexer.
    pub struct IOMUX => 0x9110_5000, iomux::RegisterBlock;
    /// General Purpose Input/Output 0.
    pub struct GPIO0 => 0x9140_B000, gpio::RegisterBlock;
    /// General Purpose Input/Output 1.
    pub struct GPIO1 => 0x9140_C000, gpio::RegisterBlock;
    /// Universal Asynchronous Receiver Transmitter 0.
    pub struct UART0 => 0x9140_0000, uart::RegisterBlock;
    /// Universal Asynchronous Receiver Transmitter 1.
    pub struct UART1 => 0x9140_1000, uart::RegisterBlock;
    /// Universal Asynchronous Receiver Transmitter 2.
    pub struct UART2 => 0x9140_2000, uart::RegisterBlock;
    /// Universal Asynchronous Receiver Transmitter 3.
    pub struct UART3 => 0x9140_3000, uart::RegisterBlock;
    /// Universal Asynchronous Receiver Transmitter 4.
    pub struct UART4 => 0x9140_4000, uart::RegisterBlock;
}

/// Peripherals available on ROM start.
pub struct Peripherals {
    /// Input/Output Multiplexer.
    pub iomux: Pads,
    /// General Purpose Input/Output 0.
    pub gpio0: GPIO0,
    /// General Purpose Input/Output 1.
    pub gpio1: GPIO1,
    /// Universal Asynchronous Receiver Transmitter 0.
    pub uart0: UART0,
    /// Universal Asynchronous Receiver Transmitter 1.
    pub uart1: UART1,
    /// Universal Asynchronous Receiver Transmitter 2.
    pub uart2: UART2,
    /// Universal Asynchronous Receiver Transmitter 3.
    pub uart3: UART3,
    /// Universal Asynchronous Receiver Transmitter 4.
    pub uart4: UART4,
}

// Used by macros only.
#[allow(unused)]
#[doc(hidden)]
#[inline(always)]
pub fn __rom_init_params() -> (Peripherals, Clocks) {
    let peripherals = Peripherals {
        iomux: Pads::new(),
        gpio0: GPIO0(()),
        gpio1: GPIO1(()),
        uart0: UART0(()),
        uart1: UART1(()),
        uart2: UART2(()),
        uart3: UART3(()),
        uart4: UART4(()),
    };
    (peripherals, Clocks)
}
