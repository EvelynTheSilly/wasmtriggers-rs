use crate::util::{assert_signature, hash::hash_function_name};
use proc_macro::TokenStream;
use quote::quote;
use syn::{ItemFn, parse_macro_input, parse_quote, spanned::Spanned};

pub fn on_press(attr: TokenStream, item: TokenStream) -> TokenStream {
    keyboard_macro(attr, item, "on_keypress")
}

pub fn on_hold(attr: TokenStream, item: TokenStream) -> TokenStream {
    keyboard_macro(attr, item, "on_keyhold")
}

pub fn on_release(attr: TokenStream, item: TokenStream) -> TokenStream {
    keyboard_macro(attr, item, "on_keyrelease")
}

fn keyboard_macro(attr: TokenStream, item: TokenStream, abi_prefix: &str) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);

    if input.sig.abi.is_some() {
        input
            .sig
            .abi
            .span()
            .unwrap()
            .error("function should not have an extern")
            .emit();
    }

    let keys_str = attr.to_string();
    let keys: Vec<&str> = keys_str
        .split(',')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .collect();

    let hash = hash_function_name(&input);
    let handler = input.sig.ident.clone();

    if keys.is_empty() {
        assert_signature(&input, &[parse_quote!(&str)], None);

        let fn_name = quote::format_ident!("{}__{}", abi_prefix, hash);
        let generated = quote! {
            #[allow(non_snake_case)]
            #[unsafe(no_mangle)]
            pub extern "C" fn #fn_name(ptr: u32, len: u32) {
                unsafe {
                    let key_str: &str = ::core::str::from_utf8_unchecked(
                        ::core::slice::from_raw_parts(ptr as *const u8, len as usize)
                    );
                    #handler(key_str);
                }
            }
        };

        quote! {
            #generated
            #input
        }
        .into()
    } else {
        assert_signature(&input, &[], None);

        let mut generated = Vec::new();
        for key in &keys {
            let fn_name = quote::format_ident!("{}_{}__{}", abi_prefix, key, hash);
            generated.push(quote! {
                #[allow(non_snake_case)]
                #[unsafe(no_mangle)]
                pub extern "C" fn #fn_name() {
                    #handler();
                }
            });
        }

        quote! {
            #(#generated)*
            #input
        }
        .into()
    }
}
