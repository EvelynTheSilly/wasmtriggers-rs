#![feature(proc_macro_diagnostic)]

use proc_macro::TokenStream;

mod chat_message;
mod init;

#[proc_macro_attribute]
pub fn init_function(attr: TokenStream, item: TokenStream) -> TokenStream {
    init::init_function(attr, item)
}

#[proc_macro_attribute]
pub fn chat_message_handler(attr: TokenStream, item: TokenStream) -> TokenStream {
    chat_message::chat_message_handler(attr, item)
}
