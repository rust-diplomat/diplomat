use super::formatter::CFormatter;
use super::header::Header;
use crate::ErrorStore;
use askama::Template;
use diplomat_core::hir::TypeContext;
use diplomat_core::hir::{
    self, CallbackInstantiationFunctionality, OpaqueOwner, ReturnableStructDef, StructPathLike,
    TyPosition, Type, TypeDef, TypeId,
};
use std::borrow::Cow;
use std::fmt::Write;

#[derive(Template)]
#[template(path = "c/enum.h.jinja", escape = "none")]
struct EnumTemplate<'a> {
    ty: &'a hir::EnumDef,
    fmt: &'a CFormatter<'a>,
    ty_name: &'a str,
    is_for_cpp: bool,
}

#[derive(Template)]
#[template(path = "c/struct.h.jinja", escape = "none")]
struct StructTemplate<'a> {
    ty_name: Cow<'a, str>,
    fields: Vec<(Cow<'a, str>, Cow<'a, str>)>,
    is_for_cpp: bool,
}

#[derive(Template)]
#[template(path = "c/opaque.h.jinja", escape = "none")]
struct OpaqueTemplate<'a> {
    ty_name: Cow<'a, str>,
    is_for_cpp: bool,
}

#[derive(Template)]
#[template(path = "c/impl.h.jinja", escape = "none")]
struct ImplTemplate<'a> {
    methods: Vec<MethodTemplate<'a>>,
    cb_structs_and_defs: Vec<CallbackAndStructDef>,
    is_for_cpp: bool,
    ty_name: Cow<'a, str>,
    dtor_name: Option<&'a str>,
}

struct MethodTemplate<'a> {
    return_ty: Cow<'a, str>,
    params: String,
    abi_name: &'a str,
}

#[derive(Clone)]
struct CallbackAndStructDef {
    name: String,
    params_types: String,
    return_type: String,
}

/// The context used for generating a particular type
///
/// Also used by C++ generation code
pub(crate) struct TyGenContext<'cx, 'tcx> {
    pub(crate) tcx: &'tcx TypeContext,
    pub(crate) formatter: &'cx CFormatter<'tcx>,
    pub(crate) errors: &'cx ErrorStore<'tcx, String>,
    pub(crate) is_for_cpp: bool,
    pub(crate) id: TypeId,
    pub(crate) decl_header_path: &'cx String,
    pub(crate) impl_header_path: &'cx String,
}

impl<'cx, 'tcx> TyGenContext<'cx, 'tcx> {
    pub(crate) fn gen_enum_def(&self, def: &'tcx hir::EnumDef) -> Header {
        let mut decl_header = Header::new(self.decl_header_path.clone(), self.is_for_cpp);
        let ty_name = self.formatter.fmt_type_name(self.id);
        EnumTemplate {
            ty: def,
            fmt: self.formatter,
            ty_name: &ty_name,
            is_for_cpp: self.is_for_cpp,
        }
        .render_into(&mut decl_header)
        .unwrap();

        decl_header
    }

    pub(crate) fn gen_opaque_def(&self, _def: &'tcx hir::OpaqueDef) -> Header {
        let mut decl_header = Header::new(self.decl_header_path.clone(), self.is_for_cpp);
        let ty_name = self.formatter.fmt_type_name(self.id);
        OpaqueTemplate {
            ty_name,
            is_for_cpp: self.is_for_cpp,
        }
        .render_into(&mut decl_header)
        .unwrap();

        decl_header
    }

    pub(crate) fn gen_struct_def<P: TyPosition>(&self, def: &'tcx hir::StructDef<P>) -> Header {
        let mut decl_header = Header::new(self.decl_header_path.clone(), self.is_for_cpp);
        let ty_name = self.formatter.fmt_type_name(self.id);
        let mut fields = vec![];
        let mut cb_structs_and_defs = vec![];
        for field in def.fields.iter() {
            fields.push(self.gen_ty_decl(
                &field.ty,
                field.name.as_str(),
                &mut decl_header,
                None,
                &mut cb_structs_and_defs, // for now this gets ignored, there are no callbacks in struct fields
            ));
        }

        StructTemplate {
            ty_name,
            fields,
            is_for_cpp: self.is_for_cpp,
        }
        .render_into(&mut decl_header)
        .unwrap();

        decl_header
    }

    pub(crate) fn gen_impl(&self, ty: hir::TypeDef<'tcx>) -> Header {
        let mut impl_header = Header::new(self.impl_header_path.clone(), self.is_for_cpp);
        let mut methods = vec![];
        let mut cb_structs_and_defs = vec![];
        for method in ty.methods() {
            if method.attrs.disable {
                // Skip method if disabled
                continue;
            }
            let _guard = self.errors.set_context_method(
                self.tcx.fmt_type_name_diagnostics(self.id),
                method.name.as_str().into(),
            );
            let (method_chunk, callback_defs) = self.gen_method(method, &mut impl_header);
            methods.push(method_chunk);
            cb_structs_and_defs.extend_from_slice(&callback_defs);
        }

        let ty_name = self.formatter.fmt_type_name(self.id);

        let dtor_name = if let TypeDef::Opaque(opaque) = ty {
            Some(opaque.dtor_abi_name.as_str())
        } else {
            None
        };
        ImplTemplate {
            ty_name,
            methods,
            cb_structs_and_defs,
            dtor_name,
            is_for_cpp: self.is_for_cpp,
        }
        .render_into(&mut impl_header)
        .unwrap();

        impl_header.decl_include = Some(self.decl_header_path.clone());

        // In some cases like generating decls for `self` parameters,
        // a header will get its own includes. Instead of
        // trying to avoid pushing them, it's cleaner to just pull them out
        // once done
        impl_header.includes.remove(self.impl_header_path);
        impl_header.includes.remove(self.decl_header_path);

        impl_header
    }

    fn gen_method(
        &self,
        method: &'tcx hir::Method,
        header: &mut Header,
    ) -> (MethodTemplate<'tcx>, Vec<CallbackAndStructDef>) {
        use diplomat_core::hir::{ReturnType, SuccessType};
        let abi_name = method.abi_name.as_str();
        // Right now these are the same, but we may eventually support renaming
        // and should be sure to use method_name when naming the result type
        let method_name = abi_name;
        let mut param_decls = Vec::new();
        let mut cb_structs_and_defs = vec![];
        if let Some(ref self_ty) = method.param_self {
            let self_ty = self_ty.ty.clone().into();
            param_decls.push(self.gen_ty_decl(
                &self_ty,
                "self",
                header,
                Some(abi_name.into()),
                &mut cb_structs_and_defs,
            ))
        }

        for param in &method.params {
            param_decls.push(self.gen_ty_decl(
                &param.ty,
                param.name.as_str(),
                header,
                Some(abi_name.into()),
                &mut cb_structs_and_defs,
            ));
        }

        let return_ty: Cow<str> = match method.output {
            ReturnType::Infallible(SuccessType::Unit) => "void".into(),
            ReturnType::Infallible(SuccessType::Write) => {
                param_decls.push((
                    format!("{}*", self.formatter.fmt_write_name()).into(),
                    "write".into(),
                ));
                "void".into()
            }
            ReturnType::Infallible(SuccessType::OutType(ref o)) => self.gen_ty_name(o, header),
            ReturnType::Fallible(ref ok, _) | ReturnType::Nullable(ref ok) => {
                // Result<T, ()> and Option<T> are the same on the ABI
                let err = if let ReturnType::Fallible(_, Some(ref e)) = method.output {
                    Some(e)
                } else {
                    None
                };
                let ok_ty = match ok {
                    SuccessType::Write => {
                        param_decls.push((
                            format!("{}*", self.formatter.fmt_write_name()).into(),
                            "write".into(),
                        ));
                        None
                    }
                    SuccessType::Unit => None,
                    SuccessType::OutType(o) => Some(o),
                    _ => unreachable!("unknown AST/HIR variant"),
                };
                self.gen_result_ty(method_name, ok_ty, err, header).into()
            }
            _ => unreachable!("unknown AST/HIR variant"),
        };

        let mut params = String::new();
        let mut first = true;
        for (decl_ty, decl_name) in param_decls {
            let comma = if first {
                first = false;
                ""
            } else {
                ", "
            };
            write!(&mut params, "{comma}{decl_ty} {decl_name}").unwrap();
        }

        if params.is_empty() {
            params.push_str("void");
        }

        (
            MethodTemplate {
                abi_name,
                return_ty,
                params,
            },
            cb_structs_and_defs,
        )
    }

    fn gen_result_ty(
        &self,
        fn_name: &str,
        ok_ty: Option<&hir::OutType>,
        err_ty: Option<&hir::OutType>,
        header: &mut Header,
    ) -> String {
        let ok_ty = ok_ty.filter(|t| {
            let Type::Struct(s) = t else {
                return true;
            };
            match s.resolve(self.tcx) {
                ReturnableStructDef::Struct(s) => !s.fields.is_empty(),
                ReturnableStructDef::OutStruct(s) => !s.fields.is_empty(),
                _ => unreachable!("unknown AST/HIR variant"),
            }
        });

        let err_ty = err_ty.filter(|t| {
            let Type::Struct(s) = t else {
                return true;
            };
            match s.resolve(self.tcx) {
                ReturnableStructDef::Struct(s) => !s.fields.is_empty(),
                ReturnableStructDef::OutStruct(s) => !s.fields.is_empty(),
                _ => unreachable!("unknown AST/HIR variant"),
            }
        });

        let ok_line = if let Some(ok) = ok_ty {
            let ok_name = self.gen_ty_name(ok, header);
            format!("{ok_name} ok;")
        } else {
            "".into()
        };
        let err_line = if let Some(err) = err_ty {
            let err_name = self.gen_ty_name(err, header);
            format!("{err_name} err;")
        } else {
            "".into()
        };

        let union_def = if ok_ty.is_some() || err_ty.is_some() {
            format!("union {{{ok_line} {err_line}}};")
        } else {
            "".into()
        };

        // We can't use an anonymous struct here: C++ doesn't like producing those in return types
        // Instead we name it something unique per-function. This is a bit ugly but works just fine.
        format!("typedef struct {fn_name}_result {{{union_def} bool is_ok;}} {fn_name}_result;\n{fn_name}_result")
    }

    /// Generates a decl for a given type, returned as (type, name)
    fn gen_ty_decl<'a, P: TyPosition>(
        &self,
        ty: &Type<P>,
        ident: &'a str,
        header: &mut Header,
        method_abi_name: Option<String>,
        cb_structs_and_defs: &mut Vec<CallbackAndStructDef>,
    ) -> (Cow<'tcx, str>, Cow<'a, str>) {
        let param_name = self.formatter.fmt_param_name(ident);
        match ty {
            Type::Callback(some_cb) => {
                let cb_wrapper_type = "DiplomatCallback_".to_owned()
                    + method_abi_name.unwrap().as_str()
                    + "_"
                    + ident;
                let params = some_cb.get_inputs().unwrap();
                let output_type = Box::new(some_cb.get_output_type().unwrap().clone());
                // this call generates any imports needed for param + output type(s)
                cb_structs_and_defs.push(self.gen_cb_param_wrapper_struct(
                    &cb_wrapper_type,
                    params,
                    &output_type,
                    header,
                ));
                (
                    cb_wrapper_type.clone().into(),
                    format!("{}_cb_wrap", param_name).into(),
                )
            }
            _ => {
                let ty = self.gen_ty_name(ty, header);
                (ty, param_name)
            }
        }
    }

    fn gen_cb_param_wrapper_struct(
        &self,
        cb_wrapper_type: &str,
        params: &[hir::CallbackParam],
        output_type: &Option<Type>,
        header: &mut Header,
    ) -> CallbackAndStructDef {
        let return_type = if output_type.is_some() {
            self.gen_ty_name(&(*output_type).clone().unwrap(), header)
                .into()
        } else {
            "void".into()
        };
        let params_types = params
            .iter()
            .map(|p| self.gen_ty_name(&p.ty, header).to_string())
            .collect::<Vec<String>>()
            .join(", ");

        CallbackAndStructDef {
            name: cb_wrapper_type.into(),
            params_types,
            return_type,
        }
    }

    // Generate the C code for referencing a particular type in the input position (i.e., not return types).
    // Handles adding imports and such as necessary
    pub fn gen_ty_name<P: TyPosition>(&self, ty: &Type<P>, header: &mut Header) -> Cow<'tcx, str> {
        let ty_name = match *ty {
            Type::Primitive(prim) => self.formatter.fmt_primitive_as_c(prim),
            Type::Opaque(ref op) => {
                let op_id = op.tcx_id.into();
                let ty_name = self.formatter.fmt_type_name_maybe_namespaced(op_id);
                if self.tcx.resolve_type(op_id).attrs().disable {
                    self.errors
                        .push_error(format!("Found usage of disabled type {ty_name}"))
                }
                // unwrap_or(mut) since owned pointers need to not be const
                let mutability = op.owner.mutability().unwrap_or(hir::Mutability::Mutable);
                let ret = self.formatter.fmt_ptr(&ty_name, mutability);
                header
                    .includes
                    .insert(self.formatter.fmt_decl_header_path(op_id));
                ret.into_owned().into()
            }
            Type::Struct(ref st) => {
                let st_id = st.id();
                let ty_name = self.formatter.fmt_type_name_maybe_namespaced(st_id);
                if self.tcx.resolve_type(st_id).attrs().disable {
                    self.errors
                        .push_error(format!("Found usage of disabled type {ty_name}"))
                }
                let ret = ty_name.clone();
                let header_path = self.formatter.fmt_decl_header_path(st_id);
                header.includes.insert(header_path);
                ret
            }
            Type::Enum(ref e) => {
                let id = e.tcx_id.into();
                let ty_name = self.formatter.fmt_type_name_maybe_namespaced(id);
                if self.tcx.resolve_type(id).attrs().disable {
                    self.errors
                        .push_error(format!("Found usage of disabled type {ty_name}"))
                }
                let header_path = self.formatter.fmt_decl_header_path(id);
                header.includes.insert(header_path);
                ty_name
            }
            Type::Slice(ref s) => match s {
                hir::Slice::Primitive(borrow, prim) => {
                    self.formatter.fmt_primitive_slice_name(*borrow, *prim)
                }
                hir::Slice::Str(_, encoding) => self.formatter.fmt_str_view_name(*encoding),
                hir::Slice::Strs(encoding) => self.formatter.fmt_strs_view_name(*encoding),
                &_ => unreachable!("unknown AST/HIR variant"),
            },
            Type::DiplomatOption(ref s) => {
                let inner = self.gen_ty_name(s, header);
                self.formatter.fmt_optional_type_name(s, &inner).into()
            }
            _ => unreachable!("{}", format!("unknown AST/HIR variant: {:?}", ty)),
        };

        ty_name
    }
}
