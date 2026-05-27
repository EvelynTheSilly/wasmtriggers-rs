use syn::{FnArg, ItemFn, ReturnType, Type, spanned::Spanned};

pub fn assert_signature(func: &ItemFn, expected_inputs: &[Type], expected_output: Option<&Type>) {
    let sig = &func.sig;

    // Check parameter count
    if sig.inputs.len() != expected_inputs.len() {
        sig.inputs.span().unwrap().error(format!(
            "expected {} parameters, found {}",
            expected_inputs.len(),
            sig.inputs.len()
        ));
    }

    // Check parameter types
    for (arg, expected) in sig.inputs.iter().zip(expected_inputs.iter()) {
        match arg {
            FnArg::Typed(pat_ty) => {
                if !type_eq(&pat_ty.ty, expected) {
                    pat_ty.span().unwrap().error(format!(
                        "expected parameter type `{}`, found `{}`",
                        quote::quote!(#expected),
                        quote::quote!(#pat_ty.ty),
                    ));
                }
            }

            FnArg::Receiver(recv) => {
                recv.span().unwrap().error("unexpected self receiver");
            }
        }
    }

    // Check return type
    match (&sig.output, expected_output) {
        (ReturnType::Default, None) => {}

        (ReturnType::Type(_, actual), Some(expected)) => {
            if !type_eq(actual, expected) {
                actual.span().unwrap().error(format!(
                    "expected return type `{}`, found `{}`",
                    quote::quote!(#expected),
                    quote::quote!(#actual),
                ));
            }
        }

        (ReturnType::Default, Some(expected)) => {
            sig.output.span().unwrap().error(format!(
                "expected return type `{}`",
                quote::quote!(#expected),
            ));
        }

        (ReturnType::Type(_, actual), None) => {
            actual.span().unwrap().error("expected no return type");
        }
    }
}

fn type_eq(a: &Type, b: &Type) -> bool {
    quote::quote!(#a).to_string() == quote::quote!(#b).to_string()
}
