use proc_macro::TokenStream;
use quote::quote;
use syn::{ItemFn, parse_macro_input, spanned::Spanned};

pub fn init_function(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    let attrs = &input.attrs;
    let vis = &input.vis;
    let sig = &input.sig;
    if sig.abi.is_some() {
        sig.abi
            .span()
            .unwrap()
            .error("function shouldnt have a extern")
            .emit();
    }
    let block = &input.block;
    let expanded = quote! { #(#attrs)* #vis extern "C" #sig #block };
    expanded.into()
}
