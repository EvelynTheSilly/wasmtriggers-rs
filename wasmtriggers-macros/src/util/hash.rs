use std::hash::{DefaultHasher, Hash, Hasher};
use syn::{Ident, ItemFn};

pub fn hash_function_name(fn_item: &ItemFn) -> Ident {
    let mut hasher = DefaultHasher::new();

    fn_item.sig.ident.to_string().hash(&mut hasher);
    fn_item.sig.asyncness.is_some().hash(&mut hasher);

    for input in &fn_item.sig.inputs {
        let ts = quote::quote!(#input);
        ts.to_string().hash(&mut hasher);
    }

    {
        let ts = quote::quote!(#fn_item);
        ts.to_string().hash(&mut hasher);
    }

    let hash = hasher.finish();
    let name = format!("{}__{:016x}", fn_item.sig.ident, hash);

    Ident::new(&name, fn_item.sig.ident.span())
}
