//! Basic interrupt and exception handling framework (initial minimal version).
//!
//! This is an MVP implementation: a fixed-size table of interrupt handlers
//! that can be registered at runtime. A real implementation may switch to
//! linker-time collection or vector mode mtvec.

#![allow(dead_code)]

use core::sync::atomic::{AtomicBool, Ordering};

// Maximum number of machine external interrupt sources we support for now.
// (Adjust according to SoC manual; kept modest for MVP.)
pub const MAX_INTERRUPTS: usize = 64;

// K230 partial IRQ mapping (from documentation snippet).
pub const IRQ_UART0: usize = 0;
pub const IRQ_UART1: usize = 1;
pub const IRQ_UART2: usize = 2;
pub const IRQ_UART3: usize = 3;
pub const IRQ_UART4: usize = 4;

type IrqHandler = fn();

static mut IRQ_TABLE: [Option<IrqHandler>; MAX_INTERRUPTS] = [None; MAX_INTERRUPTS];
static INITIALIZED: AtomicBool = AtomicBool::new(false);

/// Initialize interrupt subsystem (idempotent).
pub fn init() {
	INITIALIZED.store(true, Ordering::SeqCst);
}

/// Register an interrupt handler for a given interrupt number.
/// Safety: caller must ensure number matches actual platform IRQ mapping.
pub unsafe fn register(irq: usize, handler: IrqHandler) {
	if irq < MAX_INTERRUPTS { unsafe { IRQ_TABLE[irq] = Some(handler); } }
}

/// Dispatch an interrupt number (called from trap trampoline).
pub(crate) fn dispatch_irq(irq: usize) {
	unsafe {
		if irq < MAX_INTERRUPTS {
			if let Some(h) = IRQ_TABLE[irq] { h(); }
		}
	}
}

/// Manually trigger a registered handler in software (for demo without PLIC).
pub fn software_trigger(irq: usize) { dispatch_irq(irq); }

/// Called for unhandled exceptions (placeholder). Users can implement an
/// `#[exception] fn exceptions(tf: &mut TrapFrame)`; and assembly side will
/// call symbol `exceptions` if present.
#[inline(always)]
pub fn unhandled_exception() -> ! { loop { core::hint::spin_loop(); } }

/// Enable global machine interrupts (set MIE in mstatus).
pub fn enable() {
	unsafe {
		core::arch::asm!("csrrs zero, mstatus, {mask}", mask = const 1 << 3, options(nostack, preserves_flags));
	}
}
