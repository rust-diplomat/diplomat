mod formatter;
mod gen;
mod header;

use askama::Template;
pub(crate) use header::Header;
use std::{collections::HashMap, fmt::Write};

use crate::{read_custom_binding, ErrorStore, FileMap};
use diplomat_core::hir::{self, BackendAttrSupport, DocsUrlGenerator};
pub(crate) use gen::ItemGenContext;

pub(crate) use formatter::Cpp2Formatter;

#[derive(Default, Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CppConfig {}

impl CppConfig {
    pub fn set(&mut self, key: &str, _value: toml::Value) {
        panic!("C++ does not support any backend-specific configs, found {key}");
    }
}
pub(crate) fn attr_support() -> BackendAttrSupport {
    let mut a = BackendAttrSupport::default();

    a.namespacing = true;
    a.memory_sharing = true;
    a.non_exhaustive_structs = false;
    a.method_overloading = true;
    a.utf8_strings = true;
    a.utf16_strings = true;
    a.static_slices = true;
    a.defaults = true;

    a.constructors = false; // TODO
    a.named_constructors = false;
    a.fallible_constructors = false;
    a.accessors = false;
    a.static_accessors = false;
    a.comparators = true;
    a.stringifiers = false; // TODO
    a.iterators = true;
    a.iterables = true;
    a.indexing = true;
    a.arithmetic = true;
    a.option = true;
    a.callbacks = true;
    a.traits = false;
    a.custom_errors = false;
    a.traits_are_send = false;
    a.traits_are_sync = false;
    a.generate_mocking_interface = false;
    a.abi_compatibles = true;
    a.struct_refs = true;
    a.free_functions = true;
    a.custom_bindings = true;

    a
}

pub(crate) fn run<'tcx>(
    tcx: &'tcx hir::TypeContext,
    config: &crate::Config,
    docs_url_gen: &'tcx DocsUrlGenerator,
) -> (FileMap, ErrorStore<'tcx, String>) {
    let files = FileMap::default();
    let formatter = Cpp2Formatter::new(tcx, config, docs_url_gen);
    let errors = ErrorStore::default();

    #[derive(askama::Template)]
    #[template(path = "cpp/runtime.hpp.jinja", escape = "none")]
    struct Runtime<'a> {
        guard_prefix: &'a str,
        lib_name: Option<&'a str>,
    }
    let lib_name = config.shared_config.lib_name.as_deref();
    let include_guard_prefix = lib_name
        .map(|x| format!("{}_", x.to_ascii_uppercase()))
        .unwrap_or_default();
    let runtime = Runtime {
        guard_prefix: &include_guard_prefix,
        lib_name,
    };
    files.add_file("diplomat_runtime.hpp".into(), runtime.to_string());

    for (id, ty) in tcx.all_types() {
        let ty_attrs = ty.attrs();
        if ty_attrs.disable {
            // Skip type if disabled
            continue;
        }
        let type_name_unnamespaced = formatter.fmt_type_name(id);
        let decl_header_path = formatter.fmt_decl_header_path(id.into());
        let mut decl_header = header::Header::new(decl_header_path.clone(), lib_name);

        let impl_header_path = formatter.fmt_impl_header_path(id.into());
        let mut impl_header = header::Header::new(impl_header_path.clone(), lib_name);

        let mut context = ItemGenContext {
            formatter: &formatter,
            errors: &errors,
            config: &config.cpp_config,
            c: crate::c::ItemGenContext {
                tcx,
                formatter: &formatter.c,
                errors: &errors,
                is_for_cpp: true,
                decl_header_path: &decl_header_path,
                impl_header_path: &impl_header_path,
            },
            decl_header: &mut decl_header,
            impl_header: &mut impl_header,
            generating_struct_fields: false,
        };
        context.impl_header.decl_include = Some(decl_header_path.clone());

        let block_source = if let Some(s) = ty_attrs
            .binding_includes
            .get(&hir::IncludeLocation::DefBlock)
        {
            read_custom_binding(s, config, &errors).unwrap_or_default()
        } else {
            Default::default()
        };

        let guard = errors.set_context_ty(ty.name().as_str().into());
        match id {
            hir::TypeId::Enum(e_id) => context.gen_enum_def(e_id),
            hir::TypeId::Opaque(o_id) => context.gen_opaque_def(o_id, block_source),
            hir::TypeId::Struct(s_id) => context.gen_struct_def::<hir::Everywhere>(s_id),
            hir::TypeId::OutStruct(s_id) => context.gen_struct_def::<hir::OutputOnly>(s_id),

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

        // Decl headers require some more special logic, but we can write to the impl header body directly:
        if let Some(s) = ty_attrs
            .binding_includes
            .get(&hir::IncludeLocation::ImplBlock)
        {
            if let Ok(s) = read_custom_binding(s, config, &errors) {
                writeln!(impl_header, "{}", s).expect("Could not write to header.");
            }
        }

        files.add_file(decl_header_path, decl_header.to_string());
        files.add_file(impl_header_path, impl_header.to_string());
    }

    {
        // Group free functions by namespace, removing those which are disabled
        let mut free_func_map = HashMap::<_, Vec<_>>::new();
        for e in tcx.all_free_functions() {
            if !e.1.attrs.disable {
                free_func_map
                    .entry(e.1.attrs.namespace.clone())
                    .or_default()
                    .push(e);
            }
        }

        for (ns, funcs) in free_func_map {
            let impl_header_path = formatter.fmt_free_function_header_path(ns.clone());

            let mut free_func_impl_header = header::Header::new(impl_header_path.clone(), lib_name);

            let mut ty_context = ItemGenContext {
                formatter: &formatter,
                errors: &errors,
                config: &config.cpp_config,
                c: crate::c::ItemGenContext {
                    tcx,
                    formatter: &formatter.c,
                    errors: &errors,
                    is_for_cpp: true,
                    impl_header_path: &impl_header_path,
                    decl_header_path: "",
                },
                impl_header: &mut free_func_impl_header,
                decl_header: &mut Header::new(Default::default(), None),
                generating_struct_fields: false,
            };

            let c_header = ty_context
                .c
                .gen_function_impls(None, funcs.iter().map(|(_, m)| *m));

            let methods = funcs
                .into_iter()
                .filter_map(|(id, func)| ty_context.gen_method_info(id.into(), func))
                .collect();

            crate::cpp::gen::FuncImplTemplate {
                namespace: ns.clone(),
                fmt: &formatter,
                methods,
                c_header,
            }
            .render_into(&mut free_func_impl_header)
            .unwrap();

            files.add_file(impl_header_path, free_func_impl_header.to_string());
        }
    }

    (files, errors)
}

#[cfg(test)]
mod test {

    use diplomat_core::hir::TypeId;
    use quote::quote;

    use crate::cpp::header;
    use crate::ErrorStore;

    use super::{formatter::test::new_tcx, formatter::Cpp2Formatter, ItemGenContext};

    #[test]
    fn test_rename_param() {
        let tk_stream = quote! {
            #[diplomat::bridge]
            mod ffi {
                #[diplomat::opaque]
                struct MyStruct(u64);

                impl MyStruct {
                    pub fn new(&self, default: u8) {
                        self.0 = default;
                    }
                }
            }
        };

        let tcx = new_tcx(tk_stream);
        let mut all_types = tcx.all_types();
        let config = crate::Config::default();
        if let (TypeId::Opaque(id), _) = all_types
            .next()
            .expect("Failed to generate first opaque def")
        {
            let error_store = ErrorStore::default();
            let docs_gen = Default::default();
            let formatter = Cpp2Formatter::new(&tcx, &config, &docs_gen);
            let mut decl_header = header::Header::new("decl_thing".into(), None);
            let mut impl_header = header::Header::new("impl_thing".into(), None);

            let mut ty_gen_cx = ItemGenContext {
                errors: &error_store,
                formatter: &formatter,
                config: &config.cpp_config,
                c: crate::c::ItemGenContext {
                    tcx: &tcx,
                    formatter: &formatter.c,
                    errors: &error_store,
                    is_for_cpp: true,
                    decl_header_path: "test/",
                    impl_header_path: "test/",
                },
                decl_header: &mut decl_header,
                impl_header: &mut impl_header,
                generating_struct_fields: false,
            };

            ty_gen_cx.gen_opaque_def(id, "".into());
            insta::assert_snapshot!(decl_header.body);
            insta::assert_snapshot!(impl_header.body);
        }
    }
}
