mod formatter;
mod header;
mod ty;

use crate::common::{ErrorStore, FileMap};
use diplomat_core::hir::{self, BackendAttrSupport};
use formatter::Cpp2Formatter;
use ty::TyGenContext;

pub(crate) fn attr_support() -> BackendAttrSupport {
    let mut a = BackendAttrSupport::default();

    a.memory_sharing = true;
    a.non_exhaustive_structs = false;
    a.method_overloading = true;
    a.fallible_constructors = false;

    a
}

pub(crate) fn run(tcx: &hir::TypeContext) -> (FileMap, ErrorStore<String>) {
    let files = FileMap::default();
    let formatter = Cpp2Formatter::new(tcx);
    let errors = ErrorStore::default();

    files.add_file(
        "diplomat_c_runtime.hpp".into(),
        crate::c2::gen_runtime(true),
    );

    files.add_file(
        "diplomat_runtime.hpp".into(),
        include_str!("../../templates/cpp2/runtime.hpp").into(),
    );

    for (id, ty) in tcx.all_types() {
        if ty.attrs().disable {
            // Skip type if disabled
            continue;
        }
        let type_name_unnamespaced = formatter.fmt_type_name(id);
        let decl_header_path = formatter.fmt_decl_header_path(id);
        let mut decl_header = header::Header::new(decl_header_path.clone());
        let impl_header_path = formatter.fmt_impl_header_path(id);
        let mut impl_header = header::Header::new(impl_header_path.clone());

        let mut context = TyGenContext {
            formatter: &formatter,
            errors: &errors,
            c: crate::c2::TyGenContext {
                tcx,
                formatter: &formatter.c,
                errors: &errors,
                is_for_cpp: true,
                id,
                decl_header_path: &decl_header_path,
                impl_header_path: &impl_header_path,
            },
            decl_header: &mut decl_header,
            impl_header: &mut impl_header,
            generating_struct_fields: false,
        };
        context.impl_header.decl_include = Some(decl_header_path.clone());

        let guard = errors.set_context_ty(ty.name().as_str().into());
        match ty {
            hir::TypeDef::Enum(o) => context.gen_enum_def(o, id),
            hir::TypeDef::Opaque(o) => context.gen_opaque_def(o, id),
            hir::TypeDef::Struct(s) => context.gen_struct_def(s, id),
            hir::TypeDef::OutStruct(s) => context.gen_struct_def(s, id),
            _ => unreachable!("unknown AST/HIR variant"),
        }
        drop(guard);

        // In some cases like generating decls for `self` parameters,
        // a header will get its own forwards and includes. Instead of
        // trying to avoid pushing them, it's cleaner to just pull them out
        // once done
        context.decl_header.rm_forward(ty, &type_name_unnamespaced);
        context.impl_header.rm_forward(ty, &type_name_unnamespaced);
        context.decl_header.includes.remove(&*decl_header_path);
        context.impl_header.includes.remove(&*impl_header_path);
        context.impl_header.includes.remove(&*decl_header_path);

        files.add_file(decl_header_path, decl_header.to_string());
        files.add_file(impl_header_path, impl_header.to_string());
    }

    (files, errors)
}
