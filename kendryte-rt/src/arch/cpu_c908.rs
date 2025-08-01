//! T-Head C908 specific CPU support code.

/// Entry function for T-Head C908 core.
#[cfg(target_arch = "riscv64")]
#[unsafe(naked)]
#[unsafe(link_section = ".text.entry")]
#[unsafe(export_name = "_start")]
pub unsafe extern "C" fn start() -> ! {
    use crate::{STACK, STACK_SIZE, main};
    core::arch::naked_asm!(
        // Disable interrupt.
        "csrw   mie, zero",

        // Prepare programming language stack.
        "la    sp, {stack}
        li     t0, {stack_size}
        add    sp, sp, t0",

        // Clear `.bss` section.
        "la    t1, sbss
        la     t2, ebss
    1:  bgeu   t1, t2, 2f
        sw     zero, 0(t1)
        addi   t1, t1, 4
        j      1b
    2:",

        // Start Rust main function.
        "call   {main}",

        // Platform halt if main function returns.
        "
    3:  wfi
        j       3b",

        stack      = sym STACK,
        stack_size = const STACK_SIZE,
        main       = sym main,
    )
}

// TODO multi-core baremetal entry.
