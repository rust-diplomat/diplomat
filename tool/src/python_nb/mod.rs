mod binding;
mod formatter;
mod ty;

use std::{borrow::Cow, collections::HashSet};

use crate::{ErrorStore, FileMap};
use binding::Binding;
use diplomat_core::hir::{self, BackendAttrSupport};
use formatter::PyFormatter;
use ty::TyGenContext;

/// Python support using the nanobind c++ library to create a python binding.
///
/// The generated nanobind.cpp files requires linking with nanobind
/// Support for automated python package building is still outstanding.
/// To build, modify the following in the build.rs for your diplomat library:
///
///    let py_out_dir = Path::new(&out_dir).join("py");
///    diplomat_tool::gen(
///        Path::new("src/lib.rs"),
///        "python",
///        &py_out_dir,
///        &DocsUrlGenerator::with_base_urls(None, Default::default()),
///        None,
///        false,
///    )
///    .expect("Error generating python");
///
///    // Run python to obtain the include path & linker
///    let pyconfig_out = Command::new("python")
///        .args(["-c", "from sysconfig import get_path\nprint(get_path(\"include\"))"])
///        .output()
///        .expect("Error running python");
///    assert!(pyconfig_out.status.success());
///    let py_include = String::from_utf8_lossy(&pyconfig_out.stdout);
///    let py_lib = Path::new::<str>(py_include.borrow()).parent().unwrap().join("libs");
///
///    // Compile libnanobind
///    let nanobind_dir = build_utils::get_workspace_root().unwrap().join("external").join("nanobind");
///    cc::Build::new()
///        .cpp(true)
///        .flag("-std:c++17")
///        .opt_level(3)
///        .define("NDEBUG", None)
///        .define("NB_COMPACT_ASSERTIONS", None)
///        .include(nanobind_dir.join("include"))
///        .include(nanobind_dir.join("ext").join("robin_map").join("include"))
///        .include(py_include.trim())
///        .file(nanobind_dir.join("src").join("nb_combined.cpp"))
///        .compile("nanobind-static");
///
///    // Compile our extension
///    let mut build = cc::Build::new();
///    build
///        .cpp(true)
///        .flag("-std:c++17")
///        .opt_level_str("s")
///        .define("NDEBUG", None)
///        .define("zm_EXPORTS", None)
///        .define("NDEBUG", None)
///        .define("NB_COMPACT_ASSERTIONS", None)
///         // For windows:
///        .define("_WINDLL", None)
///        .define("_MBCS", None)    
///        .define("_WINDOWS", None)
///        .link_lib_modifier("+whole-archive")
///        .file(py_out_dir.join("nanobindings.cpp"))
///        .include(nanobind_include_dir)
///        .include(py_include.trim());
///    build.compile("zm_pyext");
///
///    println!("cargo::rustc-link-search=native={}", py_lib.display());

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

pub(crate) fn run(tcx: &hir::TypeContext) -> (FileMap, ErrorStore<String>) {
    let files = FileMap::default();
    let formatter = PyFormatter::new(tcx);
    let errors = ErrorStore::default();

    let nanobind_filepath = "nanobindings.cpp";
    let mut binding = Binding::new();
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

        // Assert everything shares the same root namespace. If this becomes too restrictive, we can generate multiple modules maybe?
        if let Some(ns) = ty
            .attrs()
            .namespace
            .as_ref()
            .and_then(|ns| ns.split("::").next())
        {
            if context.binding.module_name.is_empty() {
                context.binding.module_name = Cow::from(ns);
            } else {
                assert_eq!(context.binding.module_name, Cow::from(ns));
            }
        }

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

    (files, errors)
}

#[cfg(test)]
mod test {
    use diplomat_core::{
        ast::{self},
        hir::{self, TypeDef},
    };
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
        attr_validator.support = crate::python::attr_support();

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

        let formatter = crate::python::PyFormatter::new(&tcx);
        let errors = crate::ErrorStore::default();
        let mut binding = crate::python::Binding::new();
        binding.module_name = std::borrow::Cow::Borrowed("pymod");

        let decl_header_path = formatter.fmt_decl_header_path(type_id);
        let impl_file_path = formatter.fmt_impl_file_path(type_id);

        let mut context = crate::python::TyGenContext {
            formatter: &formatter,
            errors: &errors,
            c: crate::c::TyGenContext {
                tcx: &tcx,
                formatter: &formatter.c,
                errors: &errors,
                is_for_cpp: false,
                id: type_id.into(),
                decl_header_path: decl_header_path.clone().into(),
                impl_header_path: impl_file_path.clone().into(),
            },
            binding: &mut binding,
            generating_struct_fields: false,
            submodules: HashSet::<Cow<str>>::new(),
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
        attr_validator.support = crate::python::attr_support();

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

        let formatter = crate::python::PyFormatter::new(&tcx);
        let errors = crate::ErrorStore::default();
        let mut binding = crate::python::Binding::new();
        binding.module_name = std::borrow::Cow::Borrowed("pymod");

        let decl_header_path = formatter.fmt_decl_header_path(type_id);
        let impl_file_path = formatter.fmt_impl_file_path(type_id);

        let mut context = crate::python::TyGenContext {
            formatter: &formatter,
            errors: &errors,
            c: crate::c::TyGenContext {
                tcx: &tcx,
                formatter: &formatter.c,
                errors: &errors,
                is_for_cpp: false,
                id: type_id.into(),
                decl_header_path: decl_header_path.clone().into(),
                impl_header_path: impl_file_path.clone().into(),
            },
            binding: &mut binding,
            generating_struct_fields: false,
            submodules: HashSet::<Cow<str>>::new(),
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
                    pub mut c: f64,
                }
            }
        };
        let item = syn::parse2::<syn::File>(tokens).expect("failed to parse item ");

        let mut attr_validator = hir::BasicAttributeValidator::new("python");
        attr_validator.support = crate::python::attr_support();

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

        let formatter = crate::python::PyFormatter::new(&tcx);
        let errors = crate::ErrorStore::default();
        let mut binding = crate::python::Binding::new();
        binding.module_name = std::borrow::Cow::Borrowed("pymod");

        let decl_header_path = formatter.fmt_decl_header_path(type_id);
        let impl_file_path = formatter.fmt_impl_file_path(type_id);

        let mut context = crate::python::TyGenContext {
            formatter: &formatter,
            errors: &errors,
            c: crate::c::TyGenContext {
                tcx: &tcx,
                formatter: &formatter.c,
                errors: &errors,
                is_for_cpp: false,
                id: type_id.into(),
                decl_header_path: decl_header_path.clone().into(),
                impl_header_path: impl_file_path.clone().into(),
            },
            binding: &mut binding,
            generating_struct_fields: false,
            submodules: HashSet::<Cow<str>>::new(),
        };

        context.gen_struct_def(struct_def, type_id);
        let generated = binding.to_string();
        insta::assert_snapshot!(generated)
    }
}
