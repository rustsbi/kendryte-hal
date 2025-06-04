use arbitrary_int::{u2, u9};
use bitbybit::{bitenum, bitfield};
use volatile_register::{RO, RW};

/// UART Register Block.
///
/// This structure represents the memory-mapped registers of a UART peripheral.
/// Each field corresponds to a specific register or group of registers.
#[repr(C)]
pub struct RegisterBlock {
    /// Receive Buffer Register / Transmit Holding Register / Divisor Latch LSB.
    pub rbr_thr_dll: RW<RbrThrDll>,
    /// Interrupt Enable Register / Divisor Latch MSB.
    pub ier_dlh: RW<IerDlh>,
    /// Interrupt Identification Register / FIFO Control Register.
    pub iir_fcr: RW<IirFcr>,
    /// Line Control Register.
    pub lcr: RW<Lcr>,
    /// Modem Control Register.
    pub mcr: RW<Mcr>,
    /// Line Status Register.
    pub lsr: RO<Lsr>,
    /// Modem Status Register.
    pub msr: RO<Msr>,
    /// Scratchpad Register.
    pub scr: RW<Scr>,
    /// Low Power Divisor Latch Low Register.
    pub lpdll: RW<u32>,
    /// Low Power Divisor Latch High Register.
    pub lpdlh: RW<u32>,
    _reserved0: [u8; 0x08],
    /// Shadow Receive Buffer Register / Transmit Holding Register.
    pub srbr_sthr: [RW<u32>; 16],
    /// FIFO Access Register.
    pub far: RW<u32>,
    /// Transmit FIFO Read Register.
    pub tfr: RO<u32>,
    /// Receive FIFO Write Register.
    pub rfw: RW<u32>,
    /// UART Status Register.
    pub usr: RO<u32>,
    /// Transmit FIFO Level.
    pub tfl: RO<u32>,
    /// Receive FIFO Level.
    pub rfl: RO<u32>,
    /// Software Reset Register.
    pub srr: RW<u32>,
    /// Shadow Request to Send.
    pub srts: RW<u32>,
    /// Shadow Break Control Register.
    pub sbcr: RW<u32>,
    /// Shadow DMA Mode.
    pub sdmam: RW<u32>,
    /// Shadow FIFO Enable.
    pub sfe: RW<u32>,
    /// Shadow Receive Trigger.
    pub srt: RW<u32>,
    /// Shadow Transmit Empty Trigger.
    pub stet: RW<u32>,
    /// Halt Transmit.
    pub htx: RW<u32>,
    /// DMA Software Acknowledge.
    pub dmasa: RW<u32>,
    /// Transceiver Control Register.
    pub tcr: RW<u32>,
    /// Driver Output Enable Register.
    pub de_en: RW<u32>,
    /// Receiver Output Enable Register.
    pub re_en: RW<u32>,
    /// Driver Output Enable Timing Register.
    pub det: RW<u32>,
    /// TurnAround Timing Register.
    pub tat: RW<u32>,
    /// Divisor Latch Fraction Register.
    pub dlf: RW<u32>,
    /// Receive Address Register.
    pub rar: RW<u32>,
    /// Transmit Address Register.
    pub tar: RW<u32>,
    /// Line Extended Control Register.
    pub lcr_ext: RW<u32>,
    _reserved1: [u8; 0x24],
    /// Component Parameter Register.
    pub cpr: RO<u32>,
    /// UART Component Version.
    pub ucv: RO<u32>,
    /// Component Type Register.
    pub ctr: RO<u32>,
}

/// General UART register: can act as Receive Buffer Register (RBR), Transmitter Holding Register (THR), or Divisor Latch LSB (DLL).
#[bitfield(u32)]
#[derive(Debug, PartialEq, Eq)]
pub struct RbrThrDll {
    /// Receive Buffer Register (RBR, read access).
    #[bits(0..=7, r)]
    pub receiver_buffer: u8,

    /// Transmitter Holding Register (THR, write access).
    #[bits(0..=7, w)]
    pub transmitter_holding: u8,

    /// Divisor Latch LSB (DLL, DLAB=1, read/write).
    #[bits(0..=7, rw)]
    pub divisor_latch_lsb: u8,

    /// Receive buffer for 9-bit data mode (RBR, read access).
    #[bits(0..=8, r)]
    pub receiver_buffer_9bits: u9,

    /// Transmitter holding for 9-bit data mode (THR, write access).
    #[bits(0..=8, w)]
    pub transmitter_holding_9bits: u9,
}

/// ClearMode determines how the line status register is cleared.
#[bitenum(u1, exhaustive = true)]
#[derive(Debug, PartialEq, Eq)]
pub enum ClearMode {
    /// 0 = Clear on receiver buffer or line status register read.
    OnRbrOrLsrRead = 0,
    /// 1 = Clear on line status register read only.
    OnLsrReadOnly = 1,
}

/// Interrupt Enable Register (IER) and Divisor Latch High (DLH).
#[bitfield(u32)]
#[derive(Debug, PartialEq, Eq)]
pub struct IerDlh {
    /// Enable receive data available interrupt (ERBFI).
    #[bit(0, rw)]
    pub receive_data_available_interrupt_enable: bool,

    /// Enable transmit empty interrupt (ETBEI).
    #[bit(1, rw)]
    pub transmit_empty_interrupt_enable: bool,

    /// Enable receive line status interrupt (ELSI).
    #[bit(2, rw)]
    pub receive_line_status_interrupt_enable: bool,

    /// Enable modem status interrupt (EDSSI).
    #[bit(3, rw)]
    pub modem_status_interrupt_enable: bool,

    /// Line status register clear mode.
    #[bit(4, rw)]
    pub line_status_register_clear_mode: ClearMode,

    /// Enable programmable threshold interrupt.
    #[bit(7, rw)]
    pub programmable_threshold_interrupt_enable: bool,

    /// Divisor latch MSB.
    #[bits(8..=15, rw)]
    pub divisor_latch_hsb: u8,
}

/// Identifies different interrupt types.
#[bitenum(u4, exhaustive = false)]
#[derive(Debug, PartialEq, Eq)]
pub enum InterruptType {
    /// 0x0 = Modem status interrupt.
    ModemStatus = 0x0,
    /// 0x1 = No pending interrupt.
    NoPending = 0x1,
    /// 0x2 = Transmit holding register empty.
    TransmitHoldingEmpty = 0x2,
    /// 0x4 = Received data available.
    ReceivedDataAvailable = 0x4,
    /// 0x6 = Receiver line status change.
    ReceiverLineStatus = 0x6,
    /// 0x7 = Busy detected.
    BusyDetect = 0x7,
    /// 0xC = Character timeout.
    CharacterTimeout = 0xC,
}

/// DMA transfer mode.
#[bitenum(u1, exhaustive = true)]
#[derive(Debug, PartialEq, Eq)]
pub enum DmaTransferMode {
    /// 0 = Mode 0.
    Mode0 = 0,
    /// 1 = Mode 1.
    Mode1 = 1,
}

/// Receiver interrupt threshold.
#[bitenum(u2, exhaustive = true)]
#[derive(Debug, PartialEq, Eq)]
pub enum ReceiverInterruptThreshold {
    /// 0 = 1 character.
    OneChar = 0,
    /// 1 = 1/4 full.
    QuarterFull = 1,
    /// 2 = Half full.
    HalfFull = 2,
    /// 3 = Almost full.
    AlmostFull = 3,
}

/// Transmitter empty threshold.
#[bitenum(u2, exhaustive = true)]
#[derive(Debug, PartialEq, Eq)]
pub enum TransmitterEmptyThreshold {
    /// 0 = Empty.
    Empty = 0,
    /// 1 = 2 characters left.
    TwoCharsLeft = 1,
    /// 2 = 1/4 full.
    QuarterFull = 2,
    /// 3 = Half full.
    HalfFull = 3,
}

/// FIFO Control Register and Interrupt Identification Register.
/// Used to control FIFO, DMA, and to identify UART interrupts.
#[bitfield(u32)]
#[derive(Debug, PartialEq, Eq)]
pub struct IirFcr {
    /// Indicates the interrupt type (read only).
    #[bits(0..=3, r)]
    pub interrupt_type: Option<InterruptType>,

    // FCR (write only), bits 0..3 control FIFO and DMA.
    /// Enable FIFO (write only).
    #[bit(0, w)]
    pub fifo_enable: bool,

    /// Receiver FIFO reset (write only).
    #[bit(1, w)]
    pub receiver_fifo_reset: bool,

    /// Transmitter FIFO reset (write only).
    #[bit(2, w)]
    pub transmitter_fifo_reset: bool,

    /// Select DMA transfer mode (write only).
    #[bit(3, w)]
    pub dma_transfer_mode: DmaTransferMode,

    // FCR (write only), bits 4..5 are transmitter empty threshold.
    /// Transmitter empty interrupt threshold (write only).
    #[bits(4..=5, w)]
    pub transmitter_empty_threshold: TransmitterEmptyThreshold,

    /// IIR (read only), bits 6..7 are FIFO status.

    /// FIFO status flag (read only, 0=disabled, 3=enabled).
    #[bits(6..=7, r)]
    pub fifo_status: u2,

    // FCR (write only), bits 6..7 are receiver interrupt threshold.
    /// Receiver FIFO interrupt trigger threshold (write only).
    #[bits(6..=7, w)]
    pub receiver_interrupt_threshold: ReceiverInterruptThreshold,
}

/// Data word length configuration per UART character.
#[bitenum(u2, exhaustive = true)]
#[derive(Debug, PartialEq, Eq)]
pub enum WordLength {
    /// 5 data bits.
    _5 = 0,
    /// 6 data bits.
    _6 = 1,
    /// 7 data bits.
    _7 = 2,
    /// 8 data bits.
    _8 = 3,
}

/// Stop bits configuration.
#[bitenum(u1, exhaustive = true)]
#[derive(Debug, PartialEq, Eq)]
pub enum StopBits {
    /// 1 stop bit.
    _1 = 0,
    /// 1.5/2 stop bits (depends on word length).
    _2 = 1,
}

/// Parity type.
#[bitenum(u1, exhaustive = true)]
#[derive(Debug, PartialEq, Eq)]
pub enum ParityType {
    /// Odd parity.
    Odd = 0,
    /// Even parity.
    Even = 1,
}
/// Line Control Register.
/// Used to configure word length, stop bits, parity, and related controls.
#[bitfield(u32)]
#[derive(Debug, PartialEq, Eq)]
pub struct Lcr {
    /// Word length configuration.
    #[bits(0..=1, rw)]
    pub word_length: WordLength,

    /// Stop bits configuration.
    #[bit(2, rw)]
    pub stop_bits: StopBits,

    /// Parity enable control.
    #[bit(3, rw)]
    pub parity_enable: bool,

    /// Parity type (effective only when parity_enable is 1).
    #[bit(4, rw)]
    pub parity_type: ParityType,

    /// Stick parity enable.
    #[bit(5, rw)]
    pub stick_parity_enable: bool,

    /// Enable break signal transmission.
    #[bit(6, rw)]
    pub break_control_enable: bool,

    /// Enable divisor latch access.
    #[bit(7, rw)]
    pub divisor_latch_access_enable: bool,
}

/// Modem Control Register.
/// Used for modem handshaking and additional mode configuration.
#[bitfield(u32)]
#[derive(Debug, PartialEq, Eq)]
pub struct Mcr {
    /// Data Terminal Ready.
    #[bit(0, rw)]
    pub data_terminal_ready: bool,

    /// Request to Send.
    #[bit(1, rw)]
    pub request_to_send: bool,

    /// OUT1 control bit.
    #[bit(2, rw)]
    pub out1_signal: bool,

    /// OUT2 control bit.
    #[bit(3, rw)]
    pub out2_signal: bool,

    /// Loopback mode enable.
    #[bit(4, rw)]
    pub loopback_mode_enable: bool,

    /// Auto flow control enable.
    #[bit(5, rw)]
    pub auto_flow_control_enable: bool,

    /// SIR mode enable.
    #[bit(6, rw)]
    pub sir_mode_enable: bool,
}

/// Line Status Register.
/// Reflects the current status and error conditions of the UART.
#[bitfield(u32)]
#[derive(Debug, PartialEq, Eq)]
pub struct Lsr {
    /// Data ready in receive buffer.
    #[bit(0, r)]
    pub data_ready: bool,

    /// Overrun error in receive buffer.
    #[bit(1, r)]
    pub overrun_error: bool,

    /// Parity error.
    #[bit(2, r)]
    pub parity_error: bool,

    /// Framing error.
    #[bit(3, r)]
    pub framing_error: bool,

    /// Break interrupt detected.
    #[bit(4, r)]
    pub break_interrupt: bool,

    /// Transmitter holding register empty.
    #[bit(5, r)]
    pub transmitter_holding_empty: bool,

    /// Transmitter is completely empty.
    #[bit(6, r)]
    pub transmitter_empty: bool,

    /// Error in receiver FIFO.
    #[bit(7, r)]
    pub receiver_fifo_error: bool,

    /// Address received in 9-bit mode.
    #[bit(8, r)]
    pub address_received: bool,
}

/// Modem Status Register.
/// Provides modem status changes and signal indications.
#[bitfield(u32)]
#[derive(Debug, PartialEq, Eq)]
pub struct Msr {
    /// Delta Clear to Send.
    #[bit(0, r)]
    pub delta_clear_to_send: bool,

    /// Delta Data Set Ready.
    #[bit(1, r)]
    pub delta_data_set_ready: bool,

    /// Trailing edge of Ring Indicator.
    #[bit(2, r)]
    pub trailing_edge_ring_indicator: bool,

    /// Delta Data Carrier Detect.
    #[bit(3, r)]
    pub delta_data_carrier_detect: bool,

    /// Clear to Send status.
    #[bit(4, r)]
    pub clear_to_send: bool,

    /// Data Set Ready status.
    #[bit(5, r)]
    pub data_set_ready: bool,

    /// Ring Indicator status.
    #[bit(6, r)]
    pub ring_indicator: bool,

    /// Data Carrier Detect status.
    #[bit(7, r)]
    pub data_carrier_detect: bool,
}

/// Scratchpad Register.
/// User-accessible general-purpose register for temporary storage.
#[bitfield(u32)]
#[derive(Debug, PartialEq, Eq)]
pub struct Scr {
    /// Scratchpad register.
    #[bits(0..=7, rw)]
    pub scratchpad: u8,
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::mem::offset_of;
    #[test]
    fn struct_register_block_offset() {
        assert_eq!(offset_of!(RegisterBlock, rbr_thr_dll), 0x00);
        assert_eq!(offset_of!(RegisterBlock, ier_dlh), 0x04);
        assert_eq!(offset_of!(RegisterBlock, iir_fcr), 0x08);
        assert_eq!(offset_of!(RegisterBlock, lcr), 0x0C);
        assert_eq!(offset_of!(RegisterBlock, mcr), 0x10);
        assert_eq!(offset_of!(RegisterBlock, lsr), 0x14);
        assert_eq!(offset_of!(RegisterBlock, msr), 0x18);
        assert_eq!(offset_of!(RegisterBlock, scr), 0x1C);
        assert_eq!(offset_of!(RegisterBlock, lpdll), 0x20);
        assert_eq!(offset_of!(RegisterBlock, lpdlh), 0x24);
        assert_eq!(offset_of!(RegisterBlock, srbr_sthr), 0x30);
        assert_eq!(offset_of!(RegisterBlock, far), 0x70);
        assert_eq!(offset_of!(RegisterBlock, tfr), 0x74);
        assert_eq!(offset_of!(RegisterBlock, rfw), 0x78);
        assert_eq!(offset_of!(RegisterBlock, usr), 0x7C);
        assert_eq!(offset_of!(RegisterBlock, tfl), 0x80);
        assert_eq!(offset_of!(RegisterBlock, rfl), 0x84);
        assert_eq!(offset_of!(RegisterBlock, srr), 0x88);
        assert_eq!(offset_of!(RegisterBlock, srts), 0x8C);
        assert_eq!(offset_of!(RegisterBlock, sbcr), 0x90);
        assert_eq!(offset_of!(RegisterBlock, sdmam), 0x94);
        assert_eq!(offset_of!(RegisterBlock, sfe), 0x98);
        assert_eq!(offset_of!(RegisterBlock, srt), 0x9C);
        assert_eq!(offset_of!(RegisterBlock, stet), 0xA0);
        assert_eq!(offset_of!(RegisterBlock, htx), 0xA4);
        assert_eq!(offset_of!(RegisterBlock, dmasa), 0xA8);
        assert_eq!(offset_of!(RegisterBlock, tcr), 0xAC);
        assert_eq!(offset_of!(RegisterBlock, de_en), 0xB0);
        assert_eq!(offset_of!(RegisterBlock, re_en), 0xB4);
        assert_eq!(offset_of!(RegisterBlock, det), 0xB8);
        assert_eq!(offset_of!(RegisterBlock, tat), 0xBC);
        assert_eq!(offset_of!(RegisterBlock, dlf), 0xC0);
        assert_eq!(offset_of!(RegisterBlock, rar), 0xC4);
        assert_eq!(offset_of!(RegisterBlock, tar), 0xC8);
        assert_eq!(offset_of!(RegisterBlock, lcr_ext), 0xCC);
        assert_eq!(offset_of!(RegisterBlock, cpr), 0xF4);
        assert_eq!(offset_of!(RegisterBlock, ucv), 0xF8);
        assert_eq!(offset_of!(RegisterBlock, ctr), 0xFC);
    }
}
