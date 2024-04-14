use askama::Template;
use diplomat_core::ast::SelfParam;
use diplomat_core::hir::borrowing_param::{BorrowedLifetimeInfo, ParamBorrowInfo};
use diplomat_core::hir::{
    self, Borrow, Lifetime, LifetimeEnv, Lifetimes, MaybeOwn, MaybeStatic, Method, Mutability,
    OpaquePath, SelfType, Slice, StringEncoding, StructPathLike, TyPosition, Type, TypeContext,
    TypeDef, TypeId,
};
use diplomat_core::hir::{OpaqueDef, ReturnType, SuccessType};

use std::borrow::Cow;
use std::collections::BTreeMap;
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
    }

    #[derive(Template)]
    #[template(path = "kotlin/Slice.java.jinja", escape = "none")]
    struct SliceTpl<'a> {
        domain: &'a str,
        lib_name: &'a str,
    }

    let java_types = SliceTpl {
        domain: &domain,
        lib_name: &lib_name,
    }
    .render()
    .expect("Failed to render java types");

    files.add_file(
        format!(
            "src/main/java/{}/{lib_name}/Slice.java",
            domain.replace('.', "/")
        ),
        java_types,
    );

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
            ReturnType::Infallible(SuccessType::Writeable)
            | ReturnType::Fallible(SuccessType::Writeable, Some(_)) => {
                self.formatter.fmt_string().into()
            }
            ReturnType::Infallible(SuccessType::OutType(ref o))
            | ReturnType::Fallible(SuccessType::OutType(ref o), Some(_)) => self.gen_type_name(o),
            ReturnType::Fallible(SuccessType::Writeable, None)
            | ReturnType::Nullable(SuccessType::Writeable) => self
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
            Type::Opaque(OpaquePath {
                owner,
                ..
            }) => {
                match owner.mutability {
                    Mutability::Immutable => format!("{name}.handle").into(),
                    Mutability::Mutable => panic!("Not comfortable with mutable access in the JVM just yet. We'll add some mutexes to the code gen"),
                }
            }
            Type::Struct(_) => todo!("don't support structs yet"),
            Type::Enum(_) => todo!("don't support enums yet"),
            Type::Slice(_) => format!("{name}Slice").into(),
            _ => todo!(),
        }
    }

    fn gen_return_type_name_ffi(&self, out: &ReturnType) -> Cow<'cx, str> {
        match *out {
            ReturnType::Infallible(SuccessType::Unit) => self.formatter.fmt_void().into(),
            ReturnType::Infallible(SuccessType::Writeable) => self.formatter.fmt_void().into(),
            ReturnType::Infallible(SuccessType::OutType(ref o)) => self.gen_type_name_ffi(o),
            ReturnType::Fallible(_, _) => {
                todo!("Fallible return types not supported yet")
            }
            ReturnType::Nullable(_) => {
                todo!("nullable return types not supported")
            }
            _ => unreachable!("unknown AST/HIR variant"),
        }
    }

    fn gen_type_name_ffi<P: TyPosition>(&self, ty: &Type<P>) -> Cow<'cx, str> {
        match *ty {
            Type::Primitive(prim) => self.formatter.fmt_primitive_as_ffi(prim).into(),
            Type::Opaque(_) => "Long".into(),

            Type::Struct(_) => {
                todo!("Structs not supported yet")
            }
            Type::Enum(_) => {
                todo!("Structs not supported yet")
            }
            Type::Slice(_) => "Slice".into(),
            _ => unreachable!("unknown AST/HIR variant"),
        }
    }

    fn gen_opaque_return<'d>(
        &'d self,
        opaque_def: &'d OpaqueDef,
        ownership: MaybeOwn,
        lifetimes: &'d Lifetimes,
        method_lifetimes_map: MethodLtMap<'d>,
        lifetime_env: &'d LifetimeEnv,
        cleanups: &[Cow<'d, str>],
    ) -> String {
        #[derive(Template)]
        #[template(path = "kotlin/opaqueReturn.kt.jinja", escape = "none")]
        struct OpaqueReturn<'a, 'b> {
            return_type_name: Cow<'b, str>,
            borrows: Vec<ParamsForLt<'b>>,
            is_owned: bool,
            self_edges: Vec<Cow<'b, str>>,
            cleanups: &'a [Cow<'b, str>],
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
            }) => {
                let param = method_lifetimes_map.get(&lt)?;
                Some(
                    param
                        .incoming_edges
                        .iter()
                        .map(|edge| self.formatter.fmt_borrow(edge))
                        .collect(),
                )
            }
            _ => None,
        };

        let self_edges = self_edges();
        let is_owned = self_edges.is_none();
        let self_edges = self_edges.unwrap_or_else(Vec::new);

        let borrows = lifetimes
            .lifetimes()
            .filter_map(|lt| {
                let (lt_info, lt) = match lt {
                    MaybeStatic::Static => return None,
                    MaybeStatic::NonStatic(lt) => (
                        method_lifetimes_map.get(&lt)?,
                        lifetime_env.fmt_lifetime(lt),
                    ),
                };
                let other_params = lt_info
                    .incoming_edges
                    .iter()
                    .map(|edge| self.formatter.fmt_borrow(edge));
                let params = other_params.collect();
                Some(ParamsForLt { lt, params })
            })
            .collect();

        let opaque_return = OpaqueReturn {
            return_type_name,
            borrows,
            is_owned,
            self_edges,
            cleanups,
        };
        opaque_return
            .render()
            .expect("Failed to render opaque return block")
    }

    const WRITEABLE_RETURN: &'static str = r#"
val returnString = DW.writeableToString(writeable)
DW.lib.diplomat_buffer_writeable_destroy(writeable)
return returnString"#;

    fn gen_slice_retrn<'d>(&'d self, slice_ty: &'d Slice) -> String {
        match slice_ty {
            Slice::Str(_, enc) => match enc {
                StringEncoding::UnvalidatedUtf16 => {
                    "    return PrimitiveArrayTools.getUtf16(returnVal)".into()
                }
                StringEncoding::UnvalidatedUtf8 => {
                    "    return PrimitiveArrayTools.getUtf8(returnVal)".into()
                }
                StringEncoding::Utf8 => "    return PrimitiveArrayTools.getUtf8(returnVal)".into(),
                _ => todo!(),
            },
            Slice::Primitive(_, prim_ty) => {
                let prim_ty = self.formatter.fmt_primitive_as_ffi(*prim_ty);
                format!("    return PrimitiveArrayTools.get{prim_ty}Array(returnVal)")
            }

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
            ReturnType::Infallible(res) => match res {
                SuccessType::Writeable => Some(Self::WRITEABLE_RETURN.into()),
                SuccessType::OutType(o) => match o {
                    Type::Primitive(_) => Some("return returnVal".into()),
                    Type::Opaque(opaque_path) => {
                        let ownership = opaque_path.owner;
                        let lifetimes = &opaque_path.lifetimes;
                        Some(self.gen_opaque_return(
                            opaque_path.resolve(self.tcx),
                            ownership,
                            lifetimes,
                            method_lifetimes_map,
                            &method.lifetime_env,
                            cleanups,
                        ))
                    }
                    Type::Struct(_) => todo!("structs not yet supported"),
                    Type::Enum(_) => todo!("enums not yet supported"),
                    Type::Slice(slc) => Some(self.gen_slice_retrn(slc)),
                    _ => todo!(),
                },
                SuccessType::Unit => None,
                _ => todo!(),
            },
            ReturnType::Fallible(_, _) => todo!("fallible returns not yet supported"),
            ReturnType::Nullable(_) => todo!("nullable returns not yet supported"),
        }
    }
    fn gen_slice_conv(&self, kt_param_name: Cow<'cx, str>, slice_type: Slice) -> Cow<'cx, str> {
        #[derive(Template)]
        #[template(path = "kotlin/SliceConv.kt.jinja", escape = "none")]
        struct SliceConv<'d> {
            slice_method: Cow<'d, str>,
            kt_param_name: Cow<'d, str>,
        }
        let slice_method = match slice_type {
            Slice::Str(_, StringEncoding::UnvalidatedUtf16) => "readUtf16".into(),
            Slice::Str(_, _) => "readUtf8".into(),
            Slice::Primitive(_, _) => "native".into(),
            _ => panic!("Unsupported slice type"),
        };

        let slice_conv = SliceConv {
            kt_param_name,
            slice_method,
        };
        slice_conv
            .render()
            .expect("Failed to render slice method")
            .into()
    }

    fn gen_cleanup(&self, param_name: Cow<'cx, str>) -> Cow<'cx, str> {
        format!("{param_name}Mem.close()").into()
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
        let mut param_names_ffi = Vec::with_capacity(method.params.len());
        let mut param_conversions = Vec::with_capacity(method.params.len());
        let mut slice_conversions = Vec::with_capacity(method.params.len());
        let mut cleanups = Vec::with_capacity(method.params.len());

        match self_type {
            Some(st @ SelfType::Opaque(_)) => {
                // match o.owner.mutability {
                // hir::Mutability::Mutable => todo!("don't support mutable borrows yet"),
                // hir::Mutability::Immutable => {
                let param_type = "Long".into();
                let param_name: Cow<'_, str> = "handle".into();
                visitor.visit_param(&st.clone().into(), "this");

                param_types_ffi.push(param_type);
                param_conversions.push(param_name.clone());
                param_names_ffi.push(param_name);
                // }
            }
            Some(SelfType::Struct(_)) => todo!("structs not supported yet"),
            Some(SelfType::Enum(_)) => todo!("enums not supported yed"),
            None => (),
            _ => todo!(),
        };

        for param in method.params.iter() {
            let param_name = self.formatter.fmt_param_name(param.name.as_str());

            let param_type_ffi = self.gen_type_name_ffi(&param.ty);

            if let Type::Slice(slice) = param.ty {
                slice_conversions.push(self.gen_slice_conv(param_name.clone(), slice));

                let param_borrow_kind = visitor.visit_param(&param.ty, &param_name);

                match param_borrow_kind {
                    ParamBorrowInfo::Struct(_) => todo!("support struct borrows"),
                    ParamBorrowInfo::TemporarySlice => {
                        cleanups.push(self.gen_cleanup(param_name.clone()));
                    }
                    ParamBorrowInfo::BorrowedSlice => (),
                    ParamBorrowInfo::BorrowedOpaque => (),
                    ParamBorrowInfo::NotBorrowed => (),
                    _ => todo!(),
                };
            }

            param_decls_kt.push(format!("{param_name}: {}", self.gen_type_name(&param.ty)));
            param_types_ffi.push(param_type_ffi);
            param_conversions.push(self.gen_kt_to_c_for_type(&param.ty, param_name.clone()));
            param_names_ffi.push(param_name);
        }
        let writeable_return = matches!(
            &method.output,
            ReturnType::Infallible(SuccessType::Writeable)
        );
        if writeable_return {
            param_conversions.push("writeable".into());
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
            declaration,
            native_method_name,
            param_conversions,
            return_expression,
            lifetime_env: &method.lifetime_env,
            writeable_return,
            slice_conversions,
            cleanups,
        }
        .render()
        .expect("Failed to render string for method")
    }

    fn gen_native_method_info(&mut self, id: TypeId, method: &'cx hir::Method) -> NativeMethodInfo {
        let mut param_decls = Vec::with_capacity(method.params.len());

        let mut visitor = method.borrowing_param_visitor(self.tcx);

        if let Some(param_self) = method.param_self.as_ref() {
            match &param_self.ty {
                SelfType::Opaque(_) => param_decls.push("handle: Long".into()),
                SelfType::Struct(_) => todo!("structs not supported yet"),
                SelfType::Enum(_) => todo!("enums not supported yed"),
                _ => todo!(),
            }
        };
        for param in method.params.iter() {
            let param_name = self.formatter.fmt_param_name(param.name.as_str());

            let param_borrow_kind = visitor.visit_param(&param.ty, &param_name);

            // if let hir::Type::Slice(..) = param.ty {
            //     todo!("Slices not supported yet");
            // }
            if let ParamBorrowInfo::Struct(_) = param_borrow_kind {
                todo!("support struct borrows")
            };
            param_decls.push(format!(
                "{param_name}: {}",
                self.gen_native_type_name(&param.ty)
            ));
        }
        if let ReturnType::Infallible(SuccessType::Writeable) = method.output {
            param_decls.push("writeable: Pointer".into())
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

        #[derive(Template)]
        #[template(path = "kotlin/opaque.kt.jinja", escape = "none")]
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

    fn gen_native_type_name<P: TyPosition>(&self, ty: &Type<P>) -> Cow<'cx, str> {
        match *ty {
            Type::Primitive(prim) => self.formatter.fmt_primitive_as_ffi(prim).into(),
            Type::Opaque(_) => "Long".into(),
            Type::Struct(ref strct) => {
                let op_id = strct.id();
                let type_name = self.formatter.fmt_type_name(op_id);

                panic!("don't support structs yet: {type_name:?}")
            }
            Type::Enum(_) => panic!("don't support enums yet"),
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
                let type_name = self.formatter.fmt_type_name(op_id);

                panic!("don't support structs yet: {type_name:?}")
            }
            Type::Enum(_) => panic!("don't support enums yet"),
            Type::Slice(hir::Slice::Str(..)) => self.formatter.fmt_string().into(),
            Type::Slice(hir::Slice::Primitive(_, ty)) => {
                self.formatter.fmt_primitive_slice(ty).into()
            }
            _ => unreachable!("unknown AST/HIR variant"),
        }
    }
}

type MethodLtMap<'a> = BTreeMap<Lifetime, BorrowedLifetimeInfo<'a>>;

#[derive(Template)]
#[template(path = "kotlin/Method.kt.jinja", escape = "none")]
struct MethodTpl<'a> {
    declaration: String,
    /// The C method name
    native_method_name: Cow<'a, str>,

    /// Conversion code for each parameter
    param_conversions: Vec<Cow<'a, str>>,
    // todo: slice params and lifetimes
    return_expression: Option<Cow<'a, str>>,
    lifetime_env: &'a LifetimeEnv,
    writeable_return: bool,
    slice_conversions: Vec<Cow<'a, str>>,
    cleanups: Vec<Cow<'a, str>>,
}

struct SelfMethodInfo<'a> {
    declaration: String,
    /// The C method name
    native_method_name: Cow<'a, str>,

    /// Conversion code for each parameter
    param_conversions: Vec<Cow<'a, str>>,
    // todo: slice params and lifetimes
    return_expression: Option<Cow<'a, str>>,
    lifetime_env: &'a LifetimeEnv,
    writeable_return: bool,
    slice_conversions: Vec<Cow<'a, str>>,
    cleanups: Vec<Cow<'a, str>>,
}

struct NativeMethodInfo {
    declaration: String,
}

struct CompanionMethodInfo<'a> {
    declaration: String,
    /// The C method name
    native_method_name: Cow<'a, str>,

    /// Conversion code for each parameter
    param_conversions: Vec<Cow<'a, str>>,
    // todo: slice params and lifetimes, not this should only take self borrows
    return_expression: Option<Cow<'a, str>>,
    lifetime_env: &'a LifetimeEnv,
    writeable_return: bool,
}

#[cfg(test)]
mod test {

    use diplomat_core::hir::TypeDef;
    use quote::quote;

    use crate::common::ErrorStore;

    use super::formatter::test::new_tcx;
    use super::{formatter::KotlinFormatter, TyGenContext};

    #[test]
    fn test_test() {
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


                    pub fn borrow3<'a>(&'a self, other: &'a mut DiplomatWriteable) {
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
        while let Some((type_id, TypeDef::Opaque(opaque_def))) = all_types.next() {
            let eror_store = ErrorStore::default();
            let formatter = KotlinFormatter::new(&tcx, None);
            let mut ty_gen_cx = TyGenContext {
                tcx: &tcx,
                formatter: &formatter,
                errors: &eror_store,
            };
            let type_name = opaque_def.name.to_string();
            // test that we can render and that it doesn't panic
            let (_, boop) = ty_gen_cx.gen_opaque_def(
                opaque_def,
                type_id,
                &type_name,
                "dev.gigapixel",
                "somelib",
            );
            println!("{boop}")
        }
    }
}
