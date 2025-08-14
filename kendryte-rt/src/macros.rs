macro_rules! peripheral {
    (
        $(use $mod_path:path;)*
        $(
            $(#[$doc:meta])*
            pub struct $name:ident => $addr:expr, $register_block:ty, $mmio_register_block:ty;
        )+
    ) => {
        $(use $mod_path;)*

        $(
            $(#[$doc])*
            ///
            /// # Safety
            ///
            /// This function is unsafe because it creates a memory-mapped I/O register block
            /// at the specified address. The caller must ensure that:
            /// - The address is valid and points to the correct hardware registers
            /// - No other code is concurrently accessing the same registers
            /// - The hardware is properly initialized
            #[allow(non_camel_case_types)]
            pub struct $name(());

            impl $name {
                /// Creates a new MMIO register block for this peripheral
                ///
                /// # Safety
                ///
                /// See struct-level safety documentation
                #[inline]
                pub const unsafe fn mmio_register_block() -> $mmio_register_block {
                   unsafe { <$register_block>::new_mmio_at($addr) }
                }
            }
        )+
    };
}
