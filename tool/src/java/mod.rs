use std::borrow::Cow;
use std::collections::HashSet;
use std::fs::File;
use std::io::Write;
use std::path::Path;

use askama::Template;
use diplomat_core::hir::{
    self, OpaqueDef, ReturnType, Slice, SpecialMethod, StringEncoding, SuccessType, TypeContext,
    TypeDef, TypeId,
};
use formatter::JavaFormatter;

use crate::c2::{self};
use crate::common::{ErrorStore, FileMap};

const TMP_C_DIR: &str = "tmp";
const LIBRARY: &str = "somelib"; // todo: build from conf. Ensure that name is not the same as any
                                 // type
const GROUP: &str = "dev.diplomattest"; // todo: config
const _TMP_LIB_NAME: &str = "dev/diplomattest/somelib"; // todo: build from conf
const _JAVA_DIR: &str = "src/main/java/";

mod formatter;
pub fn run(
    tcx: &TypeContext,
    _conf_path: Option<&Path>,
    out_folder: &Path,
) -> std::io::Result<FileMap> {
    let files = FileMap::default();
    let mut context = c2::CContext::new(tcx, files, false);
    context.run();

    let errors = context.errors.take_all();

    if !errors.is_empty() {
        eprintln!("Found errors when generating c  code");
        for error in errors {
            eprintln!("\t{}: {}", error.0, error.1);
        }
    }

    let out_files = context.files.take_files();

    let tmp_path = out_folder.join(TMP_C_DIR);
    std::fs::create_dir(&tmp_path)?;
    let mut include_files = HashSet::new();
    for (subpath, text) in out_files {
        let out_path = tmp_path.join(&subpath);
        if !subpath.ends_with(".d.h") && subpath.ends_with(".h") {
            include_files.insert(subpath);
        }
        let parent = out_path
            .parent()
            .expect("Cannot create files at top level dir /");
        std::fs::create_dir_all(parent)?;
        let mut out_file = File::create(&out_path)?;
        out_file.write_all(text.as_bytes())?;
    }

    let lib_path = tmp_path.join(format!("{LIBRARY}.h"));

    let mut lib_file = File::create(&lib_path)?;
    for include in include_files {
        writeln!(lib_file, "#include \"{include}\"")?;
    }

    // jextract \
    //   --include-dir /path/to/mylib/include \
    //   --output src \
    //   --target-package org.jextract.mylib \
    //   --library mylib \
    //   /path/to/mylib/include/mylib.h

    let package = format!("{GROUP}.{LIBRARY}.ntv");
    let mut command = std::process::Command::new("jextract");
    command
        .arg("--include-dir")
        .arg(&tmp_path)
        .arg("--output")
        .arg(out_folder)
        .arg("--target-package")
        .arg(package)
        .arg("--library")
        .arg(LIBRARY)
        .arg(lib_path);

    println!("Running: {:?}", command);

    // todo: delete directory

    match command.output() {
        Err(err) => match err.kind() {
            std::io::ErrorKind::NotFound => {
                eprintln!("Check that jextract is in your path and all directories exist. See https://github.com/openjdk/jextract/blob/5715737be0a1a9de24cce3ee7190881cfc8b1350/doc/GUIDE.md");
                return Err(err);
            }
            _ => return Err(err),
        },
        Ok(ok) => {
            let stdout = String::from_utf8_lossy(&ok.stdout);
            println!("Output from jextract:\n{stdout}");

            let stderr = String::from_utf8_lossy(&ok.stderr);
            println!("Std Err from jextract:\n{stderr}");
        }
    }

    let java_formatter = JavaFormatter::new(tcx);
    let formatter = &java_formatter;
    let error_store = ErrorStore::default();
    let errors = &error_store;

    let ty_gen_cx = TyGenContext {
        tcx,
        formatter,
        errors,
    };

    let files = FileMap::default();
    for (id, ty) in tcx.all_types() {
        let _guard = ty_gen_cx.errors.set_context_ty(ty.name().as_str().into());
        if ty.attrs().disable {
            continue;
        }

        match ty {
            TypeDef::Opaque(o) => {
                // let type_name = o.name.to_string();

                // let (file_name, body) = ty_gen_cx.gen_opaque_def(o, id, GROUP, LIBRARY);

                // files.add_file(format!("src/main/kotlin/{file_name}"), body);
            }
            _ => continue,
        }
    }

    Ok(files)
}

#[derive(Clone, Debug)]
struct Param<'a> {
    name: Cow<'a, str>,
    ty: Cow<'a, str>,
}

#[derive(Clone, Debug)]
pub(crate) struct ParamConversion<'a> {
    converted_value: Cow<'a, str>,
    conversion_def: Cow<'a, str>,
}

#[derive(Template, Clone, Debug)]
#[template(path = "java/Method.java.jinja", escape = "none")]
pub(crate) struct MethodTpl<'a> {
    method_name: Option<Cow<'a, str>>,
    is_static: bool,
    return_ty: Cow<'a, str>,
    native_method: Cow<'a, str>,
    native_invocation: Cow<'a, str>,
    params: Vec<Param<'a>>,
    param_conversions: Vec<ParamConversion<'a>>,
    return_conversion: Cow<'a, str>,
    allocations: bool,
}

#[derive(Template, Clone, Debug)]
#[template(path = "java/Opaque.java.jinja", escape = "none")]
pub(crate) struct OpaqueTypeTpl<'a> {
    type_name: Cow<'a, str>,
    lib_name: Cow<'a, str>,
    domain: Cow<'a, str>,
    static_methods: Vec<Cow<'a, str>>,
    class_methods: Vec<Cow<'a, str>>,
}

#[derive(Template, Clone, Debug)]
#[template(path = "java/OpaqueReturn.java.jinja", escape = "none")]
pub(crate) struct OpaqueReturnTpl<'a> {
    return_ty: Cow<'a, str>,
}

#[derive(Template, Clone, Debug)]
#[template(path = "java/OpaqueConstructor.java.jinja", escape = "none")]
pub(crate) struct OpaqueConstructorTpl<'a> {
    return_ty: Cow<'a, str>,
}

struct TyGenContext<'a, 'cx> {
    tcx: &'cx TypeContext,
    formatter: &'a JavaFormatter<'cx>,
    errors: &'a ErrorStore<'cx, String>,
}

impl<'a, 'cx> TyGenContext<'a, 'cx> {
    fn gen_param_conversion<'b>(
        &self,
        param: &'b diplomat_core::hir::Param,
    ) -> ParamConversion<'b> {
        let diplomat_core::hir::Param { name, ty, .. } = param;
        // let java_ty = self.formatter.fmt_java_type(ty);
        let converted_value: Cow<'b, str> = format!("{name}Native").into();
        let (conversion, converted_value) = match ty {
            hir::Type::Primitive(_) => (name.as_str().into(), converted_value),
            hir::Type::Opaque(_) => (format!("{name}.inner").into(), converted_value),
            hir::Type::Struct(_) => todo!(),
            hir::Type::Enum(_) => todo!(),
            hir::Type::Slice(Slice::Str(_, StringEncoding::UnvalidatedUtf8)) => (
                format!(
                    r#"var {name}MemSeg = arena.allocateFrom({name}, StandardCharsets.UTF_8);
var {name}Len = {name}MemSeg.byteSize();"#
                )
                .into(),
                format!("{name}MemSeg, {name}Len").into(),
            ),
            hir::Type::Slice(Slice::Str(_, StringEncoding::UnvalidatedUtf16)) => (
                format!(
                    r#"var {name}MemSeg = arena.allocateFrom({name}, StandardCharsets.UTF_16);
var {name}Len = {name}MemSeg.byteSize();"#
                )
                .into(),
                format!("{name}MemSeg, {name}Len").into(),
            ),
            hir::Type::Slice(Slice::Str(_, StringEncoding::Utf8)) => (
                format!(
                    r#"var {name}MemSeg = arena.allocateFrom({name}, StandardCharsets.UTF_8);
var {name}Len = {name}MemSeg.byteSize();"#
                )
                .into(),
                format!("{name}MemSeg, {name}Len").into(),
            ),
            _ => todo!(),
        };
        ParamConversion {
            converted_value,
            conversion_def: conversion,
        }
    }

    fn gen_return_conversion(&self, ty: &ReturnType) -> Cow<'cx, str> {
        let ret = match ty {
            ReturnType::Infallible(ref ret) => ret,
            ReturnType::Fallible(ref ret, _) => ret,
            ReturnType::Nullable(ref ret) => ret,
        };
        let o = match ret {
            SuccessType::Write => return "".into(),
            SuccessType::OutType(ref o) => o,
            SuccessType::Unit => return "".into(),
            _ => todo!(),
        };
        match o {
            hir::Type::Primitive(o) => "return nativeVal;".into(),
            hir::Type::Opaque(o) => {
                let ty_name = &self.tcx.resolve_opaque(o.tcx_id).name;
                OpaqueReturnTpl {
                    return_ty: ty_name.as_str().into(),
                }
                .render()
                .unwrap_or_else(|err| {
                    panic!(
                        "Failed to render return val for type {}. Cause: {err}",
                        ty_name
                    )
                })
                .into()
            }
            hir::Type::Struct(_) => todo!(),
            hir::Type::Enum(_) => todo!(),
            hir::Type::Slice(_) => todo!(),
            _ => todo!(),
        }
    }

    fn gen_opaque_def(
        &self,
        o: &OpaqueDef,
        ty: TypeId,
        domain: &str,
        lib_name: &str,
    ) -> (Cow<str>, String) {
        let mut static_methods = Vec::new();
        let mut class_methods = Vec::new();
        for method in o.methods.iter() {
            let method_name = match method.attrs.special_method {
                // We need to reserve the default constructor for internal methods so a constructor
                // must always have params
                Some(SpecialMethod::Constructor) if !method.params.is_empty() => None,
                Some(SpecialMethod::Constructor) => {
                    eprintln!(
                        "Attempted to create constructor for {:?} type {:?}",
                        method.name, o.name
                    );
                    Some(self.formatter.fmt_method_name(method))
                }
                _ => Some(self.formatter.fmt_method_name(method)),
            };

            let return_ty = self.formatter.fmt_return_type_java(&method.output);
            let return_conversion = if method_name.is_none() {
                OpaqueConstructorTpl {
                    return_ty: return_ty.clone(),
                }
                .render()
                .unwrap_or_else(|err| {
                    panic!(
                        "Failed to render method {} for type {}. Cause: {err}",
                        method.name, o.name
                    )
                })
                .into()
            } else {
                self.gen_return_conversion(&method.output)
            };

            let allocations =
                method
                    .params
                    .iter()
                    .any(|diplomat_core::hir::Param { ty, .. }| {
                        matches!(ty, diplomat_core::hir::Type::Slice(_))
                    });
            let params = method
                .params
                .iter()
                .map(|diplomat_core::hir::Param { name, ty, .. }| Param {
                    name: self.formatter.fmt_param_name(name.as_str()).into(),
                    ty: self.formatter.fmt_java_type(ty),
                })
                .collect();
            let param_conversions: Vec<_> = method
                .param_self
                .iter()
                .map(|_| ParamConversion {
                    converted_value: "internal".into(),
                    conversion_def: "".into(),
                })
                .chain(
                    method
                        .params
                        .iter()
                        .map(|param| self.gen_param_conversion(param)),
                )
                .collect();
            let native_method: Cow<str> = format!(
                "{lib_name}_h.{}",
                self.formatter.fmt_c_method_name(ty, method)
            )
            .into();
            let native_invocation = if param_conversions.is_empty() {
                "nativeInvoker.apply".into()
            } else {
                native_method.clone()
            };
            let method_rendered = MethodTpl {
                method_name,
                is_static: method.param_self.is_none(),
                return_ty,
                native_method,
                native_invocation,
                params,
                param_conversions,
                return_conversion,
                allocations,
            }
            .render()
            .unwrap_or_else(|err| {
                panic!(
                    "Failed to render method {} for type {}. Cause: {err}",
                    method.name, o.name
                )
            });

            println!("HAve method!");
            println!("{method_rendered}");

            match method.param_self {
                Some(_) => class_methods.push(method_rendered.into()),
                None => static_methods.push(method_rendered.into()),
            }
        }

        let opaque_tpl = OpaqueTypeTpl {
            type_name: o.name.to_string().into(),
            lib_name: lib_name.into(),
            domain: domain.into(),
            static_methods,
            class_methods,
        };

        (
            format!("{}.java", o.name).into(),
            opaque_tpl.render().expect("Failed to render opaque type"),
        )
    }
}

#[cfg(test)]
mod test {

    use askama::Template;
    use diplomat_core::hir::TypeDef;
    use quote::quote;

    use crate::{common::ErrorStore, test::new_tcx};

    use super::{formatter::JavaFormatter, OpaqueTypeTpl, TyGenContext};
    #[test]
    fn test_opaque_render() {
        let opaque_type = OpaqueTypeTpl {
            type_name: "Opaque2".into(),
            lib_name: "somelib".into(),
            domain: "dev.diplomattest".into(),
            static_methods: Vec::new(),
            class_methods: Vec::new(),
        };

        let rendered = opaque_type.render().expect("Failed to render opaque type");
        insta::assert_snapshot!(rendered);
    }

    #[test]
    fn test_opaque() {
        let tk_stream = quote! {
            #[diplomat::bridge]
            mod ffi {

                #[diplomat::opaque]
                pub struct Opaque(String);

                impl Opaque {
                    #[diplomat::attr(supports = constructors, constructor)]
                    pub fn new() -> Box<Opaque> {
                        Box::new(Opaque("".into()))
                    }

                    pub fn from_str(input: &str) -> Box<Self> {
                        Box::new(Self(input.into()))
                    }


                    pub fn returns_usize() -> usize {
                        412
                    }

                    pub fn internal_len(&self) -> usize {
                        self.0.len()
                    }

                }
            }
        };

        let tcx = new_tcx(tk_stream);

        let formatter = JavaFormatter::new(&tcx);

        let errors = ErrorStore::default();
        let tcx_gen = TyGenContext {
            tcx: &tcx,
            formatter: &formatter,
            errors: &errors,
        };

        let (ty, some_opaque) = tcx
            .all_types()
            .next()
            .expect("Didn't find type despite there being one in quote");
        if let TypeDef::Opaque(opaque) = some_opaque {
            println!("Found opaque");
            let (_, rendered) = tcx_gen.gen_opaque_def(opaque, ty, "dev.diplomattest", "somelib");

            insta::assert_snapshot!(rendered);
        } else {
            panic!("Found type should have been opaque but wasn't");
        }
    }
}
