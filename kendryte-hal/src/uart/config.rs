use crate::uart::{MmioRegisterBlock, ParityType, RegisterBlock, StopBits, WordLength};
use embedded_time::rate::Baud;

/// Represents different parity checking modes for UART communication.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParityMode {
    /// No parity checking.
    None,
    /// Odd parity checking.
    Odd,
    /// Even parity checking.
    Even,
    /// Force parity bit high.
    High,
    /// Force parity bit low.
    Low,
}

/// Configuration struct for UART settings.
///
/// This struct contains all configurable parameters for the UART interface.
/// Including divisor, parity mode, stop bits and word length settings.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Config {
    /// The divisor value for baud rate generation.
    pub baud: Baud,
    /// The parity checking mode.
    pub parity_mode: ParityMode,
    /// Number of stop bits.
    pub stop_bits: StopBits,
    /// Length of data words.
    pub word_length: WordLength,
    pub fifo: bool,
}

impl Config {
    /// Creates a new Config with default settings.
    ///
    /// Default settings are:
    /// - 115200 baud.
    /// - No parity.
    /// - 1 stop bit.
    /// - 8 bits word length.
    pub fn new() -> Self {
        Self {
            baud: Baud::new(115200),
            parity_mode: ParityMode::None,
            stop_bits: StopBits::_1,
            word_length: WordLength::_8,
            fifo: false,
        }
    }

    /// Sets the baud value.
    pub fn set_baud(mut self, baud: Baud) -> Self {
        self.baud = baud;
        self
    }

    /// Sets the parity mode.
    pub fn set_parity_mode(mut self, parity_mode: ParityMode) -> Self {
        self.parity_mode = parity_mode;
        self
    }

    /// Sets the number of stop bits.
    pub fn set_stop_bits(mut self, stop_bits: StopBits) -> Self {
        self.stop_bits = stop_bits;
        self
    }

    /// Sets the word length.
    pub fn set_word_length(mut self, word_length: WordLength) -> Self {
        self.word_length = word_length;
        self
    }
    /// Sets the fifo.
    pub fn set_fifo(mut self, fifo: bool) -> Self {
        self.fifo = fifo;
        self
    }
}

/// Gets the current divisor value from UART registers.
pub(crate) fn divisor(uart: &mut MmioRegisterBlock) -> u16 {
    unsafe {
        uart.modify_lcr(|r| r.with_divisor_latch_access_enable(true));
    }
    let dll = uart.read_rbr_thr_dll().divisor_latch_lsb();
    let dlh = uart.read_ier_dlh().divisor_latch_hsb();
    unsafe {
        uart.modify_lcr(|r| r.with_divisor_latch_access_enable(false));
    }
    u16::from_le_bytes([dll, dlh])
}

/// Sets the divisor value in UART registers.
pub(crate) fn set_divisor(uart: &mut MmioRegisterBlock, divisor: u16) {
    unsafe {
        uart.modify_lcr(|r| r.with_divisor_latch_access_enable(true));
    }
    let [divisor_lsb, divisor_hsb] = divisor.to_le_bytes();
    unsafe {
        uart.modify_rbr_thr_dll(|r| r.with_divisor_latch_lsb(divisor_lsb));
        uart.modify_ier_dlh(|r| r.with_divisor_latch_hsb(divisor_hsb));
        uart.modify_lcr(|r| r.with_divisor_latch_access_enable(false));
    }
}

/// Gets the current parity mode from UART registers.
pub(crate) fn parity_mode(uart: &mut MmioRegisterBlock) -> ParityMode {
    let lcr = uart.read_lcr();
    let flags = (
        lcr.parity_enable(),
        lcr.parity_type(),
        lcr.stick_parity_enable(),
    );
    match flags {
        (false, _, _) => ParityMode::None,
        (true, ParityType::Even, false) => ParityMode::Even,
        (true, ParityType::Odd, false) => ParityMode::Odd,
        (true, ParityType::Odd, true) => ParityMode::High,
        (true, ParityType::Even, true) => ParityMode::Low,
    }
}

/// Sets the parity mode in UART registers.
pub(crate) fn set_parity_mode(uart: &mut MmioRegisterBlock, parity: ParityMode) {
    let lcr = uart.read_lcr();
    let lcr = match parity {
        ParityMode::None => lcr.with_parity_enable(false),
        ParityMode::Odd => lcr
            .with_parity_enable(true)
            .with_stick_parity_enable(false)
            .with_parity_type(ParityType::Odd),
        ParityMode::Even => lcr
            .with_parity_enable(true)
            .with_stick_parity_enable(false)
            .with_parity_type(ParityType::Even),
        ParityMode::High => lcr
            .with_parity_enable(true)
            .with_stick_parity_enable(true)
            .with_parity_type(ParityType::Odd),
        ParityMode::Low => lcr
            .with_parity_enable(true)
            .with_stick_parity_enable(true)
            .with_parity_type(ParityType::Even),
    };
    unsafe {
        uart.write_lcr(lcr);
    }
}

/// Gets the current stop bits setting from UART registers.
pub(crate) fn stop_bits(uart: &mut MmioRegisterBlock) -> StopBits {
    uart.read_lcr().stop_bits()
}

/// Sets the stop bits in UART registers.
pub(crate) fn set_stop_bits(uart: &mut MmioRegisterBlock, stop_bits: StopBits) {
    unsafe {
        uart.modify_lcr(|r| r.with_stop_bits(stop_bits));
    }
}

/// Gets the current word length from UART registers.
pub(crate) fn word_length(uart: &mut MmioRegisterBlock) -> WordLength {
    uart.read_lcr().word_length()
}

/// Sets the word length in UART registers.
pub(crate) fn set_word_length(uart: &mut MmioRegisterBlock, word_length: WordLength) {
    unsafe {
        uart.modify_lcr(|r| r.with_word_length(word_length));
    }
}

pub(crate) fn enable_fifo(uart: &mut MmioRegisterBlock) {
    unsafe {
        uart.modify_iir_fcr(|r| r.with_fifo_enable(true));
    }
}
pub(crate) fn disable_fifo(uart: &mut MmioRegisterBlock) {
    unsafe {
        uart.modify_iir_fcr(|r| r.with_fifo_enable(false));
    }
}
