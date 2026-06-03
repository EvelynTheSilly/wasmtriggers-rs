#![feature(proc_macro_diagnostic)]

use proc_macro::TokenStream;

mod chat_message;
mod init;
mod keyboard;
mod util;

#[proc_macro_attribute]
pub fn init_function(attr: TokenStream, item: TokenStream) -> TokenStream {
    init::init_function(attr, item)
}

#[proc_macro_attribute]
pub fn chat_message_handler(attr: TokenStream, item: TokenStream) -> TokenStream {
    chat_message::chat_message_handler(attr, item)
}

#[proc_macro_attribute]
pub fn on_press(attr: TokenStream, item: TokenStream) -> TokenStream {
    keyboard::on_press(attr, item)
}

#[proc_macro_attribute]
pub fn on_hold(attr: TokenStream, item: TokenStream) -> TokenStream {
    keyboard::on_hold(attr, item)
}

#[proc_macro_attribute]
pub fn on_release(attr: TokenStream, item: TokenStream) -> TokenStream {
    keyboard::on_release(attr, item)
}
