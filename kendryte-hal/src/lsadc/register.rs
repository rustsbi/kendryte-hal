use arbitrary_int::{u4, u12};
use bitbybit::{bitenum, bitfield};
use derive_mmio::Mmio;

/// LSADC Register Block.
///
/// This structure represents the memory-mapped registers of a LSADC peripheral.
/// Each field corresponds to a specific register or group of registers.
#[derive(Mmio)]
#[repr(C)]
pub struct RegisterBlock {
    /// LSADC initializes the self-calibrating control register.
    pub trim: Trim,
    /// LSADC Data conversion control register.
    pub cfg: Cfg,
    /// LSADC output mode selection control register.
    pub mode: Mode,
    /// LSADC threshold interrupt control register.
    pub thsd: Thsd,
    /// LSADC's DMA error interrupt register.
    pub dma_intr: DmaIntr,
    /// Input channel N Digital signal output.
    pub data: [Data; 6],
    /// Continuous sampling channel N digital signal output.
    pub data_dma: [DataDma; 3],
}

/// Reference voltage selection for internal voltage reference.
#[bitenum(u2, exhaustive = true)]
#[derive(Debug, PartialEq, Eq)]
pub enum ReferenceVoltage {
    V085 = 0b00,
    V090 = 0b01,
    V095 = 0b10,
    V100 = 0b11,
}

/// Reference source selection for SAR ADC.
#[bitenum(u1, exhaustive = true)]
#[derive(Debug, PartialEq, Eq)]
pub enum ReferenceSelect {
    External = 0b0,
    Internal = 0b1,
}

/// ADC input channel selection.
#[bitenum(u3, exhaustive = false)]
#[derive(Debug, PartialEq, Eq)]
pub enum ChannelSelect {
    AdcIn0 = 0b000,
    AdcIn1 = 0b001,
    AdcIn2 = 0b010,
    AdcIn3 = 0b011,
    AdcIn4 = 0b100,
    AdcIn5 = 0b101,
}

/// ADC output mode configuration.
#[bitenum(u2, exhaustive = true)]
#[derive(Debug, PartialEq, Eq)]
pub enum OutputMode {
    SingleSampleRegister = 0b00,
    SingleChannelContinuousDma = 0b01,
    DualChannelContinuousDma = 0b10,
    TripleChannelContinuousDma = 0b11,
}

/// Threshold interrupt mode selection.
#[bitenum(u2, exhaustive = true)]
#[derive(Debug, PartialEq, Eq)]
pub enum ThresholdMode {
    HighPass = 0b00,
    BandPass = 0b01,
    BandStop = 0b10,
    LowPass = 0b11,
}

/// SAR ADC trim and calibration register.
#[bitfield(u32)]
#[derive(Debug, PartialEq, Eq)]
pub struct Trim {
    /// SAR analog power supply control.
    #[bit(0, rw)]
    pub analog_power_enable: bool,

    /// SAR internal/external reference selection.
    #[bit(4, rw)]
    pub reference_select: ReferenceSelect,

    /// SAR internal reference voltage selection.
    #[bits(8..=9, rw)]
    pub reference_voltage: ReferenceVoltage,

    /// Bandgap reference voltage trim code.
    #[bits(12..=15, rw)]
    pub bandgap_trim_code: u4,

    /// SAR offset calibration enable.
    #[bit(20, rw)]
    pub offset_calibration_enable: bool,

    /// SAR offset calibration done flag.
    #[bit(24, r)]
    pub offset_calibration_done: bool,
}

/// SAR ADC configuration register.
#[bitfield(u32)]
#[derive(Debug, PartialEq, Eq)]
pub struct Cfg {
    /// SAR input channel selection.
    #[bits(0..=2, rw)]
    pub input_channel: Option<ChannelSelect>,

    /// SAR start of conversion.
    #[bit(4, rw)]
    pub start_of_conversion: bool,

    /// SAR busy flag.
    #[bit(8, r)]
    pub sar_busy: bool,

    /// SAR end of conversion.
    #[bit(12, r)]
    pub end_of_conversion: bool,

    /// Data output valid flag.
    #[bit(16, r)]
    pub data_output_valid: bool,
}

/// SAR ADC mode and DMA configuration register.
#[bitfield(u32)]
#[derive(Debug, PartialEq, Eq)]
pub struct Mode {
    /// Output mode selection.
    #[bits(0..=1, rw)]
    pub output_mode: OutputMode,

    /// Continuous sampling pause.
    #[bit(4, rw)]
    pub dma_pause: bool,

    /// DMA1 enable.
    #[bit(8, rw)]
    pub dma1_enable: bool,

    /// DMA1 input channel selection.
    #[bits(9..=11, rw)]
    pub dma1_channel: Option<ChannelSelect>,

    /// Clear DMA1 data.
    #[bit(12, rw)]
    pub dma1_clear: bool,

    /// DMA2 input channel selection.
    #[bits(16..=18, rw)]
    pub dma2_channel: Option<ChannelSelect>,

    /// Clear DMA2 data.
    #[bit(19, rw)]
    pub dma2_clear: bool,

    /// DMA3 input channel selection.
    #[bits(24..=26, rw)]
    pub dma3_channel: Option<ChannelSelect>,

    /// Clear DMA3 data.
    #[bit(27, rw)]
    pub dma3_clear: bool,
}

/// SAR ADC threshold configuration register.
#[bitfield(u32)]
#[derive(Debug, PartialEq, Eq)]
pub struct Thsd {
    /// Threshold interrupt mode selection.
    #[bits(0..=1, rw)]
    pub threshold_mode: ThresholdMode,

    /// Threshold lower limit comparison value.
    #[bits(4..=15, rw)]
    pub threshold_low: u12,

    /// Threshold upper limit comparison value.
    #[bits(16..=27, rw)]
    pub threshold_high: u12,
}

/// SAR ADC DMA interrupt status register.
#[bitfield(u32)]
#[derive(Debug, PartialEq, Eq)]
pub struct DmaIntr {
    /// DMA1 error interrupt.
    #[bit(0, r)]
    pub dma1_error: bool,

    /// DMA2 error interrupt.
    #[bit(1, r)]
    pub dma2_error: bool,

    /// DMA3 error interrupt.
    #[bit(2, r)]
    pub dma3_error: bool,
}

/// SAR ADC data register.
#[bitfield(u32)]
#[derive(Debug, PartialEq, Eq)]
pub struct Data {
    /// Input channel conversion data.
    #[bits(0..=11, r)]
    pub channel_data: u12,
}

/// SAR ADC DMA data register.
#[bitfield(u32)]
#[derive(Debug, PartialEq, Eq)]
pub struct DataDma {
    /// Continuous sampling channel conversion data.
    #[bits(0..=11, r)]
    pub dma_channel_data: u12,
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
