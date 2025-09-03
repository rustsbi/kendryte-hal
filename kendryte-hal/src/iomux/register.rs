use super::pad;
use arbitrary_int::{u1, u3};
use bitbybit::{bitenum, bitfield};
use derive_mmio::Mmio;

/// IOMUX Register Block.
///
/// This structure represents the memory-mapped registers of a IOMUX peripheral.
/// Each field corresponds to a specific register or group of registers.
#[derive(Mmio)]
#[repr(C)]
pub struct RegisterBlock {
    #[mmio(Inner)]
    pub pads: [pad::RegisterBlock; 64],
}
