use proc_macro::TokenStream;
use quote::quote;
use syn::{ItemFn, parse_macro_input, parse_quote, spanned::Spanned};

pub fn chat_message_handler(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(item as ItemFn);
    input.sig.ident = parse_quote!(__internal_chat_message_handler);
    let vis = &input.vis;
    let sig = &input.sig;
    if sig.abi.is_some() {
        sig.abi
            .span()
            .unwrap()
            .error("function shouldnt have a extern")
            .emit();
    }
    let attrs = &input.attrs;
    let block = &input.block;
    let expanded = quote! {
    #[unsafe(no_mangle)]
    pub extern "C" fn server_message_handler(msg_ptr: u32, msg_len:u32){
        unsafe {
            let message =
                ::core::str::from_utf8_unchecked(::core::slice::from_raw_parts(msg_ptr as *const u8, msg_len as usize));
            let chat = ::wasmtriggers_rs::core::chat::ChatType::Game {
                message
            };
            __internal_chat_message_handler(&chat);
            }
        }
    #[unsafe(no_mangle)]
    pub extern "C" fn chat_message_handler(name_ptr: u32, name_len: u32, msg_ptr: u32, msg_len:u32){
        unsafe {
            let name =
                ::core::str::from_utf8_unchecked(::core::slice::from_raw_parts(name_ptr as *const u8, name_len as usize));
            let message =
                ::core::str::from_utf8_unchecked(::core::slice::from_raw_parts(msg_ptr as *const u8, msg_len as usize));
            let chat = ::wasmtriggers_rs::core::chat::ChatType::Player{
                player: name,
                message
            };
            __internal_chat_message_handler(&chat);
        }
    }
    #(#attrs)* #vis #sig #block
    };
    expanded.into()
}
