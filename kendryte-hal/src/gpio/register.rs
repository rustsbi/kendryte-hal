use volatile_register::RW;

/// GPIO Register Block.
///
/// This structure represents the memory-mapped registers of a GPIO peripheral.
/// Each field corresponds to a specific register or group of registers.
#[repr(C)]
pub struct RegisterBlock {
    /// Port A data register.
    /// Used to read or write data from/to Port A.
    pub swporta_dr: RW<u32>,
    /// Port A Data Direction Register.
    /// Configures Port A pins as input or output.
    pub swporta_ddr: RW<u32>,
    /// Port A data source register.
    /// Controls the data source for Port A.
    pub swporta_ctl: RW<u32>,
    /// Port B data register.
    /// Used to read or write data from/to Port B.
    pub swportb_dr: RW<u32>,
    /// Port B Data Direction Register.
    /// Configures Port B pins as input or output.
    pub swportb_ddr: RW<u32>,
    /// Port B data source register.
    /// Controls the data source for Port B.
    pub swportb_ctl: RW<u32>,
    _reserved0: [u8; 0x18],
    /// Interrupt enable register.
    /// Enables interrupts for Port A.
    /// Note: This register is available only if Port A is configured to generate interrupts.
    pub inten: RW<u32>,
    /// Interrupt mask register.
    /// Masks specific interrupt sources.
    /// Note: This register is available only if Port A is configured to generate interrupts.
    pub intmask: RW<u32>,
    /// Interrupt level register.
    /// Configures interrupt trigger levels.
    /// Note: This register is available only if Port A is configured to generate interrupts.
    pub inttype_level: RW<u32>,
    /// Interrupt polarity register.
    /// Configures interrupt trigger polarity.
    /// Note: This register is available only if Port A is configured to generate interrupts.
    pub int_polarity: RW<u32>,
    /// Interrupt status register. Shows current interrupt status.
    /// Note: This register is available only if Port A is configured to generate interrupts.
    pub intstatus: RW<u32>,
    /// Raw interrupt status register. Shows unmasked interrupt status.
    /// Note: This register is available only if Port A is configured to generate interrupts.
    pub raw_intstatus: RW<u32>,
    /// Debounce enable register. Enables debounce functionality.
    /// Note: This register is available only if Port A is configured to generate interrupts and when the debounce logic is included.
    pub debounce: RW<u32>,
    /// Port A clear interrupt register.
    /// Clears pending interrupts.
    /// Note: This register is available only if Port A is configured to generate interrupts and when the debounce logic is included.
    pub porta_eoi: RW<u32>,
    /// External port A register. Reads external Port A pin values.
    pub ext_porta: RW<u32>,
    /// Port B external port register. Reads external Port B pin values.
    pub ext_portb: RW<u32>,
    _reserved1: [u8; 0x08],
    /// Synchronization level register. Configures input synchronization.
    pub ls_sync: RW<u32>,
    /// GPIO ID code register. Contains device identification code.
    pub id_code: RW<u32>,
    /// Interrupt Both Edge type register. Configures edge detection.
    /// Note: This register is available only if PORT A is configured to generate interrupts and interrupt detection is configured to generate on both rising and falling edges of external input signal.
    pub int_both_edge: RW<u32>,
    /// GPIO Component Version register. Contains version information.
    pub ver_id_code: RW<u32>,
    /// GPIO Configuration Register 2. Provides additional configuration parameters.
    /// This register is a read-only register that is present when the configuration parameter GPIO_ADD_ENCODED_PARAMS is set to True.
    /// If this configuration is set to False, then this register reads back 0.
    pub config_reg2: RW<u32>,
    /// GPIO Configuration Register 1. Provides additional configuration parameters.
    /// This register is present when the configuration parameter GPIO_ADD_ENCODED_PARAMS is set to True.
    /// If this parameter is set to False, this register reads back zero (0).
    pub config_reg1: RW<u32>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::mem::offset_of;
    #[test]
    fn struct_register_block_offset() {
        assert_eq!(offset_of!(RegisterBlock, swporta_dr), 0x00);
        assert_eq!(offset_of!(RegisterBlock, swporta_ddr), 0x04);
        assert_eq!(offset_of!(RegisterBlock, swporta_ctl), 0x08);
        assert_eq!(offset_of!(RegisterBlock, swportb_dr), 0x0C);
        assert_eq!(offset_of!(RegisterBlock, swportb_ddr), 0x10);
        assert_eq!(offset_of!(RegisterBlock, swportb_ctl), 0x14);
        assert_eq!(offset_of!(RegisterBlock, inten), 0x30);
        assert_eq!(offset_of!(RegisterBlock, intmask), 0x34);
        assert_eq!(offset_of!(RegisterBlock, inttype_level), 0x38);
        assert_eq!(offset_of!(RegisterBlock, int_polarity), 0x3C);
        assert_eq!(offset_of!(RegisterBlock, intstatus), 0x40);
        assert_eq!(offset_of!(RegisterBlock, raw_intstatus), 0x44);
        assert_eq!(offset_of!(RegisterBlock, debounce), 0x48);
        assert_eq!(offset_of!(RegisterBlock, porta_eoi), 0x4C);
        assert_eq!(offset_of!(RegisterBlock, ext_porta), 0x50);
        assert_eq!(offset_of!(RegisterBlock, ext_portb), 0x54);
        assert_eq!(offset_of!(RegisterBlock, ls_sync), 0x60);
        assert_eq!(offset_of!(RegisterBlock, id_code), 0x64);
        assert_eq!(offset_of!(RegisterBlock, int_both_edge), 0x68);
        assert_eq!(offset_of!(RegisterBlock, ver_id_code), 0x6C);
        assert_eq!(offset_of!(RegisterBlock, config_reg2), 0x70);
        assert_eq!(offset_of!(RegisterBlock, config_reg1), 0x74);
    }
}
