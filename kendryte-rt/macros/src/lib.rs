use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::spanned::Spanned;
use syn::{ItemFn, ReturnType, Type, Visibility, parse, parse_macro_input};

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
