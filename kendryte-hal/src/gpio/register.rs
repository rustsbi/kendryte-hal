use arbitrary_int::{u2, u5};
use bitbybit::{bitenum, bitfield};
use volatile_register::{RO, RW, WO};
/// GPIO Register Block.
///
/// This structure represents the memory-mapped registers of a GPIO peripheral.
/// Each field corresponds to a specific register or group of registers.
#[repr(C)]
pub struct RegisterBlock {
    /// Port A Data Register.
    /// Used to read or write data from/to Port A.
    pub swporta_dr: RW<Dr>,
    /// Port A Data Direction Register.
    /// Configures Port A pins as input or output.
    pub swporta_ddr: RW<Ddr>,
    /// Port A Control Register.
    /// Controls the data source for Port A.
    pub swporta_ctl: RW<Ctl>,
    /// Port B Data Register.
    /// Used to read or write data from/to Port B.
    pub swportb_dr: RW<Dr>,
    /// Port B Data Direction Register.
    /// Configures Port B pins as input or output.
    pub swportb_ddr: RW<Ddr>,
    /// Port B Control Register.
    /// Controls the data source for Port B.
    pub swportb_ctl: RW<Ctl>,
    _reserved0: [u8; 0x18],
    /// Interrupt Enable Register.
    /// Enables interrupts for Port A.
    /// Only available if Port A supports interrupts.
    pub inten: RW<IntEn>,
    /// Interrupt Mask Register.
    /// Masks specific interrupt sources.
    /// Only available if Port A supports interrupts.
    pub intmask: RW<IntMask>,
    /// Interrupt Level Register.
    /// Configures interrupt trigger levels.
    /// Only available if Port A supports interrupts.
    pub inttype_level: RW<IntTypeLevel>,
    /// Interrupt Polarity Register.
    /// Configures interrupt trigger polarity.
    /// Only available if Port A supports interrupts.
    pub int_polarity: RW<IntPolarity>,
    /// Interrupt Status Register.
    /// Shows current interrupt status.
    /// Only available if Port A supports interrupts.
    pub intstatus: RO<IntStatus>,
    /// Raw Interrupt Status Register.
    /// Shows unmasked interrupt status.
    /// Only available if Port A supports interrupts.
    pub raw_intstatus: RO<RawIntStatus>,
    /// Debounce Enable Register.
    /// Enables debounce functionality.
    /// Available with interrupt and debounce support.
    pub debounce: RW<Debounce>,
    /// Port A Clear Interrupt Register.
    /// Clears pending interrupts.
    /// Available with interrupt and debounce support.
    pub porta_eoi: WO<Eoi>,
    /// External Port A Register.
    /// Reads external Port A pin values.
    pub ext_porta: RO<Ext>,
    /// External Port B Register.
    /// Reads external Port B pin values.
    pub ext_portb: RO<Ext>,
    _reserved1: [u8; 0x08],
    /// Synchronization Level Register.
    /// Configures input synchronization.
    pub ls_sync: RW<LsSync>,
    /// GPIO ID Code Register.
    /// Contains device identification code.
    pub id_code: RO<IdCode>,
    /// Interrupt Both Edge Type Register.
    /// Configures edge detection for interrupts.
    pub int_both_edge: RW<IntBothEdge>,
    /// GPIO Version ID Register.
    /// Contains version information.
    pub ver_id_code: RO<VerIdCode>,
    /// GPIO Configuration Register 2.
    /// Additional configuration parameters when GPIO_ADD_ENCODED_PARAMS is true.
    pub config_reg2: RO<ConfigReg2>,
    /// GPIO Configuration Register 1.
    /// Additional configuration parameters when GPIO_ADD_ENCODED_PARAMS is true.
    pub config_reg1: RO<ConfigReg1>,
}

/// Data Register.
/// Used for accessing the data of GPIO ports.
#[bitfield(u32)]
#[derive(Debug, PartialEq, Eq)]
pub struct Dr {
    /// Array of pin states, each bit represents the current state of a pin.
    #[bit(0, rw)]
    pin_state: [bool; 32],
}

/// Direction of a GPIO pin.
/// Defines the direction of a GPIO pin.
#[bitenum(u1, exhaustive = true)]
#[derive(Debug, PartialEq, Eq)]
pub enum Direction {
    /// Pin is configured as input.
    Input = 0b0,
    /// Pin is configured as output.
    Output = 0b1,
}

/// Data Direction Register.
/// GPIO data direction register.
#[bitfield(u32)]
pub struct Ddr {
    /// Each bit specifies the direction of the corresponding IO.
    #[bit(0, rw)]
    direction: [Direction; 32],
}

/// Control mode for a GPIO pin.
/// Control mode for each GPIO pin.
#[bitenum(u1, exhaustive = true)]
#[derive(Debug, PartialEq, Eq)]
pub enum ControlMode {
    /// Controlled by software.
    SoftWare = 0b0,
    /// Controlled by hardware.
    Hardware = 0b1,
}

/// Control Register.
/// Control register for mode selection.
#[bitfield(u32)]
#[derive(Debug, PartialEq, Eq)]
pub struct Ctl {
    /// Each bit specifies whether the pin is controlled by software or hardware.
    #[bit(0, rw)]
    control_mode: [ControlMode; 32],
}

/// Interrupt Enable Register.
/// Interrupt enable register.
#[bitfield(u32)]
#[derive(Debug, PartialEq, Eq)]
pub struct IntEn {
    /// Each bit enables or disables interrupt for each pin.
    #[bit(0, rw)]
    interrupt_enable: [bool; 32],
}

/// Interrupt Mask Register.
/// Interrupt mask register.
#[bitfield(u32)]
#[derive(Debug, PartialEq, Eq)]
pub struct IntMask {
    /// Each bit masks interrupt for every pin.
    #[bit(0, rw)]
    interrupt_mask: [bool; 32],
}

/// Type of interrupt trigger for GPIO pins.
/// Type of interrupt trigger for GPIO pins.
#[bitenum(u1, exhaustive = true)]
#[derive(Debug, PartialEq, Eq)]
pub enum TriggerType {
    /// Level-triggered interrupt.
    Level = 0b0,
    /// Edge-triggered interrupt.
    Edge = 0b1,
}

/// Interrupt Type Level Register.
/// Register for configuring trigger type.
#[bitfield(u32)]
#[derive(Debug, PartialEq, Eq)]
pub struct IntTypeLevel {
    /// Each bit sets the trigger type for the specified pin.
    #[bit(0, rw)]
    trigger_type: [TriggerType; 32],
}

/// Polarity configuration for GPIO interrupts.
/// Polarity configuration for GPIO interrupts.
#[bitenum(u1, exhaustive = true)]
#[derive(Debug, PartialEq, Eq)]
pub enum Polarity {
    /// Triggers on low signal.
    /// Triggered on low level or falling edge.
    ActiveLow = 0b0,
    /// Triggers on high signal.
    /// Triggered on high level or rising edge.
    ActiveHigh = 0b1,
}

/// Interrupt Polarity Register.
/// Register to configure interrupt polarity.
#[bitfield(u32)]
#[derive(Debug, PartialEq, Eq)]
pub struct IntPolarity {
    /// Sets interrupt polarity for each pin.
    #[bit(0, rw)]
    interrupt_polarity: [Polarity; 32],
}

/// Interrupt Status Register.
/// Interrupt status register.
#[bitfield(u32)]
#[derive(Debug, PartialEq, Eq)]
pub struct IntStatus {
    /// Shows the interrupt status for each pin.
    #[bit(0, r)]
    interrupt_status: [bool; 32],
}

/// Raw Interrupt Status Register.
/// Raw interrupt status register.
#[bitfield(u32)]
#[derive(Debug, PartialEq, Eq)]
pub struct RawIntStatus {
    /// Shows interrupt status before masking.
    #[bit(0, r)]
    raw_interrupt_status: [bool; 32],
}

/// Debounce Register.
/// Debounce enable register.
#[bitfield(u32)]
#[derive(Debug, PartialEq, Eq)]
pub struct Debounce {
    /// Controls debounce enable for every pin.
    #[bit(0, rw)]
    debounce_enable: [bool; 32],
}

/// End of Interrupt Register.
/// Register for clearing pending interrupts.
#[bitfield(u32)]
#[derive(Debug, PartialEq, Eq)]
pub struct Eoi {
    /// Set to clear pending interrupt.
    #[bit(0, w)]
    clear_interrupt: [bool; 32],
}

/// External Pin State Register.
/// Register for reading the state of external pins.
#[bitfield(u32)]
#[derive(Debug, PartialEq, Eq)]
pub struct Ext {
    /// Reads the actual level of external pin.
    #[bit(0, r)]
    external_pin_state: [bool; 32],
}

/// Level Sensitive Sync Register.
/// Level sensitive sync register.
#[bitfield(u32)]
#[derive(Debug, PartialEq, Eq)]
pub struct LsSync {
    /// Control input synchronization.
    #[bit(0, rw)]
    sync_enable: [bool; 32],
}

/// ID Code Register.
/// Identification code register.
#[bitfield(u32)]
#[derive(Debug, PartialEq, Eq)]
pub struct IdCode {
    /// Device identification code.
    #[bits(0..=31, r)]
    id: u32,
}

/// Both Edge Interrupt Register.
/// Both edge interrupt enable register.
#[bitfield(u32)]
#[derive(Debug, PartialEq, Eq)]
pub struct IntBothEdge {
    /// Set enable for both edge trigger.
    #[bit(0, rw)]
    both_edge_enable: [bool; 32],
}

/// Version ID Code Register.
/// Version ID register.
#[bitfield(u32)]
#[derive(Debug, PartialEq, Eq)]
pub struct VerIdCode {
    /// Register version identifier.
    #[bits(0..=31, r)]
    version_id: u32,
}

/// Configuration Register 2.
/// Configuration register 2.
#[bitfield(u32)]
#[derive(Debug, PartialEq, Eq)]
pub struct ConfigReg2 {
    /// Width of port A.
    #[bits(0..=4, r)]
    port_a_width: u5,
    /// Width of port B.
    #[bits(5..=9, r)]
    port_b_width: u5,
    /// Width of port C.
    #[bits(10..=14, r)]
    port_c_width: u5,
    /// Width of port D.
    #[bits(15..=19, r)]
    port_d_width: u5,
}

/// Configuration Register 1.
/// Configuration register 1.
#[bitfield(u32)]
#[derive(Debug, PartialEq, Eq)]
pub struct ConfigReg1 {
    /// APB data bus width.
    #[bits(0..=1, r)]
    apb_data_width: u2,
    /// Number of ports.
    #[bits(2..=3, r)]
    num_ports: u2,
    /// Port A single control enable.
    #[bit(4, r)]
    porta_single_ctl_enable: bool,
    /// Port B single control enable.
    #[bit(5, r)]
    portb_single_ctl_enable: bool,
    /// Port C single control enable.
    #[bit(6, r)]
    portc_single_ctl_enable: bool,
    /// Port D single control enable.
    #[bit(7, r)]
    portd_single_ctl_enable: bool,
    /// Hardware enable of port A.
    #[bit(8, r)]
    hw_porta_enable: bool,
    /// Hardware enable of port B.
    #[bit(9, r)]
    hw_portb_enable: bool,
    /// Hardware enable of port C.
    #[bit(10, r)]
    hw_portc_enable: bool,
    /// Hardware enable of port D.
    #[bit(11, r)]
    hw_portd_enable: bool,
    /// Port A interrupt enable.
    #[bit(12, r)]
    porta_intr_enable: bool,
    /// Global debounce enable.
    #[bit(13, r)]
    debounce_enable: bool,
    /// Add encoded parameters enable.
    #[bit(14, r)]
    add_encoded_params_enable: bool,
    /// GPIO ID enable function.
    #[bit(15, r)]
    gpio_id_enable: bool,
    /// Encoded ID width.
    #[bits(16..=20, r)]
    encoded_id_width: u5,
    /// Both edge interrupt type enable.
    #[bit(21, r)]
    interrupt_both_edge_type_enable: bool,
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
