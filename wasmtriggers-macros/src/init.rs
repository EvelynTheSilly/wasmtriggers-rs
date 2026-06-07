use crate::util::{assert_signature, hash::hash_function_name};
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{ItemFn, parse_macro_input, spanned::Spanned};

pub fn init_function(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    assert_signature(&input, &[], None);
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
    let hashed_init_handler = format_ident!("init_handler__{}", hash);
    let initfn = input.sig.ident.clone();
    let fn_call = match input.sig.asyncness {
        Some(_) => {
            quote! {
                ::wasmtriggers_rs::core::executor::ASYNC_EXECUTOR.spawn(async {
                    #initfn().await;
                })
            }
        }
        None => {
            quote! {
                #initfn()
            }
        }
    };
    let abi_entry_point = quote! {
        #[allow(non_snake_case)]
        #[unsafe(no_mangle)]
        fn #hashed_init_handler () {
            #fn_call
        }
    };

    let expanded = quote! {
        #abi_entry_point
        #input
    };
    expanded.into()
}
