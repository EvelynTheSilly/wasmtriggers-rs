use crate::util::assert_signature;
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
    let mut input = parse_macro_input!(item as ItemFn);

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

    let internal_ident =
        quote::format_ident!("__internal__{}", input.sig.ident);
    input.sig.ident = internal_ident.clone();

    input.attrs.push(syn::parse_quote!(#[allow(non_snake_case)]));
    let attrs = &input.attrs;
    let vis = &input.vis;
    let sig = &input.sig;
    let block = &input.block;

    if keys.is_empty() {
        assert_signature(&input, &[parse_quote!(&str)], None);

        let fn_name = quote::format_ident!("{}", abi_prefix);
        let generated = quote! {
            #[unsafe(no_mangle)]
            pub extern "C" fn #fn_name(ptr: u32, len: u32) {
                unsafe {
                    let key_str: &str = ::core::str::from_utf8_unchecked(
                        ::core::slice::from_raw_parts(ptr as *const u8, len as usize)
                    );
                    #internal_ident(key_str);
                }
            }
        };

        quote! {
            #generated
            #(#attrs)* #vis #sig #block
        }
        .into()
    } else {
        assert_signature(&input, &[], None);

        let mut generated = Vec::new();
        for key in &keys {
            let fn_name = quote::format_ident!("{}_{}", abi_prefix, key);
            generated.push(quote! {
                #[unsafe(no_mangle)]
                pub extern "C" fn #fn_name() {
                    #internal_ident();
                }
            });
        }

        quote! {
            #(#generated)*
            #(#attrs)* #vis #sig #block
        }
        .into()
    }
}
