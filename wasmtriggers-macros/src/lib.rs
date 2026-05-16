#![feature(proc_macro_diagnostic)]

use proc_macro::TokenStream;

mod init;

#[proc_macro_attribute]
pub fn init_function(attr: TokenStream, item: TokenStream) -> TokenStream {
    init::init_function(attr, item)
}
