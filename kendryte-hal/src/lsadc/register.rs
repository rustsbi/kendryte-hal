use volatile_register::RW;

/// LSADC Register Block.
///
/// This structure represents the memory-mapped registers of a LSADC peripheral.
/// Each field corresponds to a specific register or group of registers.
#[repr(C)]
pub struct RegisterBlock {
    /// LSADC initializes the self-calibrating control register.
    pub trim: RW<u32>,
    /// LSADC Data conversion control register.
    pub cfg: RW<u32>,
    /// LSADC output mode selection control register.
    pub mode: RW<u32>,
    /// LSADC threshold interrupt control register.
    pub thsd: RW<u32>,
    /// LSADC's DMA error interrupt register.
    pub dma_intr: RW<u32>,
    /// Input channel N Digital signal output.
    pub data: [RW<u32>; 6],
    /// Continuous sampling channel N digital signal output.
    pub data_dma: [RW<u32>; 3],
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::mem::offset_of;

    #[test]
    fn struct_register_block_offset() {
        assert_eq!(offset_of!(RegisterBlock, trim), 0x00);
        assert_eq!(offset_of!(RegisterBlock, cfg), 0x04);
        assert_eq!(offset_of!(RegisterBlock, mode), 0x08);
        assert_eq!(offset_of!(RegisterBlock, thsd), 0x0C);
        assert_eq!(offset_of!(RegisterBlock, dma_intr), 0x10);
        assert_eq!(offset_of!(RegisterBlock, data), 0x14);
        assert_eq!(offset_of!(RegisterBlock, data_dma), 0x2C);
    }
}
