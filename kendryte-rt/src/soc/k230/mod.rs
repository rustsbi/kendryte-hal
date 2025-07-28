mod pads;
mod peripheral;

use crate::soc::k230::pads::Pads;
use kendryte_hal::{clocks::Clocks, gpio, iomux, uart};

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

macro_rules! soc {
    (
        $(
            $(#[$doc:meta])*
            pub struct $Ty:ident => $paddr:expr, $DerefTy:ty;
        )+
    ) => {
        $(
            $(#[$doc])*
            #[allow(non_camel_case_types)]
            pub struct $Ty (());

            impl $Ty {
                #[inline]
                pub const fn ptr() -> *const $DerefTy {
                    $paddr as *const $DerefTy
                }
            }

            impl core::ops::Deref for $Ty {
                type Target = $DerefTy;

                #[inline(always)]
                fn deref(&self) -> & 'static Self::Target {
                    unsafe { &*Self::ptr() }
                }
            }

            impl core::convert::AsRef<$DerefTy> for $Ty {
                #[inline(always)]
                fn as_ref(&self) -> & 'static $DerefTy {
                    unsafe { &*Self::ptr() }
                }
            }
        )+
    };
}

soc! {
    pub struct IOMUX => 0x9110_5000, iomux::RegisterBlock;
    pub struct GPIO0 => 0x9140_B000, gpio::RegisterBlock;
    pub struct GPIO1 => 0x9140_C000, gpio::RegisterBlock;
    pub struct UART0 => 0x9140_0000, uart::RegisterBlock;
    pub struct UART1 => 0x9140_1000, uart::RegisterBlock;
    pub struct UART2 => 0x9140_2000, uart::RegisterBlock;
    pub struct UART3 => 0x9140_3000, uart::RegisterBlock;
    pub struct UART4 => 0x9140_4000, uart::RegisterBlock;
}

/// Peripherals available on ROM start.
pub struct Peripherals {
    pub iomux: Pads,
    pub gpio0: GPIO0,
    pub gpio1: GPIO1,
    pub uart0: UART0,
    pub uart1: UART1,
    pub uart2: UART2,
    pub uart3: UART3,
    pub uart4: UART4,
}

// Used by macros only.
#[allow(unused)]
#[doc(hidden)]
#[inline(always)]
pub fn __rom_init_params() -> (Peripherals, Clocks) {
    let peripherals = Peripherals {
        iomux: Pads::new(),
        gpio0: GPIO0(()),
        gpio1: GPIO1(()),
        uart0: UART0(()),
        uart1: UART1(()),
        uart2: UART2(()),
        uart3: UART3(()),
        uart4: UART4(()),
    };
    (peripherals, Clocks)
}
