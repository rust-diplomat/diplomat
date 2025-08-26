mod formatter;
mod func;
mod header;
mod ty;

pub use self::formatter::CFormatter;
pub(crate) use self::formatter::CAPI_NAMESPACE;
pub use self::func::FuncGenContext;
pub(crate) use self::header::Header;
pub use self::ty::TyGenContext;

use crate::{ErrorStore, FileMap};
use diplomat_core::hir::BackendAttrSupport;
use diplomat_core::hir::{self, DocsUrlGenerator};

pub(crate) fn attr_support() -> BackendAttrSupport {
    let mut a = BackendAttrSupport::default();

    a.namespacing = false;
    a.memory_sharing = true;
    a.non_exhaustive_structs = false;
    a.method_overloading = false;
    a.utf8_strings = true;
    a.utf16_strings = true;
    a.static_slices = true;

    a.constructors = false;
    a.named_constructors = false;
    a.fallible_constructors = false;
    a.accessors = false;
    a.static_accessors = false;
    a.comparators = false;
    a.stringifiers = false;
    a.iterators = false;
    a.iterables = false;
    a.indexing = false;
    a.arithmetic = false;
    a.option = true;
    a.callbacks = true;
    a.traits = true;
    a.custom_errors = false;
    a.traits_are_send = false;
    a.traits_are_sync = false;
    a.generate_mocking_interface = false;
    a.abi_compatibles = true;
    a.struct_refs = true;
    a.free_functions = true;

    a
}

#[derive(askama::Template)]
#[template(path = "c/runtime.h.jinja", escape = "none")]
pub struct Runtime;

pub(crate) fn run<'tcx>(
    tcx: &'tcx hir::TypeContext,
    docs_url_gen: &'tcx DocsUrlGenerator,
) -> (FileMap, ErrorStore<'tcx, String>) {
    let files = FileMap::default();
    let formatter = CFormatter::new(tcx, false, docs_url_gen);
    let errors = ErrorStore::default();

    files.add_file("diplomat_runtime.h".into(), Runtime.to_string());

    for (id, ty) in tcx.all_types() {
        if ty.attrs().disable {
            // Skip type if disabled
            continue;
        }

        let decl_header_path = formatter.fmt_decl_header_path(id.into());
        let impl_header_path = formatter.fmt_impl_header_path(id.into());

        let _guard = errors.set_context_ty(ty.name().as_str().into());
        let context = TyGenContext {
            tcx,
            formatter: &formatter,
            errors: &errors,
            is_for_cpp: false,
            id: id.into(),
            decl_header_path: &decl_header_path,
            impl_header_path: &impl_header_path,
        };

        let decl_header = match ty {
            hir::TypeDef::Enum(e) => context.gen_enum_def(e),
            hir::TypeDef::Opaque(o) => context.gen_opaque_def(o),
            hir::TypeDef::Struct(s) => context.gen_struct_def(s),
            hir::TypeDef::OutStruct(s) => context.gen_struct_def(s),
            _ => unreachable!("unknown AST/HIR variant"),
        };

        let impl_header = context.gen_impl(ty);

        files.add_file(decl_header_path, decl_header.to_string());
        files.add_file(impl_header_path, impl_header.to_string());
    }

    for (id, trt) in tcx.all_traits() {
        if trt.attrs.disable {
            // Skip type if disabled
            continue;
        }

        let decl_header_path = formatter.fmt_decl_header_path(id.into());
        let impl_header_path = formatter.fmt_impl_header_path(id.into());

        let _guard = errors.set_context_ty(trt.name.as_str().into());
        let context = TyGenContext {
            tcx,
            formatter: &formatter,
            errors: &errors,
            is_for_cpp: false,
            id: id.into(),
            decl_header_path: &decl_header_path,
            impl_header_path: &impl_header_path,
        };

        let decl_header = context.gen_trait_def(trt);
        files.add_file(decl_header_path, decl_header.to_string());
    }
    // loop over traits too

    // Loop over free functions, put them all in one file (currently this is diplomat_runtime.h):
    let header = Header::new("diplomat_free_functions.h".into(), false);

    let mut impl_context = FuncGenContext::new(header, false);

    {
        let mut should_render = false;
        for f in tcx.all_free_functions() {
            if f.attrs.disable {
                continue;
            }
            should_render = true;
            let context = TyGenContext {
                tcx,
                formatter: &formatter,
                errors: &errors,
                is_for_cpp: false,
                id: hir::SymbolId::Function,
                decl_header_path: "diplomat_free_functions.d.h",
                impl_header_path: "diplomat_free_functions.h",
            };

            impl_context.gen_method(f, &context);
        }

        if should_render {
            impl_context.render(None, None).unwrap();
            files.add_file(
                "diplomat_free_functions.h".into(),
                impl_context.header.to_string(),
            );
        }
    }

    (files, errors)
}
