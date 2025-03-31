#[cfg(all(feature = "k230"))]
#[naked]
#[unsafe(link_section = ".text.entry")]
#[unsafe(export_name = "_start")]
unsafe extern "C" fn start() -> ! {
    use crate::soc::main;
    const STACK_SIZE: usize = 32 * 1024;

    #[unsafe(link_section = ".bss.uninit")]
    static mut STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];

    unsafe {
        core::arch::naked_asm!(
            // Disable interrupt
            "csrw   mie, zero",

            // Prepare programming language stack
            "la     sp, {stack}
             li     t0, {stack_size}
             add    sp, sp, t0",

            // Clear `.bss` section
            "la     t1, sbss
             la     t2, ebss
         1:  bgeu   t1, t2, 2f
             sw     zero, 0(t1)
             addi   t1, t1, 4
             j      1b
         2:",

            // Start Rust main function
            "call   {main}",

            // Platform halt if main function returns
            "
         3:  wfi
             j    3b",

            stack      = sym STACK,
            stack_size = const STACK_SIZE,
            main       = sym main,
        )
    }
}
