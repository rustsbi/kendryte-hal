//! Kendryte K230 AIoT chip.

mod pads;
mod peripheral;

use crate::arch::rvi::Stack;
use kendryte_hal::{clocks::Clocks, gpio, iomux, pwm, spi, uart};
use pads::Pads;

/// Platform stack size.
pub const STACK_SIZE: usize = 32 * 1024;

/// Stack for current platform.
#[cfg(any(doc, feature = "k230"))]
#[unsafe(link_section = ".bss.uninit")]
pub static mut STACK: Stack<STACK_SIZE> = Stack([0; STACK_SIZE]);

peripheral! {
    use kendryte_hal::gpio;
    use kendryte_hal::iomux;
    use kendryte_hal::uart;
    /// Input/Output Multiplexer.
    pub struct IOMUX => 0x9110_5000, iomux::RegisterBlock, iomux::MmioRegisterBlock<'static>;
    /// General Purpose Input/Output 0.
    pub struct GPIO0 => 0x9140_B000, gpio::RegisterBlock, gpio::MmioRegisterBlock<'static>;
    /// General Purpose Input/Output 1.
    pub struct GPIO1 => 0x9140_C000, gpio::RegisterBlock, gpio::MmioRegisterBlock<'static>;
    /// Universal Asynchronous Receiver Transmitter 0.
    pub struct UART0 => 0x9140_0000, uart::RegisterBlock, uart::MmioRegisterBlock<'static>;
    /// Universal Asynchronous Receiver Transmitter 1.
    pub struct UART1 => 0x9140_1000, uart::RegisterBlock, uart::MmioRegisterBlock<'static>;
    /// Universal Asynchronous Receiver Transmitter 2.
    pub struct UART2 => 0x9140_2000, uart::RegisterBlock, uart::MmioRegisterBlock<'static>;
    /// Universal Asynchronous Receiver Transmitter 3.
    pub struct UART3 => 0x9140_3000, uart::RegisterBlock, uart::MmioRegisterBlock<'static>;
    /// Universal Asynchronous Receiver Transmitter 4.
    pub struct UART4 => 0x9140_4000, uart::RegisterBlock;
    /// Serial Peripheral Interface 0.
    pub struct SPI0  => 0x9140_5000, spi::RegisterBlock;
    /// Pulse Width Modulation 0.
    pub struct PWM0  => 0x9140_A000, pwm::RegisterBlock;
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
    /// Serial Peripheral Interface 0.
    pub spi0: SPI0,
    /// Pulse Width Modulation 0.
    pub pwm0: PWM0,
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
        spi0: SPI0(()),
        pwm0: PWM0(()),
    };
    (peripherals, Clocks)
}
