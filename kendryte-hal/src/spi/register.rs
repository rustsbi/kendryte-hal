use volatile_register::RW;

/// SPI Register Block.
///
/// This structure represents the memory-mapped registers of an SPI peripheral.
/// Each field corresponds to a specific register or group of registers.
/// The registers are used to control and monitor SPI operations.
#[repr(C)]
pub struct RegisterBlock {
    /// Control Register 0.
    /// Contains basic SPI configuration settings.
    ctrlr0: RW<u32>,
    /// Control Register 1.
    /// Contains additional SPI configuration settings.
    ctrlr1: RW<u32>,
    /// SSI Enable Register.
    /// Controls the enabling/disabling of the SSI interface.
    ssienr: RW<u32>,
    /// Microwire Control Register.
    /// Controls the Microwire interface operations.
    mwcr: RW<u32>,
    /// Slave Enable Register.
    /// Controls which slave devices are selected.
    ser: RW<u32>,
    /// Baud Rate Select Register.
    /// Sets the SPI communication speed.
    baudr: RW<u32>,
    /// Transmit FIFO Threshold Level Register.
    /// Sets the threshold for TX FIFO interrupts.
    txftlr: RW<u32>,
    /// Receive FIFO Threshold Level Register.
    /// Sets the threshold for RX FIFO interrupts.
    rxftlr: RW<u32>,
    /// Transmit FIFO Level Register.
    /// Indicates current TX FIFO fill level.
    txflr: RW<u32>,
    /// Receive FIFO Level Register.
    /// Indicates current RX FIFO fill level.
    rxflr: RW<u32>,
    /// Status Register.
    /// Contains current SPI status information.
    sr: RW<u32>,
    /// Interrupt Mask Register.
    /// Controls which interrupts are enabled.
    imr: RW<u32>,
    /// Interrupt Status Register.
    /// Shows current interrupt status.
    isr: RW<u32>,
    /// Raw Interrupt Status Register.
    /// Shows unmasked interrupt status.
    risr: RW<u32>,
    /// Transmit FIFO Error Interrupt Clear Register.
    /// Clears TX FIFO error interrupts.
    txeicr: RW<u32>,
    /// Receive FIFO Overflow Interrupt Clear Register.
    /// Clears RX FIFO overflow interrupts.
    rxoicr: RW<u32>,
    /// Receive FIFO Underflow Interrupt Clear Register.
    /// Clears RX FIFO underflow interrupts.
    rxuicr: RW<u32>,
    /// Multi-Master Interrupt Clear Register.
    /// Clears multi-master conflict interrupts.
    msticr: RW<u32>,
    /// Interrupt Clear Register.
    /// Clears all interrupts.
    icr: RW<u32>,
    /// DMA Control Register.
    /// Controls DMA operations.
    dmacr: RW<u32>,
    /// DMA Transmit Data Level Register.
    /// Sets DMA TX data threshold.
    /// Destination Burst Length Register.
    /// Sets AXI destination burst length.
    dmatdlr_axiawlen: RW<u32>,
    /// DMA Receive Data Level.
    /// Shows current DMA RX data level.
    /// Source Burst Length.
    /// Sets AXI source burst length.
    dmardlr_axiarlen: RW<u32>,
    /// Identification Register.
    /// Contains peripheral identification information.
    idr: RW<u32>,
    /// Component version Register.
    /// Shows hardware component version.
    ssi_version_id: RW<u32>,
    /// Data Register.
    /// Array of data registers for SPI communication.
    // Control Register.
    /// Contains SSI control settings.
    dr_ssi_ctrl: [RW<u32>; 36],
    /// RX Sample Delay Register.
    /// Controls RX sampling delay.
    rx_sample_delay: RW<u32>,
    /// SPI Control 0 Register.
    /// Contains primary SPI control settings.
    spi_ctrlr0: RW<u32>,
    /// Transmit Drive Edge Register.
    /// Controls TX signal edge timing.
    ddr_drive_edge: RW<u32>,
    _reversed0: [u8; 0x1C],
    /// SPI Control 1 register.
    /// Contains secondary SPI control settings.
    spi_ctrlr1: RW<u32>,
    /// SPI Transmit Error Interrupt Clear Register.
    /// Clears SPI TX error interrupts.
    spitecr: RW<u32>,
    /// SPI Device Register.
    /// Controls SPI device settings.
    spidr: RW<u32>,
    /// SPI Device Address Register.
    /// Sets SPI device addressing.
    spiar: RW<u32>,
    /// AXI Address Register 0.
    /// Contains primary AXI address settings.
    axiar0: RW<u32>,
    /// AXI Address Register 1.
    /// Contains secondary AXI address settings.
    axiar1: RW<u32>,
    /// AXI Master Error Interrupt Clear Register.
    /// Clears AXI master error interrupts.
    axiecr: RW<u32>,
    /// Transfer Done Clear Interrupt Clear Register.
    /// Clears transfer completion interrupts.
    donecr: RW<u32>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::mem::offset_of;

    #[test]
    fn struct_register_block_offset() {
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
}
