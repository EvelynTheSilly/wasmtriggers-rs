use crate::util::{assert_signature, hash::hash_function_name};
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{ItemFn, parse_macro_input, spanned::Spanned};

pub fn chat_message_handler(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    assert_signature(&input, &[syn::parse_quote!(ChatType)], None);
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
    let hashed_server_handler = format_ident!("server_message_handler__{}", hash);
    let hashed_chat_handler = format_ident!("chat_message_handler__{}", hash);
    let chatfn = input.sig.ident.clone();

    let server_entry_point = quote! {
        #[allow(non_snake_case)]
        #[unsafe(no_mangle)]
        fn #hashed_server_handler<'a> (msg_ptr: u32, msg_len: u32) {
            unsafe {
                let message =
                    ::core::str::from_utf8_unchecked(::core::slice::from_raw_parts(msg_ptr as *const u8, msg_len as usize));
                let chat = ::wasmtriggers_rs::core::chat::ChatType::Game {
                    message
                };
                #chatfn(&chat);
            }
        }
    };
    let chat_entry_point = quote! {
        #[allow(non_snake_case)]
        #[unsafe(no_mangle)]
        fn #hashed_chat_handler (name_ptr: u32, name_len: u32, msg_ptr: u32, msg_len:u32) {
            unsafe {
                let name =
                    ::core::str::from_utf8_unchecked(::core::slice::from_raw_parts(name_ptr as *const u8, name_len as usize));
                let message =
                    ::core::str::from_utf8_unchecked(::core::slice::from_raw_parts(msg_ptr as *const u8, msg_len as usize));
                let chat = ::wasmtriggers_rs::core::chat::ChatType::Player {
                    player: name,
                    message
                };
                #chatfn(&chat);
            }
        }
    };

    let expanded = quote! {
        #server_entry_point
        #chat_entry_point
        #input
    };
    expanded.into()
}
