use volatile_register::RW;

/// I2C Register Block.
///
/// This structure represents the memory-mapped registers of a I2C peripheral.
/// Each field corresponds to a specific register or group of registers.
#[repr(C)]
pub struct RegisterBlock {
    /// PWM configuration register.
    pub pwmcfg: RW<u32>,
    _reverser0: [u8; 0x04],
    /// PWM counter count value register.
    pub pwmcount: RW<u32>,
    _reverser1: [u8; 0x04],
    /// PWM counter is relatively straight register.
    pub pwms: RW<u32>,
    _reverser2: [u8; 0x0C],
    /// PWM comparator register N.
    pub pwmcmpn: [RW<u32>; 4],
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::mem::offset_of;

    #[test]
    fn struct_register_block_offset() {
        assert_eq!(offset_of!(RegisterBlock, pwmcfg), 0x00);
        assert_eq!(offset_of!(RegisterBlock, pwmcount), 0x08);
        assert_eq!(offset_of!(RegisterBlock, pwms), 0x10);
        assert_eq!(offset_of!(RegisterBlock, pwmcmpn), 0x20);
    }
}
