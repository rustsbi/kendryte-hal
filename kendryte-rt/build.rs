fn main() {
    let (out, ld) = {
        use std::{env, path::PathBuf};
        let out = PathBuf::from(env::var_os("OUT_DIR").unwrap());
        let ld = out.join("kendryte-rt.ld");
        (out, ld)
    };
    #[cfg(feature = "k230")]
    std::fs::write(&ld, LINKER_SCRIPT_K230).unwrap();

    println!("cargo:rustc-link-search={}", out.display());
    let _ = (ld, out);
}

#[cfg(feature = "k230")]
const LINKER_SCRIPT_K230: &[u8] = b"
OUTPUT_ARCH(riscv)

ENTRY(_start)

MEMORY {
    SPL : ORIGIN = 0x80300000, LENGTH = 0x100000
}

SECTIONS
{
    .text : ALIGN(4) {
        stext = .;
        KEEP(*(.text.entry))
        *(.text .text.*)
        . = ALIGN(4);
        etext = .;
    } > SPL

    .rodata : ALIGN(4) {
        srodata = .;
        *(.rodata .rodata.*)
        *(.srodata .srodata.*)
        . = ALIGN(4);
        erodata = .;
    } > SPL

    .data : ALIGN(4) {
        sdata = .;
        *(.data .data.*)
        *(.sdata .sdata.*)
        . = ALIGN(4);
        edata = .;
    } > SPL
    sidata = LOADADDR(.data);

    .bss (NOLOAD) : ALIGN(4) {
        *(.bss.uninit)
        sbss = .;
        *(.bss .bss.*)
        *(.sbss .sbss.*)
        ebss = .;
    } > SPL

    /DISCARD/ : {
        *(.eh_frame)
    }
}
";
