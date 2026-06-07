use crate::util::{assert_signature, hash::hash_function_name};
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, parse_quote, ItemFn, spanned::Spanned};

pub fn on_tick(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    assert_signature(&input, &[parse_quote!(u32)], None);
    if input.sig.abi.is_some() {
        input
            .sig
            .abi
            .span()
            .unwrap()
            .error("function shouldnt have a extern")
            .emit();
    }
    let hash = hash_function_name(&input);
    let hashed_tick_handler = format_ident!("on_tick__{}", hash);
    let tickfn = input.sig.ident.clone();
    let abi_entry_point = quote! {
        #[allow(non_snake_case)]
        #[unsafe(no_mangle)]
        fn #hashed_tick_handler (tick: u32) {
            #tickfn(tick)
        }
    };

    let expanded = quote! {
        #abi_entry_point
        #input
    };
    expanded.into()
}
