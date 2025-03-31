use crate::soc::PeripheralWrapper;

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

pub type UART0 = PeripheralWrapper<0x9140_0000, kendryte_hal::uart::RegisterBlock>;
pub type UART1 = PeripheralWrapper<0x9140_1000, kendryte_hal::uart::RegisterBlock>;
pub type UART2 = PeripheralWrapper<0x9140_2000, kendryte_hal::uart::RegisterBlock>;
pub type UART3 = PeripheralWrapper<0x9140_3000, kendryte_hal::uart::RegisterBlock>;
pub type UART4 = PeripheralWrapper<0x9140_4000, kendryte_hal::uart::RegisterBlock>;
pub type I2C0 = PeripheralWrapper<0x9140_5000, kendryte_hal::i2c::RegisterBlock>;
pub type I2C1 = PeripheralWrapper<0x9140_6000, kendryte_hal::i2c::RegisterBlock>;
pub type I2C2 = PeripheralWrapper<0x9140_7000, kendryte_hal::i2c::RegisterBlock>;
pub type I2C3 = PeripheralWrapper<0x9140_8000, kendryte_hal::i2c::RegisterBlock>;
pub type I2C4 = PeripheralWrapper<0x9140_9000, kendryte_hal::i2c::RegisterBlock>;
pub type PWM = PeripheralWrapper<0x9140_A000, kendryte_hal::pwm::RegisterBlock>;
pub type GPIO0 = PeripheralWrapper<0x9140_B000, kendryte_hal::gpio::RegisterBlock>;
pub type GPIO1 = PeripheralWrapper<0x9140_C000, kendryte_hal::gpio::RegisterBlock>;
pub type LSADC = PeripheralWrapper<0x9140_D000, kendryte_hal::lsadc::RegisterBlock>;

/// Peripherals available on ROM start.
pub struct Peripherals {
    pub uart0: UART0,
    pub uart1: UART1,
    pub uart2: UART2,
    pub uart3: UART3,
    pub uart4: UART4,
    pub i2c0: I2C0,
    pub i2c1: I2C1,
    pub i2c2: I2C2,
    pub i2c3: I2C3,
    pub i2c4: I2C4,
    pub pwm: PWM,
    pub gpio0: GPIO0,
    pub gpio1: GPIO1,
    pub lsadc: LSADC,
}

pub struct Clocks();

// Used by macros only.
#[allow(unused)]
#[doc(hidden)]
#[inline(always)]
pub fn __rom_init_params() -> (Peripherals, Clocks) {
    let peripherals = Peripherals {
        uart0: PeripheralWrapper {
            _marker: Default::default(),
        },
        uart1: PeripheralWrapper {
            _marker: Default::default(),
        },
        uart2: PeripheralWrapper {
            _marker: Default::default(),
        },
        uart3: PeripheralWrapper {
            _marker: Default::default(),
        },
        uart4: PeripheralWrapper {
            _marker: Default::default(),
        },
        i2c0: PeripheralWrapper {
            _marker: Default::default(),
        },
        i2c1: PeripheralWrapper {
            _marker: Default::default(),
        },
        i2c2: PeripheralWrapper {
            _marker: Default::default(),
        },
        i2c3: PeripheralWrapper {
            _marker: Default::default(),
        },
        i2c4: PeripheralWrapper {
            _marker: Default::default(),
        },
        pwm: PeripheralWrapper {
            _marker: Default::default(),
        },
        gpio0: PeripheralWrapper {
            _marker: Default::default(),
        },
        gpio1: PeripheralWrapper {
            _marker: Default::default(),
        },
        lsadc: PeripheralWrapper {
            _marker: Default::default(),
        },
    };
    (peripherals, Clocks())
}
