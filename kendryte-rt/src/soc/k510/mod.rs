//! Kendryte K510 chip.

use crate::arch::rvi::Stack;

/// Platform stack size.
pub const STACK_SIZE: usize = 32 * 1024;

/// Stack for current platform.
#[cfg(any(doc, feature = "k510"))]
#[unsafe(link_section = ".bss.uninit")]
pub static mut STACK: Stack<STACK_SIZE> = Stack([0; STACK_SIZE]);

// TODO K510 peripherals using soc! macro
