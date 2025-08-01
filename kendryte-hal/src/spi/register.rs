use arbitrary_int::{u1, u2, u3, u4, u5, u7, u9, u14, u15, u17, u20, u24, u25, u26, u29, u30, u31};
use bitbybit::{bitenum, bitfield};
use volatile_register::{RO, RW};

// These definitions are from the K230 Technical Reference Manual

/// Working mode for the SPI peripheral.
#[bitenum(u1, exhaustive = true)]
#[derive(Debug, PartialEq, Eq)]
pub enum WorkingMode {
    /// Slave mode.
    Slave = 0b0,
    /// Master mode.
    Master = 0b1,
}

/// Generic enable/disable enum for single-bit flags.
#[bitenum(u1, exhaustive = true)]
#[derive(Debug, PartialEq, Eq)]
pub enum Enable {
    /// The feature is disabled.
    Disabled = 0b0,
    /// The feature is enabled.
    Enabled = 0b1,
}

/// Generic mask enum for single-bit flags.
#[bitenum(u1, exhaustive = true)]
#[derive(Debug, PartialEq, Eq)]
pub enum Masked {
    /// The flag is not masked.
    UnMasked = 0b0,
    /// The flag is masked.
    Masked = 0b1,
}
/// Generic active/inactive enum for single-bit flags.
#[bitenum(u1, exhaustive = true)]
#[derive(Debug, PartialEq, Eq)]
pub enum Active {
    /// The feature is inactive.
    Inactive = 0b0,
    /// The feature is active.
    Active = 0b1,
}

/// Microwire transfer mode.
#[bitenum(u1, exhaustive = true)]
#[derive(Debug, PartialEq, Eq)]
pub enum MicrowireTransferMode {
    /// The transfer is not sequential.
    NonSequential = 0b0,
    /// The transfer is sequential.
    Sequential = 0b1,
}
/// Microwire transfer mode.
#[bitenum(u1, exhaustive = true)]
#[derive(Debug, PartialEq, Eq)]
pub enum MicrowireControlMode {
    /// SSI receives data.
    Receive = 0b0,
    /// SSI transmits data.
    Transmit = 0b1,
}
/// Frame format selection.
#[bitenum(u2, exhaustive = true)]
#[derive(Debug, PartialEq, Eq)]
pub enum FrameFormat {
    /// Motorola SPI frame format.
    MotorolaSpi = 0b00,
    /// Texas Instruments SSP frame format.
    TexasInstrumentsSsp = 0b01,
    /// National Semiconductors Microwire frame format.
    NationalMicrowire = 0b10,
    /// Reserved.
    Reserved = 0b11,
}

/// Serial clock phase.
#[bitenum(u1, exhaustive = true)]
#[derive(Debug, PartialEq, Eq)]
pub enum SerialClockPhase {
    /// Serial clock toggles in middle of first data bit.
    Middle = 0b0,
    /// Serial clock toggles at start of first data bit.
    Start = 0b1,
}

/// Serial clock polarity.
#[bitenum(u1, exhaustive = true)]
#[derive(Debug, PartialEq, Eq)]
pub enum SerialClockPolarity {
    /// Inactive state of serial clock is low.
    Low = 0b0,
    /// Inactive state of serial clock is high.
    High = 0b1,
}

/// Transfer mode.
#[bitenum(u2, exhaustive = true)]
#[derive(Debug, PartialEq, Eq)]
pub enum TransferMode {
    /// Transmit and receive.
    TransmitAndReceive = 0b00,
    /// Transmit only.
    TransmitOnly = 0b01,
    /// Receive only.
    ReceiveOnly = 0b10,
    /// EEPROM read.
    EepromRead = 0b11,
}

/// Control Register 0 (CTRLR0)
///
/// This register controls the serial data transfer. It is impossible to write to this register when the SSI is enabled.
#[bitfield(u32)]
pub struct ControlReg0 {
    /// Data Frame Size (DFS).
    /// Note: When SSIC_SPI_MODE is set to "Dual", "Quad" or "Octal" mode and SPI_FRF is not set to 2'b00: - DFS value must be a multiple of 2 if SPI_FRF = 01 - DFS value must be multiple of 4 if SPI_FRF = 10 - DFS value must be multiple of 8 if SPI_FRF = 11
    #[bits(0..=4, rw)]
    pub data_frame_size: u5, /* Reset value is 0x07 */
    /// Reserved bit.
    #[bit(5, r)]
    pub _reserved_5: u1,
    /// Frame Format (FRF).
    /// 0x0 (SPI): Motorola SPI Frame Format
    // FIXME: access is `Varies`
    #[bits(6..=7, rw)]
    pub frame_format: FrameFormat,
    /// Serial Clock Phase (SCPH).
    /// Values:
    /// - 0x1 (START_BIT): Serial clock toggles at start of first bit
    /// - 0x0 (MIDDLE_BIT): Serial clock toggles in middle of first bit
    // FIXME: access is `Varies`
    #[bit(8, rw)]
    pub serial_clock_phase: SerialClockPhase,
    /// Serial Clock Polarity (SCPOL).
    /// Values:
    /// - 0x0 (INACTIVE_HIGH): Inactive state of serial clock is low
    /// - 0x1 (INACTIVE_LOW): Inactive state of serial clock is high
    // FIXME: access is `Varies`
    #[bit(9, rw)]
    pub serial_clock_polarity: SerialClockPolarity,
    /// Transfer Mode (TMOD).
    /// Values:
    /// - 0x0 (TX_AND_RX): Transmit & Receive; Not Applicable in enhanced SPI operating mode or when SSIC_HAS_TX_RX_EN is set to 0
    /// 0x1 (TX_ONLY): Transmit only mode; Or Write in enhanced SPI operating mode
    /// 0x2 (RX_ONLY): Receive only mode; Or Read in enhanced SPI operating mode
    /// 0x3 (EEPROM_READ): EEPROM Read mode; Not Applicable in enhanced SPI operating mode
    #[bits(10..=11, rw)]
    pub transfer_mode: TransferMode,
    /// Slave Output Enable (SLV_OE).
    /// Values:
    /// -0x1 (DISABLED): Slave Output is disabled
    /// -0x0 (ENABLED): Slave Output is enabled
    #[bit(12, rw)]
    pub slave_output_enable: Enable,
    /// Shift Register Loop (SRL).
    /// Used for testing purposes only.
    /// Values:
    /// -0x1 (TESTING_MODE): Test Mode Operation
    /// -0x0 (NORMAL_MODE): Normal mode operation
    // FIXME: access is `Varies`
    #[bit(13, rw)]
    pub shift_register_loop: Enable,
    /// Slave Select Toggle Enable (SSTE).
    /// Values:
    /// - 0x1 (TOGGLE_EN): ss_n line will toggle between consecutive data frames, with the serial clock (sclk) being held to its Reset value while ssn is high
    /// - 0x0 (TOGGLE_DISABLE): ss*_n will stay low and sclk will run continuously for the duration of the transfer
    #[bit(14, rw)]
    pub slave_select_toggle_enable: Enable, /* Default is 0x1 (TOGGLE_ENABLE) */
    /// Reserved
    #[bit(15, r)]
    pub _reserved_15: u1,

    /// Control Frame Size.
    #[bits(16..=19, rw)]
    pub control_frame_size: u4,
    /// Data Frame Size in 32-bit mode (CFS).
    /// Used to select the data frame length when SSI_MAX_XFER_SIZE configuration parameter is set to 32.
    #[bits(16..=20, rw)]
    pub data_frame_size_32: u5,

    /// Reserved.
    #[bits(20..=21, r)]
    pub reserved_20_21: u2,
    /// SPI Frame Format (SPR_FRF).
    /// Values:
    /// - 0x0 (SPI_STANDARD): Standard SPI Format
    /// - 0x1 (SPI_DUAL): Dual SPI Format
    /// - 0x2 (SPI_QUAD): Quad SPI Format
    /// - 0x3 (SPI_OCTAL): Octal SPI Format
    // FIXME: access is `Varies`
    #[bits(22..=23, rw)]
    pub spi_frame_format: u2,

    /// SPI Hyperbus Frame Format Enable (SPI_HYPERBUS_EN):
    /// Values:
    /// - 0x0 (DISABLE): Disable Hyperbus Format
    /// - 0x1 (ENABLE): Enable Hyperbus Format
    // FIXME: access is `Varies`
    #[bit(24, rw)]
    pub spi_hyperbus_enable: Enable,
    /// SPI Dynamic Wait State Enable (SPI_DWS_EN).
    /// Enable Dynamic wait states in SPI mode of operation.
    /// Values:
    /// - 0x0 (DISABLE): Disable SPI Dynamic Wait State
    /// - 0x1 (ENABLE): Enable SPI Dynamic Wait State
    #[bit(25, rw)]
    pub spi_dynamic_wait_enable: Enable,

    /// Reserved.
    #[bits(26..=30, r)]
    _reserved: u5,

    /// SSI is Master or Slave (SSI_IS_MST).
    /// This field selects if SPI is working in Master or Slave mode
    /// Values:
    /// - 0x1 (MASTER)
    /// - 0x0 (SLAVE)
    // FIXME: access is `Varies`
    #[bit(31, rw)]
    pub ssi_is_master: WorkingMode,
}

/// Control Register 1 (CTRLR1)
///
/// CTRLR1 is a Control Register 1 Offset Address: 0x4 Total Reset Value:0x0
#[bitfield(u32)]
pub struct ControlReg1 {
    ///Number of Data Frames (NDF).
    /// When TMOD = 10 or TMOD = 11 , this register field sets the number of data frames to be continuously received by the SPI.
    /// The SPI continues to receive serial data until the number of data frames received is equal to this register value plus 1,which enables you to receive up to 64 KB of data in a continuous transfer.
    /// When SPI_CTRLR0.CLK_STRETCH_EN=1 and TMOD = 01, this register field sets the number of data frames to be continously transmitted by SPI.
    /// If the Transmit FIFO goes empty in-between, SPI masks the serial clock (sclk_out) and wait for rest of the data until the programmed amount of frames are transferred successfully.When the SPI is configured as a serial slave, the transfer continues for as long as the slave is selected.
    /// Therefore, this register serves no purpose and is not present when the SPI is configured as a serial slave.

    #[bits(0..=15, rw)]
    pub number_of_data_frames: u16,
    /// Reserved.
    #[bits(16..=31, r)]
    _reserved: u16,
}

/// SSI Enable Register (SSIENR)
///
/// SSIENR is a SSI Enable Register
/// Offset Address: 0x8
/// Total Reset Value:0x0
#[bitfield(u32)]
pub struct SsiEnableReg {
    ///SSI Enable.
    /// Enables and disables all FMC operations.
    /// When disabled, all serial transfers are halted immediately.
    /// Transmit and receive FIFO buffers are cleared when the device is disabled.
    /// It is impossible to program some of the FMC control registers when enabled.
    /// When disabled, the ssi sleep output is set (after delay) to inform the system that it is safe to remove the ssi_clk, thus saving power consumption in the system.
    /// Values:
    /// - 0x1 (ENABLED): Enables FMC
    /// - 0x0 (DISABLE): Disables FMC
    #[bit(0, rw)]
    pub ssi_enable: Enable,
    /// Reserved.
    #[bits(1..=31, r)]
    _reserved: u31,
}

/// Microwire Control Register (MWCR)
/// MWCR is a Microwire Control Register
/// Offset Address: 0xc
/// Total Reset Value:0x0
#[bitfield(u32)]
pub struct MicrowireControlReg {
    /// Microwire Transfer Mode (MWMOD).
    /// Values:
    /// - 0x1 (SEQUENTIAL): Sequential Transfer
    /// - 0x0 (NON_SEQUENTIAL): Non-Sequential Transfer
    #[bit(0, rw)]
    pub microwire_mode: MicrowireTransferMode,
    /// Microwire Control (MDD).
    /// Values:
    /// - 0x1 (TRANSMIT): SSI transmits data
    /// - 0x0 (RECEIVE): SSI receives data
    #[bit(1, rw)]
    pub microwire_direction: MicrowireControlMode,
    /// Microwire Handshaking (MHS).
    /// Values:
    /// - 0x1 (ENABLED): handshaking interface is enabled
    /// - 0x0 (DISABLE): handshaking interface is disabled
    // FIXME: access is `Varies`
    #[bit(2, rw)]
    pub microwire_handshaking: Enable,
    /// Reserved.
    #[bits(3..=31, r)]
    _reserved: u29,
}

/// Slave Enable Register (SER)
/// Offset Address: 0x10
/// Total Reset Value:0x0
#[bitfield(u32)]
pub struct SlaveEnableReg {
    ///Slave Select Enable Flag (SER).
    /// Each bit in this register corresponds to a slave select line (ss_x_n) from the controller.
    /// When a bit in this register is set (1), the corresponding slave select line from the master is activated when a serial transfer begins.
    /// It should be noted that setting or clearing bits in x:0.
    /// This register have no effect on the corresponding slave select outputs until a Before beginning a transfer, you should enable the bit in this register that corresponds to the slave device with which the master wants to communicate.
    /// When not operating in broadcast mode, only one bit in this field should be set.
    // TODO: the two fields has a length that varies, currently we use a 30:2 separation.
    #[bits(0..=29, rw)]
    pub slave_select_enable: u30,
    /// Reserved.
    #[bits(30..=31,r)]
    pub _reserved: u2,
}

/// Baud Rate Select Register (BAUDR)
/// Offset Address: 0x14
/// Total Reset Value:0x0
#[bitfield(u32)]
pub struct BaudRateSelectReg {
    /// Reserved.
    #[bit(1, r)]
    pub _reserved_0: u1,
    /// SSI Clock Divider (SCKDV).  
    /// The LSB for this field is always set to 0 and is unaffected by a write operation, which ensures an even value is held in this register.
    /// If the value is 0, the serial output clock (sclk_out) is disabled.
    /// The frequency of the sclk_out is derived from the following equation:Fsclk_out = Fssi_clk/BAUDR where BAUDR is any even value between 2 and 65534 (BAUDR = {SCKDV*2}).
    /// For example: for Fssi_clk = 3.6864MHz and BAUDR =2 Fsclk_out =3.6864/2 = 1.8432MHz

    #[bits(1..=15, rw)]
    pub ssi_clock_divider: u15, /* Reset value is SSIC_DFLT_BAUDR/2 */
    /// Reserved.
    #[bits(16..=31, r)]
    _reserved_15_31: u16,
}

/// Transmit FIFO Threshold Level Register (TXFTLR)
/// Offset Address: 0x18
/// Total Reset Value:0x0
// TODO, the four fields has a length that varies, currently we use a 2:14:2:14 separation.
#[bitfield(u32)]
pub struct TransmitFifoThresholdLevelReg {
    /// Transmit FIFO Threshold (TFT).
    /// Controls the level of entries (or below) at which the transmit FIFO controller triggers an interrupt.
    /// The FIFO depth is configurable in the range 8-256; this register is sized to the number of address bits needed to access the FIFO.
    /// If you attempt to set this value greater than or equal to the depth of the FIFO, this field is not written and retains its current value.
    /// When the number of transmit FIFO entries is less than or equal to this value, the transmit FIFO empty interrupt is triggered.
    // TODO: actually the field is x:0
    #[bits(0..=1, rw)]
    pub transmit_fifo_threshold: u2,
    /// Transfer start FIFO level (TXFTHR).
    /// Used to control the level of entries in transmit FIFO above which transfer will start on serial line.
    /// This register can be used to ensure that sufficient data is present in transmit FIFO before starting a write operation on serial line.
    /// In Internal DMA mode, this field sets the minimum amount of data frames present in the FIFO after which the controller starts the transfer.

    // TODO: actually the field is x:16
    // FIXME: access is `Varies`
    #[bits(2..=15, rw)]
    pub transfer_start_fifo_level: u14,

    /// Reserved.
    // TODO: actually the field is 16:y
    #[bits(16..=17, r)]
    pub _reserved_0: u2,

    /// Reserved.
    // TODO: actually the field is 16:y
    #[bits(18..=31, r)]
    pub _reserved_1: u14,
}

/// Receive FIFO Threshold Level Register (RXFTLR)
/// Offset Address: 0x1c
/// Total Reset Value:0x0
#[bitfield(u32)]
pub struct ReceiveFifoThresholdLevelReg {
    /// Receive FIFO Threshold (RFT).
    /// Controls the level of entries (or above) at which the receive FIFO controller triggers an interrupt.
    /// The FIFO depth is configurable in the range 8-256.
    /// This register is sized to the number of address bits needed to access the FIFO.
    /// If you attempt to set this value greater than the depth of the FIFO, this field is not written and retains its current value.
    /// When the number of receive FIFO entries is greater than or equal to this value + 1, the receive FIFO full interrupt is triggered.

    // TODO: the field is x:0
    #[bits(0..=7, rw)]
    pub receive_fifo_threshold: u8,
    /// Reserved.
    // TODO: the field is 31:y
    #[bits(8..=31, r)]
    _reserved: u24,
}

/// Transmit FIFO Level Register (TXFLR)
/// Offset Address: 0x20
/// Total Reset Value:0x0
#[bitfield(u32)]
pub struct TransmitFifoLevelReg {
    /// Transmit FIFO Level.
    /// Contains the number of valid data entries in the transmit FIFO.
    // TODO: the field is x:0
    #[bits(0..=7, r)]
    pub transmit_fifo_level: u8,
    /// Reserved.
    // TODO: the field is 31:y
    #[bits(8..=31, r)]
    _reserved: u24,
}

/// Receive FIFO Level Register (RXFLR)
/// Offset Address: 0x24
/// Total Reset Value:0x0
#[bitfield(u32)]
pub struct ReceiveFifoLevelReg {
    /// Receive FIFO Level.
    /// Contains the number of valid data entries in the receive FIFO.
    // TODO: the field is x:0
    #[bits(0..=7, r)]
    pub receive_fifo_level: u8,
    /// Reserved.
    // TODO: the field is 31:y
    #[bits(8..=31, r)]
    _reserved: u24,
}

/// Status Register (SR)
/// Offset Address: 0x28
/// Total Reset Value:0x00000006
#[bitfield(u32)]
pub struct StatusReg {
    /// SSI Busy Flag (BUSY).
    /// When set, indicates that a serial transfer is in progress; when cleared indicates that the controller is idle or disabled.
    /// Values:
    /// - 0x1 (ACTIVE): FMC is actively transferring data
    /// - 0x0 (INACTIVE): FMC is idle or disabled
    #[bit(0, r)]
    pub busy: bool, /* Reset value is 0x01 */
    /// Transmit FIFO Not Full (TFNF).
    /// Set when the transmit FIFO contains one or more empty locations, and is cleared when the FIFO is full.
    /// Values:
    /// - 0x1 (NOT_FULL): Tx FIFO is not Full
    /// - 0x0 (FULL): Tx FIFO is full
    #[bit(1, r)]
    pub transmit_fifo_not_full: bool, /* Reset value is 0x01 */
    /// Transmit FIFO Empty (TFE).
    /// When the transmit FIFO is completely empty, this bit is set. When the transmit FIFO contains one or more valid entries, this bit is cleared. This bit field does not request an interrupt.
    /// Values:
    /// - 0x1 (EMPTY): Transmit FIFO is empty
    /// - 0x0 (NOT_EMPTY): Transmit FIFO is not empty
    #[bit(2, r)]
    pub transmit_fifo_empty: bool, /* Reset value is 0x01 */
    /// Receive FIFO Not Empty (RFNE).
    /// Set when the receive FIFO contains one or more entries and is cleared when the receive FIFO is empty. This bit can be polled by software to completely empty the receive FIFO.
    /// Values:
    /// - 0x1 (NOT_EMPTY): Receive FIFO is not empty
    /// - 0x0 (EMPTY): Receive FIFO is empty.
    #[bit(3, r)]
    pub receive_fifo_not_empty: bool,
    /// Receive FIFO Full (RFF).
    /// When the receive FIFO is completely full, this bit is set. When the receive FIFO contains one or more empty location, this bit is cleared.
    /// Values:
    /// - 0x1 (FULL): Receive FIFO is full
    /// - 0x0 (NOT_FULL): Receive FIFO is not full
    #[bit(4, r)]
    pub receive_fifo_full: bool,
    /// Transmission Error (TE).
    /// Values:
    /// - 0x1 (TX_ERROR): Transmission Error
    /// - 0x0 (NO_ERROR): No Error
    #[bit(5, r)]
    pub transmission_error: bool,
    /// Data Collision Error (DCOL).
    /// This bit will be set if ss_in_n input is asserted by other master, when the controller is in the middle of the transfer. This informs the processor that the last data transfer was halted before completion. This bit is cleared when read.
    /// Values:
    /// - 0x1 (TX_COLLISION_ERROR): Transmit Data Collision Error
    /// - 0x0 (NO_ERROR_CONDITION): No Error
    #[bit(6, r)]
    pub data_collision_error: bool,
    /// Reserved.
    #[bits(7..=14, r)]
    pub _reserved_0: u8,

    /// Completed Data frames (CMPLTD_DF)
    /// This field indicates total data frames transferred in the previous internal DMA transfer
    #[bits(15..=31, r)]
    pub completed_data_frames: u17,
}

/// Interrupt Mask Register (IMR)
/// Offset Address: 0x2c
/// Total Reset Value:0x0000003f
#[bitfield(u32)]
pub struct InterruptMaskReg {
    /// Transmit FIFO Empty Interrupt Mask (TXEIM).
    /// Values:
    /// - 0x1 (UNMASKED): ssi_txe_intr interrupt is not masked
    /// - 0x0 (MASKED): ssi_txe_intr interrupt is masked
    #[bit(0, rw)]
    pub transmit_fifo_empty_interrupt_mask: Masked, /* Reset value is `Masked` (0x01) */
    /// Transmit FIFO Overflow Interrupt Mask (TXOIM).
    /// Values:
    /// - 0x1 (UNMASKED): ssi_txo_intr interrupt is not masked
    /// - 0x0 (MASKED): ssi_txo_intr interrupt is masked
    #[bit(1, rw)]
    pub transmit_fifo_overflow_interrupt_mask: Masked, /* Reset value is `Masked` (0x01) */
    /// Receive FIFO Underflow Interrupt Mask (RFUIM).
    /// Values:
    /// - 0x1 (UNMASKED): ssi_rfu_intr interrupt is not masked
    /// - 0x0 (MASKED): ssi_rfu_intr interrupt is masked
    #[bit(2, rw)]
    pub receive_fifo_underflow_interrupt_mask: Masked, /* Reset value is `Masked` (0x01) */
    /// Receive FIFO Overflow Interrupt Mask (RXUIM) .
    /// Values:
    /// - 0x1 (UNMASKED): ssi_rxu_intr interrupt is not masked
    /// - 0x0 (MASKED): ssi_rxu_intr interrupt is masked
    #[bit(3, rw)]
    pub receive_fifo_overflow_interrupt_mask: Masked, /* Reset value is `Masked` (0x01) */
    /// Receive FIFO Full Interrupt Mask (RXFIM).
    /// Values:
    /// - 0x1 (UNMASKED): ssi_rxf_intr interrupt is not masked
    /// - 0x0 (MASKED): ssi_rxf_intr interrupt is masked
    #[bit(4, rw)]
    pub receive_fifo_full_interrupt_mask: Masked, /* Reset value is `Masked` (0x01) */
    /// Multi-Master Contention Interrupt Mask (MSTIM).
    /// Values:
    /// - 0x1 (UNMASKED): ssi_xrxo_intr interrupt is not masked
    /// - 0x0 (MASKED): ssi_xrxo_intr interrupt is masked
    // FIXME: access is `Varies`
    #[bit(5, rw)]
    pub multi_master_contention_interrupt_mask: Masked, /* Reset value is `Masked` (0x01) */

    /// XIP Receive FIFO Overflow Interrupt Mask (XRXIOM).
    /// Values:
    /// - 0x1 (UNMASKED): ssi_xrxo_intr interrupt is not masked
    /// - 0x0 (MASKED): ssi_xrxo_intr interrupt is masked
    // FIXME: access is `Varies`
    #[bit(6, rw)]
    pub xip_receive_fifo_overflow_interrupt_mask: Masked,

    /// Transmit FIFO Underflow Interrupt Mask (TXUIM)
    /// Values:
    /// - 0x1 (UNMASKED): ssi_txu_intr interrupt is not masked
    /// - 0x0 (MASKED): ssi_txu_intr interrupt is masked
    // FIXME: access is `Varies`
    #[bit(7, rw)]
    pub transmit_fifo_underflow_interrupt_mask: Masked,
    /// AXI Error Interrupt Mask
    /// Values:
    /// - 0x1 (UNMASKED): ssi_axie_intr interrupt is not masked
    /// - 0x0 (MASKED): ssi_axie_intr interrupt is masked
    // FIXME: access is `Varies`
    #[bit(8, rw)]
    pub axi_error_interrupt_mask: Masked,

    /// Reserved.
    #[bit(9, r)]
    _reserved_9: u1,

    /// SPI Transmit Error Interrupt Mask (SPITEM)
    /// Values:
    /// - 0x1 (UNMASKED): ssi_spite_intr interrupt is not masked
    /// - 0x0 (MASKED): ssi_spite_intr interrupt is masked
    // FIXME: access is `Varies`
    #[bit(10, rw)]
    pub spi_transmit_error_interrupt_mask: Masked,

    /// SSI Done Interrupt Mask (DONEM)
    /// Values:
    /// - 0x1 (UNMASKED): ssi_done_intr interrupt is not masked
    /// - 0x0 (MASKED): ssi_done_intr interrupt is masked
    // FIXME: access is `Varies`
    #[bit(11, rw)]
    pub ssi_done_interrupt_mask: Masked,

    /// Reserved.
    #[bits(12..=31, r)]
    // FIXME: should it be 31 instead of 32? (this is directly from the TRM)
    _reserved_12_32: u20,
}

/// Interrupt Status Register (ISR)
/// Offset Address: 0x30
/// Total Reset Value:0x0
#[bitfield(u32)]
pub struct InterruptStatusReg {
    /// Transmit FIFO Empty Interrupt Status (TXEIS).
    /// Values:
    /// - 0x1 (ACTIVE): ssi_txe_intr interrupt is active after masking
    /// - 0x0 (INACTIVE): ssi_txe_intr interrupt is not active after masking
    #[bit(0, r)]
    pub transmit_fifo_empty_interrupt_status: Active,
    /// Transmit FIFO Overflow Interrupt Status (TXOIS).
    /// Values:
    /// - 0x1 (ACTIVE): ssi_txo_intr interrupt is active after masking
    /// - 0x0 (INACTIVE): ssi_txo_intr interrupt is not active after masking
    #[bit(1, r)]
    pub transmit_fifo_overflow_interrupt_status: Active,
    /// Receive FIFO Underflow Interrupt Status (RXUIS).
    /// Values:
    /// - 0x1 (ACTIVE): ssi_rxu_intr interrupt is active after masking
    /// - 0x0 (INACTIVE): ssi_rxu_intr interrupt is not active after masking
    #[bit(2, r)]
    pub receive_fifo_underflow_interrupt_status: Active,
    /// Receive FIFO Overflow Interrupt Statusa (RXOIS).
    /// Values:
    /// - 0x1 (ACTIVE): ssi_rxo_intr interrupt is active after masking
    /// - 0x0 (INACTIVE): ssi_rxo_intr interrupt is not active after masking
    #[bit(3, r)]
    pub receive_fifo_overflow_interrupt_status: Active,
    /// Receive FIFO Full Interrupt Status (RXFIS).
    /// Values:
    /// - 0x1 (ACTIVE): ssi_rxf_intr interrupt is active after masking
    /// - 0x0 (INACTIVE): ssi_rxf_intr interrupt is not active after masking
    #[bit(4, r)]
    pub receive_fifo_full_interrupt_status: bool,
    /// Multi-Master Contention Interrupt Status (MSTIX).
    /// This bit field is not present if the SPI is configured as a serial-slave device.
    /// Values:
    /// - 0x1 (ACTIVE): ssi_mst_intr interrupt is active after masking
    /// - 0x0 (INACTIVE): ssi_mst_intr interrupt is not active after masking
    #[bit(5, r)]
    pub multi_master_contention_interrupt_status: Active,
    /// XIP Receive FIFO Overflow Interrupt Status (XRXOIS).
    /// Values:
    /// - 0x1 (ACTIVE): ssi_xrxo_intr interrupt is active after masking
    /// - 0x0 (INACTIVE): ssi_xrxo_intr interrupt is not active after masking
    #[bit(6, r)]
    pub xip_receive_fifo_overflow_interrupt_status: Active,
    /// Transmit FIFO Underflow Interrupt Status (TXUIS).
    /// Values:
    /// - 0x1 (ACTIVE): ssi_txu_intr interrupt is active after masking
    /// - 0x0 (INACTIVE): ssi_txu_intr interrupt is not active after masking
    #[bit(7, r)]
    pub transmit_fifo_underflow_interrupt_status: Active,
    /// AXI Error Interrupt Status (AXIES).
    /// Values:
    /// - 0x1 (ACTIVE): ssi_axie_intr interrupt is active after masking
    /// - 0x0 (INACTIVE): ssi_axie_intr interrupt is not active after masking
    #[bit(8, r)]
    pub axi_error_interrupt_status: Active,

    /// Reserved.
    #[bit(9, r)]
    _reserved_9: u1,

    /// SPI Transmit Error Interrupt Status (SPITES).
    /// Values:
    /// - 0x1 (ACTIVE): ssi_spite_intr interrupt is active after masking
    /// - 0x0 (INACTIVE): ssi_spite_intr interrupt is not active after masking
    #[bit(10, r)]
    pub spi_transmit_error_interrupt_status: Active,

    /// SSI Done Interrupt Status (DONES).
    /// Values: - 0x1 (ACTIVE): ssi_done_intr interrupt is active after masking
    /// - 0x0 (INACTIVE): ssi_done_intr interrupt is not active after masking
    #[bit(11, r)]
    pub ssi_done_interrupt_status: Active,

    /// Reserved.
    #[bits(12..=31, r)]
    _reserved_12_31: u20,
}

/// Raw Interrupt Status Register (RISR)
///
/// Offset Address: 0x34
/// Total Reset Value:0x0
#[bitfield(u32)]
pub struct RawInterruptStatusReg {
    /// Transmit FIFO Empty Raw Interrupt Status (TXEIR).
    /// Values:
    /// - 0x1 (ACTIVE): ssi_txe_intr interrupt is active prior to masking
    /// - 0x0 (INACTIVE): ssi_txe_intr interrupt is not active prior masking
    #[bit(0, r)]
    pub transmit_fifo_empty_raw_interrupt_status: Active,
    /// Transmit FIFO Overflow Raw Interrupt Status (TXOIR).
    /// Values:
    /// - 0x1 (ACTIVE): ssi_txo_intr interrupt is active prior to masking
    /// - 0x0 (INACTIVE): ssi_txo_intr interrupt is not active prior masking
    #[bit(1, r)]
    pub transmit_fifo_overflow_raw_interrupt_status: Active,
    /// Receive FIFO Underflow Raw Interrupt Status (RXUIR).
    /// Values:
    /// - 0x1 (ACTIVE): ssi_rxu_intr interrupt is active prior to masking
    /// - 0x0 (INACTIVE): ssi_rxu_intr interrupt is not active prior masking
    #[bit(2, r)]
    pub receive_fifo_underflow_raw_interrupt_status: Active,
    /// Receive FIFO Overflow Raw Interrupt Status (RXOIR).
    /// Values:
    /// - 0x1 (ACTIVE): ssi_rxo_intr interrupt is active prior to masking
    /// - 0x0 (INACTIVE): ssi_rxo_intr interrupt is not active prior masking
    #[bit(3, r)]
    pub receive_fifo_overflow_raw_interrupt_status: Active,
    /// Receive FIFO Full Raw Interrupt Status (RXFIR).
    /// Values:
    /// - 0x1 (ACTIVE): ssi_rxf_intr interrupt is active prior to masking
    /// - 0x0 (INACTIVE): ssi_rxf_intr interrupt is not active prior masking
    #[bit(4, r)]
    pub receive_fifo_full_raw_interrupt_status: Active,
    /// Multi-Master Contention Raw Interrupt Status (MSTIR).
    /// Values:
    /// - 0x1 (ACTIVE): ssi_mst_intr interrupt is active prior to masking
    /// - 0x0 (INACTIVE): ssi_mst_intr interrupt is not active prior masking
    #[bit(5, r)]
    pub multi_master_contention_raw_interrupt_status: Active,

    /// XIP Receive FIFO Overflow Raw Interrupt Status (XRXOIR).
    /// Values:
    /// - 0x1 (ACTIVE): ssi_xrxo_intr interrupt is active piror masking
    /// - 0x0 (INACTIVE): ssi_xrxo_intr interrupt is not active piror masking
    #[bit(6, r)]
    pub xip_receive_fifo_overflow_raw_interrupt_status: Active,

    /// Transmit FIFO Underflow Raw Interrupt Status (TXUIR).
    /// Values:
    /// - 0x1 (ACTIVE): ssi_txu_intr interrupt is active prior masking
    /// - 0x0 (INACTIVE): ssi_txu_intr interrupt is not active prior masking
    #[bit(7, r)]
    pub transmit_fifo_underflow_raw_interrupt_status: Active,

    /// AXI Error Interrupt Raw Status (AXIER).
    /// Values:
    /// - 0x1 (ACTIVE): ssi_axie_intr interrupt is active pior masking
    /// - 0x0 (INACTIVE): ssi_axie_intr interrupt is not active prior masking
    #[bit(8, r)]
    pub axi_error_interrupt_raw_status: Active,

    /// Reserved.
    #[bit(9, r)]
    _reserved_9: u1,

    /// SPI Transmit Error Interrupt Status (SPITER).
    /// This bit gets set, If SPI Master fails to get a READY status from the slave until the amount of time defined in SPI_CTRLR1.MAX_WS field, then it will stop the SPI transfer and the FIFO is flushed (in case of write operation).

    /// Values:
    /// - 0x1 (ACTIVE): ssi_spite_intr interrupt is active prior masking
    /// - 0x0 (INACTIVE): ssi_spite_intr interrupt is not active prior masking
    #[bit(10, r)]
    pub spi_transmit_error_interrupt_status: Active,

    /// SSI Done Interrupt Raw Status (DONER).
    /// Values:
    /// - 0x1 (ACTIVE): ssi_done_intr interrupt is active prior masking
    /// - 0x0 (INACTIVE): ssi_done_intr interrupt is not active prior masking
    #[bit(11, r)]
    pub ssi_done_interrupt_raw_status: Active,

    /// Reserved.
    /// FIXME: should it be 31 instead of 32? (this is directly from the TRM)
    #[bits(12..=31, r)]
    _reserved_12_32: u20,
}

/// Transmit FIFO Error Interrupt Clear Registers (TXEICR)
///  Offset Address: 0x38
/// Total Reset Value:0x0
#[bitfield(u32)]
pub struct TransmitFifoErrorInterruptClearReg {
    /// Transmit FIFO Error Interrupt Clear (TXFEIC).
    ///Clear Transmit FIFO Overflow/Underflow Interrupt.This register reflects the status of the interrupt.
    /// A read from this register clears the 0x0 ssi_txo_intr/ssi_txu_intr interrupt; writing has no effect.
    // FIXME: access is `RC`
    #[bit(0, rw)]
    pub transmit_fifo_error_interrupt_clear: bool,

    /// Reserved.
    #[bits(1..=31, r)]
    pub _reserved: u31,
}

/// Receive FIFO Overflow Interrupt Clear Register (RXOICR)
/// Offset Address: 0x3c
/// Total Reset Value:0x0
#[bitfield(u32)]
pub struct ReceiveFifoOverflowInterruptClearReg {
    /// Receive FIFO Overflow Interrupt Clear (RXFOIC).
    /// Clear Receive FIFO Overflow Interrupt.This register reflects the status of the interrupt.
    /// A read from this register clears the ssi_rxo_intr 0x0 interrupt; writing has no effect.
    // FIXME: access is `RC`
    #[bit(0, rw)]
    pub receive_fifo_overflow_interrupt_clear: bool,
    /// Reserved.
    #[bits(1..=31, r)]
    pub _reserved: u31,
}

/// Receive FIFO Underflow Interrupt Clear Register (RXUICR)
/// Offset Address: 0x40
/// Total Reset Value:0x0
#[bitfield(u32)]
pub struct ReceiveFifoUnderflowInterruptClearReg {
    /// Receive FIFO Underflow Interrupt Clear (RXFUIC).
    /// Clear Receive FIFO Underflow Interrupt.This register reflects the status of the interrupt.
    /// A read from this register clears the ssi_rxu_intr 0x0 interrupt
    /// writing has no effect.
    /// FIXME: access is `RC`
    #[bit(0, rw)]
    pub receive_fifo_underflow_interrupt_clear: bool,
    /// Reserved.
    #[bits(1..=31, r)]
    pub _reserved: u31,
}

/// Multi-Master Interrupt Clear Register (MSTICR)
/// Offset Address: 0x44
/// Total Reset Value:0x0
#[bitfield(u32)]
pub struct MultiMasterInterruptClearReg {
    /// Multi-Master Interrupt Clear (MSTIC).
    /// Clear Multi-Master Contention Interrupt.This register reflects the status of the interrupt.
    /// A read from this register clears the ssi_mst_intr 0x0 interrupt
    /// writing has no effect.
    // FIXME: access is `RC`
    #[bit(0, rw)]
    pub multi_master_interrupt_clear: bool,
    /// Reserved.
    #[bits(1..=31, r)]
    pub _reserved: u31,
}

/// Interrupt Clear Register (ICR)
/// Offset Address: 0x48
/// Total Reset Value:0x0   
#[bitfield(u32)]
pub struct InterruptClearReg {
    /// Interrupt Clear (ICR).
    ///Clear Interrupts.This register is set if any of the interrupts below are active.
    /// A read clears the ssi_txo_intr, ssi_rxu_intr, ssi_rxo_intr, and the ssi_mst_intr interrupts
    /// Writing to this register has no effect.
    #[bit(0, rw)]
    pub interrupt_clear: bool,
    /// Reserved.
    #[bits(1..=31, r)]
    pub _reserved: u31,
}

/// DMA Control Register (DMACR)
/// Offset Address: 0x4c
/// Total Reset Value:0x0
#[bitfield(u32)]
pub struct DmaControlReg {
    /// Receive DMA Enable.
    /// Transmit DMA Enable (RDMAE).
    /// This bit enables/disables the receive FIFO DMA channel.
    /// Values:
    /// - 0x1 (ENABLED): Receive DMA channel is enabled
    /// - 0x0 (DISABLED): Receive DMA channel is disabled
    // FIXME: access is `Varies`
    #[bit(0, rw)]
    pub receive_dma_enable: Enable,
    /// Transmit DMA Enable (TDMAE).
    /// This bit enables/disables the transmit FIFO DMA channel.
    /// Values:
    /// - 0x1 (ENABLED): Transmit DMA channel is enabled
    /// - 0x0 (DISABLED): Transmit DMA channel is disabled
    // FIXME: access is `Varies`
    #[bit(1, rw)]
    pub transmit_dma_enable: Enable,
    /// Internal DMA Enable (IDMAE).
    /// This bit should be enabled only when CTRLR0.FRF = 0 (Motorola SPI) and CTRLR0.SPI_FRF > 0.
    // FIXME: access is `Varies`
    #[bit(2, rw)]
    pub internal_dma_enable: Enable,
    /// AXI Transfer Width for DMA transfer mapped to arsize/awsize.
    /// This value must be less than or equal to SSIC_AXI_DW.
    /// Values:
    /// 0x0: 1 byte
    /// 0x1: 2 bytes
    /// 0x2: 4 bytes
    /// 0x3: 8 bytes
    /// Note: When SSIC_AXI_DW is set to 32 bits, if user programs this field to 0x8(3 bytes). SPI will use 4 bytes as transfer size for the AXI transfers.
    // FIXME: access is `Varies`
    #[bits(3..=4, rw)]
    pub axi_transfer_width: u2,
    /// Reserved.
    #[bit(5, r)]
    pub _reserved_5: u1,
    /// Address Increment (AINC).
    /// Indicates whether to increment the AXI address on every transfer. 1 = Increment 0 = No Change
    /// Note: Increment aligns the address to the next DMACR.ATW boundary
    // FIXME: access is `Varies`
    #[bit(6, rw)]
    pub address_increment: Enable,

    /// Reserved.
    #[bit(7, r)]
    pub _reserved_7: u1,
    /// AXI arcache/awcache signal value (ACACHE).
    // FIXME: access is `Varies`
    #[bits(8..=11, rw)]
    /// AXI arprot/awprot signal value (APROT).
    // FIXME: access is `Varies`
    pub axi_cache: u4,
    #[bits(12..=14, rw)]
    pub axi_prot: u3,
    /// AXI awid/arid signal value (AID).
    // FIXME: access is `Varies`
    // TODO: the field is x:15
    #[bits(15..=16, rw)]
    pub axi_id: u2,
    /// Reserved.
    // TODO: the field is 31:y
    #[bits(17..=31, r)]
    _reserved: u15,
}

/// DMA Transmit Data Level Register / AXI Write Length Register (DMATDLR_AXI_AWLEN)
/// Offset Address: 0x50
/// Total Reset Value:0x0
#[bitfield(u32)]
pub struct DmaTransmitDataLevelReg {
    /// Transmit Data Level(DMATDL).
    /// This bit field controls the level at which a DMA request is made by the transmit logic.
    /// It is equal to the watermark level; that is, the dma_tx_req signal is generated when the number of valid data entries in the transmit FIFO is equal to or below this field value, and TDMAE = 1.
    // TODO: the field is x:0
    #[bits(0..=15, rw)]
    pub transmit_data_level: u16,
    /// Reserved.
    // TODO: the field is 31:y
    #[bits(16..=31, r)]
    pub _reserved: u16,
}

/// Destination Burst Length Register (AXIAWLEN)
/// Offset Address: 0x50
/// Total Reset Value:0x00000700
///
#[bitfield(u32)]
pub struct DestinationBurstLengthReg {
    /// Reserved.
    #[bits(0..=7, r)]
    pub _reserved_0: u8,
    /// Destination Burst Length (AWLEN).
    // TODO: the field is x:8
    #[bits(8..=15, rw)]
    pub destination_burst_length: u8, /* Reset value is 0x07 */
    /// Reserved.
    // TODO : the field is 31:y
    #[bits(16..=31, r)]
    pub _reserved_1: u16,
}
/// DMA Receive Data Level Register (DMARDLR)
/// Offset Address: 0x54
/// Total Reset Value:0x0
#[bitfield(u32)]
pub struct DmaReceiveDataLevelReg {
    /// Receive Data Level (DMARDL).
    /// This bit field controls the level at which a DMA request is made by the receive logic.
    /// The watermark level = DMARDL+1; that is,dma_rx_req is generated when the number of valid data entries in the receive FIFO is equal to or above this field value + 1, and RDMAE=1.
    // TODO: the field is x:0
    #[bits(0..=15, rw)]
    pub receive_data_level: u16,
    /// Reserved.
    // TODO: the field is 31:y
    #[bits(16..=31, r)]
    pub _reserved: u16,
}

/// Source Burst Length Register (AXIARLEN)
/// Offset Address: 0x54
/// Total Reset Value:0x00000700
#[bitfield(u32)]
pub struct SourceBurstLengthReg {
    /// Reserved.
    #[bits(0..=7, r)]
    pub _reserved_0: u8,
    /// Source Burst Length (ARLEN).
    // TODO: the field is x:8
    #[bits(8..=15, rw)]
    pub source_burst_length: u8, /* Reset value is 0x07 */
    /// Reserved.
    // TODO : the field is 31:y
    #[bits(16..=31, r)]
    pub _reserved_1: u16,
}

/// Identification Register (IDR)
/// Offset Address: 0x58
/// Total Reset Value:0xha1b2c3d5
#[bitfield(u32)]
pub struct IdentificationReg {
    /// Identification Code (IDCODE).
    /// The register contains the peripheral's identification code, which is written into the register at configuration time using CoreConsultant.
    #[bits(0..=31, r)]
    pub identification_code: u32, /* Reset Value = 0xha1b2c3d5 */
}

/// Component Version Register (SSI_VERSION_ID)
/// Offset Address: 0x5c
/// Total Reset Value:0xh3130332a
#[bitfield(u32)]
pub struct ComponentVersionReg {
    /// Component Version (SSIC_COMP_VERSION).
    /// Contains the hex representation of the Synopsys component version. Consists of ASCII value for each number in the version, followed by . For example 31_30_33_2A represents the version 1.03.
    #[bits(0..=31, r)]
    pub component_version: u32,
}

/// Data Register (DR[0])
/// Offset Address 0x60 +i*0x4
/// Total Reset Value:0x0
#[bitfield(u32)]
pub struct DataReg {
    /// Data Register (DR).
    /// When writing to this register, you must right-justify the data. Read data are automatically right-justified.
    /// Read = Receive FIFO buffer
    /// Write = Transmit FIFO buffer.
    #[bits(0..=31, rw)]
    pub data: u32,
}

/// Control Register (SSI_CTRL)
/// Register Offset Address:0x68
/// Total Reset Value:0x00004000
// FIXME: This register lack documentation in the TRM
#[bitfield(u32)]
pub struct ControlReg {
    #[bit(0, rw)]
    pub ssi0_xip_en: Enable,
    // FIXME: 1 is actually missing in TRM
    #[bits(1..=3, r)]
    pub _reserved_1_3: u3,
    #[bit(4, r)]
    pub ssi0_ssi_sleep: bool,
    #[bits(5..=6, r)]
    pub ssi0_spi_mode: u2,
    #[bit(7, r)]
    pub ssi1_ssi_sleep: bool,
    #[bits(8..=9, r)]
    pub ssi1_spi_mode: u2,
    #[bit(10, r)]
    pub ssi2_ssi_sleep: bool,
    // FIXEME: in the TRM it is 17:13, need double check
    #[bits(11..=12, r)]
    pub ssi2_spi_mode: u2,

    /// rxds delay line number
    /// bit[0]:0:negedge 1:posedge
    /// bit[3:1]:delay number
    // FIXEME: in the TRM it is 17:13, need double check
    #[bits(13..=16, rw)]
    pub rxds_delay_num: u4,
}
/// RX Sample Delay Register (RX_SAMPLE_DLY)
/// Offset Address: 0xf0
/// Total Reset Value:0x0
#[bitfield(u32)]
pub struct RxSampleDelayReg {
    /// Receive Data (rxd) Sample Delay.
    /// This register is used to delay the sample of the rxd input port.
    /// Each value represents a single ssi_clk delay on the sample of rxd.
    /// Note; If this register is programmed with a value that exceeds the depth of the internal shift registers (SSIC_RX_DLY_SR_DEPTH) zero delay will be applied to the rxd sample.
    #[bits(0..=7, rw)]
    pub rx_sample_delay: u8,
    /// Reserved.
    #[bits(8..=15, r)]
    pub _reserved_8_15: u8,
    /// Receive Data (rxd) Sampling Edge (SE).
    /// This register is used to decide the sampling edge for RXD signal with ssi_clk.
    /// Then this bit is set to 1 then negative edge of ssi_clk will be used to sample the incoming data, otherwise positive edge will be used for sampling
    // FIXME: access is `Varies`
    #[bit(16, rw)]
    pub rx_sampling_edge: bool,
    /// Reserved.
    #[bits(17..=31, r)]
    pub _reserved_17_31: u15,
}

/// SPI Control Register 0 (SPI_CTRLR0)
/// Offset Address: 0xf4
/// Total Reset Value:0x0
#[bitfield(u32)]
pub struct SpiControlReg0 {
    /// Address and instruction transfer format (TRANS_TYPE).
    /// Selects whether the controller will transmit instruction/address either in Standard SPI mode or the SPI mode selected in CTRLR0.SPI_FRF field.
    /// Values:
    /// - 0x0 (TT0): Instruction and Address will be sent in Standard SPI Mode.
    /// - 0x1 (TT1): Instruction will be sent in Standard SPI Mode and Address will be sent in the mode specified by CTRLR0.SPI_FRF.
    /// - 0x2 (TT2): Both Instruction and Address will be sent in the mode specified by SPI_FRF.
    /// - 0x3 (TT3): Reserved.
    #[bits(0..=1, rw)]
    pub trans_type: u2,
    /// Address Length (ADDR_L).
    /// This bit defines Length of Address to be transmitted.
    /// Only valid when ADDRLEN = 11
    /// Values:
    /// 0x0 (ADDR_WIDTH_0_BITS): 0-bit Address Width
    /// 0x1 (ADDR_WIDTH_4_BITS): 4-bit Address Width  
    /// 0x2 (ADDR_WIDTH_8_BITS): 8-bit Address Width
    /// 0x3 (ADDR_WIDTH_12_BITS): 12-bit Address Width
    /// 0x4 (ADDR_WIDTH_16_BITS): 16-bit Address Width
    /// 0x5 (ADDR_WIDTH_20_BITS): 20-bit Address Width
    /// 0x6 (ADDR_WIDTH_24_BITS): 24-bit Address Width
    /// 0x7 (ADDR_WIDTH_28_BITS): 28-bit Address Width
    /// 0x8 (ADDR_WIDTH_32_BITS): 32-bit Address Width
    /// 0x9 (ADDR_WIDTH_36_BITS): 36-bit Address Width
    /// 0xa (ADDR_WIDTH_40_BITS): 40-bit Address Width
    /// 0xb (ADDR_WIDTH_44_BITS): 44-bit Address Width
    /// 0xc (ADDR_WIDTH_48_BITS): 48-bit Address Width
    /// 0xd (ADDR_WIDTH_52_BITS): 52-bit Address Width
    /// 0xe (ADDR_WIDTH_56_BITS): 56-bit Address Width
    /// 0xf (ADDR_WIDTH_60_BITS): 60-bit Address Width
    #[bits(2..=5, rw)]
    pub addr_len: u4,
    /// Reserved.
    #[bit(6, r)]
    pub _reserved_6: u1,
    /// Mode bits enable in XIP mode (XIP_MD_BIT_EN).
    /// If this bit is set to 1, then in XIP mode of operation the controller will insert mode bits after the address phase.
    /// These bits are set in register XIP_MODE_BITS register.
    /// The length of mode bits is always set to 8 bits.
    // FIXME: access is `Varies`
    #[bit(7, rw)]
    pub xip_mode_bits_enable: Enable,
    /// Instruction Length (INST_L).
    /// Dual/Quad/Octal mode instruction length in bits.
    /// Values:
    /// 0x0 (INST_L_0_BITS): 0-bit (No Instruction)
    /// 0x1 (INST_L_4_BITS): 4-bit Instruction
    /// 0x2 (INST_L_8_BITS): 8-bit Instruction
    /// 0x3 (INST_L_16_BITS): 16-bit Instruction
    ///
    #[bits(8..=9, rw)]
    pub inst_len: u2, /* Reset value is 0x2 */
    /// Reserved.
    #[bits(10..=10, r)]
    pub _reserved_10: bool,
    /// Wait cycles in Dual/Quad/Octal mode between control frames transmit and data reception. (WAIT_CYCLES)
    /// Specified as number of SPI clock cycles.
    #[bits(11..=15, rw)]
    pub wait_cycles: u5,
    /// SPI DDR Enable (SPI_DDR_EN). This will enable Dual-data rate transfers in Dual/Quad/Octal frame formats of SPI
    // FIXME: access is `Varies`
    #[bit(16, rw)]
    pub spi_ddr_en: Enable,
    /// Instruction DDR Enable (INST_DDR_EN). This will enable Dual-data rate transfer for Instruction phase.
    // FIXME: access is `Varies`
    #[bit(17, rw)]
    pub inst_ddr_en: Enable,
    /// Read data strobe enable bit.
    /// Once this bit is set to 1 the controller will use Read data strobe (rxds) to capture read data in DDR mode.

    // FIXME: access is `Varies`
    #[bit(18, rw)]
    pub spi_rxds_en: Enable,
    ///Fix DFS for XIP transfers (XIP_DFS_HC).
    /// If this bit is set to 1 then data frame size for XIP transfers will be fixed to the programmed value in CTRLR0.DFS.
    /// The number of data frames to fetch will be determined by HSIZE and HBURST signals.
    /// If this bit is set to 0 then data frame size and number of data frames to fetch will be determined by HSIZE and HBURST signals
    // FIXME: access is `Varies`
    #[bit(19, rw)]
    pub dfs_for_xip_transfer_fixed: bool,

    /// XIP instruction enable bit (XIP_INST_EN).
    /// If this bit is set to 1 then XIP transfers will also have instruction phase.
    /// The instruction op-codes will be chosen from XIP_INCR_INST or XIP_WRAP_INST registers bases on AHB transfer type.
    // FIXME: access is `Varies`
    #[bit(20, rw)]
    pub xip_inst_enabled: Enable,

    /// Enable continuous transfer in XIP mode (SSIC_XIP_CONT_XFER_EN).
    /// If this bit is set to 1 then continuous transfer mode in XIP will be enabled, in this mode the controller will keep slave selected until a non-XIP transfer is detected on the AHB interface.
    // FIXME: access is `Varies`
    #[bit(21, rw)]
    pub xip_continuous_transfer_enabled: Enable,

    /// Reserved.
    #[bits(22..=23, r)]
    pub _reserved_22_23: u2,

    /// SPI data mask enable bit (SPI_DM_EN).
    /// When this bit is enabled, the txd_dmsignal is used to mask the data on the txd data line.
    /// This bit is enabled only when the SSIC_DM_EN parameter is set to 1.
    // FIXME: access is `Varies`
    #[bit(24, rw)]
    pub spi_data_mask_enabled: Enable,

    ///Enable rxds signaling during address and command phase of Hypebus transfer.(SPI_RXDS_SIG_EN)
    /// This bit enables rxds signaling by Hyperbus slave devices during Command-Address (CA) phase.
    /// If the rxds signal is set to 1 during the CA phase of transfer, the controller transmits (2*SPI_CTRLR0.WAIT_CYCLES-1) wait cycles after the address phase is complete.
    // FIXME: access is `Varies`
    #[bit(25, rw)]
    pub spi_rxds_signal_enabled: Enable,

    /// XIP Mode bits length (XIP_MBL).
    /// Sets the length of mode bits in XIP mode of operation.
    /// These bits are valid only when SPI_CTRLR0.XIP_MD_BIT_EN is set to 1.
    /// Values:
    /// - 0x0 (MBL_2): Mode bits length equal to 2
    /// - 0x1 (MBL_4): Mode bits length equal to 4
    /// - 0x2 (MBL_6): Mode bits length equal to 8
    /// - 0x3 (MBL_8): Mode bits length equal to 16

    // FIXME: access is `Varies`
    #[bits(26..=27, rw)]
    pub xip_mode_bits_length: u2,

    /// Reserved.
    #[bit(28, r)]
    pub _reserved_28: u1,

    /// Enables XIP pre-fetch functionality in the controller (XIP_PREFETCH_EN).
    /// Once enabled the controller will pre-fetch data frames from next contigous location, to reduce the latency for the upcoming contiguous transfer.
    /// If the next XIP request is not contigous then pre-fetched bits will be discarded.
    // FIXME: access is `Varies`
    #[bit(29, rw)]
    pub xip_prefetch_enabled: Enable,

    /// Enables clock stretching capability in SPI transfers (CLK_STRETCH_EN).
    /// In case of write, if the FIFO becomes empty the controller will stretch the clock until FIFO has enough data to continue the transfer.In case of read, if the receive FIFO becomes full the controller will stop the clock until data has been read from the FIFO.
    // FIXME: access is `Varies`
    #[bit(30, rw)]
    pub clock_stretching_enabled: Enable,
    /// Reserved.
    #[bit(31, r)]
    pub _reserved_31: bool,
}

/// DDR Drive Edge Register (DDR_DRIVE_EDGE)
/// Offset Address: 0xf8
/// Total Reset Value:0x0
#[bitfield(u32)]
pub struct DdrDriveEdgeReg {
    /// TXD Drive Edge.
    /// Decided the driving edge of transmit data.The maximum value of this register is = (BAUDR/2) -1.
    #[bits(0..=7, rw)]
    pub drive_edge: u8,
    /// Reserved.
    #[bits(8..=31, r)]
    pub _reserved: u24,
}

/// SPI Control Register 1 (SPI_CTRLR1)
/// Offset Address: 0x118
/// Total Reset Value:0x0
#[bitfield(u32)]
pub struct SpiControlReg1 {
    /// SPI Dynamic Wait states field (DYN_WS)
    ///This field is used to set the value for wait states which will be introduced when SPI slave sends BUSY status to the Master.The programmed value of wait States will be introduced before checking status again.Number of wait states = DYN_WS+1

    #[bits(0..=2, rw)]
    pub spi_dynamic_wait_states: u3,

    /// Reserved.
    #[bits(3..=7, r)]
    pub _reserved_3_7: u5,
    ///Maximum wait cycles allowed per transaction (MAX_WS).This field indicate, up to how many times SPI slave could insert the wait states.
    /// The internal counter is incremented every time when the controller checks for the status from the slave and receives wait response.

    #[bits(8..=11, rw)]
    pub max_ws: u4,

    /// Reserved.
    #[bits(12..=31, r)]
    pub _reserved_12_32: u20,
}

/// SPI Transmit Error Clear Register (SPITECR)
/// Offset Address: 0x11c
/// Total Reset Value:0x0
#[bitfield(u32)]
pub struct SpiTransmitErrorClearReg {
    /// Clear SPI Transmit Error interrupt (SPITECR).
    /// This register will reflect the status of the interrupt.A read from this register clears the ssi_spite_intr interrupt. Writing to this register has no effect..
    #[bit(0, rw)]
    pub spi_transmit_error_clear: bool,
    /// Reserved.
    #[bits(1..=31, r)]
    pub _reserved: u31,
}

/// SPI Device Register (SPIDR)
/// Offset Address: 0x120
/// Total Reset Value:0x0
#[bitfield(u32)]
pub struct SpiDeviceReg {
    /// SPI Instruction code (SPI_INST).
    /// This instruction code will be used for SPI operations.
    #[bits(0..=15, rw)]
    pub spi_device: u16,
    /// Reserved.
    #[bits(16..=31, r)]
    pub _reserved: u16,
}

/// SPI Address Register (SPIAR)
/// Offset Address: 0x124
/// Total Reset Value:0x0
#[bitfield(u32)]
pub struct SpiAddressReg {
    /// SPI Device Address Register (SDAR).
    /// This address will be used during read/write on SPI interface for DMA transfer.
    #[bits(0..=31, rw)]
    pub spi_address: u32,
}

/// AXI Address Register 0 (AXIAR0)
/// Offset Address: 0x128
/// Total Reset Value:0x0
#[bitfield(u32)]
pub struct AxiAddressReg0 {
    /// LSB for AXI address for DMA operation.
    #[bits(0..=31, rw)]
    pub axi_address: u32,
}

/// AXI Address Register 1 (AXIAR1)
/// Offset Address: 0x12c
/// Total Reset Value:0x0
#[bitfield(u32)]
pub struct AxiAddressReg1 {
    /// MSB for source address for DMA operation.
    // TODO: the field is x:0
    #[bits(0..=15, rw)]
    pub axi_address: u16,

    /// Reserved.
    // TODO: the field is 31:y
    #[bits(16..=31, r)]
    pub _reserved: u16,
}

/// AXI Error Clear Register (AXIECR)
/// Offset Address: 0x130
/// Total Reset Value:0x0
#[bitfield(u32)]
pub struct AxiErrorClearReg {
    /// Clear AXI Error Interrupt (AXIECR).
    /// This register will reflect the status of the  interrupt. A read from this register clears the ssi_axie_intr interrupt. Writing to this register has no effect.
    #[bit(0, rw)]
    pub axi_error_clear: bool,
    /// Reserved.
    #[bits(1..=31, r)]
    pub _reserved: u31,
}

/// Done Clear Register (DONECR)
/// Offset Address: 0x134
/// Total Reset Value:0x0
#[bitfield(u32)]
pub struct DoneClearReg {
    /// Clear Transfer Done Interrupt (DONECR).
    /// This register will reflect the status of the interrupt. A read from this register clears the ssi_done_intr 0x0 interrupt.Writing to this register has no effect.
    #[bit(0, rw)]
    pub done_clear: bool,
    /// Reserved.
    #[bits(1..=31, r)]
    pub _reserved: u31,
}

/// SPI Register Block
///
/// Represents the memory-mapped registers for the SPI peripheral.
/// Each field corresponds to a specific hardware register as described in the K230 manual.
#[repr(C)]
pub struct RegisterBlock {
    /// Control Register 0.
    /// Contains basic SPI configuration settings.
    ctrlr0: RW<ControlReg0>,
    /// Control Register 1.
    /// Contains additional SPI configuration settings.
    ctrlr1: RW<ControlReg1>,
    /// SSI Enable Register.
    /// Controls the enabling/disabling of the SSI interface.
    ssienr: RW<SsiEnableReg>,
    /// Microwire Control Register.
    /// Controls the Microwire interface operations.
    mwcr: RW<MicrowireControlReg>,
    /// Slave Enable Register.
    /// Controls which slave devices are selected.
    ser: RW<SlaveEnableReg>,
    /// Baud Rate Select Register.
    /// Sets the SPI communication speed.
    baudr: RW<BaudRateSelectReg>,
    /// Transmit FIFO Threshold Level Register.
    /// Sets the threshold for TX FIFO interrupts.
    txftlr: RW<TransmitFifoThresholdLevelReg>,
    /// Receive FIFO Threshold Level Register.
    /// Sets the threshold for RX FIFO interrupts.
    rxftlr: RW<ReceiveFifoThresholdLevelReg>,
    /// Transmit FIFO Level Register.
    /// Indicates current TX FIFO fill level.
    txflr: RW<TransmitFifoLevelReg>,
    /// Receive FIFO Level Register.
    /// Indicates current RX FIFO fill level.
    rxflr: RW<ReceiveFifoLevelReg>,
    /// Status Register.
    /// Contains current SPI status information.
    sr: RW<StatusReg>,
    /// Interrupt Mask Register.
    /// Controls which interrupts are enabled.
    imr: RW<InterruptMaskReg>,
    /// Interrupt Status Register.
    /// Shows current interrupt status.
    isr: RW<InterruptStatusReg>,
    /// Raw Interrupt Status Register.
    /// Shows unmasked interrupt status.
    risr: RW<RawInterruptStatusReg>,
    /// Transmit FIFO Error Interrupt Clear Register.
    /// Clears TX FIFO error interrupts.
    txeicr: RW<TransmitFifoErrorInterruptClearReg>,
    /// Receive FIFO Overflow Interrupt Clear Register.
    /// Clears RX FIFO overflow interrupts.
    rxoicr: RW<ReceiveFifoOverflowInterruptClearReg>,
    /// Receive FIFO Underflow Interrupt Clear Register.
    /// Clears RX FIFO underflow interrupts.
    rxuicr: RW<ReceiveFifoUnderflowInterruptClearReg>,
    /// Multi-Master Interrupt Clear Register.
    /// Clears multi-master conflict interrupts.
    msticr: RW<MultiMasterInterruptClearReg>,
    /// Interrupt Clear Register.
    /// Clears all interrupts.
    icr: RW<InterruptClearReg>,
    /// DMA Control Register.
    /// Controls DMA operations.
    dmacr: RW<DmaControlReg>,
    /// DMA Transmit Data Level Register.
    /// Sets DMA TX data threshold.
    /// Destination Burst Length Register.
    /// Sets AXI destination burst length.
    dmatdlr_axiawlen: RW<DmaTransmitDataLevelReg>,
    /// DMA Receive Data Level.
    /// Shows current DMA RX data level.
    /// Source Burst Length.
    /// Sets AXI source burst length.
    dmardlr_axiarlen: RW<DmaReceiveDataLevelReg>,
    /// Identification Register.
    /// Contains peripheral identification information.
    idr: RW<IdentificationReg>,
    /// Component version Register.
    /// Shows hardware component version.
    ssi_version_id: RW<ComponentVersionReg>,
    /// Data Register.
    /// Array of data registers for SPI communication.
    // Control Register.
    /// Contains SSI control settings.
    dr_ssi_ctrl: [RW<DataReg>; 36],
    /// RX Sample Delay Register.
    /// Controls RX sampling delay.
    rx_sample_delay: RW<RxSampleDelayReg>,
    /// SPI Control 0 Register.
    /// Contains primary SPI control settings.
    spi_ctrlr0: RW<SpiControlReg0>,
    /// Transmit Drive Edge Register.
    /// Controls TX signal edge timing.
    ddr_drive_edge: RW<DdrDriveEdgeReg>,
    _reversed0: [u8; 0x1C],
    /// SPI Control 1 register.
    /// Contains secondary SPI control settings.
    spi_ctrlr1: RW<SpiControlReg1>,
    /// SPI Transmit Error Interrupt Clear Register.
    /// Clears SPI TX error interrupts.
    spitecr: RW<SpiTransmitErrorClearReg>,
    /// SPI Device Register.
    /// Controls SPI device settings.
    spidr: RW<SpiDeviceReg>,
    /// SPI Device Address Register.
    /// Sets SPI device addressing.
    spiar: RW<SpiAddressReg>,
    /// AXI Address Register 0.
    /// Contains primary AXI address settings.
    axiar0: RW<AxiAddressReg0>,
    /// AXI Address Register 1.
    /// Contains secondary AXI address settings.
    axiar1: RW<AxiAddressReg1>,
    /// AXI Master Error Interrupt Clear Register.
    /// Clears AXI master error interrupts.
    axiecr: RW<AxiErrorClearReg>,
    /// Transfer Done Clear Interrupt Clear Register.
    /// Clears transfer completion interrupts.
    donecr: RW<DoneClearReg>,
}
#[cfg(test)]
mod tests {
    use super::*;
    use core::mem::{offset_of, size_of};

    #[test]
    fn test_register_offsets() {
        assert_eq!(offset_of!(RegisterBlock, ctrlr0), 0x00);
        assert_eq!(offset_of!(RegisterBlock, ctrlr1), 0x04);
        assert_eq!(offset_of!(RegisterBlock, ssienr), 0x08);
        assert_eq!(offset_of!(RegisterBlock, mwcr), 0x0C);
        assert_eq!(offset_of!(RegisterBlock, ser), 0x10);
        assert_eq!(offset_of!(RegisterBlock, baudr), 0x14);
        assert_eq!(offset_of!(RegisterBlock, txftlr), 0x18);
        assert_eq!(offset_of!(RegisterBlock, rxftlr), 0x1C);
        assert_eq!(offset_of!(RegisterBlock, txflr), 0x20);
        assert_eq!(offset_of!(RegisterBlock, rxflr), 0x24);
        assert_eq!(offset_of!(RegisterBlock, sr), 0x28);
        assert_eq!(offset_of!(RegisterBlock, imr), 0x2C);
        assert_eq!(offset_of!(RegisterBlock, isr), 0x30);
        assert_eq!(offset_of!(RegisterBlock, risr), 0x34);
        assert_eq!(offset_of!(RegisterBlock, txeicr), 0x38);
        assert_eq!(offset_of!(RegisterBlock, rxoicr), 0x3C);
        assert_eq!(offset_of!(RegisterBlock, rxuicr), 0x40);
        assert_eq!(offset_of!(RegisterBlock, msticr), 0x44);
        assert_eq!(offset_of!(RegisterBlock, icr), 0x48);
        assert_eq!(offset_of!(RegisterBlock, dmacr), 0x4C);
        assert_eq!(offset_of!(RegisterBlock, dmatdlr_axiawlen), 0x50);
        assert_eq!(offset_of!(RegisterBlock, dmardlr_axiarlen), 0x54);
        assert_eq!(offset_of!(RegisterBlock, idr), 0x58);
        assert_eq!(offset_of!(RegisterBlock, ssi_version_id), 0x5C);
        assert_eq!(offset_of!(RegisterBlock, dr_ssi_ctrl), 0x60);
        assert_eq!(offset_of!(RegisterBlock, rx_sample_delay), 0xF0);
        assert_eq!(offset_of!(RegisterBlock, spi_ctrlr0), 0xF4);
        assert_eq!(offset_of!(RegisterBlock, ddr_drive_edge), 0xF8);
        assert_eq!(offset_of!(RegisterBlock, spi_ctrlr1), 0x118);
        assert_eq!(offset_of!(RegisterBlock, spitecr), 0x11C);
        assert_eq!(offset_of!(RegisterBlock, spidr), 0x120);
        assert_eq!(offset_of!(RegisterBlock, spiar), 0x124);
        assert_eq!(offset_of!(RegisterBlock, axiar0), 0x128);
        assert_eq!(offset_of!(RegisterBlock, axiar1), 0x12C);
        assert_eq!(offset_of!(RegisterBlock, axiecr), 0x130);
        assert_eq!(offset_of!(RegisterBlock, donecr), 0x134);
    }

    #[test]
    fn test_bitfield_sizes() {
        assert_eq!(size_of::<ControlReg0>(), 4);
        assert_eq!(size_of::<ControlReg1>(), 4);
        assert_eq!(size_of::<SsiEnableReg>(), 4);
        assert_eq!(size_of::<MicrowireControlReg>(), 4);
        assert_eq!(size_of::<SlaveEnableReg>(), 4);
        assert_eq!(size_of::<BaudRateSelectReg>(), 4);
        assert_eq!(size_of::<TransmitFifoThresholdLevelReg>(), 4);
        assert_eq!(size_of::<ReceiveFifoThresholdLevelReg>(), 4);
        assert_eq!(size_of::<TransmitFifoLevelReg>(), 4);
        assert_eq!(size_of::<ReceiveFifoLevelReg>(), 4);
        assert_eq!(size_of::<StatusReg>(), 4);
        assert_eq!(size_of::<InterruptMaskReg>(), 4);
        assert_eq!(size_of::<InterruptStatusReg>(), 4);
        assert_eq!(size_of::<RawInterruptStatusReg>(), 4);
        assert_eq!(size_of::<DmaControlReg>(), 4);
        assert_eq!(size_of::<DmaTransmitDataLevelReg>(), 4);
        assert_eq!(size_of::<DmaReceiveDataLevelReg>(), 4);
        assert_eq!(size_of::<RxSampleDelayReg>(), 4);
        assert_eq!(size_of::<SpiControlReg0>(), 4);
        assert_eq!(size_of::<DdrDriveEdgeReg>(), 4);
        assert_eq!(size_of::<SpiControlReg1>(), 4);
        assert_eq!(size_of::<SpiTransmitErrorClearReg>(), 4);
        assert_eq!(size_of::<SpiDeviceReg>(), 4);
        assert_eq!(size_of::<SpiAddressReg>(), 4);
        assert_eq!(size_of::<AxiAddressReg0>(), 4);
        assert_eq!(size_of::<AxiAddressReg1>(), 4);
        assert_eq!(size_of::<AxiErrorClearReg>(), 4);
        assert_eq!(size_of::<DoneClearReg>(), 4);
    }
}
