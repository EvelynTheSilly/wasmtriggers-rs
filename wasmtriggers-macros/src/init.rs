use proc_macro::TokenStream;
use quote::quote;
use syn::{Attribute, ItemFn, parse_macro_input, parse_quote, spanned::Spanned};

pub fn init_function(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(item as ItemFn);
    let vis = &input.vis;
    let sig = &input.sig;
    if sig.abi.is_some() {
        sig.abi
            .span()
            .unwrap()
            .error("function shouldnt have a extern")
            .emit();
    }
    let no_mangle: Attribute = parse_quote! {
        #[unsafe(no_mangle)]
    };
    if !input
        .attrs
        .iter()
        .any(|attr| attr.path().is_ident("no_mangle"))
    {
        input.attrs.push(no_mangle);
    }
    let attrs = &input.attrs;
    let block = &input.block;
    let expanded = quote! { #(#attrs)* #vis extern "C" #sig #block };
    expanded.into()
}
