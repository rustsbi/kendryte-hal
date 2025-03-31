use volatile_register::RW;

/// I2C Register Block.
///
/// This structure represents the memory-mapped registers of a I2C peripheral.
/// Each field corresponds to a specific register or group of registers.
#[repr(C)]
pub struct RegisterBlock {
    /// I2C Control Register.
    /// This register can be written only when the I2C controller is disabled, which corresponds to the IC_ENABLE[0] register being set to 0.
    /// Writes at other times have no effect.
    pub con: RW<u32>,
    /// I2C Target Address Register.
    /// This register stores the target I2C address for master mode operations.
    pub tar: RW<u32>,
    /// I2C Slave Address Register.
    /// This register holds the slave address when operating in slave mode.
    pub sar: RW<u32>,
    /// I2C High Speed Master Mode Code Address Register.
    /// This register contains the master code address for high speed mode.
    pub hs_maddr: RW<u32>,
    /// I2C Rx/Tx Data Buffer and Command Register.
    /// This is the register the CPU writes to when filling the TX FIFO and reads from when retrieving bytes from RX FIFO.
    pub data_cmd: RW<u32>,
    /// Standard Speed I2C Clock SCL High Count Register.
    /// This register controls the SCL clock high time for standard speed mode.
    /// Ultra-Fast Speed I2C Clock SCL High Count Register.
    /// This register controls the SCL clock high time for ultra-fast speed mode.
    pub ss_scl_hcnt_ufm_scl_hcnt: RW<u32>,
    /// Standard Speed I2C Clock SCL Low Count Register.
    /// This register controls the SCL clock low time for standard speed mode.
    /// Ultra-Fast Speed I2C Clock SCL Low Count Register.
    /// This register controls the SCL clock low time for ultra-fast speed mode.
    pub ss_scl_lcnt_ufm_scl_lcnt: RW<u32>,
    /// Fast Mode or Fast Mode Plus I2C Clock SCL High Count Register.
    /// This register controls the SCL clock high time for fast modes.
    /// Ultra-Fast Speed mode TBuf Idle Count Register.
    /// This register controls the bus idle time for ultra-fast speed mode.
    pub fs_scl_hcnt_ufm_tbuf_cnt: RW<u32>,
    /// Fast Mode or Fast Mode Plus I2C Clock SCL Low Count Register.
    /// This register controls the SCL clock low time for fast modes.
    pub fs_scl_lcnt: RW<u32>,
    /// High Speed I2C Clock SCL High Count Register.
    /// This register controls the SCL clock high time for high speed mode.
    pub hs_scl_hcnt: RW<u32>,
    /// High Speed I2C Clock SCL Low Count Register.
    /// This register controls the SCL clock low time for high speed mode.
    pub hs_scl_lcnt: RW<u32>,
    /// I2C Interrupt Status Register.
    /// Each bit in this register has a corresponding mask bit in the IC_INTR_MASK register.
    /// These bits are cleared by reading the matching interrupt clear register.
    /// The unmasked raw versions of these bits are available in the IC_RAW_INTR_STAT register.
    pub intr_stat: RW<u32>,
    /// I2C Interrupt Mask Register.
    /// These bits mask their corresponding interrupt status bits.
    /// This register is active low; a value of 0 masks the interrupt, whereas a value of 1 unmasks the interrupt.
    pub intr_mask: RW<u32>,
    /// I2C Raw Interrupt Status Register.
    /// Unlike the IC_INTR_STAT register, these bits are not masked so they always show the true status of the I2C controller.
    pub raw_intr_stat: RW<u32>,
    /// I2C Receive FIFO Threshold Register.
    /// This register controls the threshold level for receive FIFO operations.
    pub rx_tl: RW<u32>,
    /// I2C Transmit FIFO Threshold Register.
    /// This register controls the threshold level for transmit FIFO operations.
    pub tx_tl: RW<u32>,
    /// Clear Combined and Individual Interrupt Register.
    /// This register clears all active interrupts.
    pub clr_intr: RW<u32>,
    /// Clear RX_UNDER Interrupt Register.
    /// This register clears the RX_UNDER interrupt.
    pub clr_rx_under: RW<u32>,
    /// Clear RX_OVER Interrupt Register.
    /// This register clears the RX_OVER interrupt.
    pub clr_rx_over: RW<u32>,
    /// Clear TX_OVER Interrupt Register.
    /// This register clears the TX_OVER interrupt.
    pub clr_tx_over: RW<u32>,
    /// Clear RD_REQ Interrupt Register.
    /// This register clears the RD_REQ interrupt.
    pub clr_rd_req: RW<u32>,
    /// Clear TX_ABRT Interrupt Register.
    /// This register clears the TX_ABRT interrupt.
    pub clr_tx_abrt: RW<u32>,
    /// Clear RX_DONE Interrupt Register.
    /// This register clears the RX_DONE interrupt.
    pub clr_rx_done: RW<u32>,
    /// Clear ACTIVITY Interrupt Register.
    /// This register clears the ACTIVITY interrupt.
    pub clr_activity: RW<u32>,
    /// Clear STOP_DET Interrupt Register.
    /// This register clears the STOP_DET interrupt.
    pub clr_stop_det: RW<u32>,
    /// Clear START_DET Interrupt Register.
    /// This register clears the START_DET interrupt.
    pub clr_start_det: RW<u32>,
    /// Clear GEN_CALL Interrupt Register.
    /// This register clears the GEN_CALL interrupt.
    pub clr_gen_call: RW<u32>,
    /// I2C Enable Register.
    /// This register enables or disables the I2C controller.
    pub enable: RW<u32>,
    /// I2C Status Register.
    /// This is a read-only register used to indicate the current transfer status and FIFO status.
    /// The status register may be read at any time.
    /// None of the bits in this register request an interrupt.
    pub status: RW<u32>,
    /// I2C Transmit FIFO Level Register.
    /// This register contains the number of valid data entries in the transmit FIFO buffer.
    pub txflr: RW<u32>,
    /// I2C Receive FIFO Level Register.
    /// This register contains the number of valid data entries in the receive FIFO buffer.
    pub rxflr: RW<u32>,
    /// I2C SDA Hold Time Length Register.
    /// This register controls the SDA hold time.
    pub sda_hold: RW<u32>,
    /// I2C Transmit Abort Source Register.
    /// This register indicates the source of a transmission abort.
    pub tx_abrt_source: RW<u32>,
    /// Generate Slave Data NACK Register.
    /// The register is used to generate a NACK for the data part of a transfer when I2C controller is acting as a slave-receiver.
    pub slv_data_nack_only: RW<u32>,
    /// DMA Control Register.
    /// This register is only valid when I2C controller is configured with a set of DMA Controller interface signals (IC_HAS_DMA = 1).
    pub dma_cr: RW<u32>,
    /// DMA Transmit Data Level Register.
    /// This register is only valid when the I2C controller is configured with a set of DMA interface signals (IC_HAS_DMA = 1).
    pub dma_tdlr: RW<u32>,
    /// I2C Receive Data Level Register.
    /// This register is only valid when I2C controller is configured with a set of DMA interface signals (IC_HAS_DMA = 1).
    pub dma_rdlr: RW<u32>,
    /// I2C SDA Setup Register.
    /// Controls SDA to SCL rising edge delay for slave-transmitter read operations.
    pub sda_setup: RW<u32>,
    /// I2C ACK General Call Register.
    /// The register controls whether I2C controller responds with a ACK or NACK when it receives an I2C General Call address.
    pub ack_general_call: RW<u32>,
    /// I2C Enable Status Register.
    /// The register is used to report the I2C controller hardware status when the IC_ENABLE[0] register is set from 1 to 0;
    /// that is, when I2C controller is disabled.
    pub enable_status: RW<u32>,
    /// I2C SS, FS or FM+ spike suppression limit Register.
    /// This register controls spike suppression in various speed modes.
    /// I2C UFM spike suppression limit Register.
    /// This register controls spike suppression in Ultra-Fast mode.
    pub fs_spklen_ufm_spklen: RW<u32>,
    /// I2C HS spike suppression limit Register.
    /// This register controls spike suppression in High Speed mode.
    pub hs_spklen: RW<u32>,
    /// Clear RESTART_DET Interrupt Register.
    /// This register clears the RESTART_DET interrupt.
    pub clr_restart_det: RW<u32>,
    /// I2C SCL Stuck at Low Timeout Register.
    /// This register controls timeout detection for SCL stuck low condition.
    pub scl_stuck_at_low_timeout: RW<u32>,
    /// I2C SDA Stuck at Low Timeout Register.
    /// This register controls timeout detection for SDA stuck low condition.
    pub sda_stuck_at_low_timeout: RW<u32>,
    /// Clear SCL Stuck at Low Detect Interrupt Register.
    /// This register clears the SCL stuck low detection interrupt.
    pub clr_scl_stuck_det: RW<u32>,
    /// I2C Device-ID Register.
    /// This register contains device identification information.
    pub device_id: RW<u32>,
    /// SMBus Slave Clock Extend Timeout Register.
    /// This register controls slave clock extension timeout.
    pub smbus_clk_low_sext: RW<u32>,
    /// SMBus Master Clock Extend Timeout Register.
    /// This register controls master clock extension timeout.
    pub smbus_clk_low_mext: RW<u32>,
    /// SMBus Master THigh MAX Bus-idle count Register.
    /// This register controls bus idle detection timing.
    pub smbus_thigh_max_idle_count: RW<u32>,
    /// SMBUS Interrupt Status Register.
    /// This register shows the status of SMBus interrupts.
    pub smbus_intr_stat: RW<u32>,
    /// SMBus Interrupt Mask Register.
    /// This register controls which SMBus interrupts are enabled.
    pub smbus_intr_mask: RW<u32>,
    /// SMBus Raw Interrupt Status Register.
    /// This register shows the raw status of SMBus interrupts.
    pub smbus_raw_intr_stat: RW<u32>,
    /// SMBus Clear Interrupt Register.
    /// This register clears SMBus interrupts.
    pub clr_smbus_intr: RW<u32>,
    /// I2C Optional Slave Address Register.
    /// This register holds an optional slave address for SMBus mode.
    pub optional_sar: RW<u32>,
    /// SMBUS ARP UDID LSB Register.
    /// This register holds the least significant bits of the UDID for SMBus ARP.
    pub smbus_udid_lsb: RW<u32>,
    _reversed0: [u8; 0x14],
    /// Component Parameter Register 1.
    /// This register contains component configuration parameters.
    pub comp_param_1: RW<u32>,
    /// I2C Component Version Register.
    /// This register contains the component version number.
    pub comp_version: RW<u32>,
    /// I2C Component Type Register.
    /// This register identifies the type of I2C component.
    pub comp_type: RW<u32>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::mem::offset_of;

    #[test]
    fn struct_register_block_offset() {
        assert_eq!(offset_of!(RegisterBlock, con), 0x00);
        assert_eq!(offset_of!(RegisterBlock, tar), 0x04);
        assert_eq!(offset_of!(RegisterBlock, sar), 0x08);
        assert_eq!(offset_of!(RegisterBlock, hs_maddr), 0x0C);
        assert_eq!(offset_of!(RegisterBlock, data_cmd), 0x10);
        assert_eq!(offset_of!(RegisterBlock, ss_scl_hcnt_ufm_scl_hcnt), 0x14);
        assert_eq!(offset_of!(RegisterBlock, ss_scl_lcnt_ufm_scl_lcnt), 0x18);
        assert_eq!(offset_of!(RegisterBlock, fs_scl_hcnt_ufm_tbuf_cnt), 0x1C);
        assert_eq!(offset_of!(RegisterBlock, fs_scl_lcnt), 0x20);
        assert_eq!(offset_of!(RegisterBlock, hs_scl_hcnt), 0x24);
        assert_eq!(offset_of!(RegisterBlock, hs_scl_lcnt), 0x28);
        assert_eq!(offset_of!(RegisterBlock, intr_stat), 0x2C);
        assert_eq!(offset_of!(RegisterBlock, intr_mask), 0x30);
        assert_eq!(offset_of!(RegisterBlock, raw_intr_stat), 0x34);
        assert_eq!(offset_of!(RegisterBlock, rx_tl), 0x38);
        assert_eq!(offset_of!(RegisterBlock, tx_tl), 0x3C);
        assert_eq!(offset_of!(RegisterBlock, clr_intr), 0x40);
        assert_eq!(offset_of!(RegisterBlock, clr_rx_under), 0x44);
        assert_eq!(offset_of!(RegisterBlock, clr_rx_over), 0x48);
        assert_eq!(offset_of!(RegisterBlock, clr_tx_over), 0x4C);
        assert_eq!(offset_of!(RegisterBlock, clr_rd_req), 0x50);
        assert_eq!(offset_of!(RegisterBlock, clr_tx_abrt), 0x54);
        assert_eq!(offset_of!(RegisterBlock, clr_rx_done), 0x58);
        assert_eq!(offset_of!(RegisterBlock, clr_activity), 0x5C);
        assert_eq!(offset_of!(RegisterBlock, clr_stop_det), 0x60);
        assert_eq!(offset_of!(RegisterBlock, clr_start_det), 0x64);
        assert_eq!(offset_of!(RegisterBlock, clr_gen_call), 0x68);
        assert_eq!(offset_of!(RegisterBlock, enable), 0x6C);
        assert_eq!(offset_of!(RegisterBlock, status), 0x70);
        assert_eq!(offset_of!(RegisterBlock, txflr), 0x74);
        assert_eq!(offset_of!(RegisterBlock, rxflr), 0x78);
        assert_eq!(offset_of!(RegisterBlock, sda_hold), 0x7C);
        assert_eq!(offset_of!(RegisterBlock, tx_abrt_source), 0x80);
        assert_eq!(offset_of!(RegisterBlock, slv_data_nack_only), 0x84);
        assert_eq!(offset_of!(RegisterBlock, dma_cr), 0x88);
        assert_eq!(offset_of!(RegisterBlock, dma_tdlr), 0x8C);
        assert_eq!(offset_of!(RegisterBlock, dma_rdlr), 0x90);
        assert_eq!(offset_of!(RegisterBlock, sda_setup), 0x94);
        assert_eq!(offset_of!(RegisterBlock, ack_general_call), 0x98);
        assert_eq!(offset_of!(RegisterBlock, enable_status), 0x9C);
        assert_eq!(offset_of!(RegisterBlock, fs_spklen_ufm_spklen), 0xA0);
        assert_eq!(offset_of!(RegisterBlock, hs_spklen), 0xA4);
        assert_eq!(offset_of!(RegisterBlock, clr_restart_det), 0xA8);
        assert_eq!(offset_of!(RegisterBlock, scl_stuck_at_low_timeout), 0xAC);
        assert_eq!(offset_of!(RegisterBlock, sda_stuck_at_low_timeout), 0xB0);
        assert_eq!(offset_of!(RegisterBlock, clr_scl_stuck_det), 0xB4);
        assert_eq!(offset_of!(RegisterBlock, device_id), 0xB8);
        assert_eq!(offset_of!(RegisterBlock, smbus_clk_low_sext), 0xBC);
        assert_eq!(offset_of!(RegisterBlock, smbus_clk_low_mext), 0xC0);
        assert_eq!(offset_of!(RegisterBlock, smbus_thigh_max_idle_count), 0xC4);
        assert_eq!(offset_of!(RegisterBlock, smbus_intr_stat), 0xC8);
        assert_eq!(offset_of!(RegisterBlock, smbus_intr_mask), 0xCC);
        assert_eq!(offset_of!(RegisterBlock, smbus_raw_intr_stat), 0xD0);
        assert_eq!(offset_of!(RegisterBlock, clr_smbus_intr), 0xD4);
        assert_eq!(offset_of!(RegisterBlock, optional_sar), 0xD8);
        assert_eq!(offset_of!(RegisterBlock, smbus_udid_lsb), 0xDC);
        assert_eq!(offset_of!(RegisterBlock, comp_param_1), 0xF4);
        assert_eq!(offset_of!(RegisterBlock, comp_version), 0xF8);
        assert_eq!(offset_of!(RegisterBlock, comp_type), 0xFC);
    }
}
