use askama::Template;
use diplomat_core::hir::borrowing_param::{BorrowedLifetimeInfo, ParamBorrowInfo};
use diplomat_core::hir::{
    self, Borrow, Lifetime, LifetimeEnv, Lifetimes, MaybeOwn, MaybeStatic, Method, Mutability,
    OpaquePath, Optional, ReturnableStructDef, SelfType, Slice, StringEncoding, StructField,
    StructPathLike, TyPosition, Type, TypeContext, TypeDef, TypeId,
};
use diplomat_core::hir::{ReturnType, SuccessType};

use std::borrow::Cow;
use std::collections::BTreeMap;
use std::iter::once;
use std::path::Path;

mod formatter;
use formatter::KotlinFormatter;

use crate::common::{ErrorStore, FileMap};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
struct KotlinConfig {
    domain: String,
    lib_name: String,
}

pub fn run(tcx: &TypeContext, conf_path: Option<&Path>) -> FileMap {
    let conf_path = conf_path.expect("Kotlin library needs to be called with config");

    let conf_str = std::fs::read_to_string(conf_path)
        .unwrap_or_else(|err| panic!("Failed to open config file {conf_path:?}: {err}"));
    let KotlinConfig { domain, lib_name } = toml::from_str::<KotlinConfig>(&conf_str)
        .expect("Failed to parse config. Required fields are `domain` and `lib_name`");

    let formatter = KotlinFormatter::new(tcx, None);

    let files = FileMap::default();
    let errors = ErrorStore::default();

    let mut ty_gen_cx = TyGenContext {
        tcx,
        errors: &errors,
        formatter: &formatter,
    };

    for (id, ty) in tcx.all_types() {
        let _guard = ty_gen_cx.errors.set_context_ty(ty.name().as_str().into());
        if ty.attrs().disable {
            continue;
        }
        if let TypeDef::Opaque(o) = ty {
            let type_name = o.name.to_string();

            let (file_name, body) = ty_gen_cx.gen_opaque_def(o, id, &type_name, &domain, &lib_name);

            files.add_file(format!("src/main/kotlin/{file_name}"), body);
        }

        if let TypeDef::OutStruct(o) = ty {
            let type_name = o.name.to_string();

            let (file_name, body) = ty_gen_cx.gen_struct_def(o, id, &type_name, &domain, &lib_name);

            files.add_file(format!("src/main/kotlin/{file_name}"), body);
        }

        if let TypeDef::Struct(struct_def) = ty {
            let type_name = struct_def.name.to_string();

            let (file_name, body) =
                ty_gen_cx.gen_struct_def(struct_def, id, &type_name, &domain, &lib_name);

            files.add_file(format!("src/main/kotlin/{file_name}"), body);
        }

        if let TypeDef::Enum(enum_def) = ty {
            let type_name = enum_def.name.to_string();

            let (file_name, body) =
                ty_gen_cx.gen_enum_def(enum_def, id, &type_name, &domain, &lib_name);

            files.add_file(format!("src/main/kotlin/{file_name}"), body);
        }
    }

    #[derive(Template)]
    #[template(path = "kotlin/build.gradle.kts.jinja", escape = "none")]
    struct Build<'a> {
        domain: &'a str,
        lib_name: &'a str,
    }

    let build = Build {
        domain: &domain,
        lib_name: &lib_name,
    }
    .render()
    .expect("Failed to render build file");

    files.add_file("build.gradle.kts".to_string(), build);

    #[derive(Template)]
    #[template(path = "kotlin/settings.gradle.kts.jinja", escape = "none")]
    struct Settings<'a> {
        lib_name: &'a str,
    }
    let settings = Settings {
        lib_name: &lib_name,
    }
    .render()
    .expect("Failed to render settings file");

    files.add_file("settings.gradle.kts".to_string(), settings);

    #[derive(Template)]
    #[template(path = "kotlin/init.kt.jinja", escape = "none")]
    struct Init<'a> {
        domain: &'a str,
        lib_name: &'a str,
    }

    let init = Init {
        domain: &domain,
        lib_name: &lib_name,
    }
    .render()
    .expect("Failed to lib top level file");

    files.add_file(
        format!(
            "src/main/kotlin/{}/{lib_name}/Lib.kt",
            domain.replace('.', "/")
        ),
        init,
    );

    files
}

struct TyGenContext<'a, 'cx> {
    tcx: &'cx TypeContext,
    formatter: &'a KotlinFormatter<'cx>,
    errors: &'a ErrorStore<'cx, String>,
}

impl<'a, 'cx> TyGenContext<'a, 'cx> {
    fn gen_return_type_name(&self, result_ty: &ReturnType) -> Cow<'cx, str> {
        match *result_ty {
            ReturnType::Infallible(SuccessType::Unit)
            | ReturnType::Fallible(SuccessType::Unit, Some(_)) => self.formatter.fmt_void().into(),
            ReturnType::Infallible(SuccessType::Write)
            | ReturnType::Fallible(SuccessType::Write, Some(_)) => {
                self.formatter.fmt_string().into()
            }
            ReturnType::Infallible(SuccessType::OutType(ref o))
            | ReturnType::Fallible(SuccessType::OutType(ref o), Some(_)) => self.gen_type_name(o),
            ReturnType::Fallible(SuccessType::Write, None)
            | ReturnType::Nullable(SuccessType::Write) => self
                .formatter
                .fmt_nullable(self.formatter.fmt_string())
                .into(),
            ReturnType::Fallible(SuccessType::Unit, None)
            | ReturnType::Nullable(SuccessType::Unit) => self
                .formatter
                .fmt_primitive_as_ffi(hir::PrimitiveType::Bool)
                .into(),
            ReturnType::Fallible(SuccessType::OutType(ref o), None)
            | ReturnType::Nullable(SuccessType::OutType(ref o)) => {
                self.formatter.fmt_nullable(&self.gen_type_name(o)).into()
            }
            _ => unreachable!("unknown AST/HIR variant"),
        }
    }

    fn gen_kt_to_c_for_type(&self, ty: &Type, name: Cow<'cx, str>) -> Cow<'cx, str> {
        match *ty {
            Type::Primitive(_) => name,
            Type::Opaque(ref op @ OpaquePath { owner, .. }) => {
                let optional = if op.is_optional() { "?" } else { "" };
                match owner.mutability {
                    Mutability::Immutable => format!("{name}{optional}.handle").into(),
                    Mutability::Mutable => panic!("Not comfortable with mutable access in the JVM just yet. We'll add some mutexes to the code gen"),
                }
            }
            Type::Struct(_) => format!("{name}.nativeStruct").into(),
            Type::Enum(_) => format!("{name}.toNative()").into(),
            Type::Slice(Slice::Str(None, _)) | Type::Slice(Slice::Primitive(None, _)) => {
                format!("{name}Slice").into()
            }
            Type::Slice(_) => format!("{name}Slice").into(),
            _ => todo!(),
        }
    }

    fn gen_return_type_name_ffi(&self, out: &ReturnType) -> Cow<'cx, str> {
        match *out {
            ReturnType::Infallible(SuccessType::Unit) => self.formatter.fmt_void().into(),
            ReturnType::Infallible(SuccessType::Write) => self.formatter.fmt_void().into(),
            ReturnType::Infallible(SuccessType::OutType(ref o)) => self.gen_type_name_ffi(o),
            ReturnType::Fallible(_, _) => {
                todo!("Fallible return types not supported yet")
            }
            ReturnType::Nullable(SuccessType::Unit | SuccessType::Write) => {
                format!("{}?", self.formatter.fmt_void()).into()
            }

            ReturnType::Nullable(SuccessType::OutType(ref o)) => {
                format!("{}?", self.gen_type_name_ffi(o)).into()
            }
            _ => unreachable!("unknown AST/HIR variant"),
        }
    }

    fn gen_type_name_ffi<P: TyPosition>(&self, ty: &Type<P>) -> Cow<'cx, str> {
        match *ty {
            Type::Primitive(prim) => self.formatter.fmt_primitive_as_ffi(prim).into(),
            Type::Opaque(ref op) => {
                let optional = if op.is_optional() { "?" } else { "" };
                format!("Pointer{optional}").into()
            }

            Type::Struct(ref strct) => {
                let type_id = strct.id();
                let resolved = self.tcx.resolve_type(type_id);
                format!("{}Native", resolved.name()).into()
            }
            Type::Enum(_) => "Int".into(),
            Type::Slice(_) => "Slice".into(),
            _ => unreachable!("unknown AST/HIR variant"),
        }
    }

    fn gen_opaque_return<'d>(
        &'d self,
        opaque_path: &'d OpaquePath<Optional, MaybeOwn>,
        method_lifetimes_map: MethodLtMap<'d>,
        lifetime_env: &'d LifetimeEnv,
        cleanups: &[Cow<'d, str>],
    ) -> String {
        let opaque_def = opaque_path.resolve(self.tcx);

        let ownership = opaque_path.owner;
        let lifetimes = &opaque_path.lifetimes;
        let optional = opaque_path.is_optional();
        #[derive(Template)]
        #[template(path = "kotlin/OpaqueReturn.kt.jinja", escape = "none")]
        struct OpaqueReturn<'a, 'b> {
            return_type_name: Cow<'b, str>,
            borrows: Vec<ParamsForLt<'b>>,
            is_owned: bool,
            self_edges: Vec<Cow<'b, str>>,
            cleanups: &'a [Cow<'b, str>],
            optional: bool,
        }

        struct ParamsForLt<'c> {
            lt: Cow<'c, str>,
            params: Vec<Cow<'c, str>>,
        }

        let return_type_name = opaque_def.name.to_string().into();
        let self_edges = || match ownership {
            MaybeOwn::Borrow(Borrow {
                lifetime: MaybeStatic::NonStatic(lt),
                ..
            }) => Some(
                method_lifetimes_map
                    .get(&lt)
                    .iter()
                    .flat_map(|param| param.incoming_edges.iter())
                    .map(move |edge| self.formatter.fmt_borrow(edge))
                    .collect(),
            ),
            _ => None,
        };

        let self_edges = self_edges();
        let is_owned = self_edges.is_none();
        let self_edges = self_edges.unwrap_or_else(Vec::new);

        let borrows = lifetimes
            .lifetimes()
            .filter_map(|lt| {
                let lt = match lt {
                    MaybeStatic::Static => return None,
                    MaybeStatic::NonStatic(lt) => lt,
                };
                let params = method_lifetimes_map
                    .get(&lt)
                    .iter()
                    .flat_map(|got| got.incoming_edges.iter())
                    .map(|edge| self.formatter.fmt_borrow(edge))
                    .collect();
                let lt = lifetime_env.fmt_lifetime(lt);
                Some(ParamsForLt { lt, params })
            })
            .collect::<Vec<_>>();

        let opaque_return = OpaqueReturn {
            return_type_name,
            borrows,
            is_owned,
            self_edges,
            cleanups,
            optional,
        };
        opaque_return
            .render()
            .expect("Failed to render opaque return block")
    }

    const WRITE_RETURN: &'static str = "\nreturn DW.writeToString(write)";

    fn boxed_slice_return(encoding: &str) -> String {
        format!(
            r#"    val string = PrimitiveArrayTools.get{encoding}(returnVal)
Native.free(Pointer.nativeValue(returnVal.data))
return string"#
        )
    }

    fn gen_slice_retrn<'d>(&'d self, slice_ty: &'d Slice) -> String {
        match slice_ty {
            Slice::Str(Some(_), enc) => match enc {
                StringEncoding::UnvalidatedUtf16 => {
                    "    return PrimitiveArrayTools.getUtf16(returnVal)".into()
                }
                StringEncoding::UnvalidatedUtf8 => {
                    "    return PrimitiveArrayTools.getUtf8(returnVal)".into()
                }
                StringEncoding::Utf8 => "    return PrimitiveArrayTools.getUtf8(returnVal)".into(),
                _ => todo!(),
            },
            Slice::Str(None, enc) => match enc {
                StringEncoding::UnvalidatedUtf16 => Self::boxed_slice_return("Utf16"),
                StringEncoding::UnvalidatedUtf8 => Self::boxed_slice_return("Utf8"),
                StringEncoding::Utf8 => Self::boxed_slice_return("Utf8"),
                _ => todo!(),
            },
            Slice::Primitive(Some(_), prim_ty) => {
                let prim_ty = self.formatter.fmt_primitive_as_ffi(*prim_ty);
                format!("    return PrimitiveArrayTools.get{prim_ty}Array(returnVal)")
            }
            Slice::Primitive(None, prim_ty) => {
                let prim_ty = self.formatter.fmt_primitive_as_ffi(*prim_ty);
                let prim_ty_array = format!("{prim_ty}Array");
                Self::boxed_slice_return(prim_ty_array.as_str())
            }

            _ => todo!(),
        }
    }

    fn gen_struct_return<'d>(
        &'d self,
        struct_def: &'d ReturnableStructDef,
        // ownership: MaybeOwn,
        lifetimes: &'d Lifetimes,
        method_lifetimes_map: MethodLtMap<'d>,
        lifetime_env: &'d LifetimeEnv,
        cleanups: &[Cow<'d, str>],
    ) -> String {
        #[derive(Template)]
        #[template(path = "kotlin/StructReturn.kt.jinja", escape = "none")]
        struct StructReturn<'a, 'b> {
            return_type_name: Cow<'b, str>,
            borrows: Vec<ParamsForLt<'b>>,
            cleanups: &'a [Cow<'b, str>],
        }

        struct ParamsForLt<'c> {
            lt: Cow<'c, str>,
            params: Vec<Cow<'c, str>>,
        }

        let return_type_name = match struct_def {
            ReturnableStructDef::Struct(strct) => strct.name.to_string().into(),
            ReturnableStructDef::OutStruct(out_strct) => out_strct.name.to_string().into(),
            _ => todo!(),
        };

        let borrows = lifetimes
            .lifetimes()
            .filter_map(|lt| {
                let lt = match lt {
                    MaybeStatic::Static => return None,
                    MaybeStatic::NonStatic(lt) => lt,
                };
                let params = method_lifetimes_map
                    .get(&lt)
                    .iter()
                    .flat_map(|got| got.incoming_edges.iter())
                    .map(|edge| self.formatter.fmt_borrow(edge))
                    .collect();
                let lt = lifetime_env.fmt_lifetime(lt);
                Some(ParamsForLt { lt, params })
            })
            .collect::<Vec<_>>();

        let opaque_return = StructReturn {
            return_type_name,
            borrows,
            cleanups,
        };
        opaque_return
            .render()
            .expect("Failed to render opaque return block")
    }

    fn gen_infallible_return<'d>(
        &'d self,
        res: &'d SuccessType,
        method: &'d Method,
        method_lifetimes_map: MethodLtMap<'d>,
        cleanups: &[Cow<'d, str>],
    ) -> Option<String> {
        match res {
            SuccessType::Write => Some(Self::WRITE_RETURN.into()),
            SuccessType::OutType(o) => match o {
                // todo: unsigned need to be handled
                Type::Primitive(_) => Some("    return returnVal".into()),
                Type::Opaque(opaque_path) => Some(self.gen_opaque_return(
                    opaque_path,
                    method_lifetimes_map,
                    &method.lifetime_env,
                    cleanups,
                )),
                Type::Struct(strct) => {
                    let lifetimes = strct.lifetimes();
                    Some(self.gen_struct_return(
                        &strct.resolve(self.tcx),
                        lifetimes,
                        method_lifetimes_map,
                        &method.lifetime_env,
                        cleanups,
                    ))
                }
                Type::Enum(enm) => {
                    let return_type = enm.resolve(self.tcx);
                    Some(format!(
                        "    return {}.fromNative(returnVal)",
                        return_type.name
                    ))
                }
                Type::Slice(slc) => Some(self.gen_slice_retrn(slc)),
                _ => todo!(),
            },
            SuccessType::Unit => None,
            _ => todo!(),
        }
    }

    fn gen_return<'d>(
        &'d self,
        method: &'d Method,
        method_lifetimes_map: MethodLtMap<'d>,
        cleanups: &[Cow<'d, str>],
    ) -> Option<String> {
        match &method.output {
            ReturnType::Infallible(res) => {
                self.gen_infallible_return(res, method, method_lifetimes_map, cleanups)
            }
            ReturnType::Fallible(_, _) => todo!("fallible returns not yet supported"),
            ReturnType::Nullable(res) => self
                .gen_infallible_return(res, method, method_lifetimes_map, cleanups)
                .map(|return_val| {
                    format!(
                        r#"
if (returnVal == null) {{
    return null
}} else {{
    {return_val}
}}"#,
                    )
                }),
        }
    }
    fn gen_slice_conv(&self, kt_param_name: Cow<'cx, str>, slice_type: Slice) -> Cow<'cx, str> {
        #[derive(Template)]
        #[template(path = "kotlin/SliceConv.kt.jinja", escape = "none")]
        struct SliceConv<'d> {
            slice_method: Cow<'d, str>,
            kt_param_name: Cow<'d, str>,
            closeable: bool,
        }
        let (slice_method, closeable): (Cow<'cx, str>, bool) = match slice_type {
            Slice::Str(_, StringEncoding::UnvalidatedUtf16) => ("readUtf16".into(), true),
            Slice::Str(_, _) => ("readUtf8".into(), true),
            Slice::Primitive(_, _) => ("native".into(), true),
            Slice::Strs(StringEncoding::UnvalidatedUtf16) => ("readUtf16s".into(), true),
            Slice::Strs(_) => ("readUtf8s".into(), true),
            _ => {
                self.errors
                    .push_error("Found unsupported slice type".into());
                ("".into(), false)
            }
        };

        SliceConv {
            kt_param_name,
            slice_method,
            closeable,
        }
        .render()
        .expect("Failed to render slice method")
        .into()
    }

    fn gen_cleanup(&self, param_name: Cow<'cx, str>, slice: Slice) -> Option<Cow<'cx, str>> {
        match slice {
            Slice::Str(Some(_), _) => Some(format!("{param_name}Mem.close()").into()),
            Slice::Str(_, _) => None,
            Slice::Primitive(Some(_), _) => Some(format!("{param_name}Mem.close()").into()),
            Slice::Primitive(_, _) => None,
            Slice::Strs(_) => Some(format!("{param_name}Mem.forEach {{it.close()}}").into()),
            _ => todo!(),
        }
    }

    fn gen_method(
        &mut self,
        id: TypeId,
        method: &'cx hir::Method,
        self_type: Option<&'cx SelfType>,
    ) -> String {
        let mut visitor = method.borrowing_param_visitor(self.tcx);
        let native_method_name = self.formatter.fmt_c_method_name(id, method);

        let mut param_decls_kt = Vec::with_capacity(method.params.len());
        let mut param_types_ffi = Vec::with_capacity(method.params.len());
        let mut param_conversions = Vec::with_capacity(method.params.len());
        let mut slice_conversions = Vec::with_capacity(method.params.len());
        let mut cleanups = Vec::with_capacity(method.params.len());

        match self_type {
            Some(st @ SelfType::Opaque(_)) => {
                let param_type = "Pointer".into();
                let param_name: Cow<'_, str> = "handle".into();
                visitor.visit_param(&st.clone().into(), "this");

                param_types_ffi.push(param_type);
                param_conversions.push(param_name.clone());
            }
            Some(st @ SelfType::Struct(s)) => {
                let param_type =
                    format!("{}Native", self.tcx.resolve_struct(s.tcx_id).name.as_str()).into();
                let param_name: Cow<'_, str> = "nativeStruct".into();
                visitor.visit_param(&st.clone().into(), "this");
                param_types_ffi.push(param_type);
                param_conversions.push(param_name.clone());
            }
            Some(SelfType::Enum(_)) => {
                let param_type = "Int".into();
                let param_conversion: Cow<'_, str> = "this.toNative()".into();
                param_types_ffi.push(param_type);
                param_conversions.push(param_conversion.clone());
            }
            None => (),
            _ => todo!(),
        };

        for param in method.params.iter() {
            let param_name = self.formatter.fmt_param_name(param.name.as_str());

            let param_type_ffi = self.gen_type_name_ffi(&param.ty);

            match param.ty {
                Type::Slice(slice) => {
                    slice_conversions.push(self.gen_slice_conv(param_name.clone(), slice));

                    let param_borrow_kind = visitor.visit_param(&param.ty, &param_name);

                    match param_borrow_kind {
                        ParamBorrowInfo::Struct(_) => (),
                        ParamBorrowInfo::TemporarySlice => {
                            if let Some(cleanup) = self.gen_cleanup(param_name.clone(), slice) {
                                cleanups.push(cleanup)
                            }
                        }
                        ParamBorrowInfo::BorrowedSlice => (),
                        ParamBorrowInfo::BorrowedOpaque => (),
                        ParamBorrowInfo::NotBorrowed => (),
                        _ => todo!(),
                    };
                }

                Type::Struct(_) | Type::Opaque(_) => {
                    visitor.visit_param(&param.ty, &param_name);
                }
                _ => (),
            }

            param_decls_kt.push(format!("{param_name}: {}", self.gen_type_name(&param.ty)));
            param_types_ffi.push(param_type_ffi);
            param_conversions.push(self.gen_kt_to_c_for_type(&param.ty, param_name.clone()));
        }
        let write_return = matches!(&method.output, ReturnType::Infallible(SuccessType::Write));
        if write_return {
            param_conversions.push("write".into());
        }
        let params = param_decls_kt.join(", ");

        let return_ty = self.gen_return_type_name(&method.output);

        let declaration = format!(
            "fun {}({}): {return_ty}",
            self.formatter.fmt_method_name(method),
            params
        );
        let method_lifetimes_map = visitor.borrow_map();
        let return_expression = self
            .gen_return(method, method_lifetimes_map, cleanups.as_ref())
            .map(From::from);

        MethodTpl {
            // todo: comment,
            declaration,
            native_method_name,
            param_conversions,
            return_expression,
            write_return,
            slice_conversions,
        }
        .render()
        .expect("Failed to render string for method")
    }

    fn gen_native_method_info(&mut self, id: TypeId, method: &'cx hir::Method) -> NativeMethodInfo {
        let mut param_decls = Vec::with_capacity(method.params.len());

        let mut visitor = method.borrowing_param_visitor(self.tcx);

        if let Some(param_self) = method.param_self.as_ref() {
            match &param_self.ty {
                SelfType::Opaque(_) => param_decls.push("handle: Pointer".into()),
                SelfType::Struct(s) => param_decls.push(format!(
                    "nativeStruct: {}Native",
                    self.tcx.resolve_struct(s.tcx_id).name.as_str()
                )),
                SelfType::Enum(_) => param_decls.push("inner: Int".into()),
                _ => todo!(),
            }
        };
        for param in method.params.iter() {
            let param_name = self.formatter.fmt_param_name(param.name.as_str());

            visitor.visit_param(&param.ty, &param_name);

            param_decls.push(format!(
                "{param_name}: {}",
                self.gen_native_type_name(&param.ty)
            ));
        }
        if let ReturnType::Infallible(SuccessType::Write) = method.output {
            param_decls.push("write: Pointer".into())
        }
        let params = param_decls.join(", ");
        let native_method = self.formatter.fmt_c_method_name(id, method);
        let return_ty = self.gen_return_type_name_ffi(&method.output);

        NativeMethodInfo {
            declaration: format!("fun {native_method}({params}): {return_ty}"),
        }
    }

    fn gen_opaque_def(
        &mut self,
        ty: &'cx hir::OpaqueDef,
        id: TypeId,
        type_name: &str,
        domain: &str,
        lib_name: &str,
    ) -> (String, String) {
        let native_methods = ty
            .methods
            .iter()
            .filter(|m| !m.attrs.disable)
            .map(|method| self.gen_native_method_info(id, method))
            .collect::<Vec<_>>();

        let self_methods = ty
            .methods
            .iter()
            .filter(|m| !m.attrs.disable)
            .filter_map(|method| {
                method
                    .param_self
                    .as_ref()
                    .map(|self_param| (&self_param.ty, method))
            })
            .map(|(self_param, method)| self.gen_method(id, method, Some(self_param)))
            .collect::<Vec<_>>();

        let companion_methods = ty
            .methods
            .iter()
            .filter(|m| !m.attrs.disable)
            .filter(|method| method.param_self.is_none())
            .map(|method| self.gen_method(id, method, None))
            .collect::<Vec<_>>();

        let lifetimes = ty
            .lifetimes
            .lifetimes()
            .lifetimes()
            .filter_map(|lt| match lt {
                MaybeStatic::Static => None,
                MaybeStatic::NonStatic(lt) => Some(lt),
            })
            .map(|lt| ty.lifetimes.fmt_lifetime(lt))
            .collect();

        #[derive(Template)]
        #[template(path = "kotlin/Opaque.kt.jinja", escape = "none")]
        struct ImplTemplate<'a> {
            domain: &'a str,
            lib_name: &'a str,
            type_name: &'a str,
            self_methods: &'a [String],
            companion_methods: &'a [String],
            native_methods: &'a [NativeMethodInfo],
            lifetimes: Vec<Cow<'a, str>>,
        }

        (
            format!("{}/{lib_name}/{type_name}.kt", domain.replace('.', "/")),
            ImplTemplate {
                domain,
                lib_name,
                type_name,
                self_methods: self_methods.as_ref(),
                companion_methods: companion_methods.as_ref(),
                native_methods: native_methods.as_ref(),
                lifetimes,
            }
            .render()
            .expect("failed to generate struct"),
        )
    }

    fn gen_struct_def<P: TyPosition>(
        &mut self,
        ty: &'cx hir::StructDef<P>,
        id: TypeId,
        type_name: &str,
        domain: &str,
        lib_name: &str,
    ) -> (String, String) {
        let native_methods = ty
            .methods
            .iter()
            .filter(|m| !m.attrs.disable)
            .map(|method| self.gen_native_method_info(id, method))
            .collect::<Vec<_>>();

        let self_methods = ty
            .methods
            .iter()
            .filter_map(|method| {
                method
                    .param_self
                    .as_ref()
                    .map(|self_param| (&self_param.ty, method))
            })
            .map(|(self_param, method)| self.gen_method(id, method, Some(self_param)))
            .collect::<Vec<_>>();

        let companion_methods = ty
            .methods
            .iter()
            .filter(|method| method.param_self.is_none())
            .map(|method| self.gen_method(id, method, None))
            .collect::<Vec<_>>();

        let lifetimes = ty
            .lifetimes
            .lifetimes()
            .lifetimes()
            .filter_map(|lt| match lt {
                MaybeStatic::Static => None,
                MaybeStatic::NonStatic(lt) => Some(lt),
            })
            .map(|lt| ty.lifetimes.fmt_lifetime(lt))
            .collect();

        struct StructFieldDef<'d> {
            name: Cow<'d, str>,
            ffi_type_default: Cow<'d, str>,
            ffi_cast_type_name: Cow<'d, str>,
            field_type: Cow<'d, str>,
            native_to_kt: Cow<'d, str>,
        }

        #[derive(Template)]
        #[template(path = "kotlin/Struct.kt.jinja", escape = "none")]
        struct ImplTemplate<'a> {
            domain: &'a str,
            lib_name: &'a str,
            type_name: &'a str,
            fields: Vec<StructFieldDef<'a>>,
            self_methods: &'a [String],
            companion_methods: &'a [String],
            native_methods: &'a [NativeMethodInfo],
            lifetimes: Vec<Cow<'a, str>>,
        }

        let fields = ty
            .fields
            .iter()
            .map(|field: &StructField<P>| {
                let field_name = self.formatter.fmt_field_name(field.name.as_str());

                StructFieldDef {
                    name: field_name.clone(),
                    ffi_type_default: self.formatter.fmt_field_default(&field.ty),
                    ffi_cast_type_name: self.formatter.fmt_struct_field_type_native(&field.ty),
                    field_type: self.formatter.fmt_struct_field_type_kt(&field.ty),
                    native_to_kt: self.formatter.fmt_struct_field_native_to_kt(
                        field_name.as_ref(),
                        &ty.lifetimes,
                        &field.ty,
                    ),
                }
            })
            .collect();

        (
            format!("{}/{lib_name}/{type_name}.kt", domain.replace('.', "/"),),
            ImplTemplate {
                domain,
                lib_name,
                type_name,
                fields,
                self_methods: self_methods.as_ref(),
                companion_methods: companion_methods.as_ref(),
                native_methods: native_methods.as_ref(),
                lifetimes,
            }
            .render()
            .expect("Failed to render struct template"),
        )
    }

    fn gen_enum_def(
        &mut self,
        ty: &'cx hir::EnumDef,
        id: TypeId,
        type_name: &str,
        domain: &str,
        lib_name: &str,
    ) -> (String, String) {
        let native_methods = ty
            .methods
            .iter()
            .filter(|m| !m.attrs.disable)
            .map(|method| self.gen_native_method_info(id, method))
            .collect::<Vec<_>>();

        let self_methods = ty
            .methods
            .iter()
            .filter(|m| !m.attrs.disable)
            .filter_map(|method| {
                method
                    .param_self
                    .as_ref()
                    .map(|self_param| (&self_param.ty, method))
            })
            .map(|(self_param, method)| self.gen_method(id, method, Some(self_param)))
            .collect::<Vec<_>>();

        let companion_methods = ty
            .methods
            .iter()
            .filter(|m| !m.attrs.disable)
            .filter(|method| method.param_self.is_none())
            .map(|method| self.gen_method(id, method, None))
            .collect::<Vec<_>>();

        #[derive(Clone, Debug)]
        struct NonContiguousEnumVariant<'d> {
            index: i32,
            name: Cow<'d, str>,
        }

        #[derive(Clone, Debug)]
        enum EnumVariants<'d> {
            Contiguous(Vec<Cow<'d, str>>),
            NonContiguous(Vec<NonContiguousEnumVariant<'d>>),
        }

        impl<'d> EnumVariants<'d> {
            fn new(ty: &'d hir::EnumDef) -> Self {
                let n_variants = ty.variants.len();
                ty.variants.iter().enumerate().fold(
                    EnumVariants::Contiguous(Vec::with_capacity(n_variants)),
                    |variants, (i, v)| match variants {
                        EnumVariants::Contiguous(mut vec) if i as isize == v.discriminant => {
                            vec.push(v.name.as_str().into());
                            EnumVariants::Contiguous(vec)
                        }

                        EnumVariants::Contiguous(vec) => {
                            let new_vec = vec
                                .into_iter()
                                .enumerate()
                                .map(|(index, name)| NonContiguousEnumVariant {
                                    name,
                                    index: index as i32,
                                })
                                .chain(once(NonContiguousEnumVariant {
                                    name: v.name.as_str().into(),
                                    index: v.discriminant as i32,
                                }))
                                .collect();

                            EnumVariants::NonContiguous(new_vec)
                        }
                        EnumVariants::NonContiguous(mut vec) => {
                            vec.push(NonContiguousEnumVariant {
                                index: v.discriminant as i32,
                                name: v.name.as_str().into(),
                            });
                            EnumVariants::NonContiguous(vec)
                        }
                    },
                )
            }
        }

        #[derive(Template)]
        #[template(path = "kotlin/Enum.kt.jinja", escape = "none")]
        struct EnumDef<'d> {
            lib_name: Cow<'d, str>,
            domain: Cow<'d, str>,
            type_name: Cow<'d, str>,
            variants: &'d EnumVariants<'d>,
            self_methods: &'d [String],
            companion_methods: &'d [String],
            native_methods: &'d [NativeMethodInfo],
        }

        let variants = EnumVariants::new(ty);

        let enum_def = EnumDef {
            lib_name: lib_name.into(),
            domain: domain.into(),
            type_name: type_name.into(),
            variants: &variants,
            self_methods: self_methods.as_ref(),
            companion_methods: companion_methods.as_ref(),
            native_methods: native_methods.as_ref(),
        }
        .render()
        .unwrap_or_else(|err| panic!("Failed to render Enum {{type_name}}\n\tcause: {err}"));

        (
            format!("{}/{lib_name}/{type_name}.kt", domain.replace('.', "/"),),
            enum_def,
        )
    }

    fn gen_native_type_name<P: TyPosition>(&self, ty: &Type<P>) -> Cow<'cx, str> {
        match *ty {
            Type::Primitive(prim) => self.formatter.fmt_primitive_as_ffi(prim).into(),
            Type::Opaque(ref op) => {
                let optional = if op.is_optional() { "?" } else { "" };
                format!("Pointer{optional}").into()
            }
            Type::Struct(ref strct) => {
                let op_id = strct.id();
                format!("{}Native", self.formatter.fmt_type_name(op_id)).into()
            }
            Type::Enum(_) => "Int".into(),
            Type::Slice(_) => "Slice".into(),

            _ => unreachable!("unknown AST/HIR variant"),
        }
    }

    fn gen_type_name<P: TyPosition>(&self, ty: &Type<P>) -> Cow<'cx, str> {
        match *ty {
            Type::Primitive(prim) => self.formatter.fmt_primitive_as_ffi(prim).into(),
            Type::Opaque(ref op) => {
                let op_id = op.tcx_id.into();
                let type_name = self.formatter.fmt_type_name(op_id);

                if self.tcx.resolve_type(op_id).attrs().disable {
                    self.errors
                        .push_error(format!("Found usage of disabled type {type_name}"))
                }
                let ret = if op.is_optional() {
                    self.formatter.fmt_nullable(&type_name).into()
                } else {
                    type_name
                };

                ret.into_owned().into()
            }
            Type::Struct(ref strct) => {
                let op_id = strct.id();
                self.formatter.fmt_type_name(op_id)
            }
            Type::Enum(ref enum_def) => self.formatter.fmt_type_name(enum_def.tcx_id.into()),
            Type::Slice(hir::Slice::Str(_, _)) => self.formatter.fmt_string().into(),
            Type::Slice(hir::Slice::Primitive(_, ty)) => {
                self.formatter.fmt_primitive_slice(ty).into()
            }

            Type::Slice(hir::Slice::Strs(_)) => self.formatter.fmt_str_slices().into(),
            _ => unreachable!("unknown AST/HIR variant"),
        }
    }
}

type MethodLtMap<'a> = BTreeMap<Lifetime, BorrowedLifetimeInfo<'a>>;

#[derive(Template)]
#[template(path = "kotlin/Method.kt.jinja", escape = "none")]
struct MethodTpl<'a> {
    // todo: comment: String,
    declaration: String,
    /// The C method name
    native_method_name: Cow<'a, str>,

    /// Conversion code for each parameter
    param_conversions: Vec<Cow<'a, str>>,
    return_expression: Option<Cow<'a, str>>,
    write_return: bool,
    slice_conversions: Vec<Cow<'a, str>>,
}

struct NativeMethodInfo {
    declaration: String,
}

#[cfg(test)]
mod test {

    use diplomat_core::hir::TypeDef;
    use quote::quote;

    use crate::common::ErrorStore;

    use super::formatter::test::new_tcx;
    use super::{formatter::KotlinFormatter, TyGenContext};

    #[test]
    fn test_enum() {
        let tk_stream = quote! {
            #[diplomat::bridge]
            mod ffi {

                pub enum Cont {
                    A,
                    B,
                    C,
                    D,
                }

                pub enum ContNumbered {
                    Alpha=0,
                    Beta=1,
                    Gamma=2,
                }

                pub enum NonCont {
                    Aleph=0,
                    Bet=1,
                    Tav=22,
                }

                pub enum Neg {
                    Neg3=-3,
                    Neg1=-1,
                    Thirteen=13,
                }

            }
        };

        let tcx = new_tcx(tk_stream);
        let mut all_types = tcx.all_types();
        if let (type_id, TypeDef::Enum(enum_def)) = all_types
            .next()
            .expect("Failed to generate first opaque def")
        {
            let error_store = ErrorStore::default();
            let formatter = KotlinFormatter::new(&tcx, None);
            let mut ty_gen_cx = TyGenContext {
                tcx: &tcx,
                formatter: &formatter,
                errors: &error_store,
            };
            let type_name = enum_def.name.to_string();
            // test that we can render and that it doesn't panic
            let (_, enum_code) =
                ty_gen_cx.gen_enum_def(enum_def, type_id, &type_name, "dev.gigapixel", "somelib");
            insta::assert_snapshot!(enum_code)
        }
    }

    #[test]
    fn test_struct() {
        let tk_stream = quote! {
            #[diplomat::bridge]
            mod ffi {


                #[diplomat::opaque]
                pub struct Opaque {
                    string: String
                }

                pub struct OtherNariveStruct {
                    i: i32,
                }

                pub struct MyNativeStruct<'b> {
                    a: bool,
                    b: i8,
                    c: u8,
                    d: i16,
                    e: u16,
                    f: i32,
                    g: u32,
                    h: i64,
                    i: u64,
                    j: DiplomatChar,
                    k: f32,
                    l: f64,
                    m: &'b [f64],
                    n: &'b Opaque,
                }

                impl<'b> MyNativeStruct<'b> {
                    pub fn new() -> MyNativeStruct<'b> {
                        todo!()
                    }
                }
            }
        };

        let tcx = new_tcx(tk_stream);
        let mut all_types = tcx.all_types();
        if let (type_id, TypeDef::Struct(strct)) = all_types
            .next()
            .expect("Failed to generate first opaque def")
        {
            let error_store = ErrorStore::default();
            let formatter = KotlinFormatter::new(&tcx, None);
            let mut ty_gen_cx = TyGenContext {
                tcx: &tcx,
                formatter: &formatter,
                errors: &error_store,
            };
            let type_name = strct.name.to_string();
            // test that we can render and that it doesn't panic
            let (_, struct_code) =
                ty_gen_cx.gen_struct_def(strct, type_id, &type_name, "dev.gigapixel", "somelib");
            insta::assert_snapshot!(struct_code)
        }
    }

    #[test]
    fn test_opaque_gen() {
        let tk_stream = quote! {
            #[diplomat::bridge]
            mod ffi {
                #[diplomat::opaque]
                struct MyOpaqueStruct<'b> {
                    a: SomeExternalType
                }

                #[diplomat::opaque]
                struct InputStruct {
                }

                #[diplomat::opaque]
                struct BorrowWrapper<'a, 'b> {
                    my_opaque: &'b MyOpaqueStruct<'a>

                }

                impl<'b> MyOpaqueStruct<'b> {

                    pub fn get_byte() -> u8 {
                        unimplemented!()
                    }

                    pub fn get_string_wrapper(in1: i32) -> i32 {
                        unimplemented!()
                    }

                    pub fn copy(&self, borrow: &MyOpaqueStruct<'b>) -> i32 {
                        unimplemented!()
                    }

                    pub fn borrow_other<'a>(inp_1: &'a InputStruct, inp_2: &'a InputStruct, borrow: &'a MyOpaqueStruct<'b>) -> &'a MyOpaqueStruct<'b> {
                        unimplemented!()
                    }

                    pub fn create(in1: i32) -> Box<MyOpaqueStruct<'b>> {
                        unimplemented!()
                    }


                    pub fn do_stuff(&self, in1: i32) -> f64 {
                        unimplemented!()
                    }

                    pub fn borrow<'a>(&'a self ) -> Box<BorrowWrapper<'b, 'a>> {
                        Box::new(BorrowWrapper {
                            my_opaque: self.as_ref()
                        })
                    }

                    pub fn borrow2<'a>(&'a self ) -> &'a MyOpaqueStruct<'b> {
                        self
                    }


                    pub fn borrow3<'a>(&'a self, other: &'a mut DiplomatWrite) {
                        todo!()
                    }

                    pub fn borrow<'a>(other: &'a MyOpaqueStruct<'b>) -> Box<BorrowWrapper<'b, 'a>> {
                        Box::new(BorrowWrapper {
                            my_opaque: other.as_ref()
                        })
                    }


                    pub fn string_stuff<'a, 'c>(&'a self,  some_str: &'c DiplomatStr)  -> &'c MyOpaqueStruct<'b> {
                        self.0.as_ref()
                    }


                    pub fn string_stuff_2<'a, 'c>(&'a self,  some_str: &'c DiplomatStr)  -> &'a MyOpaqueStruct<'b> {
                        self.0.as_ref()
                    }

                }

            }
        };
        let tcx = new_tcx(tk_stream);
        let mut all_types = tcx.all_types();
        if let (type_id, TypeDef::Opaque(opaque_def)) = all_types
            .next()
            .expect("Failed to generate first opaque def")
        {
            let eror_store = ErrorStore::default();
            let formatter = KotlinFormatter::new(&tcx, None);
            let mut ty_gen_cx = TyGenContext {
                tcx: &tcx,
                formatter: &formatter,
                errors: &eror_store,
            };
            let type_name = opaque_def.name.to_string();
            // test that we can render and that it doesn't panic
            let (_, result) = ty_gen_cx.gen_opaque_def(
                opaque_def,
                type_id,
                &type_name,
                "dev.gigapixel",
                "somelib",
            );
            insta::assert_snapshot!(result)
        }
    }
}
