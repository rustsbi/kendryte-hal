//! Architecture support for Kendryte chips.

// RISC-V standard support, including stack and trap frame definitions.
pub mod rve;
pub mod rvi;

// CPU specific supports, including entry assembly code and stack implementation.

// K230 cpu supports.
#[cfg(any(doc, feature = "cpu-c908"))]
pub mod cpu_c908;

// K510 cpu supports.
// #[cfg(any(doc, feature = "cpu-andesv5"))]
// TODO pub mod cpu_andesv5;

// For K210 chip, which is actually a BOOM RISC-V IP core with RISC-V privileged
// specification version 1.9.1.
// TODO pub mod generic;
