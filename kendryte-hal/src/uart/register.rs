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

/// RbrThrDll represents a register that can be used as Receive Buffer Register (RBR), Transmit Holding Register (THR), or Divisor Latch LSB (DLL).
/// This register serves multiple purposes depending on the DLAB bit setting in the Line Control Register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct RbrThrDll(u32);

impl RbrThrDll {
    /// Bit mask for Receive Buffer Register.
    const RBR: u32 = 0xFF << 0;
    /// Bit mask for Receive Buffer Register.
    const RBR_9_BITS: u32 = 0x1FF << 0;
    /// Bit mask for Transmit Holding Register.
    const THR: u32 = 0xFF << 0;
    /// Bit mask for Receive Buffer Register.
    const THR_9_BITS: u32 = 0x1FF << 0;
    /// Bit mask for Divisor Latch Low Byte.
    const DLL: u32 = 0xFF << 0;

    /// Gets the value from the Receive Buffer Register.
    /// Returns the received data byte.
    #[inline]
    pub const fn receiver_data(self) -> u8 {
        (self.0 & Self::RBR) as u8
    }

    /// Gets the value from the Receive Buffer Register.
    /// Returns the received data byte.
    #[inline]
    pub const fn receiver_data_9bits(self) -> u16 {
        (self.0 & Self::RBR_9_BITS) as u16
    }

    /// Sets the value to the Transmit Holding Register.
    /// This function writes a byte to be transmitted.
    #[inline]
    pub const fn set_transmitter_data(self, val: u8) -> Self {
        Self((self.0 & !Self::THR) | ((val as u32) & Self::THR))
    }

    /// Sets the value to the Transmit Holding Register.
    /// This function writes a byte to be transmitted.
    #[inline]
    pub const fn set_transmitter_data_9bits(self, val: u16) -> Self {
        Self((self.0 & !Self::THR_9_BITS) | ((val as u32) & Self::THR_9_BITS))
    }

    /// Gets the value from the Divisor Latch Low Byte.
    /// Returns the lower byte of the baud rate divisor.
    #[inline]
    pub const fn divisor_latch_low_byte(self) -> u8 {
        (self.0 & Self::DLL) as u8
    }

    /// Sets the value to the Divisor Latch Low Byte.
    /// This function sets the lower byte of the baud rate divisor.
    #[inline]
    pub const fn set_divisor_latch_low_byte(self, val: u8) -> Self {
        Self((self.0 & !Self::DLL) | ((val as u32) & Self::DLL))
    }
}

/// Represents the mode for clearing LSR status bits.
/// This enum defines different methods for clearing the Line Status Register status bits.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LsrStatusBitsClearingMode {
    /// Clear LSR status bits when either RBR or LSR register is read.
    Mode0 = 0x00,
    /// Clear LSR status bits only when LSR register is read.
    Mode1 = 0x01,
}

/// IerDlh represents the Interrupt Enable Register (IER) and Divisor Latch High (DLH) register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct IerDlh(u32);

impl IerDlh {
    // IER Fields
    /// Enable Received Data Available Interrupt.
    const ERBFI: u32 = 0x01 << 0;
    /// Enable Transmit Holding Register Empty Interrupt.
    const ETBEI: u32 = 0x01 << 1;
    /// Enable Receiver Line Status Interrupt.
    const ELSI: u32 = 0x01 << 2;
    /// Enable Modem Status Interrupt.
    const EDSSI: u32 = 0x01 << 3;

    /// ELCOLR: Controls LSR status bits clearing method.
    /// 0 = Clear on RBR/LSR read.
    /// 1 = Clear on LSR read only.
    const ELCOLR: u32 = 0x01 << 4;
    /// Programmable THRE Interrupt Mode Enable.
    const PTIME: u32 = 0x01 << 7;

    // DLH Fields
    /// Divisor Latch High Byte.
    const DLH: u32 = 0xFF << 0;

    /// Enables the Received Data Available Interrupt.
    /// This function sets the ERBFI bit in the IER register to enable the interrupt.
    #[inline]
    pub const fn enable_received_data_available_interrupt(self) -> Self {
        Self(self.0 | Self::ERBFI)
    }

    /// Disables the Received Data Available Interrupt.
    /// This function clears the ERBFI bit in the IER register to disable the interrupt.
    #[inline]
    pub const fn disable_received_data_available_interrupt(self) -> Self {
        Self(self.0 & !Self::ERBFI)
    }

    /// Checks if the Received Data Available Interrupt is enabled.
    /// Returns true if the ERBFI bit is set in the IER register.
    #[inline]
    pub const fn is_received_data_available_interrupt_enabled(self) -> bool {
        (self.0 & Self::ERBFI) != 0
    }

    /// Enables the Transmit Holding Register Empty Interrupt.
    /// This function sets the ETBEI bit in the IER register to enable the interrupt.
    #[inline]
    pub const fn enable_transmitter_empty_interrupt(self) -> Self {
        Self(self.0 | Self::ETBEI)
    }

    /// Disables the Transmit Holding Register Empty Interrupt.
    /// This function clears the ETBEI bit in the IER register to disable the interrupt.
    #[inline]
    pub const fn disable_transmitter_empty_interrupt(self) -> Self {
        Self(self.0 & !Self::ETBEI)
    }

    /// Checks if the Transmit Holding Register Empty Interrupt is enabled.
    /// Returns true if the ETBEI bit is set in the IER register.
    #[inline]
    pub const fn is_transmitter_empty_interrupt_enabled(self) -> bool {
        (self.0 & Self::ETBEI) != 0
    }

    /// Enables the Receiver Line Status Interrupt.
    /// This function sets the ELSI bit in the IER register to enable the interrupt.
    #[inline]
    pub const fn enable_receiver_line_status_interrupt(self) -> Self {
        Self(self.0 | Self::ELSI)
    }

    /// Disables the Receiver Line Status Interrupt.
    /// This function clears the ELSI bit in the IER register to disable the interrupt.
    #[inline]
    pub const fn disable_receiver_line_status_interrupt(self) -> Self {
        Self(self.0 & !Self::ELSI)
    }

    /// Checks if the Receiver Line Status Interrupt is enabled.
    /// Returns true if the ELSI bit is set in the IER register.
    #[inline]
    pub const fn is_receiver_line_status_interrupt_enabled(self) -> bool {
        (self.0 & Self::ELSI) != 0
    }

    /// Enables the Modem Status Interrupt.
    /// This function sets the EDSSI bit in the IER register to enable the interrupt.
    #[inline]
    pub const fn enable_modem_status_interrupt(self) -> Self {
        Self(self.0 | Self::EDSSI)
    }

    /// Disables the Modem Status Interrupt.
    /// This function clears the EDSSI bit in the IER register to disable the interrupt.
    #[inline]
    pub const fn disable_modem_status_interrupt(self) -> Self {
        Self(self.0 & !Self::EDSSI)
    }

    /// Checks if the Modem Status Interrupt is enabled.
    /// Returns true if the EDSSI bit is set in the IER register.
    #[inline]
    pub const fn is_modem_status_interrupt_enabled(self) -> bool {
        (self.0 & Self::EDSSI) != 0
    }

    /// Gets the LSR status bits clearing mode.
    /// Returns the current mode for clearing LSR status bits.
    #[inline]
    pub const fn lsr_status_bits_clearing_mode(self) -> LsrStatusBitsClearingMode {
        if (self.0 & Self::ELCOLR) != 0 {
            LsrStatusBitsClearingMode::Mode1
        } else {
            LsrStatusBitsClearingMode::Mode0
        }
    }

    /// Sets the LSR status bits clearing mode.
    /// This function configures how the LSR status bits are cleared.
    #[inline]
    pub const fn set_lsr_status_bits_clearing_mode(self, val: LsrStatusBitsClearingMode) -> Self {
        match val {
            LsrStatusBitsClearingMode::Mode0 => Self(self.0 & !Self::ELCOLR),
            LsrStatusBitsClearingMode::Mode1 => Self(self.0 | Self::ELCOLR),
        }
    }

    /// Enables the Programmable THRE Interrupt Mode.
    /// This function sets the PTIME bit in the IER register to enable the mode.
    #[inline]
    pub const fn enable_programmable_thre_mode(self) -> Self {
        Self(self.0 | Self::PTIME)
    }

    /// Disables the Programmable THRE Interrupt Mode.
    /// This function clears the PTIME bit in the IER register to disable the mode.
    #[inline]
    pub const fn disable_programmable_thre_mode(self) -> Self {
        Self(self.0 & !Self::PTIME)
    }

    /// Checks if the Programmable THRE Interrupt Mode is enabled.
    /// Returns true if the PTIME bit is set in the IER register.
    #[inline]
    pub const fn is_programmable_thre_enabled(self) -> bool {
        (self.0 & Self::PTIME) != 0
    }

    /// Gets the value from the Divisor Latch High Byte.
    /// Returns the higher byte of the baud rate divisor.
    #[inline]
    pub const fn divisor_latch_high_byte(self) -> u8 {
        (self.0 & Self::DLH) as u8
    }

    /// Sets the value to the Divisor Latch High Byte.
    /// This function sets the higher byte of the baud rate divisor.
    #[inline]
    pub const fn set_divisor_latch_high_byte(self, val: u8) -> Self {
        Self((self.0 & !Self::DLH) | ((val as u32) & Self::DLH))
    }
}

/// Represents the different types of interrupts that can be identified.
/// Each variant corresponds to a specific interrupt condition.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InterruptId {
    /// Indicates a modem status interrupt has occurred.
    ModemStatus = 0x00,
    /// Indicates no interrupt is currently pending.
    NoInterruptPending = 0x01,
    /// Indicates the transmit holding register is empty.
    ThrEmpty = 0x02,
    /// Indicates new data is available to be read.
    ReceivedDataAvailable = 0x04,
    /// Indicates a change in the receiver line status.
    ReceiverLineStatus = 0x06,
    /// Indicates a busy condition has been detected.
    BusyDetect = 0x07,
    /// Indicates a character timeout has occurred.
    CharacterTimeout = 0x0C,
}

/// Represents the DMA operating modes.
/// Controls how DMA transfers are handled.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DmaMode {
    /// Mode 0 DMA operation.
    Mode0 = 0x00,
    /// Mode 1 DMA operation.
    Mode1 = 0x01,
}

/// Defines the FIFO level at which receiver interrupts are triggered.
/// Controls receiver sensitivity.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReceiverTriggerLevel {
    /// Trigger when 1 character is in FIFO.
    Char1 = 0x00,
    /// Trigger when FIFO is quarter full.
    FifoQuarterFull = 0x01,
    /// Trigger when FIFO is half full.
    FifoHalfFull = 0x02,
    /// Trigger when FIFO is almost full.
    FifoFullSub2 = 0x03,
}

/// Defines the FIFO level at which transmitter empty interrupts are triggered.
/// Controls transmitter behavior.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransmitterEmptyTriggerLevel {
    /// Trigger when FIFO becomes empty.
    FifoEmpty = 0x00,
    /// Trigger when 2 characters remain in FIFO.
    FifoChar2 = 0x01,
    /// Trigger when FIFO is quarter full.
    FifoQuarterFull = 0x02,
    /// Trigger when FIFO is half full.
    FifoHalfFull = 0x03,
}

/// IirFcr represents the Interrupt Identification Register (IIR) and FIFO Control Register (FCR).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct IirFcr(u32);

impl IirFcr {
    // IIR Fields
    /// Interrupt ID bits.
    const IID: u32 = 0x0F << 0;
    /// FIFO Status Enable.
    const FIFOSE: u32 = 0x03 << 6;

    // FCR Fields
    /// FIFO Enable.
    const FIFOE: u32 = 0x01 << 0;
    /// Receiver FIFO Reset.
    const RFIFOR: u32 = 0x01 << 1;
    /// Transmitter FIFO Reset.
    const XFIFOR: u32 = 0x01 << 2;
    /// DMA Mode Select.
    const DMAM: u32 = 0x01 << 3;
    /// Transmitter Empty Trigger.
    const TET: u32 = 0x03 << 4;
    /// Receiver Trigger.
    const RT: u32 = 0x03 << 6;

    /// Gets the interrupt identification from the IIR register.
    /// Returns the type of interrupt that is currently pending.
    #[inline]
    pub const fn interrupt_id(self) -> InterruptId {
        let iid = (self.0 & Self::IID) as u8;
        match iid {
            0x00 => InterruptId::ModemStatus,
            0x01 => InterruptId::NoInterruptPending,
            0x02 => InterruptId::ThrEmpty,
            0x04 => InterruptId::ReceivedDataAvailable,
            0x06 => InterruptId::ReceiverLineStatus,
            0x07 => InterruptId::BusyDetect,
            0x0C => InterruptId::CharacterTimeout,
            _ => InterruptId::NoInterruptPending,
        }
    }

    /// Checks if FIFOs are enabled.
    /// Returns true if both transmit and receive FIFOs are enabled.
    #[inline]
    pub const fn is_fifos_enabled(self) -> bool {
        const ENABLE: u32 = 0x03;

        (self.0 & Self::FIFOSE) == ENABLE << 6
    }

    /// Enables both transmit and receive FIFOs.
    /// This function sets the FIFOE bit in the FCR register.
    #[inline]
    pub const fn enable_fifo(self) -> Self {
        Self(self.0 | Self::FIFOE)
    }

    /// Disables both transmit and receive FIFOs.
    /// This function clears the FIFOE bit in the FCR register.
    #[inline]
    pub const fn disable_fifo(self) -> Self {
        Self(self.0 & !Self::FIFOE)
    }

    /// Resets the receiver FIFO.
    /// This function sets the RFIFOR bit in the FCR register.
    #[inline]
    pub const fn reset_receiver_fifo(self) -> Self {
        Self(self.0 | Self::RFIFOR)
    }

    /// Resets the transmitter FIFO.
    /// This function sets the XFIFOR bit in the FCR register.
    #[inline]
    pub const fn reset_transmitter_fifo(self) -> Self {
        Self(self.0 | Self::XFIFOR)
    }

    /// Sets the DMA mode.
    /// This function configures the DMAM bit in the FCR register.
    #[inline]
    pub const fn set_dma_mode(self, val: DmaMode) -> Self {
        Self((self.0 & !Self::DMAM) | ((val as u32) << 3))
    }

    /// Gets the current transmitter empty trigger level.
    /// Returns the FIFO level at which transmitter empty interrupts are triggered.
    #[inline]
    pub const fn transmitter_empty_trigger_level(self) -> TransmitterEmptyTriggerLevel {
        let tet = ((self.0 & Self::TET) >> 4) as u8;
        match tet {
            0 => TransmitterEmptyTriggerLevel::FifoEmpty,
            1 => TransmitterEmptyTriggerLevel::FifoChar2,
            2 => TransmitterEmptyTriggerLevel::FifoQuarterFull,
            _ => TransmitterEmptyTriggerLevel::FifoHalfFull,
        }
    }

    /// Sets the transmitter empty trigger level.
    /// This function configures when transmitter empty interrupts are triggered.
    #[inline]
    pub const fn set_transmitter_empty_trigger_level(
        self,
        val: TransmitterEmptyTriggerLevel,
    ) -> Self {
        Self((self.0 & !Self::TET) | ((val as u32) << 4))
    }

    /// Sets the receiver trigger level.
    /// This function configures when receiver interrupts are triggered.
    #[inline]
    pub const fn set_receiver_trigger_level(self, val: ReceiverTriggerLevel) -> Self {
        Self((self.0 & !Self::RT) | ((val as u32) << 6))
    }
}

/// Represents the word length configuration for UART communication.
/// This enum defines the number of data bits per character.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WordLength {
    /// 5 data bits per character.
    Bits5 = 0x00,
    /// 6 data bits per character.
    Bits6 = 0x01,
    /// 7 data bits per character.
    Bits7 = 0x02,
    /// 8 data bits per character.
    Bits8 = 0x03,
}

/// Represents the parity configuration for UART communication.
/// This enum defines the type of parity checking to be used.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Parity {
    /// Odd parity checking.
    Odd = 0x00,
    /// Even parity checking.
    Even = 0x01,
}

/// Represents the stop bits configuration for UART communication.
/// This enum defines the number of stop bits to be used.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StopBits {
    /// One stop bit.
    Bit1 = 0x00,
    /// 1.5 or 2 stop bits depending on the word length.
    Bits2OrBits1_5 = 0x01,
}

/// Lcr represents the Line Control Register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct Lcr(u32);

impl Lcr {
    /// Data Length Select.
    const DLS: u32 = 0x03 << 0;
    /// Number of Stop Bits.
    const STOP: u32 = 0x01 << 2;
    /// Parity Enable.
    const PEN: u32 = 0x01 << 3;
    /// Even Parity Select.
    const EPS: u32 = 0x01 << 4;
    /// Stick Parity.
    const SP: u32 = 0x01 << 5;
    /// Break Control.
    const BC: u32 = 0x01 << 6;
    /// Divisor Latch Access Bit.
    const DLAB: u32 = 0x01 << 7;

    /// Gets the current word length setting.
    /// Returns the word length configuration from the Line Control Register.
    #[inline]
    pub const fn word_length(self) -> WordLength {
        match (self.0 & Self::DLS) >> 0 {
            0x00 => WordLength::Bits5,
            0x01 => WordLength::Bits6,
            0x02 => WordLength::Bits7,
            _ => WordLength::Bits8,
        }
    }
    /// Sets the word length configuration.
    /// This function configures the number of word bits per character.
    /// The word length can be set to 5, 6, 7, or 8 bits.
    /// The setting takes effect immediately after configuration.
    #[inline]
    pub const fn set_word_length(self, val: WordLength) -> Self {
        Self((self.0 & !Self::DLS) | ((val as u32) & Self::DLS))
    }

    /// Gets the current stop bits configuration.
    /// Returns the stop bits setting from the Line Control Register.
    #[inline]
    pub const fn stop_bits(self) -> StopBits {
        if (self.0 & Self::STOP) != 0 {
            StopBits::Bits2OrBits1_5
        } else {
            StopBits::Bit1
        }
    }

    /// Sets the stop bits configuration.
    /// This function configures the number of stop bits used in communication.
    #[inline]
    pub const fn set_stop_bits(self, val: StopBits) -> Self {
        match val {
            StopBits::Bit1 => Self(self.0 & !Self::STOP),
            StopBits::Bits2OrBits1_5 => Self(self.0 | Self::STOP),
        }
    }

    /// Enables parity checking.
    /// This function sets the parity enable bit in the Line Control Register.
    #[inline]
    pub const fn enable_parity(self) -> Self {
        Self(self.0 | Self::PEN)
    }

    /// Disables parity checking.
    /// This function clears the parity enable bit in the Line Control Register.
    #[inline]
    pub const fn disable_parity(self) -> Self {
        Self(self.0 & !Self::PEN)
    }

    /// Checks if parity is enabled.
    /// Returns true if parity checking is enabled.
    #[inline]
    pub const fn is_parity_enabled(self) -> bool {
        (self.0 & Self::PEN) != 0
    }

    /// Gets the current parity configuration.
    /// Returns the parity setting from the Line Control Register.
    #[inline]
    pub const fn parity(self) -> Parity {
        if (self.0 & Self::EPS) != 0 {
            Parity::Even
        } else {
            Parity::Odd
        }
    }

    /// Sets the parity configuration.
    /// This function configures the type of parity checking used.
    #[inline]
    pub const fn set_parity(self, val: Parity) -> Self {
        match val {
            Parity::Odd => Self(self.0 & !Self::EPS),
            Parity::Even => Self(self.0 | Self::EPS),
        }
    }

    /// Enables stick parity.
    /// This function sets the stick parity bit in the Line Control Register.
    #[inline]
    pub const fn enable_stick_parity(self) -> Self {
        Self(self.0 | Self::SP)
    }

    /// Disables stick parity.
    /// This function clears the stick parity bit in the Line Control Register.
    #[inline]
    pub const fn disable_stick_parity(self) -> Self {
        Self(self.0 & !Self::SP)
    }

    /// Checks if stick parity is enabled.
    /// Returns true if stick parity is enabled.
    #[inline]
    pub const fn is_stick_parity_enabled(self) -> bool {
        (self.0 & Self::SP) != 0
    }

    /// Enables break control.
    /// This function sets the break control bit in the Line Control Register.
    #[inline]
    pub const fn enable_break_control(self) -> Self {
        Self(self.0 | Self::BC)
    }

    /// Disables break control.
    /// This function clears the break control bit in the Line Control Register.
    #[inline]
    pub const fn disable_break_control(self) -> Self {
        Self(self.0 & !Self::BC)
    }

    /// Checks if break control is enabled.
    /// Returns true if break control is enabled.
    #[inline]
    pub const fn is_break_control_enabled(self) -> bool {
        (self.0 & Self::BC) != 0
    }

    /// Enables divisor latch access.
    /// This function sets the divisor latch access bit in the Line Control Register.
    #[inline]
    pub const fn enable_divisor_latch_access(self) -> Self {
        Self(self.0 | Self::DLAB)
    }

    /// Disables divisor latch access.
    /// This function clears the divisor latch access bit in the Line Control Register.
    #[inline]
    pub const fn disable_divisor_latch_access(self) -> Self {
        Self(self.0 & !Self::DLAB)
    }

    /// Checks if divisor latch access is enabled.
    /// Returns true if divisor latch access is enabled.
    #[inline]
    pub const fn is_divisor_latch_access_enabled(self) -> bool {
        (self.0 & Self::DLAB) != 0
    }
}

/// Mcr represents the Modem Control Register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct Mcr(u32);

impl Mcr {
    /// Data Terminal Ready.
    const DTR: u32 = 0x01 << 0;
    /// Request to Send.
    const RTS: u32 = 0x01 << 1;
    /// Output 1.
    const OUT1: u32 = 0x01 << 2;
    /// Output 2.
    const OUT2: u32 = 0x01 << 3;
    /// Loopback Mode.
    const LB: u32 = 0x01 << 4;
    /// Auto Flow Control Enable.
    const AFCE: u32 = 0x01 << 5;
    /// SIR Mode Enable.
    const SIRE: u32 = 0x01 << 6;

    /// Gets the Data Terminal Ready (DTR) signal status.
    /// Returns true if DTR is asserted.
    #[inline]
    pub const fn data_terminal_ready(self) -> bool {
        (self.0 & Self::DTR) != 0
    }

    /// Sets the Data Terminal Ready (DTR) signal.
    /// This function controls the DTR output signal.
    #[inline]
    pub const fn set_data_terminal_ready(self, val: bool) -> Self {
        if val {
            Self(self.0 | Self::DTR)
        } else {
            Self(self.0 & !Self::DTR)
        }
    }

    /// Gets the Request to Send (RTS) signal status.
    /// Returns true if RTS is asserted.
    #[inline]
    pub const fn request_to_send(self) -> bool {
        (self.0 & Self::RTS) != 0
    }

    /// Sets the Request to Send (RTS) signal.
    /// This function controls the RTS output signal.
    #[inline]
    pub const fn set_request_to_send(self, val: bool) -> Self {
        if val {
            Self(self.0 | Self::RTS)
        } else {
            Self(self.0 & !Self::RTS)
        }
    }

    /// Gets the OUT1 signal status.
    /// Returns true if OUT1 is asserted.
    #[inline]
    pub const fn out1(self) -> bool {
        (self.0 & Self::OUT1) != 0
    }

    /// Sets the OUT1 signal.
    /// This function controls the OUT1 output signal.
    #[inline]
    pub const fn set_out1(self, val: bool) -> Self {
        if val {
            Self(self.0 | Self::OUT1)
        } else {
            Self(self.0 & !Self::OUT1)
        }
    }

    /// Gets the OUT2 signal status.
    /// Returns true if OUT2 is asserted.
    #[inline]
    pub const fn out2(self) -> bool {
        (self.0 & Self::OUT2) != 0
    }

    /// Sets the OUT2 signal.
    /// This function controls the OUT2 output signal.
    #[inline]
    pub const fn set_out2(self, val: bool) -> Self {
        if val {
            Self(self.0 | Self::OUT2)
        } else {
            Self(self.0 & !Self::OUT2)
        }
    }

    /// Enables loopback mode.
    /// This function sets the loopback bit in the Modem Control Register.
    #[inline]
    pub const fn enable_loop_back(self) -> Self {
        Self(self.0 | Self::LB)
    }

    /// Disables loopback mode.
    /// This function clears the loopback bit in the Modem Control Register.
    #[inline]
    pub const fn disable_loop_back(self) -> Self {
        Self(self.0 & !Self::LB)
    }

    /// Checks if loopback mode is enabled.
    /// Returns true if the loopback bit is set in the Modem Control Register.
    #[inline]
    pub const fn is_loop_back_enabled(self) -> bool {
        (self.0 & Self::LB) != 0
    }

    /// Enables auto flow control.
    /// This function sets the auto flow control bit in the Modem Control Register.
    #[inline]
    pub const fn enable_auto_flow_control(self) -> Self {
        Self(self.0 | Self::AFCE)
    }

    /// Disables auto flow control.
    /// This function clears the auto flow control bit in the Modem Control Register.
    #[inline]
    pub const fn disable_auto_flow_control(self) -> Self {
        Self(self.0 & !Self::AFCE)
    }

    /// Checks if auto flow control is enabled.
    /// Returns true if the auto flow control bit is set in the Modem Control Register.
    #[inline]
    pub const fn is_auto_flow_control_enabled(self) -> bool {
        (self.0 & Self::AFCE) != 0
    }

    /// Enables SIR mode.
    /// This function sets the SIR mode bit in the Modem Control Register.
    #[inline]
    pub const fn enable_sir_mode(self) -> Self {
        Self(self.0 | Self::SIRE)
    }

    /// Disables SIR mode.
    /// This function clears the SIR mode bit in the Modem Control Register.
    #[inline]
    pub const fn disable_sir_mode(self) -> Self {
        Self(self.0 & !Self::SIRE)
    }

    /// Checks if SIR mode is enabled.
    /// Returns true if the SIR mode bit is set in the Modem Control Register.
    #[inline]
    pub const fn is_sir_mode_enabled(self) -> bool {
        (self.0 & Self::SIRE) != 0
    }
}

/// Lsr represents the Line Status Register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct Lsr(u32);

impl Lsr {
    /// Data Ready.
    const DR: u32 = 0x01 << 1;
    /// Overflow Error.
    const OE: u32 = 0x01 << 1;
    /// Parity Error.
    const PE: u32 = 0x01 << 2;
    /// Framing Error.
    const FE: u32 = 0x01 << 3;
    /// Break Interrupt.
    const BI: u32 = 0x01 << 4;
    /// Transmit Holding Register Empty.
    const THRE: u32 = 0x01 << 5;
    /// Transmitter Empty.
    const TEMT: u32 = 0x01 << 6;
    /// Receiver FIFO Error.
    const RFE: u32 = 0x01 << 7;
    /// Address Received Bit.
    const ADDR_RCVD: u32 = 0x01 << 8;

    /// Checks if data is ready to be read from the receiver buffer.
    /// Returns true if there is data available to be read.
    #[inline]
    pub const fn is_data_ready(self) -> bool {
        (self.0 & Self::DR) != 0
    }

    /// Checks if an overrun error has occurred.
    /// Returns true if the receiver buffer was overrunned.
    #[inline]
    pub const fn is_overrun_error(self) -> bool {
        (self.0 & Self::OE) != 0
    }

    /// Checks if a parity error has occurred.
    /// Returns true if there was a parity mismatch in received data.
    #[inline]
    pub const fn is_parity_error(self) -> bool {
        (self.0 & Self::PE) != 0
    }

    /// Checks if a framing error has occurred.
    /// Returns true if the stop bit was invalid in received data.
    #[inline]
    pub const fn is_framing_error(self) -> bool {
        (self.0 & Self::FE) != 0
    }

    /// Checks if a break interrupt has occurred.
    /// Returns true if a break condition was detected.
    #[inline]
    pub const fn is_broken(self) -> bool {
        (self.0 & Self::BI) != 0
    }

    /// Checks if the transmit holding register is empty.
    /// Returns true if the transmit holding register can accept new data.
    #[inline]
    pub const fn is_transmitter_fifo_empty(self) -> bool {
        (self.0 & Self::THRE) != 0
    }

    /// Checks if the transmitter is completely empty.
    /// Returns true if both the transmit holding and shift registers are empty.
    #[inline]
    pub const fn is_transmitter_empty(self) -> bool {
        (self.0 & Self::TEMT) != 0
    }

    /// Checks if there is an error in the receiver FIFO.
    /// Returns true if at least one character in FIFO has an error.
    #[inline]
    pub const fn is_receiver_fifo_error(self) -> bool {
        (self.0 & Self::RFE) != 0
    }

    /// Checks if an address byte was received.
    /// Returns true if the received byte was an address in 9-bit mode.
    #[inline]
    pub const fn is_address_received(self) -> bool {
        (self.0 & Self::ADDR_RCVD) != 0
    }
}

/// Modem Status Register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct Msr(u32);

impl Msr {
    /// Delta Clear to Send.
    const DCTS: u32 = 0x01 << 0;
    /// Delta Data Set Ready.
    const DDSR: u32 = 0x01 << 1;
    /// Trailing Edge of Ring Indicator.
    const TERI: u32 = 0x01 << 2;
    /// Delta Data Carrier Detect.
    const DDCD: u32 = 0x01 << 3;
    /// Clear to Send.
    const CTS: u32 = 0x01 << 4;
    /// Data Set Ready.
    const DSR: u32 = 0x01 << 5;
    /// Ring Indicator.
    const RI: u32 = 0x01 << 6;
    /// Data Carrier Detect.
    const DCD: u32 = 0x01 << 7;

    /// Gets the Delta Clear to Send status.
    /// Returns true if the Clear to Send signal has changed since the last read.
    #[inline]
    pub const fn delta_clear_to_send(self) -> bool {
        (self.0 & Self::DCTS) != 0
    }

    /// Gets the Delta Data Set Ready status.
    /// Returns true if the Data Set Ready signal has changed since the last read.
    #[inline]
    pub const fn delta_data_set_ready(self) -> bool {
        (self.0 & Self::DDSR) != 0
    }

    /// Gets the Trailing Edge of Ring Indicator status.
    /// Returns true if a falling edge has been detected on the Ring Indicator input.
    #[inline]
    pub const fn trailing_edge_of_ring_indicator(self) -> bool {
        (self.0 & Self::TERI) != 0
    }

    /// Gets the Delta Data Carrier Detect status.
    /// Returns true if the Data Carrier Detect signal has changed since the last read.
    #[inline]
    pub const fn delta_data_carrier_detect(self) -> bool {
        (self.0 & Self::DDCD) != 0
    }

    /// Gets the Clear to Send signal status.
    /// Returns true if the Clear to Send input is active.
    #[inline]
    pub const fn clear_to_send(self) -> bool {
        (self.0 & Self::CTS) != 0
    }

    /// Gets the Data Set Ready signal status.
    /// Returns true if the Data Set Ready input is active.
    #[inline]
    pub const fn data_set_ready(self) -> bool {
        (self.0 & Self::DSR) != 0
    }

    /// Gets the Ring Indicator signal status.
    /// Returns true if the Ring Indicator input is active.
    #[inline]
    pub const fn ring_indicator(self) -> bool {
        (self.0 & Self::RI) != 0
    }

    /// Gets the Data Carrier Detect signal status.
    /// Returns true if the Data Carrier Detect input is active.
    #[inline]
    pub const fn data_carrier_detect(self) -> bool {
        (self.0 & Self::DCD) != 0
    }
}

/// Scratchpad Register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct Scr(u32);

impl Scr {
    /// This register is for programmers to use as a temporary storage space.
    const SCR: u32 = 0xFF << 0;

    /// Gets the value from the Scratchpad Register.
    /// Returns the current value stored in the scratchpad register.
    #[inline]
    pub const fn scratchpad(self) -> u8 {
        (self.0 & Self::SCR) as u8
    }

    /// Sets the value to the Scratchpad Register.
    /// This function writes a byte to the scratchpad register for temporary storage.
    #[inline]
    pub const fn set_scratchpad(self, val: u8) -> Self {
        Self((self.0 & !Self::SCR) | ((val as u32) & Self::SCR))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::mem::offset_of;
    #[test]
    fn struct_block_offset() {
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
    #[test]
    fn struct_rbr_thr_dll_functions() {
        // Test 8-bit reception
        let mut val = RbrThrDll(0xFF);
        assert_eq!(val.receiver_data(), 0xFF);

        val = RbrThrDll(0xAA);
        assert_eq!(val.receiver_data(), 0xAA);

        // Test 9-bit reception
        val = RbrThrDll(0x1FF);
        assert_eq!(val.receiver_data_9bits(), 0x1FF);

        val = RbrThrDll(0x155);
        assert_eq!(val.receiver_data_9bits(), 0x155);

        // Test 8-bit transmission
        val = RbrThrDll(0x0);
        val = val.set_transmitter_data(0xFF);
        assert_eq!(val.0, 0xFF);

        val = RbrThrDll(0x0);
        val = val.set_transmitter_data(0xAA);
        assert_eq!(val.0, 0xAA);

        // Test 9-bit transmission
        val = RbrThrDll(0x0);
        val = val.set_transmitter_data_9bits(0x1FF);
        assert_eq!(val.0, 0x1FF);

        val = RbrThrDll(0x0);
        val = val.set_transmitter_data_9bits(0x155);
        assert_eq!(val.0, 0x155);

        // Test divisor latch low byte related functions
        val = RbrThrDll(0x0);
        val = val.set_divisor_latch_low_byte(0xFF);
        assert_eq!(val.0, 0xFF);
        assert_eq!(val.divisor_latch_low_byte(), 0xFF);

        val = RbrThrDll(0x0);
        val = val.set_divisor_latch_low_byte(0xAA);
        assert_eq!(val.0, 0xAA);
        assert_eq!(val.divisor_latch_low_byte(), 0xAA);

        // Test if bit mask correctly preserves other bits
        val = RbrThrDll(0xFFFF0000);
        val = val.set_transmitter_data(0xFF);
        assert_eq!(val.0, 0xFFFF00FF);

        val = RbrThrDll(0xFFFF0000);
        val = val.set_transmitter_data_9bits(0x1FF);
        assert_eq!(val.0, 0xFFFF01FF);

        val = RbrThrDll(0xFFFF0000);
        val = val.set_divisor_latch_low_byte(0xFF);
        assert_eq!(val.0, 0xFFFF00FF);
    }
    #[test]
    fn struct_ier_dlh_functions() {
        let mut val = IerDlh(0x0);

        // Test enabling received data interrupt
        val = val.enable_received_data_available_interrupt();
        assert_eq!(val.0, 0x00000001);
        assert_eq!(val.is_received_data_available_interrupt_enabled(), true);

        // Test disabling received data interrupt
        val = val.disable_received_data_available_interrupt();
        assert_eq!(val.0, 0x00000000);
        assert_eq!(val.is_received_data_available_interrupt_enabled(), false);

        let mut val = IerDlh(0x0);

        // Test enabling transmitter empty interrupt
        val = val.enable_transmitter_empty_interrupt();
        assert_eq!(val.0, 0x00000002);
        assert_eq!(val.is_transmitter_empty_interrupt_enabled(), true);

        // Test disabling transmitter empty interrupt
        val = val.disable_transmitter_empty_interrupt();
        assert_eq!(val.0, 0x00000000);
        assert_eq!(val.is_transmitter_empty_interrupt_enabled(), false);

        let mut val = IerDlh(0x0);

        // Test enabling receiver line status interrupt
        val = val.enable_receiver_line_status_interrupt();
        assert_eq!(val.0, 0x00000004);
        assert_eq!(val.is_receiver_line_status_interrupt_enabled(), true);

        // Test disabling receiver line status interrupt
        val = val.disable_receiver_line_status_interrupt();
        assert_eq!(val.0, 0x00000000);
        assert_eq!(val.is_receiver_line_status_interrupt_enabled(), false);

        let mut val = IerDlh(0x0);

        // Test enabling modem status interrupt
        val = val.enable_modem_status_interrupt();
        assert_eq!(val.0, 0x00000008);
        assert_eq!(val.is_modem_status_interrupt_enabled(), true);

        // Test disabling modem status interrupt
        val = val.disable_modem_status_interrupt();
        assert_eq!(val.0, 0x00000000);
        assert_eq!(val.is_modem_status_interrupt_enabled(), false);

        let mut val = IerDlh(0x0);

        // Test enabling modem status interrupt
        val = val.enable_modem_status_interrupt();
        assert_eq!(val.0, 0x00000008);
        assert_eq!(val.is_modem_status_interrupt_enabled(), true);

        // Test disabling modem status interrupt
        val = val.disable_modem_status_interrupt();
        assert_eq!(val.0, 0x00000000);
        assert_eq!(val.is_modem_status_interrupt_enabled(), false);

        let mut val = IerDlh(0x0);

        // Test setting LSR status bits clearing mode to Mode1
        val = val.set_lsr_status_bits_clearing_mode(LsrStatusBitsClearingMode::Mode1);
        assert_eq!(val.0, 0x00000010);
        assert_eq!(
            val.lsr_status_bits_clearing_mode(),
            LsrStatusBitsClearingMode::Mode1
        );

        // Test setting LSR status bits clearing mode to Mode0
        val = val.set_lsr_status_bits_clearing_mode(LsrStatusBitsClearingMode::Mode0);
        assert_eq!(val.0, 0x00000000);
        assert_eq!(
            val.lsr_status_bits_clearing_mode(),
            LsrStatusBitsClearingMode::Mode0
        );

        let mut val = IerDlh(0x0);

        // Test enabling programmable THRE interrupt mode
        val = val.enable_programmable_thre_mode();
        assert_eq!(val.0, 0x00000080);
        assert_eq!(val.is_programmable_thre_enabled(), true);

        // Test disabling programmable THRE interrupt mode
        val = val.disable_programmable_thre_mode();
        assert_eq!(val.0, 0x00000000);
        assert_eq!(val.is_programmable_thre_enabled(), false);

        let mut val = IerDlh(0x0);

        // Test setting divisor latch high byte to 0x11
        val = val.set_divisor_latch_high_byte(0x11);
        assert_eq!(val.0, 0x00000011);
        assert_eq!(val.divisor_latch_high_byte(), 0x11);

        // Test setting divisor latch high byte to 0xFF
        val = val.set_divisor_latch_high_byte(0xFF);
        assert_eq!(val.0, 0x000000FF);
        assert_eq!(val.divisor_latch_high_byte(), 0xFF);
    }

    #[test]
    fn struct_iir_fcr_functions() {
        // Test all interrupt IDs
        let mut val = IirFcr(0x00);
        assert_eq!(val.interrupt_id(), InterruptId::ModemStatus);

        val = IirFcr(0x01);
        assert_eq!(val.interrupt_id(), InterruptId::NoInterruptPending);

        val = IirFcr(0x02);
        assert_eq!(val.interrupt_id(), InterruptId::ThrEmpty);

        val = IirFcr(0x04);
        assert_eq!(val.interrupt_id(), InterruptId::ReceivedDataAvailable);

        val = IirFcr(0x06);
        assert_eq!(val.interrupt_id(), InterruptId::ReceiverLineStatus);

        val = IirFcr(0x07);
        assert_eq!(val.interrupt_id(), InterruptId::BusyDetect);

        val = IirFcr(0x0C);
        assert_eq!(val.interrupt_id(), InterruptId::CharacterTimeout);

        // Test invalid value should return NoInterruptPending
        val = IirFcr(0xFF);
        assert_eq!(val.interrupt_id(), InterruptId::NoInterruptPending);

        // Test FIFO enable status check
        let mut val = IirFcr(0xC0); // Set FIFOSE to 0x03
        assert!(val.is_fifos_enabled());

        val = IirFcr(0x00);
        assert!(!val.is_fifos_enabled());

        // Test FIFO enable and disable
        val = IirFcr(0x0);
        val = val.enable_fifo();
        assert_eq!(val.0 & 0x01, 0x01);

        val = val.disable_fifo();
        assert_eq!(val.0 & 0x01, 0x00);

        // Test FIFO reset functionality
        val = IirFcr(0x0);
        val = val.reset_receiver_fifo();
        assert_eq!(val.0 & 0x02, 0x02);

        val = IirFcr(0x0);
        val = val.reset_transmitter_fifo();
        assert_eq!(val.0 & 0x04, 0x04);

        let mut val = IirFcr(0x0);

        // Test DMA mode setting
        val = val.set_dma_mode(DmaMode::Mode0);
        assert_eq!(val.0 & 0x08, 0x00);

        val = val.set_dma_mode(DmaMode::Mode1);
        assert_eq!(val.0 & 0x08, 0x08);

        // Test transmitter empty trigger level
        let mut val = IirFcr(0x00);
        assert_eq!(
            val.transmitter_empty_trigger_level(),
            TransmitterEmptyTriggerLevel::FifoEmpty
        );

        val = IirFcr(0x10);
        assert_eq!(
            val.transmitter_empty_trigger_level(),
            TransmitterEmptyTriggerLevel::FifoChar2
        );

        val = IirFcr(0x20);
        assert_eq!(
            val.transmitter_empty_trigger_level(),
            TransmitterEmptyTriggerLevel::FifoQuarterFull
        );

        val = IirFcr(0x30);
        assert_eq!(
            val.transmitter_empty_trigger_level(),
            TransmitterEmptyTriggerLevel::FifoHalfFull
        );

        // Test setting transmitter empty trigger level
        val = IirFcr(0x0);
        val = val.set_transmitter_empty_trigger_level(TransmitterEmptyTriggerLevel::FifoEmpty);
        assert_eq!(val.0 & 0x30, 0x00);

        val = val.set_transmitter_empty_trigger_level(TransmitterEmptyTriggerLevel::FifoChar2);
        assert_eq!(val.0 & 0x30, 0x10);

        val =
            val.set_transmitter_empty_trigger_level(TransmitterEmptyTriggerLevel::FifoQuarterFull);
        assert_eq!(val.0 & 0x30, 0x20);

        val = val.set_transmitter_empty_trigger_level(TransmitterEmptyTriggerLevel::FifoHalfFull);
        assert_eq!(val.0 & 0x30, 0x30);

        // Test setting receiver trigger level
        val = IirFcr(0x0);
        val = val.set_receiver_trigger_level(ReceiverTriggerLevel::Char1);
        assert_eq!(val.0 & 0xC0, 0x00);

        val = val.set_receiver_trigger_level(ReceiverTriggerLevel::FifoQuarterFull);
        assert_eq!(val.0 & 0xC0, 0x40);

        val = val.set_receiver_trigger_level(ReceiverTriggerLevel::FifoHalfFull);
        assert_eq!(val.0 & 0xC0, 0x80);

        val = val.set_receiver_trigger_level(ReceiverTriggerLevel::FifoFullSub2);
        assert_eq!(val.0 & 0xC0, 0xC0);
    }

    #[test]
    fn struct_lcr_functions() {
        let mut val = Lcr(0x0);

        // Test setting and getting 5-bit word length
        val = val.set_word_length(WordLength::Bits5);
        assert_eq!(val.0, 0x00000000);
        assert_eq!(val.word_length(), WordLength::Bits5);

        // Test setting and getting 6-bit word length
        val = val.set_word_length(WordLength::Bits6);
        assert_eq!(val.0, 0x00000001);
        assert_eq!(val.word_length(), WordLength::Bits6);

        // Test setting and getting 7-bit word length
        val = val.set_word_length(WordLength::Bits7);
        assert_eq!(val.0, 0x00000002);
        assert_eq!(val.word_length(), WordLength::Bits7);

        // Test setting and getting 8-bit word length
        val = val.set_word_length(WordLength::Bits8);
        assert_eq!(val.0, 0x00000003);
        assert_eq!(val.word_length(), WordLength::Bits8);

        let mut val = Lcr(0x0);

        // Test setting and getting 1 stop bit
        val = val.set_stop_bits(StopBits::Bit1);
        assert_eq!(val.0, 0x00000000);
        assert_eq!(val.stop_bits(), StopBits::Bit1);

        // Test setting and getting 1.5/2 stop bits
        val = val.set_stop_bits(StopBits::Bits2OrBits1_5);
        assert_eq!(val.0, 0x00000004);
        assert_eq!(val.stop_bits(), StopBits::Bits2OrBits1_5);

        let mut val = Lcr(0x0);

        // Test enabling and disabling parity
        val = val.enable_parity();
        assert_eq!(val.0, 0x00000008);
        assert_eq!(val.is_parity_enabled(), true);

        val = val.disable_parity();
        assert_eq!(val.0, 0x00000000);
        assert_eq!(val.is_parity_enabled(), false);

        // Test setting and getting parity type
        val = val.set_parity(Parity::Odd);
        assert_eq!(val.0, 0x00000000);
        assert_eq!(val.parity(), Parity::Odd);

        val = val.set_parity(Parity::Even);
        assert_eq!(val.0, 0x00000010);
        assert_eq!(val.parity(), Parity::Even);

        let mut val = Lcr(0x0);

        // Test enabling and disabling stick parity
        val = val.enable_stick_parity();
        assert_eq!(val.0, 0x00000020);
        assert_eq!(val.is_stick_parity_enabled(), true);

        val = val.disable_stick_parity();
        assert_eq!(val.0, 0x00000000);
        assert_eq!(val.is_stick_parity_enabled(), false);

        let mut val = Lcr(0x0);

        // Test enabling and disabling break control
        val = val.enable_break_control();
        assert_eq!(val.0, 0x00000040);
        assert_eq!(val.is_break_control_enabled(), true);

        val = val.disable_break_control();
        assert_eq!(val.0, 0x00000000);
        assert_eq!(val.is_break_control_enabled(), false);

        let mut val = Lcr(0x0);

        // Test enabling and disabling divisor latch access
        val = val.enable_divisor_latch_access();
        assert_eq!(val.0, 0x00000080);
        assert_eq!(val.is_divisor_latch_access_enabled(), true);

        val = val.disable_divisor_latch_access();
        assert_eq!(val.0, 0x00000000);
        assert_eq!(val.is_divisor_latch_access_enabled(), false);
    }

    #[test]
    fn struct_mcr_functions() {
        let mut val = Mcr(0x0);

        // Test setting DTR to true
        val = val.set_data_terminal_ready(true);
        assert_eq!(val.0, 0x00000001);
        assert_eq!(val.data_terminal_ready(), true);

        // Test setting DTR to false
        val = val.set_data_terminal_ready(false);
        assert_eq!(val.0, 0x00000000);
        assert_eq!(val.data_terminal_ready(), false);

        let mut val = Mcr(0x0);

        // Test setting RTS to true
        val = val.set_request_to_send(true);
        assert_eq!(val.0, 0x00000002);
        assert_eq!(val.request_to_send(), true);

        // Test setting RTS to false
        val = val.set_request_to_send(false);
        assert_eq!(val.0, 0x00000000);
        assert_eq!(val.request_to_send(), false);

        let mut val = Mcr(0x0);

        // Test setting OUT1 to true
        val = val.set_out1(true);
        assert_eq!(val.0, 0x00000004);
        assert_eq!(val.out1(), true);

        // Test setting OUT1 to false
        val = val.set_out1(false);
        assert_eq!(val.0, 0x00000000);
        assert_eq!(val.out1(), false);

        let mut val = Mcr(0x0);

        // Test setting OUT2 to true
        val = val.set_out2(true);
        assert_eq!(val.0, 0x00000008);
        assert_eq!(val.out2(), true);

        // Test setting OUT2 to false
        val = val.set_out2(false);
        assert_eq!(val.0, 0x00000000);
        assert_eq!(val.out2(), false);

        let mut val = Mcr(0x0);

        // Test enabling loop back mode
        val = val.enable_loop_back();
        assert_eq!(val.0, 0x00000010);
        assert_eq!(val.is_loop_back_enabled(), true);

        // Test disabling loop back mode
        val = val.disable_loop_back();
        assert_eq!(val.0, 0x00000000);
        assert_eq!(val.is_loop_back_enabled(), false);

        let mut val = Mcr(0x0);

        // Test enabling auto flow control
        val = val.enable_auto_flow_control();
        assert_eq!(val.0, 0x00000020);
        assert_eq!(val.is_auto_flow_control_enabled(), true);

        // Test disabling auto flow control
        val = val.disable_auto_flow_control();
        assert_eq!(val.0, 0x00000000);
        assert_eq!(val.is_auto_flow_control_enabled(), false);

        let mut val = Mcr(0x0);

        // Test enabling SIR mode
        val = val.enable_sir_mode();
        assert_eq!(val.0, 0x00000040);
        assert_eq!(val.is_sir_mode_enabled(), true);

        // Test disabling SIR mode
        val = val.disable_sir_mode();
        assert_eq!(val.0, 0x00000000);
        assert_eq!(val.is_sir_mode_enabled(), false);
    }

    #[test]
    fn struct_lsr_functions() {
        // Test data ready flag
        let mut val = Lsr(0x02); // Set DR bit
        assert_eq!(val.is_data_ready(), true);

        val = Lsr(0x0);
        assert_eq!(val.is_data_ready(), false);

        // Test overrun error flag
        let mut val = Lsr(0x02); // Set OE bit
        assert_eq!(val.is_overrun_error(), true);

        val = Lsr(0x0);
        assert_eq!(val.is_overrun_error(), false);

        // Test parity error flag
        let mut val = Lsr(0x04); // Set PE bit
        assert_eq!(val.is_parity_error(), true);

        val = Lsr(0x0);
        assert_eq!(val.is_parity_error(), false);

        // Test framing error flag
        let mut val = Lsr(0x08); // Set FE bit
        assert_eq!(val.is_framing_error(), true);

        val = Lsr(0x0);
        assert_eq!(val.is_framing_error(), false);

        // Test break interrupt flag
        let mut val = Lsr(0x10); // Set BI bit
        assert_eq!(val.is_broken(), true);

        val = Lsr(0x0);
        assert_eq!(val.is_broken(), false);

        // Test transmitter FIFO empty flag
        let mut val = Lsr(0x20); // Set THRE bit
        assert_eq!(val.is_transmitter_fifo_empty(), true);

        val = Lsr(0x0);
        assert_eq!(val.is_transmitter_fifo_empty(), false);

        // Test transmitter empty flag
        let mut val = Lsr(0x40); // Set TEMT bit
        assert_eq!(val.is_transmitter_empty(), true);

        val = Lsr(0x0);
        assert_eq!(val.is_transmitter_empty(), false);

        // Test receiver FIFO error flag
        let mut val = Lsr(0x80); // Set RFE bit
        assert_eq!(val.is_receiver_fifo_error(), true);

        val = Lsr(0x0);
        assert_eq!(val.is_receiver_fifo_error(), false);

        // Test address received flag
        let mut val = Lsr(0x100); // Set ADDR_RCVD bit
        assert_eq!(val.is_address_received(), true);

        val = Lsr(0x0);
        assert_eq!(val.is_address_received(), false);
    }

    #[test]
    fn struct_msr_functions() {
        // Test DCTS bit
        let mut val = Msr(0x01); // Set DCTS bit
        assert_eq!(val.delta_clear_to_send(), true);

        val = Msr(0x0);
        assert_eq!(val.delta_clear_to_send(), false);

        // Test DDSR bit
        let mut val = Msr(0x02); // Set DDSR bit
        assert_eq!(val.delta_data_set_ready(), true);

        val = Msr(0x00);
        assert_eq!(val.delta_data_set_ready(), false);

        // Test TERI bit
        let mut val = Msr(0x04); // Set TERI bit
        assert_eq!(val.trailing_edge_of_ring_indicator(), true);

        val = Msr(0x0);
        assert_eq!(val.trailing_edge_of_ring_indicator(), false);

        // Test DDCD bit
        let mut val = Msr(0x08); // Set DDCD bit
        assert_eq!(val.delta_data_carrier_detect(), true);

        val = Msr(0x0);
        assert_eq!(val.delta_data_carrier_detect(), false);

        // Test CTS bit
        let mut val = Msr(0x10); // Set CTS bit
        assert_eq!(val.clear_to_send(), true);

        val = Msr(0x0);
        assert_eq!(val.clear_to_send(), false);

        // Test DSR bit
        let mut val = Msr(0x20); // Set DSR bit
        assert_eq!(val.data_set_ready(), true);

        val = Msr(0x0);
        assert_eq!(val.data_set_ready(), false);

        // Test RI bit
        let mut val = Msr(0x40); // Set RI bit
        assert_eq!(val.ring_indicator(), true);

        val = Msr(0x0);
        assert_eq!(val.ring_indicator(), false);

        // Test RI bit
        let mut val = Msr(0x40); // Set RI bit
        assert_eq!(val.ring_indicator(), true);

        val = Msr(0x0);
        assert_eq!(val.ring_indicator(), false);

        // Test RI bit
        let mut val = Msr(0x40); // Set RI bit
        assert_eq!(val.ring_indicator(), true);

        val = Msr(0x0);
        assert_eq!(val.ring_indicator(), false);

        // Test DCD bit
        let mut val = Msr(0x80); // Set DCD bit
        assert_eq!(val.data_carrier_detect(), true);

        val = Msr(0x0);
        assert_eq!(val.data_carrier_detect(), false);
    }

    #[test]
    fn struct_scr_functions() {
        let mut val = Scr(0x0);

        // 测试设置和获取临时存储值 0x11
        val = val.set_scratchpad(0x11);
        assert_eq!(val.0, 0x00000011);
        assert_eq!(val.scratchpad(), 0x11);

        // 测试设置和获取临时存储值 0xFF
        val = Scr(0x0);
        val = val.set_scratchpad(0xFF);
        assert_eq!(val.0, 0x000000FF);
        assert_eq!(val.scratchpad(), 0xFF);

        // 测试设置和获取临时存储值 0x00
        val = Scr(0x0);
        val = val.set_scratchpad(0x00);
        assert_eq!(val.0, 0x00000000);
        assert_eq!(val.scratchpad(), 0x00);

        // 测试设置和获取临时存储值 0xAA
        val = Scr(0x0);
        val = val.set_scratchpad(0xAA);
        assert_eq!(val.0, 0x000000AA);
        assert_eq!(val.scratchpad(), 0xAA);
    }
}
