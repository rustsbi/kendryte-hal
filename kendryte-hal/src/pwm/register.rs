use arbitrary_int::{u2, u4, u31};
use bitbybit::{bitenum, bitfield};
use derive_mmio::Mmio;
// These definitions are from the K230 Technical Reference Manual V0.3.1_20241118 Chapter 12.8.5

/// Defines the behavior of the interrupt pending bits.
#[bitenum(u1, exhaustive = true)]
#[derive(Debug, PartialEq, Eq)]
pub enum StickyMode {
    /// Interrupt pending bits are cleared automatically when the condition is no longer true.
    AutoClear = 0b0,
    /// Interrupt pending bits must be cleared by writing a 1 to them.
    ManualClear = 0b1,
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

/// Defines the alignment of the PWM output.
#[bitenum(u1, exhaustive = true)]
#[derive(Debug, PartialEq, Eq)]
pub enum Alignment {
    /// The PWM output is left-aligned.
    Left = 0b0,
    /// The PWM output is center-aligned.
    Center = 0b1,
}

/// Represents the state of an interrupt pending flag.
#[bitenum(u1, exhaustive = true)]
#[derive(Debug, PartialEq, Eq)]
pub enum InterruptPending {
    /// No interrupt is pending.
    NotPending = 0b0,
    /// An interrupt is pending.
    Pending = 0b1,
}

/// PWM configuration register (PWM_CFG)
///
/// Controls the main configuration of the PWM peripheral, including scaling, modes, alignment, and interrupt status.
#[bitfield(u32)]
pub struct PwmCfg {
    /// PWM scale factor (bits 0-3):
    ///
    /// The 4-bit pwmscale field scales the PWM counter value before feeding it to the PWM comparators.
    /// The value in pwmscale is the bit position within the pwmcount register of the start of a cmpwidth-bit pwms field.
    /// A value of 0 in pwmscale indicates no scaling, and pwms would then be equal to the low cmpwidth bits of pwmcount.
    /// The maximum value of 15 in pwmscale corresponds to dividing the clock rate by 215, so for an input bus clock of 16MHz, the LSB of pwms will increment at 488.3Hz.
    #[bits(0..=3, rw)]
    pub pwm_scale: u4,
    /// Reserved (bits 4-7):
    #[bits(4..=7, r)]
    _reserved0: u4,
    /// PWM sticky bit (bit 8):
    ///
    /// The pwmsticky bit will disallow the pwmcmpXip registers from clearing if they’re already set, and is used to ensure interrupts are seen from the pwmcmpXip bits.
    #[bit(8, rw)]
    pub pwm_sticky: StickyMode,
    /// PWM zero compare bit (bit 9):
    ///
    /// The pwmzerocmp bit, if set, causes the PWM counter pwmcount to be automatically reset to zero one cycle after the pwms counter value matches the compare value in pwmcmp0.
    /// This is normally used to set the period of the PWM cycle.
    /// This feature can also be used to implement periodic counter interrupts, where the period is independent of interrupt service time.
    #[bit(9, rw)]
    pub pwm_zero_cmp: Enable,
    /// PWM deglitch bit (bit 10):
    ///
    /// To avoid glitches in the PWM waveforms when changing pwmcmpX register values, the pwmdeglitch bit in pwmcfg can be set to capture any high output of a PWM comparator in a sticky bit(pwmcmpXip for comparator X) and prevent the output falling again within the same PWM cycle.
    /// The pwmcmpXip bits are only allowed to change at the start of the next PWM cycle.
    /// Note the pwmcmp0ip bit will only be high for one cycle when pwmdeglitch and pwmzerocmp are set where pwmcmp0 is used to define the PWM cycle, but can be used as a regular PWM edge otherwise.
    /// If pwmdeglitch is set, but pwmzerocmp is clear, the deglitch circuit is still operational but is now triggered when pwms contains all 1s and will cause a carry out of the high bit of the pwms incrementer just before the counter wraps to zero.
    #[bit(10, rw)]
    pub pwm_deglitch: Enable,
    /// Reserved (bit 11):
    #[bit(11, r)]
    _reserved1: bool,
    /// PWM always-on enable (bit 12):
    ///
    /// The pwmen* bits control the conditions under which the PWM counter pwmcount is incremented.
    /// The counter increments by one each cycle only if any of the enabled conditions are true.
    /// If the pwmenalways bit is set, the PWM counter increments continuously.
    #[bit(12, rw)]
    pub pwm_en_always: Enable,
    /// PWM one-shot enable (bit 13):
    ///
    /// The pwmenbits control the conditions under which the PWM counter pwmcount is incremented.
    /// The counter increments by one each cycle only if any of the enabled conditions are true.
    /// When pwmenoneshot is set, the counter can increment but pwmenoneshot is reset to zero once the counter resets, disabling further counting (unless pwmenalways is set).
    /// The pwmenoneshot bit provides a way for software to generate a single PWM cycle then stop.
    /// Software can set the pwnenoneshot again at any time to replay the one-shot waveform.
    /// The pwmen bits are reset at wakeup reset, which disables the PWM counter and saves power.
    #[bit(13, rw)]
    pub pwm_en_oneshot: Enable,
    /// Reserved (bits 14-15):
    #[bits(14..=15, r)]
    _reserved2: u2,

    // PWM compare center alignment bits.
    /// PWM compare 0 center alignment (bit 16):
    ///
    /// A per-comparator pwmcmp0center bit in pwmcfg allows a single PWM comparator to generate a center-aligned symmetric duty-cycle
    #[bit(16, rw)]
    pub pwm_cmp0_center: Alignment,
    /// PWM compare 1 center alignment (bit 17):
    ///
    /// A per-comparator pwmcmp1center bit in pwmcfg allows a single PWM comparator to generate a center-aligned symmetric duty-cycle
    #[bit(17, rw)]
    pub pwm_cmp1_center: Alignment,
    /// PWM compare 2 center alignment (bit 18):
    ///
    /// A per-comparator pwmcmp2center bit in pwmcfg allows a single PWM comparator to generate a center-aligned symmetric duty-cycle
    #[bit(18, rw)]
    pub pwm_cmp2_center: Alignment,
    /// PWM compare 3 center alignment (bit 19):
    ///
    /// A per-comparator pwmcmp3center bit in pwmcfg allows a single PWM comparator to generate a center-aligned symmetric duty-cycle
    #[bit(19, rw)]
    pub pwm_cmp3_center: Alignment,
    /// Reserved (bits 20-23):
    #[bits(20..=23, r)]
    _reserved3: u4,

    // PWM compare gang bits.
    /// PWM compare 0 gang enable (bit 24):
    ///
    /// A comparator can be ganged together with its next-highest-numbered neighbor to generate arbitrary PWM pulses.
    /// When the pwmcmp0gang bit is set, comparator 0 fires and raises its pwm0gpio signal.
    /// When comparator 0 + 1 (or pwmcmp0 for pwmcmp3) fires, the pwm0gpio output is reset to zero.
    #[bit(24, rw)]
    pub pwm_cmp0_gang: Enable,
    /// PWM compare 1 gang enable (bit 25):
    ///
    ///A comparator can be ganged together with its next-highest-numbered neighbor to generate arbitrary PWM pulses.
    /// When the pwmcmp1gang bit is set, comparator 1 fires and raises its pwm1gpio signal.
    /// When comparator 1 + 1 (or pwmcmp0 for pwmcmp3) fires, the pwm1gpio output is reset to zero.
    #[bit(25, rw)]
    pub pwm_cmp1_gang: Enable,
    /// PWM compare 2 gang enable (bit 26):
    ///
    /// A comparator can be ganged together with its next-highest-numbered neighbor to generate arbitrary PWM pulses.
    /// When the pwmcmp2gang bit is set, comparator 2 fires and raises its pwm2gpio signal.
    /// When comparator 2 + 1 (or pwmcmp0 for pwmcmp3) fires, the pwm2gpio output is reset to zero.
    #[bit(26, rw)]
    pub pwm_cmp2_gang: Enable,
    /// PWM compare 3 gang enable (bit 27):
    ///
    /// A comparator can be ganged together with its next-highest-numbered neighbor to generate arbitrary PWM pulses.
    /// When the pwmcmp3gang bit is set, comparator 3 fires and raises its pwm3gpio signal.
    /// When comparator 3 + 1 (or pwmcmp0 for pwmcmp3) fires, the pwm3gpio output is reset to zero.
    #[bit(27, rw)]
    pub pwm_cmp3_gang: Enable,

    // Interrupt pending bits
    /// PWM compare 0 interrupt pending (bit 28):
    ///
    /// The interrupt pending bits pwmcmp0ip The pwmcmp0ip bits are only allowed to change at the start of the next PWM cycle.
    /// Note the pwmcmp0ip bit will only be high for one cycle when pwmdeglitch and pwmzerocmp are set where pwmcmp0 is used to define the PWM cycle, but can be used as a regular PWM edge otherwise.
    /// The interrupt pending bits pwmcmp0ip can be cleared down using writes to the pwmcfg register.
    /// The PWM peripheral can also be used as a regular timer with no counter reset (pwmzerocmp=0), where the comparators are now used to provide timer interrupts.
    #[bit(28, rw)]
    pub pwm_cmp0_ip: InterruptPending,
    /// PWM compare 1 interrupt pending (bit 29):
    ///
    /// The interrupt pending bits pwmcmp1ip The pwmcmp1ip bits are only allowed to change at the start of the next PWM cycle.
    /// The interrupt pending bits pwmcmp1ip can be cleared down using writes to the pwmcfg register.
    /// The PWM peripheral can also be used as a regular timer with no counter reset (pwmzerocmp=0), where the comparators are now used to provide timer interrupts.
    #[bit(29, rw)]
    pub pwm_cmp1_ip: InterruptPending,
    /// PWM compare 2 interrupt pending (bit 30):
    ///
    /// The interrupt pending bits pwmcmp2ip The pwmcmp2ip bits are only allowed to change at the start of the next PWM cycle.
    /// The interrupt pending bits pwmcmp2ip can be cleared down using writes to the pwmcfg register.
    /// The PWM peripheral can also be used as a regular timer with no counter reset (pwmzerocmp=0), where the comparators are now used to provide timer interrupts.
    #[bit(30, rw)]
    pub pwm_cmp2_ip: InterruptPending,
    /// PWM compare 3 interrupt pending (bit 31):
    ///
    /// The interrupt pending bits pwmcmp3ip The pwmcmp3ip bits are only allowed to change at the start of the next PWM cycle.
    /// The interrupt pending bits pwmcmp3ip can be cleared down using writes to the pwmcfg register.
    /// The PWM peripheral can also be used as a regular timer with no counter reset (pwmzerocmp=0), where the comparators are now used to provide timer interrupts.
    #[bit(31, rw)]
    pub pwm_cmp3_ip: InterruptPending,
}

#[bitfield(u32)]
pub struct PwmCount {
    #[bits(0..=30, rw)]
    ///The PWM unit is based around a counter held in pwmcount.
    /// The counter can be read or written over the bus.
    /// The pwmcount register is (15 + cmpwidth) 30:0 R/W counter 0x0 bits wide.
    /// For example, for cmpwidth of 16 bits, the counter is held in pwmcount\[30:0\], and bit 31 of pwmcount returns a zero when read.
    pub counter: u31,
    /// Reserved (bit 31).
    #[bit(31, r)]
    pub _reserved: bool,
}

#[bitfield(u32)]
pub struct Pwms {
    #[bits(0..=15, rw)]
    ///The value of pwms is memory-mapped and can be read as a single 15:0 R/W pwms 0x0 cmpwidth-bit value over the bus.
    pub pwms: u16,
    #[bits(16..=31, r)]
    /// Reserved (bits 16-31).
    _reserved: u16,
}

#[bitfield(u32)]
pub struct PwmCmpn {
    #[bits(0..=30, rw)]
    /// The primary use of the ncmp PWM compare registers is to define the edges of the PWM waveforms within the PWM cycle.
    /// PWM_CMPN Each compare register is a cmpwidth-bit value against which the current 30:0 R/W 0x0 (N=0,1,2,3) pwms value is compared every cycle.
    /// The output of each comparator is high whenever the value of pwms is greater than or equal to the corresponding pwmcmpN.
    pub pwm_cpmn: u31,
    /// Reserved (bit 31).
    #[bit(31, r)]
    pub _reserved: bool,
}

#[derive(Mmio)]
#[repr(C)]
pub struct RegisterBlock {
    /// PWM configuration register.
    pub pwm_cfg: PwmCfg,
    _reverser0: [u8; 0x04],
    /// PWM counter count value register.
    pub pwm_count: PwmCount,
    _reverser1: [u8; 0x04],
    /// PWM counter is relatively straight register.
    pub pwms: Pwms,
    _reverser2: [u8; 0x0C],
    /// PWM comparator register N.
    pub pwm_cmpn: [PwmCmpn; 4],
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::mem::offset_of;

    #[test]
    fn struct_register_block_offset() {
        assert_eq!(offset_of!(RegisterBlock, pwm_cfg), 0x00);
        assert_eq!(offset_of!(RegisterBlock, pwm_count), 0x08);
        assert_eq!(offset_of!(RegisterBlock, pwms), 0x10);
        assert_eq!(offset_of!(RegisterBlock, pwm_cmpn), 0x20);
    }
}
