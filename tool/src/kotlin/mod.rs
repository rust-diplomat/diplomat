use askama::Template;
use diplomat_core::hir::borrowing_param::{BorrowedLifetimeInfo, ParamBorrowInfo};
use diplomat_core::hir::{
    self, Borrow, Lifetime, LifetimeEnv, Lifetimes, MaybeOwn, MaybeStatic, Method, Mutability,
    OpaquePath, SelfType, StructPathLike, TyPosition, Type, TypeContext, TypeDef, TypeId,
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
    #[template(path = "kotlin/DiplomatStr.java.jinja", escape = "none")]
    struct JavaTypes<'a> {
        domain: &'a str,
        lib_name: &'a str,
    }

    let java_types = JavaTypes {
        domain: &domain,
        lib_name: &lib_name,
    }
    .render()
    .expect("Failed to render java types");

    files.add_file(
        format!(
            "src/main/java/{}/{lib_name}/DiplomatStr.java",
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
            Type::Slice(_) => todo!("don't support slices yet"),
            _ => todo!(),
        }
    }

    fn gen_return_type_name_ffi(&self, out: &ReturnType) -> Cow<'cx, str> {
        match *out {
            ReturnType::Infallible(SuccessType::Unit) => self.formatter.fmt_void().into(),
            ReturnType::Infallible(SuccessType::Writeable) => self.formatter.fmt_void().into(),
            ReturnType::Infallible(SuccessType::OutType(ref o)) => {
                if let hir::OutType::Slice(_) = o {
                    todo!("slices not supported yet")
                } else {
                    self.gen_type_name_ffi(o)
                }
            }
            ReturnType::Fallible(_, _) => {
                todo!("Fallible return types not supported yet")
                // self.gen_result(ok.as_type(), err.as_ref()).into()
            }
            ReturnType::Nullable(_) => {
                todo!("nullable return types not supported")
                // self.gen_result(ok.as_type(), None).into()
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
            Type::Slice(_) => todo!("slices not supported yet"),
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
    ) -> String {
        #[derive(Template)]
        #[template(path = "kotlin/opaqueReturn.kt.jinja", escape = "none")]
        struct OpaqueReturn<'b> {
            return_type_name: Cow<'b, str>,
            borrows: Vec<ParamsForLt<'b>>,
            is_owned: bool,
            self_edges: Vec<Cow<'b, str>>,
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
                        .map(|edge| self.formatter.fmt_param_name(edge.param_name.as_ref()))
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
                    .map(|edge| self.formatter.fmt_param_name(edge.param_name.as_ref()));
                let params = other_params.collect();
                Some(ParamsForLt { lt, params })
            })
            .collect();

        let opaque_return = OpaqueReturn {
            return_type_name,
            borrows,
            is_owned,
            self_edges,
        };
        opaque_return
            .render()
            .expect("Failed to render opaque return block")
    }

    fn gen_return<'d>(
        &'d self,
        method: &'d Method,
        method_lifetimes_map: MethodLtMap<'d>,
    ) -> Option<String> {
        match &method.output {
            ReturnType::Infallible(res) => match res {
                SuccessType::Writeable => todo!("writeable not yet supported"),
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
                        ))
                    }
                    Type::Struct(_) => todo!("structs not yet supported"),
                    Type::Enum(_) => todo!("enums not yet supported"),
                    Type::Slice(_) => todo!("slices not yet supported"),
                    _ => todo!(),
                },
                SuccessType::Unit => None,
                _ => todo!(),
            },
            ReturnType::Fallible(_, _) => todo!("fallible returns not yet supported"),
            ReturnType::Nullable(_) => todo!("nullable returns not yet supported"),
        }
    }

    fn gen_self_method_info(
        &mut self,
        id: TypeId,
        method: &'cx hir::Method,
        self_type: &'cx SelfType,
    ) -> SelfMethodInfo<'cx> {
        let mut visitor = method.borrowing_param_visitor(self.tcx);
        let native_method_name = self.formatter.fmt_c_method_name(id, method);

        let mut param_decls_kt = Vec::with_capacity(method.params.len());
        let mut param_types_ffi = Vec::with_capacity(method.params.len());
        let mut param_names_ffi = Vec::with_capacity(method.params.len());
        let mut param_conversions = Vec::with_capacity(method.params.len());

        match self_type {
            SelfType::Opaque(o) => match o.owner.mutability {
                hir::Mutability::Mutable => todo!("don't support mutable borrows yet"),
                hir::Mutability::Immutable => {
                    let param_type = "Long".into();
                    let param_name: Cow<'_, str> = "handle".into();
                    visitor.visit_param(&self_type.clone().into(), "this");

                    param_types_ffi.push(param_type);
                    param_conversions.push(param_name.clone());
                    param_names_ffi.push(param_name);
                }
            },
            SelfType::Struct(_) => todo!("structs not supported yet"),
            SelfType::Enum(_) => todo!("enums not supported yed"),
            _ => todo!(),
        };

        for param in method.params.iter() {
            let param_name = self.formatter.fmt_param_name(param.name.as_str());

            let param_borrow_kind = visitor.visit_param(&param.ty, &param_name);

            let param_type_ffi = self.gen_type_name_ffi(&param.ty);

            // todo: rethink this
            if let hir::Type::Slice(..) = param.ty {
                todo!("Slices not supported yet");
            }
            /*
            if let hir::Type::Struct(..) = param.ty {
                needs_temp_arena = true;
            }*/
            if let ParamBorrowInfo::Struct(_) = param_borrow_kind {
                todo!("support struct borrows")
            };
            param_decls_kt.push(format!("{param_name}: {}", self.gen_type_name(&param.ty)));
            param_types_ffi.push(param_type_ffi);
            param_conversions.push(self.gen_kt_to_c_for_type(&param.ty, param_name.clone()));
            param_names_ffi.push(param_name);
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
            .gen_return(method, method_lifetimes_map)
            .map(From::from);

        SelfMethodInfo {
            declaration,
            native_method_name,
            param_conversions,
            return_expression,
            lifetime_env: &method.lifetime_env,
        }
    }

    fn gen_native_method_info(&mut self, id: TypeId, method: &'cx hir::Method) -> NativeMethodInfo {
        let mut param_decls = Vec::with_capacity(method.params.len());

        let mut visitor = method.borrowing_param_visitor(self.tcx);

        if let Some(param_self) = method.param_self.as_ref() {
            match &param_self.ty {
                SelfType::Opaque(o) => match o.owner.mutability {
                    hir::Mutability::Mutable => todo!("don't support mutable borrows yet"),
                    hir::Mutability::Immutable => param_decls.push("handle: Long".into()),
                },
                SelfType::Struct(_) => todo!("structs not supported yet"),
                SelfType::Enum(_) => todo!("enums not supported yed"),
                _ => todo!(),
            }
        };
        for param in method.params.iter() {
            let param_name = self.formatter.fmt_param_name(param.name.as_str());

            let param_borrow_kind = visitor.visit_param(&param.ty, &param_name);

            if let hir::Type::Slice(..) = param.ty {
                todo!("Slices not supported yet");
            }
            if let ParamBorrowInfo::Struct(_) = param_borrow_kind {
                todo!("support struct borrows")
            };
            param_decls.push(format!(
                "{param_name}: {}",
                self.gen_native_type_name(&param.ty)
            ));
        }
        let params = param_decls.join(", ");
        let native_method = self.formatter.fmt_c_method_name(id, method);
        let return_ty = self.gen_return_type_name_ffi(&method.output);

        NativeMethodInfo {
            declaration: format!("fun {native_method}({params}): {return_ty}"),
        }
    }

    fn gen_companion_method_info(
        &mut self,
        id: TypeId,
        method: &'cx hir::Method,
    ) -> Option<CompanionMethodInfo<'cx>> {
        assert!(method.param_self.is_none());
        if method.attrs.disable {
            return None;
        }
        let mut visitor = method.borrowing_param_visitor(self.tcx);
        let native_method_name = self.formatter.fmt_c_method_name(id, method);

        let mut param_decls_kt = Vec::with_capacity(method.params.len());
        let mut param_types_ffi = Vec::with_capacity(method.params.len());
        let mut param_names_ffi = Vec::with_capacity(method.params.len());
        let mut param_conversions = Vec::with_capacity(method.params.len());

        for param in method.params.iter() {
            let param_name = self.formatter.fmt_param_name(param.name.as_str());

            let param_borrow_kind = visitor.visit_param(&param.ty, &param_name);

            // assert!(
            //     matches!(param_borrow_kind, ParamBorrowInfo::NotBorrowed),
            //     "Only support non-borrown non-self params... for now. Received {param_borrow_kind:?}"
            // );

            let param_type_ffi = self.gen_type_name_ffi(&param.ty);

            // todo: rethink this
            if let hir::Type::Slice(..) = param.ty {
                todo!("Slices not supported yet");
            }
            /*
            if let hir::Type::Struct(..) = param.ty {
                needs_temp_arena = true;
            }*/
            if let ParamBorrowInfo::Struct(_) = param_borrow_kind {
                todo!("support struct borrows")
            };
            param_decls_kt.push(format!("{param_name}: {}", self.gen_type_name(&param.ty)));
            param_types_ffi.push(param_type_ffi);
            param_conversions.push(self.gen_kt_to_c_for_type(&param.ty, param_name.clone()));
            param_names_ffi.push(param_name);
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
            .gen_return(method, method_lifetimes_map)
            .map(From::from);

        Some(CompanionMethodInfo {
            declaration,
            native_method_name,
            param_conversions,
            return_expression,
            lifetime_env: &method.lifetime_env,
        })
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
            .map(|(self_param, method)| self.gen_self_method_info(id, method, self_param))
            .collect::<Vec<_>>();

        let companion_methods = ty
            .methods
            .iter()
            .filter(|method| method.param_self.is_none())
            .flat_map(|method| self.gen_companion_method_info(id, method))
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
            self_methods: &'a [SelfMethodInfo<'a>],
            companion_methods: &'a [CompanionMethodInfo<'a>],
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
            Type::Slice(hir::Slice::Str(..)) => self.formatter.fmt_string().into(),
            Type::Slice(hir::Slice::Primitive(_, _)) => {
                panic!("don't support primitive slices yet")
            }
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
            Type::Slice(hir::Slice::Primitive(_, _)) => {
                panic!("don't support primitive slices yet")
            }
            _ => unreachable!("unknown AST/HIR variant"),
        }
    }
}

type MethodLtMap<'a> = BTreeMap<Lifetime, BorrowedLifetimeInfo<'a>>;

struct SelfMethodInfo<'a> {
    declaration: String,
    /// The C method name
    native_method_name: Cow<'a, str>,

    /// Conversion code for each parameter
    param_conversions: Vec<Cow<'a, str>>,
    // todo: slice params and lifetimes
    return_expression: Option<Cow<'a, str>>,
    lifetime_env: &'a LifetimeEnv,
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


                    pub fn borrow<'a>(other: &'a MyOpaqueStruct<'b>) -> Box<BorrowWrapper<'b, 'a>> {
                        Box::new(BorrowWrapper {
                            my_opaque: other.as_ref()
                        })
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
