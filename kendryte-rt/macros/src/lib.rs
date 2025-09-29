use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::spanned::Spanned;
use syn::{ItemFn, ReturnType, Type, Visibility, parse, parse_macro_input};

// Temporary SoC helper module. In future this should validate interrupt names against
// the concrete SoC (k230, k510, etc.) and map a symbol name to an IRQ number.
// For now we only perform a very light syntactic check (must start with a letter and
// contain only valid Rust ident chars), always returning Ok.
mod soc {
    use proc_macro2::Ident;
    use syn::parse::Error;

    pub(crate) fn check_interrupt_name(_ident: &Ident) -> Option<Error> {
        None
    }
}

/// ROM runtime function entry.
#[proc_macro_attribute]
pub fn entry(args: TokenStream, input: TokenStream) -> TokenStream {
    if !args.is_empty() {
        return parse::Error::new(Span::call_site(), "#[entry] attribute accepts no arguments")
            .to_compile_error()
            .into();
    }

    let f = parse_macro_input!(input as ItemFn);

    if f.sig.inputs.len() != 2 {
        return parse::Error::new(
            f.sig.inputs.span(),
            "`#[entry]` function should include exactly two parameters",
        )
        .to_compile_error()
        .into();
    }

    let valid_signature = f.sig.constness.is_none()
        && f.sig.asyncness.is_none()
        && f.vis == Visibility::Inherited
        && f.sig.abi.is_none()
        && f.sig.generics.params.is_empty()
        && f.sig.generics.where_clause.is_none()
        && f.sig.variadic.is_none()
        && matches!(f.sig.output, ReturnType::Type(_, ref t) if matches!(t.as_ref(), &Type::Never(_)));

    if !valid_signature {
        return parse::Error::new(
            f.sig.span(),
            "`#[entry]` function must have signature `[unsafe] fn(p: Peripherals, c: Clocks) -> !`",
        )
        .to_compile_error()
        .into();
    }

    let attrs = f.attrs;
    let unsafety = f.sig.unsafety;
    let stmts = f.block.stmts;
    let inputs = f.sig.inputs;

    quote!(
        #[unsafe(no_mangle)]
        pub extern "C" fn main() -> ! {
            let (p, c) = ::kendryte_rt::__rom_init_params();
            unsafe { __kendryte_rt_macros__main(p, c) }
        }
        #[allow(non_snake_case)]
        #[inline(always)]
        #(#attrs)*
        #unsafety fn __kendryte_rt_macros__main(#inputs) -> ! {
            #(#stmts)*
        }
    )
    .into()
}

/// Interrupt handler function attribute.
///
/// This macro validates the signature of an interrupt handler and exposes it as a
/// `extern "C"` symbol with the same name (no mangling) so that the runtime's
/// dispatch table / trap trampoline can call into it.
///
/// Expected signature: `[unsafe] fn() [-> !]` (no parameters, optional never return type).
#[proc_macro_attribute]
pub fn interrupt(args: TokenStream, input: TokenStream) -> TokenStream {
    if !args.is_empty() {
        return parse::Error::new(
            Span::call_site(),
            "#[interrupt] attribute accepts no arguments",
        )
        .to_compile_error()
        .into();
    }

    let f = parse_macro_input!(input as ItemFn);

    if f.sig.inputs.len() != 0 {
        return parse::Error::new(
            f.sig.inputs.span(),
            "`#[interrupt]` function should not include any parameter",
        )
        .to_compile_error()
        .into();
    }

    let valid_signature = f.sig.constness.is_none()
        && f.vis == Visibility::Inherited
        && f.sig.abi.is_none()
        && f.sig.inputs.is_empty()
        && f.sig.generics.params.is_empty()
        && f.sig.generics.where_clause.is_none()
        && f.sig.variadic.is_none()
        && match f.sig.output {
            ReturnType::Default => true,
            ReturnType::Type(_, ref ty) => match **ty {
                Type::Tuple(ref tuple) => tuple.elems.is_empty(),
                Type::Never(..) => true,
                _ => false,
            },
        };

    if !valid_signature {
        return parse::Error::new(
            f.sig.span(),
            "`#[interrupt]` handlers must have signature `[unsafe] fn() [-> !]`",
        )
        .to_compile_error()
        .into();
    }

    if let Some(syntax_err) = soc::check_interrupt_name(&f.sig.ident) {
        return syntax_err.to_compile_error().into();
    }

    let attrs = f.attrs;
    let unsafety = f.sig.unsafety;
    let stmts = f.block.stmts;
    let ident = f.sig.ident;
    let output = f.sig.output;

    #[cfg(feature = "nightly")]
    let no_mangle_attr = quote!(#[unsafe(no_mangle)]);
    #[cfg(not(feature = "nightly"))]
    let no_mangle_attr = quote!(#[no_mangle]);

    quote!(
        #(#attrs)*
        #no_mangle_attr
        pub #unsafety extern "C" fn #ident() #output {
            #(#stmts)*
        }
    )
    .into()
}

/// Exception handler function attribute.
///
/// Expected signature: `[unsafe] fn(&mut TrapFrame) [-> !]`.
///
/// The generated function is exported with symbol name `exceptions` so a single
/// entry point can be invoked by the trap trampoline. Only one such function
/// should be defined in a program.
#[proc_macro_attribute]
pub fn exception(args: TokenStream, input: TokenStream) -> TokenStream {
    if !args.is_empty() {
        return parse::Error::new(
            Span::call_site(),
            "#[exception] attribute accepts no arguments",
        )
        .to_compile_error()
        .into();
    }

    let f = parse_macro_input!(input as ItemFn);

    if f.sig.inputs.len() != 1 {
        return parse::Error::new(
            f.sig.inputs.span(),
            "`#[exception]` function should include exactly one parameter",
        )
        .to_compile_error()
        .into();
    }

    let valid_signature = f.sig.constness.is_none()
        && f.vis == Visibility::Inherited
        && f.sig.abi.is_none()
        && f.sig.generics.params.is_empty()
        && f.sig.generics.where_clause.is_none()
        && f.sig.variadic.is_none()
        && match f.sig.output {
            ReturnType::Default => true,
            ReturnType::Type(_, ref ty) => match **ty {
                Type::Tuple(ref tuple) => tuple.elems.is_empty(),
                Type::Never(..) => true,
                _ => false,
            },
        };

    if !valid_signature {
        return parse::Error::new(
            f.sig.span(),
            "`#[exception]` handlers must have signature `[unsafe] fn(&mut TrapFrame) [-> !]`",
        )
        .to_compile_error()
        .into();
    }

    let attrs = f.attrs;
    let unsafety = f.sig.unsafety;
    let stmts = f.block.stmts;
    let ident = f.sig.ident;
    let output = f.sig.output;
    let inputs = f.sig.inputs;

    #[cfg(feature = "nightly")]
    let export_attr = quote!(#[unsafe(export_name = "exceptions")]);
    #[cfg(not(feature = "nightly"))]
    let export_attr = quote!(#[export_name = "exceptions"]);

    quote!(
        #(#attrs)*
        #export_attr
        pub #unsafety extern "C" fn #ident(#inputs) #output {
            #(#stmts)*
        }
    )
    .into()
}
