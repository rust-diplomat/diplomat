mod formatter;
pub(crate) mod gen;
mod root_module;

use std::{
    collections::{BTreeMap, BTreeSet},
    fmt::Write,
};

use crate::{
    cpp::Header, nanobind::gen::MethodInfo, read_custom_binding, Config, ErrorStore, FileMap,
};
use askama::Template;
use diplomat_core::hir::{self, BackendAttrSupport, DocsUrlGenerator};
use formatter::PyFormatter;
use gen::ItemGenContext;
use itertools::Itertools;
use root_module::RootModule;

use crate::cpp;

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
    a.defaults = true;

    a.constructors = true;
    a.named_constructors = false;
    a.fallible_constructors = true;
    a.accessors = true;
    a.static_accessors = true;
    a.comparators = true;
    a.stringifiers = true;
    a.iterators = true;
    a.iterables = true;
    a.arithmetic = true;
    a.indexing = true;
    a.option = true;
    a.callbacks = true;
    a.traits = false;
    a.generate_mocking_interface = false;
    a.abi_compatibles = true;
    a.struct_refs = true;
    a.free_functions = true;
    a.custom_bindings = true;

    a
}

pub(crate) fn run<'cx>(
    tcx: &'cx hir::TypeContext,
    conf: Config,
    docs: &'cx DocsUrlGenerator,
) -> (FileMap, ErrorStore<'cx, String>) {
    let files = FileMap::default();

    let formatter = PyFormatter::new(tcx, &conf, docs);
    let errors = ErrorStore::default();

    let lib_name = conf
        .shared_config
        .lib_name
        .as_ref()
        .expect("Nanobind backend requires lib_name to be set in the config")
        .clone();

    // Output the C++ bindings we rely on
    let (cpp_files, cpp_errors) = cpp::run(tcx, &conf, docs);

    files.files.borrow_mut().extend(
        cpp_files
            .files
            .take()
            .into_iter()
            .map(|(k, v)| (format!("include/{k}"), v)),
    );
    errors.errors.borrow_mut().extend(cpp_errors.errors.take());

    let nanobind_common_filepath = "include/diplomat_nanobind_common.hpp";

    #[derive(Template)]
    #[template(path = "nanobind/common.h.jinja", escape = "none")]
    struct Common {
        lib_name: String,
    }

    files.add_file(
        nanobind_common_filepath.to_owned(),
        Common {
            lib_name: lib_name.clone(),
        }
        .to_string(),
    );

    let nanobind_filepath = format!("{lib_name}_ext.cpp");

    let mut root_module = RootModule::new();
    root_module.module_name = lib_name.clone().into();

    let mut submodules = BTreeMap::new();
    for (id, ty) in tcx.all_types() {
        if ty.attrs().disable {
            // Skip type if disabled
            continue;
        }

        let cpp_decl_path = formatter.cxx.fmt_decl_header_path(id.into());
        let cpp_impl_path = formatter.cxx.fmt_impl_header_path(id.into());
        let binding_impl_path = format!("sub_modules/{}", formatter.fmt_binding_impl_path(id));

        let mut context = ItemGenContext {
            formatter: &formatter,
            errors: &errors,
            cpp: crate::cpp::ItemGenContext {
                c: crate::c::ItemGenContext {
                    tcx,
                    formatter: &formatter.cxx.c,
                    errors: &errors,
                    decl_header_path: &cpp_decl_path,
                    impl_header_path: &cpp_impl_path,
                    is_for_cpp: false,
                },
                config: &conf.cpp_config,
                formatter: &formatter.cxx,
                errors: &errors,
                impl_header: &mut crate::cpp::Header::default(),
                decl_header: &mut crate::cpp::Header::default(),
                generating_struct_fields: false,
            },
            root_module: &mut root_module,
            submodules: &mut submodules,
            generating_struct_fields: false,
        };

        context
            .cpp
            .impl_header
            .includes
            .insert(cpp_impl_path.clone());

        let guard = errors.set_context_ty(ty.name().as_str().into());

        #[derive(Template)]
        #[template(path = "nanobind/binding.cpp.jinja", escape = "none")]
        struct Binding {
            includes: BTreeSet<String>,
            lib_name: String,
            namespace: String,
            unqualified_type: String,
            body: String,
            binding_prefix: String,
        }

        let mut body = String::default();
        let mut binding_prefix = String::default();
        match ty {
            hir::TypeDef::Enum(o) => context.gen_enum_def(o, id, &mut body),
            hir::TypeDef::Opaque(o) => context.gen_opaque_def(o, id, &mut body),
            hir::TypeDef::Struct(s) => {
                context.gen_struct_def(s, id, &mut body, &mut binding_prefix)
            }
            hir::TypeDef::OutStruct(s) => {
                context.gen_struct_def(s, id, &mut body, &mut binding_prefix)
            }
            _ => unreachable!("unknown AST/HIR variant"),
        }
        drop(guard);

        let binding_info = &ty.attrs().binding_includes;

        if let Some(s) = binding_info.get(&hir::IncludeLocation::InitializationBlock) {
            if let Ok(s) = read_custom_binding(s, &conf, &errors) {
                writeln!(body, "\n{}", s).expect("Could not write to body.");
            }
        }

        let binding_impl = Binding {
            includes: context.cpp.impl_header.includes.clone(),
            lib_name: lib_name.clone(),
            namespace: formatter.fmt_namespaces(id.into()).join("::"),
            unqualified_type: formatter.cxx.fmt_type_name_unnamespaced(id).to_string(),
            body,
            binding_prefix,
        };

        files.add_file(binding_impl_path, binding_impl.to_string());
    }

    let mut ty_context = ItemGenContext {
        formatter: &formatter,
        errors: &errors,
        cpp: crate::cpp::ItemGenContext {
            c: crate::c::ItemGenContext {
                tcx,
                formatter: &formatter.cxx.c,
                is_for_cpp: false,
                errors: &errors,
                decl_header_path: "",
                impl_header_path: "",
            },
            config: &conf.cpp_config,
            errors: &errors,
            formatter: &formatter.cxx,
            impl_header: &mut Header::default(),
            decl_header: &mut Header::default(),
            generating_struct_fields: false,
        },
        root_module: &mut root_module,
        submodules: &mut submodules,
        generating_struct_fields: false,
    };

    #[derive(Default)]
    struct FuncGenContext<'a> {
        namespace: Option<String>,
        namespaces: Vec<String>,
        functions: Vec<gen::MethodInfo<'a>>,
        includes: BTreeSet<String>,
    }

    let mut func_map = BTreeMap::new();

    for (func_id, func) in tcx.all_free_functions() {
        let Some(func_info) = ty_context.gen_method_info(func_id.into(), func) else {
            continue;
        };

        let include = formatter
            .cxx
            .fmt_free_function_header_path(func.attrs.namespace.clone());

        ty_context.gen_modules(func_id.into(), None);
        let key = func.attrs.namespace.clone().unwrap_or_default();
        let context = func_map.entry(key).or_insert_with(|| FuncGenContext {
            namespace: func.attrs.namespace.clone(),
            namespaces: formatter
                .fmt_namespaces(func_id.into())
                .map(|n| n.to_string())
                .collect(),
            ..Default::default()
        });

        context.includes.insert(include);
        context.functions.push(func_info);
    }

    for (_, ctx) in func_map {
        let binding_impl_path = format!(
            "sub_modules/{}/func_bindings.cpp",
            ctx.namespace.clone().unwrap_or_default().replace("::", "/"),
        );

        use diplomat_core::hir::Type;

        #[derive(Template)]
        #[template(path = "nanobind/binding.cpp.jinja", escape = "none")]
        struct Binding {
            includes: BTreeSet<String>,
            lib_name: String,
            namespace: String,
            unqualified_type: String,
            body: String,
            binding_prefix: String,
        }

        let unqualified_type = "free_function".to_string(); // fake type name

        ItemGenContext::gen_binding_fn(
            ty_context.root_module,
            ctx.namespaces.iter().map(|s| s.as_str()),
            format!("add_{unqualified_type}_binding"),
        );

        let b = Binding {
            includes: ctx.includes,
            namespace: ctx.namespace.unwrap_or_default(),
            unqualified_type,
            lib_name: lib_name.clone(),
            body: format!(
                "mod\n{};",
                ctx.functions
                    .into_iter()
                    .map(|m| {
                        #[derive(Template)]
                        #[template(path = "nanobind/function_impl.cpp.jinja", escape = "none")]
                        struct FuncBlock<'a> {
                            m: MethodInfo<'a>,
                        }
                        FuncBlock { m }.to_string()
                    })
                    .join("\n")
            ),
            binding_prefix: String::new(),
        };

        files.add_file(binding_impl_path, b.render().unwrap());
    }

    // Traverse the module_fns keys list and expand into the list of submodules needing generation.
    // In particular we're concerned about the case of nested modules that only contain other modules
    for module_path in root_module.module_fns.keys() {
        let mut path = module_path.clone();
        while !path.is_empty() {
            println!("Adding module with path: {}", path.join("::"));
            root_module
                .sub_modules
                .insert(path.iter().cloned().collect_vec());

            path.pop();
        }
    }
    root_module
        .sub_modules
        .remove(&vec![root_module.module_name.clone().into()]); // remove the root module from the list of submodules

    files.add_file(nanobind_filepath.to_owned(), root_module.to_string());

    (files, errors)
}

#[cfg(test)]
mod test {
    use diplomat_core::hir::{self, TypeDef};
    use quote::quote;
    use std::collections::BTreeMap;

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
        let config = crate::Config::default();

        let mut attr_validator = hir::BasicAttributeValidator::new("python");
        attr_validator.support = crate::nanobind::attr_support();

        let tcx = match hir::TypeContext::from_syn(&item, Default::default(), attr_validator) {
            Ok(context) => context,
            Err(e) => {
                for (_cx, err) in e {
                    eprintln!("Lowering error: {err}");
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

        let docs_gen = Default::default();
        let formatter = crate::nanobind::PyFormatter::new(&tcx, &config, &docs_gen);
        let errors = crate::ErrorStore::default();
        let mut root_module = crate::nanobind::root_module::RootModule::new();
        root_module.module_name = std::borrow::Cow::Borrowed("pymod");

        let decl_header_path = formatter.cxx.fmt_decl_header_path(type_id.into());
        let impl_file_path = formatter.cxx.fmt_impl_header_path(type_id.into());

        let mut submodules = BTreeMap::new();

        let mut context = crate::nanobind::ItemGenContext {
            formatter: &formatter,
            errors: &errors,
            cpp: crate::cpp::ItemGenContext {
                c: crate::c::ItemGenContext {
                    tcx: &tcx,
                    formatter: &formatter.cxx.c,
                    errors: &errors,
                    is_for_cpp: false,
                    decl_header_path: &decl_header_path,
                    impl_header_path: &impl_file_path,
                },
                formatter: &formatter.cxx,
                errors: &errors,
                config: &config.cpp_config,
                impl_header: &mut crate::cpp::Header::default(),
                decl_header: &mut crate::cpp::Header::default(),
                generating_struct_fields: false,
            },
            root_module: &mut root_module,
            generating_struct_fields: false,
            submodules: &mut submodules,
        };
        let mut generated = String::default();
        context.gen_opaque_def(opaque_def, type_id, &mut generated);
        let generated = root_module.to_string();
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
        let config = crate::Config::default();

        let mut attr_validator = hir::BasicAttributeValidator::new("python");
        attr_validator.support = crate::nanobind::attr_support();

        let tcx = match hir::TypeContext::from_syn(&item, Default::default(), attr_validator) {
            Ok(context) => context,
            Err(e) => {
                for (_cx, err) in e {
                    eprintln!("Lowering error: {err}");
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

        let docs_gen = Default::default();
        let formatter = crate::nanobind::PyFormatter::new(&tcx, &config, &docs_gen);
        let errors = crate::ErrorStore::default();
        let mut root_module = crate::nanobind::RootModule::new();
        root_module.module_name = std::borrow::Cow::Borrowed("pymod");

        let decl_header_path = formatter.cxx.fmt_decl_header_path(type_id.into());
        let impl_file_path = formatter.cxx.fmt_impl_header_path(type_id.into());

        let mut submodules = BTreeMap::new();

        let mut context = crate::nanobind::ItemGenContext {
            formatter: &formatter,
            errors: &errors,
            cpp: crate::cpp::ItemGenContext {
                c: crate::c::ItemGenContext {
                    tcx: &tcx,
                    formatter: &formatter.cxx.c,
                    errors: &errors,
                    is_for_cpp: false,
                    decl_header_path: &decl_header_path,
                    impl_header_path: &impl_file_path,
                },
                formatter: &formatter.cxx,
                config: &config.cpp_config,
                errors: &errors,
                impl_header: &mut crate::cpp::Header::default(),
                decl_header: &mut crate::cpp::Header::default(),
                generating_struct_fields: false,
            },
            root_module: &mut root_module,
            generating_struct_fields: false,
            submodules: &mut submodules,
        };
        let mut enum_gen = String::new();
        context.gen_enum_def(enum_def, type_id, &mut enum_gen);
        insta::assert_snapshot!(enum_gen)
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
        let config = crate::Config::default();

        let mut attr_validator = hir::BasicAttributeValidator::new("python");
        attr_validator.support = crate::nanobind::attr_support();

        let tcx = match hir::TypeContext::from_syn(&item, Default::default(), attr_validator) {
            Ok(context) => context,
            Err(e) => {
                for (_cx, err) in e {
                    eprintln!("Lowering error: {err}");
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

        let docs_gen = Default::default();
        let formatter = crate::nanobind::PyFormatter::new(&tcx, &config, &docs_gen);
        let errors = crate::ErrorStore::default();
        let mut root_module = crate::nanobind::RootModule::new();
        root_module.module_name = std::borrow::Cow::Borrowed("pymod");

        let decl_header_path = formatter.cxx.fmt_decl_header_path(type_id.into());
        let impl_file_path = formatter.cxx.fmt_impl_header_path(type_id.into());

        let mut submodules = BTreeMap::new();

        let mut context = crate::nanobind::ItemGenContext {
            formatter: &formatter,
            errors: &errors,
            cpp: crate::cpp::ItemGenContext {
                c: crate::c::ItemGenContext {
                    tcx: &tcx,
                    formatter: &formatter.cxx.c,
                    errors: &errors,
                    is_for_cpp: false,
                    decl_header_path: &decl_header_path,
                    impl_header_path: &impl_file_path,
                },
                formatter: &formatter.cxx,
                errors: &errors,
                config: &config.cpp_config,
                impl_header: &mut crate::cpp::Header::default(),
                decl_header: &mut crate::cpp::Header::default(),
                generating_struct_fields: false,
            },
            root_module: &mut root_module,
            generating_struct_fields: false,
            submodules: &mut submodules,
        };

        let mut struct_gen = String::new();
        let mut header = String::new();
        context.gen_struct_def(struct_def, type_id, &mut struct_gen, &mut header);
        insta::assert_snapshot!(struct_gen)
    }
}
