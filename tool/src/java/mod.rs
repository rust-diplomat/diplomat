use std::borrow::Cow;
use std::collections::HashSet;
use std::fs::File;
use std::io::Write;
use std::path::Path;

use askama::Template;
use diplomat_core::hir::{
    self, EnumDef, EnumId, EnumVariant, FloatType, IntSizeType, IntType, MaybeStatic, Method,
    OpaqueDef, ReturnType, Slice, SpecialMethod, StringEncoding, StructDef, StructField,
    StructPathLike, SuccessType, TypeContext, TypeDef, TypeId,
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
    let tcx_config = Config {
        domain: "dev.diplomattest".into(),
        lib_name: "somelib".into(),
    };

    let ty_gen_cx = TyGenContext {
        tcx,
        tcx_config,
        formatter,
        errors,
    };

    let files = FileMap::default();
    for (_id, ty) in tcx.all_types() {
        let _guard = ty_gen_cx.errors.set_context_ty(ty.name().as_str().into());
        if ty.attrs().disable {
            continue;
        }

        match ty {
            TypeDef::Opaque(_) => {
                // let type_name = o.name.to_string();

                // let (file_name, body) = ty_gen_cx.gen_opaque_def(o, id, GROUP, LIBRARY);
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
    make_invoker: bool,
    native_invocation: Cow<'a, str>,
    params: Vec<Param<'a>>,
    param_conversions: Vec<ParamConversion<'a>>,
    return_conversion: Cow<'a, str>,
    allocations: bool,
    native_return_void: bool,
}

#[derive(Template, Clone, Debug)]
#[template(path = "java/Struct.java.jinja", escape = "none")]
pub(crate) struct StructTypeTpl<'a> {
    type_name: Cow<'a, str>,
    lib_name: Cow<'a, str>,
    domain: Cow<'a, str>,
    edges: Vec<Cow<'a, str>>,
    fields: Vec<FieldTpl<'a>>,
    methods: Vec<Cow<'a, str>>,
}

#[derive(Clone, Debug)]
struct FieldTpl<'a> {
    name: Cow<'a, str>,
    field_transform: Option<Cow<'a, str>>,
    ty: Cow<'a, str>,
}

#[derive(Template, Clone, Debug)]
#[template(path = "java/Enum.java.jinja", escape = "none")]
pub(crate) struct EnumTypeTpl<'a> {
    type_name: Cow<'a, str>,
    lib_name: Cow<'a, str>,
    domain: Cow<'a, str>,
    variants: Vec<VariantTpl<'a>>,
    methods: Vec<Cow<'a, str>>,
}

#[derive(Clone, Debug)]
struct VariantTpl<'a> {
    name: Cow<'a, str>,
    index: Cow<'a, str>,
}

#[derive(Template, Clone, Debug)]
#[template(path = "java/Opaque.java.jinja", escape = "none")]
pub(crate) struct OpaqueTypeTpl<'a> {
    type_name: Cow<'a, str>,
    lib_name: Cow<'a, str>,
    domain: Cow<'a, str>,
    edges: Vec<Cow<'a, str>>,
    static_methods: Vec<Cow<'a, str>>,
    class_methods: Vec<Cow<'a, str>>,
}

#[derive(Template, Clone, Debug)]
#[template(path = "java/OpaqueReturn.java.jinja", escape = "none")]
pub(crate) struct OpaqueReturnTpl<'a> {
    lifetime_edges: Vec<String>,
    return_ty: Cow<'a, str>,
}

#[derive(Template, Clone, Debug)]
#[template(path = "java/OpaqueConstructor.java.jinja", escape = "none")]
pub(crate) struct OpaqueConstructorTpl<'a> {
    return_ty: Cow<'a, str>,
}

struct Config<'cx> {
    domain: Cow<'cx, str>,
    lib_name: Cow<'cx, str>,
}

struct TyGenContext<'a, 'cx> {
    tcx: &'cx TypeContext,
    tcx_config: Config<'cx>,
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
        let name = self.formatter.fmt_param_name(name.as_str());
        let (conversion, converted_value) = match ty {
            hir::Type::Primitive(_) => (name.into(), converted_value),
            hir::Type::Opaque(_) => (format!("{name}.internal").into(), converted_value),
            hir::Type::Struct(_) => (
                format!(r#"var {name}Native = {name}.internal;"#).into(),
                converted_value,
            ),
            hir::Type::Enum(_) => todo!(),
            hir::Type::Slice(Slice::Str(borrow, StringEncoding::UnvalidatedUtf16)) => {
                let arena_name = borrow.map(|_| "arena").unwrap_or("Arena.global()");
                (
                    format!(
                        r#"var {name}MemSeg = {arena_name}.allocateFrom({name}, StandardCharsets.UTF_16);
var {name}Len = {name}MemSeg.byteSize();"#
                    )
                    .into(),
                    format!("{name}MemSeg, {name}Len - 1").into(),
                )
            }
            hir::Type::Slice(Slice::Str(
                borrow,
                StringEncoding::Utf8 | StringEncoding::UnvalidatedUtf8,
            )) => {
                let arena_name = borrow.map(|_| "arena").unwrap_or("Arena.global()");
                (
                    format!(
                        r#"var {name}MemSeg = {arena_name}.allocateFrom({name}, StandardCharsets.UTF_8);
var {name}Len = {name}MemSeg.byteSize();"#
                    )
                    .into(),
                    format!("{name}MemSeg, {name}Len - 1").into(),
                    // by default java native creates null terminated strings
                )
            }
            hir::Type::Slice(Slice::Primitive(borrow, p)) => {
                let primitive_ty = match p {
                    hir::PrimitiveType::Bool => "JAVA_BOOLEAN",
                    hir::PrimitiveType::Char => "JAVA_INT",
                    hir::PrimitiveType::Byte => "JAVA_BYTE",
                    hir::PrimitiveType::Int(IntType::I8 | IntType::U8) => "JAVA_BYTE",
                    hir::PrimitiveType::Int(IntType::I16 | IntType::U16) => "JAVA_SHORT",
                    hir::PrimitiveType::Int(IntType::I32 | IntType::U32) => "JAVA_INT",
                    hir::PrimitiveType::Int(IntType::I64 | IntType::U64) => "JAVA_LONG",
                    hir::PrimitiveType::IntSize(_) => "JAVA_LONG",
                    hir::PrimitiveType::Int128(_) => {
                        panic!("java backend doesn't support 128 bit integers")
                    }
                    hir::PrimitiveType::Float(hir::FloatType::F32) => "JAVA_FLOAT",
                    hir::PrimitiveType::Float(hir::FloatType::F64) => "JAVA_DOUBLE",
                };
                let arena_name = borrow.map(|_| "arena").unwrap_or("Arena.global()");
                (
                    format!(
                        r#"var {name}Len = {name}.length;
var {name}MemSeg = {arena_name}.allocateFrom({primitive_ty}, {name});"#
                    )
                    .into(),
                    format!("{name}MemSeg, {name}Len").into(),
                )
            }
            hir::Type::Slice(Slice::Strs(StringEncoding::UnvalidatedUtf16)) => (
                format!(
                    r#"var {name}Data = SliceUtils.strs16(arena, {name});
var {name}Len = {name}.length;"#
                )
                .into(),
                format!(r#"{name}Data, {name}Len"#).into(),
            ),
            hir::Type::Slice(Slice::Strs(_)) => (
                format!(
                    r#"var {name}Data = SliceUtils.strs8(arena, {name});
var {name}Len = {name}.length;"#
                )
                .into(),
                format!(r#"{name}Data, {name}Len"#).into(),
            ),
            x => panic!("Unexpected slice type {x:?}"),
        };
        ParamConversion {
            converted_value,
            conversion_def: conversion,
        }
    }

    fn gen_slice_return_conversion(&self, ty: &Slice) -> Result<Cow<'cx, str>, String> {
        let return_conversion: Cow<'cx, str> = match ty {
            Slice::Str(_, encoding) => match encoding {
                StringEncoding::Utf8 | StringEncoding::UnvalidatedUtf8 => {
                    "return SliceUtils.readUtf8(nativeVal);"
                }
                StringEncoding::UnvalidatedUtf16 => "return SliceUtils.readUtf16(nativeVal);",
                _ => unreachable!("Not a valid string encoding for diplomat"),
            }
            .into(),
            Slice::Primitive(_, p) => {
                let lib_name = self.tcx_config.lib_name.as_ref();
                let domain = self.tcx_config.domain.as_ref();
                let primitive_ty = match p {
                    hir::PrimitiveType::Bool => "Bool",
                    hir::PrimitiveType::Char => "Char",
                    hir::PrimitiveType::Byte => "U8",
                    hir::PrimitiveType::Int(IntType::U8) => "U8",
                    hir::PrimitiveType::Int(IntType::I8) => "I8",
                    hir::PrimitiveType::Int(IntType::U16) => "U16",
                    hir::PrimitiveType::Int(IntType::I16) => "I16",
                    hir::PrimitiveType::Int(IntType::U32) => "U32",
                    hir::PrimitiveType::Int(IntType::I32) => "I32",
                    hir::PrimitiveType::Int(IntType::U64) => "U64",
                    hir::PrimitiveType::Int(IntType::I64) => "I64",
                    hir::PrimitiveType::IntSize(IntSizeType::Usize) => "Usize",
                    hir::PrimitiveType::IntSize(IntSizeType::Isize) => "Isize",
                    hir::PrimitiveType::Int128(_) => {
                        panic!("Java backend doesn't support Int128 types")
                    }
                    hir::PrimitiveType::Float(FloatType::F32) => "F32",
                    hir::PrimitiveType::Float(FloatType::F64) => "F64",
                };

                let java_primitive_ty = self.formatter.fmt_primitive(p);
                format!(
                    r#"var data = {domain}.{lib_name}.ntv.Diplomat{primitive_ty}View.data(nativeVal);
var len = {domain}.{lib_name}.ntv.Diplomat{primitive_ty}View.len(nativeVal);
return SliceUtils.{java_primitive_ty}SliceToArray(nativeVal);"#
                )
                .into()
            }
            Slice::Strs(_) => {
                panic!("[&str] not allowed in return position")
            }
            _ => todo!(),
        };
        return_conversion.wrap_ok()
    }

    fn gen_return_conversion(
        &self,
        ty: &ReturnType,
        lifetime_edges: Vec<String>,
    ) -> Result<Cow<'cx, str>, String> {
        let Config { lib_name, .. } = &self.tcx_config;
        let ret = match ty {
            ReturnType::Infallible(ref ret) => ret,
            ReturnType::Fallible(ref ret, _) => ret,
            ReturnType::Nullable(ref ret) => ret,
        };
        let o = match ret {
            /*
            var writeable = somelib_h.diplomat_buffer_write_create(0);
            somelib_h.Opaque_get_debug_str(internal, writeable);
            var buffer = DiplomatWrite.buf(writeable);
            var string = buffer.getString(0, StandardCharsets.UTF_8);
            somelib_h.diplomat_buffer_write_destroy(writeable);
            return string;
            */
            SuccessType::Write => {
                let write_return: Cow<'cx, str> = format!(
                    r#"var buffer = DiplomatWrite.buf(writeable);
var string = buffer.getString(0, StandardCharsets.UTF_8);
{}_h.diplomat_buffer_write_destroy(writeable);
return string;"#,
                    lib_name
                )
                .into();
                return write_return.wrap_ok();
            }
            SuccessType::OutType(ref o) => o,
            SuccessType::Unit => return Cow::<'cx, str>::default().wrap_ok(),
            _ => todo!(),
        };
        let return_statment: Cow<'cx, str> = match o {
            hir::Type::Primitive(_) => "return nativeVal;".into(),
            hir::Type::Opaque(o) => {
                let ty_name = &self.tcx.resolve_opaque(o.tcx_id).name;
                OpaqueReturnTpl {
                    lifetime_edges,
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
            hir::Type::Struct(s) => {
                let ty_name = &self.tcx.resolve_type(s.id()).name();
                let lifetime_edges = lifetime_edges.join("\n");
                format!(
                    r#"var returnVal = new {ty_name}(returnArena);
{lifetime_edges}
returnVal.initFromSegment(nativeVal);
return returnVal;"#
                )
                .into()
            }
            hir::Type::Enum(e) => {
                let enum_ty = self.tcx.resolve_enum(e.tcx_id).name.as_str();
                format!(r#"return {enum_ty}.fromInt(nativeVal);"#).into()
            }
            hir::Type::Slice(ref slice) => self.gen_slice_return_conversion(slice)?,
            unknown => panic!("Got to unknown return type: {unknown:?}"),
        };

        Ok(return_statment)
    }

    fn gen_methods(
        &self,
        ty_id: TypeId,
        ty_name: &str,
        methods: &[Method],
    ) -> (Vec<Cow<'cx, str>>, Vec<Cow<'cx, str>>) {
        let Config { lib_name, .. } = &self.tcx_config;
        let mut static_methods = Vec::new();
        let mut class_methods = Vec::new();
        methods
            .iter()
            .filter_map(|method| -> Option<(bool, Cow<'cx, str>)> {
                let mut visitor = method.borrowing_param_visitor(self.tcx);

                let (method_name, is_valid_constructor) = match method.attrs.special_method {
                    // We need to reserve the default constructor for internal methods so a constructor
                    // must always have params
                    Some(SpecialMethod::Constructor) if !method.params.is_empty() => (None, true),
                    Some(SpecialMethod::Constructor) => {
                        eprintln!(
                            "Attempted to create constructor for {:?} type {:?}",
                            method.name, ty_name
                        );
                        (Some(self.formatter.fmt_method_name(method)), false)
                    }
                    _ => (Some(self.formatter.fmt_method_name(method)), false),
                };

                let return_ty = self.formatter.fmt_return_type_java(&method.output);

                if let Some(param) = &method.param_self {
                    visitor.visit_param(&param.ty.clone().into(), "this");
                }
                let params = method
                    .params
                    .iter()
                    .map(|diplomat_core::hir::Param { name, ty, .. }| {
                        let name: Cow<str> = self.formatter.fmt_param_name(name.as_str()).into();
                        visitor.visit_param(ty, name.as_ref());
                        Param {
                            name,
                            ty: self.formatter.fmt_java_type(ty),
                        }
                    })
                    .collect();
                let lt_lookup = visitor.borrow_map();
                let (lifetime_edges, boxed_return) = match &method.output {
                    ReturnType::Fallible(SuccessType::OutType(o), _)
                    | ReturnType::Nullable(SuccessType::OutType(o))
                    | ReturnType::Infallible(SuccessType::OutType(o)) => {
                        let boxed_return = match o {
                            hir::Type::Slice(Slice::Str(None, _) | Slice::Primitive(None, _)) => {
                                Some(ParamConversion {
                                    converted_value: "boxArena".into(),
                                    conversion_def: "var boxArena = Arena.ofConfined();".into(),
                                })
                            }
                            hir::Type::Slice(
                                Slice::Str(Some(_), _) | Slice::Primitive(Some(_), _),
                            ) => Some(ParamConversion {
                                converted_value: "arena".into(),
                                conversion_def: "".into(),
                            }),
                            hir::Type::Struct(_) => Some(ParamConversion {
                                converted_value: "returnArena".into(),
                                conversion_def:
                                    "var returnArena = (SegmentAllocator) Arena.ofAuto();".into(),
                            }),
                            _ => None,
                        };
                        let method_lts = &method.lifetime_env;
                        let lifetime_edges = o
                            .lifetimes()
                            .filter_map(|lt| match lt {
                                MaybeStatic::Static => None,
                                MaybeStatic::NonStatic(lt) => Some(lt),
                            })
                            .filter_map(|lt| lt_lookup.get(&lt).map(|param| (lt, param)))
                            .map(|(lt, param)| {
                                let mut iter = param
                                    .incoming_edges
                                    .iter()
                                    .map(|edge| edge.param_name.as_str());
                                let first = iter.next().unwrap_or("").to_string();
                                let edges = iter.fold(first, |mut accum, next| {
                                    accum.push_str(", ");
                                    accum.push_str(next);
                                    accum
                                });
                                format!(
                                    "returnVal.{}Edges = List.of({edges});",
                                    method_lts.fmt_lifetime(lt)
                                )
                            })
                            .collect::<Vec<_>>();
                        (lifetime_edges, boxed_return)
                    }
                    _ => (Vec::new(), None),
                };
                let return_conversion = if method_name.is_none() {
                    OpaqueConstructorTpl {
                        return_ty: return_ty.clone(),
                    }
                    .render()
                    .unwrap_or_else(|err| {
                        panic!(
                            "Failed to render method {} for type {}. Cause: {err}",
                            method.name, ty_name
                        )
                    })
                    .cown()
                } else {
                    match self.gen_return_conversion(&method.output, lifetime_edges) {
                        Ok(ok) => ok,
                        Err(err) => {
                            self.errors.push_error(format!(
                                "can't construct method {} because {err}",
                                method.name
                            ));
                            return None;
                        }
                    }
                };
                let allocations =
                    method
                        .params
                        .iter()
                        .any(|diplomat_core::hir::Param { ty, .. }| {
                            matches!(
                                ty,
                                diplomat_core::hir::Type::Slice(
                                    Slice::Str(Some(_), _)
                                        | Slice::Primitive(Some(_), _)
                                        | Slice::Strs(_)
                                )
                            )
                        });
                let allocations = allocations
                    || match &method.output {
                        ReturnType::Infallible(SuccessType::OutType(o))
                        | ReturnType::Fallible(SuccessType::OutType(o), _)
                        | ReturnType::Nullable(SuccessType::OutType(o)) => matches!(
                            o,
                            hir::Type::Slice(Slice::Str(Some(_), _) | Slice::Primitive(Some(_), _),)
                        ),
                        _ => false,
                    };

                let mut param_conversions: Vec<_> = boxed_return
                    .into_iter()
                    .chain(method.param_self.iter().map(|_| ParamConversion {
                        converted_value: "internal".into(),
                        conversion_def: "".into(),
                    }))
                    .chain(
                        method
                            .params
                            .iter()
                            .map(|param| self.gen_param_conversion(param)),
                    )
                    .collect();
                let write_return = matches!(
                    method.output,
                    ReturnType::Fallible(SuccessType::Write, _)
                        | ReturnType::Infallible(SuccessType::Write)
                        | ReturnType::Nullable(SuccessType::Write)
                );
                if write_return {
                    param_conversions.push(ParamConversion {
                        converted_value: "writeable".into(),
                        conversion_def: format!(
                            "var writeable = {lib_name}_h.diplomat_buffer_write_create(0);"
                        )
                        .into(),
                    })
                }
                let native_method: Cow<str> = format!(
                    "{lib_name}_h.{}",
                    self.formatter.fmt_c_method_name(ty_id, method)
                )
                .into();
                let make_invoker =
                    method.params.is_empty() && !write_return && method.param_self.is_none();
                let native_invocation = if make_invoker {
                    "nativeInvoker.apply".into()
                } else {
                    native_method.clone()
                };
                let native_return_void = matches!(
                    method.output,
                    ReturnType::Infallible(SuccessType::Unit | SuccessType::Write)
                );
                let method_rendered = MethodTpl {
                    method_name,
                    is_static: method.param_self.is_none() && !is_valid_constructor,
                    return_ty,
                    native_method,
                    native_invocation,
                    make_invoker,
                    params,
                    param_conversions,
                    return_conversion,
                    allocations,
                    native_return_void,
                }
                .render()
                .unwrap_or_else(|err| {
                    panic!(
                        "Failed to render method {} for type {}. Cause: {err}",
                        method.name, ty_name
                    )
                });

                (method.param_self.is_some(), method_rendered.cown()).wrap_some()
            })
            .for_each(|(self_param, method_rendered)| match self_param {
                true => class_methods.push(method_rendered),
                false => static_methods.push(method_rendered),
            });

        (static_methods, class_methods)
    }

    fn gen_enum_def(&self, e: &EnumDef, ty: EnumId) -> (Cow<str>, String) {
        let Config { domain, lib_name } = &self.tcx_config;
        let type_name = e.name.as_str();
        let variants = e
            .variants
            .iter()
            .map(
                |EnumVariant {
                     name, discriminant, ..
                 }| {
                    let name = name.as_str().into();
                    let index = discriminant.to_string().into();
                    VariantTpl { name, index }
                },
            )
            .collect();
        let (methods, _) = self.gen_methods(ty.into(), e.name.as_str(), &e.methods);
        (
            format!("{type_name}.java").into(),
            EnumTypeTpl {
                type_name: type_name.into(),
                lib_name: lib_name.clone(),
                domain: domain.clone(),
                variants,
                methods,
            }
            .render()
            .expect("failed to render struct type"),
        )
    }
    fn gen_struct_def(&self, s: &StructDef, ty: TypeId) -> (Cow<str>, String) {
        let Config { domain, lib_name } = &self.tcx_config;
        let type_name = s.name.as_str();
        let fields = s
            .fields
            .iter()
            .map(|field @ StructField { ty, .. }| {
                let name = self.formatter.fmt_field_name(field);
                let struct_return = match ty {
                    hir::Type::Enum(enum_def) => Some(
                        format!("{}.fromInt", self.tcx.resolve_enum(enum_def.tcx_id).name).into(),
                    ),
                    hir::Type::Struct(struct_def) => Some(
                        format!(
                            "{}.fromSegment",
                            self.tcx.resolve_struct(struct_def.tcx_id).name
                        )
                        .into(),
                    ),
                    _ => None,
                };
                let ty = self.formatter.fmt_java_type(ty);
                FieldTpl {
                    name,
                    ty,
                    field_transform: struct_return,
                }
            })
            .collect();
        let edges = s
            .lifetimes
            .lifetimes()
            .lifetimes()
            .filter_map(|lt| match lt {
                MaybeStatic::Static => None,
                MaybeStatic::NonStatic(lt) => Some(lt),
            })
            .map(|lt| s.lifetimes.fmt_lifetime(lt))
            .collect();
        let (methods, _) = self.gen_methods(ty, s.name.as_str(), &s.methods);
        (
            format!("{type_name}.java").into(),
            StructTypeTpl {
                type_name: type_name.into(),
                lib_name: lib_name.clone(),
                domain: domain.clone(),
                edges,
                fields,
                methods,
            }
            .render()
            .expect("failed to render struct type"),
        )
    }

    fn gen_opaque_def(&self, o: &OpaqueDef, ty: TypeId) -> (Cow<str>, String) {
        let Config { domain, lib_name } = &self.tcx_config;
        let (static_methods, class_methods) = self.gen_methods(ty, o.name.as_str(), &o.methods);

        let edges = o
            .lifetimes
            .lifetimes()
            .lifetimes()
            .filter_map(|lt| match lt {
                MaybeStatic::Static => None,
                MaybeStatic::NonStatic(lt) => Some(lt),
            })
            .map(|lt| o.lifetimes.fmt_lifetime(lt))
            .collect();

        let opaque_tpl = OpaqueTypeTpl {
            type_name: o.name.to_string().into(),
            lib_name: lib_name.clone(),
            domain: domain.clone(),
            edges,
            static_methods,
            class_methods,
        };

        (
            format!("{}.java", o.name).into(),
            opaque_tpl.render().expect("Failed to render opaque type"),
        )
    }
}

trait PostFix: Sized {
    fn wrap_ok<E>(self) -> Result<Self, E> {
        Ok(self)
    }
    fn wrap_err<O>(self) -> Result<O, Self> {
        Err(self)
    }

    fn wrap_some(self) -> Option<Self> {
        Some(self)
    }
}

impl<T> PostFix for T {}

trait PostFixCown {
    fn cown<'a>(self) -> Cow<'a, str>;
}

impl PostFixCown for String {
    fn cown<'a>(self) -> Cow<'a, str> {
        Cow::Owned(self)
    }
}

#[cfg(test)]
mod test {

    use askama::Template;
    use diplomat_core::hir::{TypeDef, TypeId};
    use quote::quote;

    use crate::{common::ErrorStore, java::Config, test::new_tcx};

    use super::{formatter::JavaFormatter, OpaqueTypeTpl, TyGenContext};
    #[test]
    fn test_opaque_render() {
        let opaque_type = OpaqueTypeTpl {
            type_name: "Opaque2".into(),
            lib_name: "somelib".into(),
            domain: "dev.diplomattest".into(),
            edges: Vec::new(),
            static_methods: Vec::new(),
            class_methods: Vec::new(),
        };

        let rendered = opaque_type.render().expect("Failed to render opaque type");
        insta::assert_snapshot!(rendered);
    }
    #[test]
    fn test_slice() {
        let tk_stream = quote! {
            #[diplomat::bridge]
            mod ffi {

                #[diplomat::opaque]
                struct MyString(String);

                impl MyString {
                    #[diplomat::attr(supports = constructors, constructor)]
                    pub fn new(v: &DiplomatStr) -> Box<MyString> {
                        Box::new(Self(String::from_utf8(v.to_owned()).unwrap()))
                    }

                    #[diplomat::attr(supports = named_constructors, named_constructor = "unsafe")]
                    pub fn new_unsafe(v: &str) -> Box<MyString> {
                        Box::new(Self(v.to_string()))
                    }

                    pub fn new_owned(v: Box<DiplomatStr>) -> Box<MyString> {
                        Box::new(Self(String::from_utf8(v.into()).unwrap()))
                    }

                    #[diplomat::skip_if_ast]
                    pub fn new_from_first(v: &[&DiplomatStr]) -> Box<MyString> {
                        Box::new(Self(core::str::from_utf8(v[0]).unwrap().into()))
                    }

                    #[diplomat::attr(supports = accessors, setter = "str")]
                    pub fn set_str(&mut self, new_str: &DiplomatStr) {
                        self.0 = String::from_utf8(new_str.to_owned()).unwrap();
                    }

                    #[diplomat::attr(supports = accessors, getter = "str")]
                    pub fn get_str(&self, write: &mut DiplomatWrite) {
                        let _infallible = write!(write, "{}", self.0);
                    }

                    #[diplomat::skip_if_ast]
                    pub fn get_boxed_str(&self) -> Box<str> {
                        self.0.as_str().into()
                    }
                }

                #[diplomat::opaque]
                struct Float64Vec(Vec<f64>);

                impl Float64Vec {
                    pub fn new(v: &[f64]) -> Box<Float64Vec> {
                        Box::new(Self(v.to_vec()))
                    }

                    #[diplomat::attr(supports = accessors, getter = "asBoxedSlice")]
                    pub fn as_boxed_slice(&self) -> Box<[f64]> {
                        self.0.clone().into()
                    }

                    #[diplomat::attr(supports = accessors, getter = "asSlice")]
                    pub fn as_slice<'a>(&'a self) -> &'a [f64] {
                        &self.0
                    }
                }
            }
        };

        let tcx = new_tcx(tk_stream);

        let formatter = JavaFormatter::new(&tcx);

        let errors = ErrorStore::default();
        let tcx_gen = TyGenContext {
            tcx: &tcx,
            tcx_config: Config {
                domain: "dev.diplomattest".into(),
                lib_name: "somelib".into(),
            },
            formatter: &formatter,
            errors: &errors,
        };

        let mut res = String::new();
        for (ty, def) in tcx.all_types() {
            let rendered = match def {
                TypeDef::Opaque(opaque) => {
                    let (_, rendered) = tcx_gen.gen_opaque_def(opaque, ty);
                    rendered
                }
                _ => String::new(),
            };

            res.push_str(&rendered);
            res.push_str("\n============================\n")
        }
        insta::assert_snapshot!(res);
    }

    #[test]
    fn test_enum_and_struct() {
        let tk_stream = quote! {
            #[diplomat::bridge]
            mod ffi {

                pub struct MyStruct {
                    a: u8,
                    b: bool,
                    c: u8,
                    d: u64,
                    e: i32,
                    f: DiplomatChar,
                    g: MyEnum,
                }

                impl MyStruct {
                    #[diplomat::attr(supports = constructors, constructor)]
                    pub fn new() -> MyStruct {
                        MyStruct {
                            a: 17,
                            b: true,
                            c: 209,
                            d: 1234,
                            e: 5991,
                            f: '餐' as DiplomatChar,
                            g: MyEnum::B,
                        }
                    }

                    pub fn into_a(self) -> u8 {
                        self.a
                    }

                    fn assert_value(&self) {
                        assert_eq!(self.a, 17);
                        assert!(self.b);
                        assert_eq!(self.c, 209);
                        assert_eq!(self.d, 1234);
                        assert_eq!(self.e, 5991);
                        assert_eq!(self.f, '餐' as DiplomatChar);
                        assert_eq!(self.g, MyEnum::B);
                    }

                }

                #[derive(Debug, PartialEq, Eq)]
                pub enum MyEnum {
                    A = -2,
                    B = -1,
                    C = 0,
                    D = 1,
                    E = 2,
                    F = 3,
                }

                impl MyEnum {
                    pub fn into_value(self) -> i8 {
                        self as i8
                    }

                    pub fn get_a() -> MyEnum {
                        MyEnum::A
                    }
                }
            }
        };

        let tcx = new_tcx(tk_stream);

        let formatter = JavaFormatter::new(&tcx);

        let errors = ErrorStore::default();
        let tcx_gen = TyGenContext {
            tcx: &tcx,
            tcx_config: Config {
                domain: "dev.diplomattest".into(),
                lib_name: "somelib".into(),
            },
            formatter: &formatter,
            errors: &errors,
        };

        let mut res = String::new();
        for (ty, def) in tcx.all_types() {
            let rendered = match (ty, def) {
                (_, TypeDef::Struct(struct_def)) => {
                    let (_, rendered) = tcx_gen.gen_struct_def(struct_def, ty);
                    rendered
                }
                (TypeId::Enum(enum_id), TypeDef::Enum(enum_def)) => {
                    let (_, rendered) = tcx_gen.gen_enum_def(enum_def, enum_id);
                    rendered
                }
                _ => String::new(),
            };

            res.push_str(&rendered);
            res.push_str("\n============================\n")
        }
        insta::assert_snapshot!(res);
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

                    pub fn get_debug_str(&self, write: &mut DiplomatWrite) {
                        let _infallible = write!(write, "{:?}", &self.0);
                    }
                }

            }
        };

        let tcx = new_tcx(tk_stream);

        let formatter = JavaFormatter::new(&tcx);

        let errors = ErrorStore::default();
        let tcx_gen = TyGenContext {
            tcx: &tcx,
            tcx_config: Config {
                domain: "dev.diplomattest".into(),
                lib_name: "somelib".into(),
            },
            formatter: &formatter,
            errors: &errors,
        };

        let mut res = String::new();
        for (ty, def) in tcx.all_types() {
            let rendered = match (ty, def) {
                (_, TypeDef::Opaque(opaque)) => {
                    let (_, rendered) = tcx_gen.gen_opaque_def(opaque, ty);
                    rendered
                }
                (_, TypeDef::Struct(struct_def)) => {
                    let (_, rendered) = tcx_gen.gen_struct_def(struct_def, ty);
                    rendered
                }

                (TypeId::Enum(enum_id), TypeDef::Enum(enum_def)) => {
                    let (_, rendered) = tcx_gen.gen_enum_def(enum_def, enum_id);
                    rendered
                }
                _ => String::new(),
            };

            res.push_str(&rendered);
            res.push_str("\n============================\n")
        }
        insta::assert_snapshot!(res);
    }

    #[test]
    fn test_lifetimes() {
        let tk_stream = quote! {
            #[diplomat::bridge]
            pub mod ffi {
                use diplomat_runtime::DiplomatStr16;

                #[diplomat::opaque]
                pub struct Foo<'a>(&'a DiplomatStr);

                #[diplomat::opaque]
                #[diplomat::transparent_convert]
                pub struct Bar<'b, 'a: 'b>(&'b Foo<'a>);

                pub struct BorrowedFields<'a> {
                    a: &'a DiplomatStr16,
                    b: &'a DiplomatStr,
                    c: &'a str,
                }

                pub struct BorrowedFieldsWithBounds<'a, 'b: 'a, 'c: 'b> {
                    field_a: &'a DiplomatStr16,
                    field_b: &'b DiplomatStr,
                    field_c: &'c str,
                }

                pub struct BorrowedFieldsReturning<'a> {
                    bytes: &'a DiplomatStr,
                }

                impl<'a> Foo<'a> {
                    pub fn new(x: &'a DiplomatStr) -> Box<Self> {
                        Box::new(Foo(x))
                    }

                    pub fn get_bar<'b>(&'b self) -> Box<Bar<'b, 'a>> {
                        Box::new(Bar(self))
                    }

                    pub fn new_static(x: &'static DiplomatStr) -> Box<Self> {
                        Box::new(Foo(x))
                    }

                    pub fn as_returning(&self) -> BorrowedFieldsReturning<'a> {
                        BorrowedFieldsReturning { bytes: self.0 }
                    }

                    pub fn extract_from_fields(fields: BorrowedFields<'a>) -> Box<Self> {
                        Box::new(Foo(fields.b))
                    }

                    /// Test that the extraction logic correctly pins the right fields
                    pub fn extract_from_bounds<'x, 'y: 'x + 'a, 'z: 'x + 'y>(
                        bounds: BorrowedFieldsWithBounds<'x, 'y, 'z>,
                        another_string: &'a DiplomatStr,
                    ) -> Box<Self> {
                        if bounds.field_b.is_empty() {
                            Box::new(Self(another_string))
                        } else {
                            Box::new(Self(bounds.field_b))
                        }
                    }
                }
            }
        };

        let tcx = new_tcx(tk_stream);

        let formatter = JavaFormatter::new(&tcx);

        let errors = ErrorStore::default();
        let tcx_gen = TyGenContext {
            tcx: &tcx,
            tcx_config: Config {
                domain: "dev.diplomattest".into(),
                lib_name: "somelib".into(),
            },
            formatter: &formatter,
            errors: &errors,
        };

        let mut res = String::new();
        for (ty, def) in tcx.all_types() {
            let rendered = match (ty, def) {
                (_, TypeDef::Opaque(opaque)) => {
                    let (_, rendered) = tcx_gen.gen_opaque_def(opaque, ty);
                    rendered
                }
                (_, TypeDef::Struct(struct_def)) => {
                    let (_, rendered) = tcx_gen.gen_struct_def(struct_def, ty);
                    rendered
                }

                (TypeId::Enum(enum_id), TypeDef::Enum(enum_def)) => {
                    let (_, rendered) = tcx_gen.gen_enum_def(enum_def, enum_id);
                    rendered
                }
                _ => String::new(),
            };

            res.push_str(&rendered);
            res.push_str("\n============================\n")
        }
        insta::assert_snapshot!(res);
    }
}
