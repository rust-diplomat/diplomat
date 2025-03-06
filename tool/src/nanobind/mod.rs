mod binding;
mod formatter;
mod ty;

use std::{borrow::Cow, collections::HashSet, path::Path};

use crate::{ErrorStore, FileMap};
use binding::Binding;
use diplomat_core::hir::{self, BackendAttrSupport};
use formatter::PyFormatter;
use serde::{Deserialize, Serialize};
use ty::TyGenContext;

// Python support using the nanobind c++ library to create a python binding.
//
// The generated nanobind.cpp files requires linking with nanobind
// See the feature_test project for an example of a pyproject & CMakeLists.txt which can be compiled
// using pip install. Compilation requires a C++ compiler & CMake, as well as a downloaded
// copy of libnanobind.

pub(crate) fn attr_support() -> BackendAttrSupport {
    let mut a = BackendAttrSupport::default();

    a.namespacing = true;
    a.memory_sharing = true;
    a.non_exhaustive_structs = false;
    a.method_overloading = true;
    a.utf8_strings = true;
    a.utf16_strings = true;
    a.static_slices = true;

    a.constructors = false; // TODO
    a.named_constructors = false;
    a.fallible_constructors = false;
    a.accessors = false;
    a.comparators = false; // TODO
    a.stringifiers = false; // TODO
    a.iterators = false; // TODO
    a.iterables = false; // TODO
    a.indexing = false; // TODO
    a.option = true;
    a.callbacks = true;
    a.traits = false;

    a
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct PythonConfig {
    lib_name: String,
}

pub(crate) fn run<'tcx>(
    tcx: &'tcx hir::TypeContext,
    conf_path: Option<&Path>,
) -> (FileMap, ErrorStore<'tcx, String>) {
    let conf_path = conf_path.expect("Nanobind library needs to be called with config");

    let conf_str = std::fs::read_to_string(conf_path)
        .unwrap_or_else(|err| panic!("Failed to open config file {conf_path:?}: {err}"));
    let PythonConfig { lib_name } = toml::from_str::<PythonConfig>(&conf_str)
        .expect("Failed to parse config. Required field is 'lib_name'");

    let files = FileMap::default();
    let formatter = PyFormatter::new(tcx);
    let errors = ErrorStore::default();

    let nanobind_filepath = format!("{lib_name}_ext.cpp");
    let mut binding = Binding::new();
    binding.module_name = Cow::from(lib_name);

    let mut submodules = HashSet::<Cow<str>>::new();
    for (id, ty) in tcx.all_types() {
        if ty.attrs().disable {
            // Skip type if disabled
            continue;
        }

        let _type_name_unnamespaced = formatter.fmt_type_name(id);
        let decl_header_path = formatter.fmt_decl_header_path(id);
        let impl_file_path = formatter.fmt_impl_file_path(id);

        let mut context = TyGenContext {
            formatter: &formatter,
            errors: &errors,
            c2: crate::c::TyGenContext {
                tcx,
                formatter: &formatter.c,
                errors: &errors,
                is_for_cpp: false,
                id: id.into(),
                decl_header_path: &decl_header_path,
                impl_header_path: &impl_file_path,
            },
            binding: &mut binding,
            submodules: &mut submodules,
            generating_struct_fields: false,
        };

        context
            .binding
            .includes
            .insert(impl_file_path.clone().into());

        let guard = errors.set_context_ty(ty.name().as_str().into());
        match ty {
            hir::TypeDef::Enum(o) => context.gen_enum_def(o, id),
            hir::TypeDef::Opaque(o) => context.gen_opaque_def(o, id),
            hir::TypeDef::Struct(s) => context.gen_struct_def(s, id),
            hir::TypeDef::OutStruct(s) => context.gen_struct_def(s, id),
            _ => unreachable!("unknown AST/HIR variant"),
        }
        drop(guard);
    }
    files.add_file(nanobind_filepath.to_owned(), binding.to_string());

    // Write out wheel metadata files
    {
        // #[derive(Template)]
        // #[template(path = "nanobind/pyproject.toml.jinja", escape = "none")]
        // struct ImplTemplate<'a> {
        //     _ty: &'a hir::EnumDef,
        //     _fmt: &'a PyFormatter<'a>,
        //     type_name: &'a str,
        //     _ctype: &'a str,
        //     values: &'a [&'a EnumVariant],
        //     module: &'a str,
        //     modules: Vec<(Cow<'a, str>, Cow<'a, str>)>,
        // }

        // ImplTemplate {
        //     _ty: ty,
        //     _fmt: self.formatter,
        //     type_name: &type_name,
        //     _ctype: &ctype,
        //     values: values.as_slice(),
        //     module: self.formatter.fmt_module(id).borrow(),
        //     modules: self.get_module_defs(id, None),
        // }
        // .render_into(self.binding)
        // .unwrap();
    }
    (files, errors)
}

#[cfg(test)]
mod test {
    use diplomat_core::hir::{self, TypeDef};
    use quote::quote;
    use std::borrow::Cow;
    use std::collections::HashSet;

    #[test]
    fn test_opaque_gen() {
        let tokens = quote! {
            #[diplomat::bridge]
            #[diplomat::attr(auto, namespace = "mylib")]
            mod ffi {

                #[diplomat::opaque]
                struct OpaqueStruct;

                impl OpaqueStruct {
                    pub fn new() -> Box<OpaqueStruct> {
                        Box::new(OpaqueStruct{})
                    }

                    pub fn do_thing() -> bool {
                        return true;
                    }
                }
            }
        };
        let item = syn::parse2::<syn::File>(tokens).expect("failed to parse item ");

        let mut attr_validator = hir::BasicAttributeValidator::new("python");
        attr_validator.support = crate::nanobind::attr_support();

        let tcx = match hir::TypeContext::from_syn(&item, attr_validator) {
            Ok(context) => context,
            Err(e) => {
                for (_cx, err) in e {
                    eprintln!("Lowering error: {}", err);
                }
                panic!("Failed to create context")
            }
        };

        let (type_id, opaque_def) = match tcx
            .all_types()
            .next()
            .expect("Failed to generate first opaque def")
        {
            (type_id, TypeDef::Opaque(opaque_def)) => (type_id, opaque_def),
            _ => panic!("Failed to find opaque type from AST"),
        };

        let formatter = crate::nanobind::PyFormatter::new(&tcx);
        let errors = crate::ErrorStore::default();
        let mut binding = crate::nanobind::Binding::new();
        binding.module_name = std::borrow::Cow::Borrowed("pymod");

        let decl_header_path = formatter.fmt_decl_header_path(type_id);
        let impl_file_path = formatter.fmt_impl_file_path(type_id);

        let mut submodules = HashSet::<Cow<str>>::new();
        let mut context = crate::nanobind::TyGenContext {
            formatter: &formatter,
            errors: &errors,
            c2: crate::c::TyGenContext {
                tcx: &tcx,
                formatter: &formatter.c,
                errors: &errors,
                is_for_cpp: false,
                id: type_id.into(),
                decl_header_path: &decl_header_path,
                impl_header_path: &impl_file_path,
            },
            binding: &mut binding,
            generating_struct_fields: false,
            submodules: &mut submodules,
        };

        context.gen_opaque_def(opaque_def, type_id);
        let generated = binding.to_string();
        insta::assert_snapshot!(generated)
    }

    #[test]
    fn test_enum_gen() {
        let tokens = quote! {
            #[diplomat::bridge]
            #[diplomat::attr(auto, namespace = "mylib")]
            mod ffi {

                #[diplomat::enum_convert(my_thingy::SpeedSetting)]
                pub enum SpeedSetting {
                    Fast, Medium, Slow
                }
            }
        };
        let item = syn::parse2::<syn::File>(tokens).expect("failed to parse item ");

        let mut attr_validator = hir::BasicAttributeValidator::new("python");
        attr_validator.support = crate::nanobind::attr_support();

        let tcx = match hir::TypeContext::from_syn(&item, attr_validator) {
            Ok(context) => context,
            Err(e) => {
                for (_cx, err) in e {
                    eprintln!("Lowering error: {}", err);
                }
                panic!("Failed to create context")
            }
        };

        let (type_id, enum_def) = match tcx
            .all_types()
            .next()
            .expect("Failed to generate first opaque def")
        {
            (type_id, TypeDef::Enum(enum_def)) => (type_id, enum_def),
            _ => panic!("Failed to find opaque type from AST"),
        };

        let formatter = crate::nanobind::PyFormatter::new(&tcx);
        let errors = crate::ErrorStore::default();
        let mut binding = crate::nanobind::Binding::new();
        binding.module_name = std::borrow::Cow::Borrowed("pymod");

        let decl_header_path = formatter.fmt_decl_header_path(type_id);
        let impl_file_path = formatter.fmt_impl_file_path(type_id);

        let mut submodules = HashSet::<Cow<str>>::new();
        let mut context = crate::nanobind::TyGenContext {
            formatter: &formatter,
            errors: &errors,
            c2: crate::c::TyGenContext {
                tcx: &tcx,
                formatter: &formatter.c,
                errors: &errors,
                is_for_cpp: false,
                id: type_id.into(),
                decl_header_path: &decl_header_path,
                impl_header_path: &impl_file_path,
            },
            binding: &mut binding,
            generating_struct_fields: false,
            submodules: &mut submodules,
        };

        context.gen_enum_def(enum_def, type_id);
        let generated = binding.to_string();
        insta::assert_snapshot!(generated)
    }

    #[test]
    fn test_struct_gen() {
        let tokens = quote! {
            #[diplomat::bridge]
            #[diplomat::attr(auto, namespace = "mylib")]
            mod ffi {
                pub struct Thingy {
                    pub a: bool,
                    pub b: u8,
                    pub c: f64,
                }
            }
        };
        let item = syn::parse2::<syn::File>(tokens).expect("failed to parse item ");

        let mut attr_validator = hir::BasicAttributeValidator::new("python");
        attr_validator.support = crate::nanobind::attr_support();

        let tcx = match hir::TypeContext::from_syn(&item, attr_validator) {
            Ok(context) => context,
            Err(e) => {
                for (_cx, err) in e {
                    eprintln!("Lowering error: {}", err);
                }
                panic!("Failed to create context")
            }
        };

        let (type_id, struct_def) = match tcx
            .all_types()
            .next()
            .expect("Failed to generate first opaque def")
        {
            (type_id, TypeDef::Struct(struct_def)) => (type_id, struct_def),
            _ => panic!("Failed to find opaque type from AST"),
        };

        let formatter = crate::nanobind::PyFormatter::new(&tcx);
        let errors = crate::ErrorStore::default();
        let mut binding = crate::nanobind::Binding::new();
        binding.module_name = std::borrow::Cow::Borrowed("pymod");

        let decl_header_path = formatter.fmt_decl_header_path(type_id);
        let impl_file_path = formatter.fmt_impl_file_path(type_id);

        let mut submodules = HashSet::<Cow<str>>::new();
        let mut context = crate::nanobind::TyGenContext {
            formatter: &formatter,
            errors: &errors,
            c2: crate::c::TyGenContext {
                tcx: &tcx,
                formatter: &formatter.c,
                errors: &errors,
                is_for_cpp: false,
                id: type_id.into(),
                decl_header_path: &decl_header_path,
                impl_header_path: &impl_file_path,
            },
            binding: &mut binding,
            generating_struct_fields: false,
            submodules: &mut submodules,
        };

        context.gen_struct_def(struct_def, type_id);
        let generated = binding.to_string();
        insta::assert_snapshot!(generated)
    }
}
