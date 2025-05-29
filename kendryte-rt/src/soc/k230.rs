use kendryte_hal::instance::{ExclusiveInstance, SharedInstance};
use kendryte_hal::{clocks::Clocks, gpio, iomux, pad, uart};

#[cfg(all(feature = "k230"))]
#[unsafe(naked)]
#[unsafe(link_section = ".text.entry")]
#[unsafe(export_name = "_start")]
unsafe extern "C" fn start() -> ! {
    use crate::arch::rvi::Stack;
    use crate::soc::main;
    const STACK_SIZE: usize = 32 * 1024;

    #[unsafe(link_section = ".bss.uninit")]
    static mut STACK: Stack<STACK_SIZE> = Stack([0; STACK_SIZE]);

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

/// Peripherals available on ROM start.
pub struct Peripherals {
    pub iomux: pad::pads::Pads,
    pub gpio0: &'static gpio::Instance<0>,
    pub gpio1: &'static gpio::Instance<1>,
    pub uart0: &'static mut uart::Instance<0>,
    pub uart1: &'static mut uart::Instance<1>,
    pub uart2: &'static mut uart::Instance<2>,
    pub uart3: &'static mut uart::Instance<3>,
    pub uart4: &'static mut uart::Instance<4>,
}

// Used by macros only.
#[allow(unused)]
#[doc(hidden)]
#[inline(always)]
pub fn __rom_init_params() -> (Peripherals, Clocks) {
    let iomux = unsafe { iomux::Instance::transmute_at(0x9110_5000) };
    let peripherals = Peripherals {
        iomux: pad::pads::Pads::new(iomux.inner_mut()),
        gpio0: unsafe { gpio::Instance::<0>::transmute_at(0x9140_B000) },
        gpio1: unsafe { gpio::Instance::<1>::transmute_at(0x9140_C000) },
        uart0: unsafe { uart::Instance::<0>::transmute_at(0x9140_0000) },
        uart1: unsafe { uart::Instance::<1>::transmute_at(0x9140_1000) },
        uart2: unsafe { uart::Instance::<2>::transmute_at(0x9140_2000) },
        uart3: unsafe { uart::Instance::<3>::transmute_at(0x9140_3000) },
        uart4: unsafe { uart::Instance::<4>::transmute_at(0x9140_4000) },
    };
    (peripherals, Clocks)
}
