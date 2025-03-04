use diplomat_core::hir;
use diplomat_core::hir::TypeContext;
use proc_macro2::TokenStream;

pub(crate) fn new_tcx(
    backend_name: &str,
    backend_attr_support: hir::BackendAttrSupport,
    tk_stream: TokenStream,
) -> TypeContext {
    let file = syn::parse2::<syn::File>(tk_stream).expect("failed to parse item ");

    let mut attr_validator = hir::BasicAttributeValidator::new(backend_name);
    attr_validator.support = backend_attr_support;

    match TypeContext::from_syn(&file, attr_validator) {
        Ok(context) => context,
        Err(e) => {
            for (_cx, err) in e {
                eprintln!("Lowering error: {}", err);
            }
            panic!("Failed to create context")
        }
    }
}
