use super::formatter::CFormatter;
use super::header::Header;
use crate::common::ErrorStore;
use askama::Template;
use diplomat_core::hir::TypeContext;
use diplomat_core::hir::{
    self, OpaqueOwner, ReturnableStructDef, StructPathLike, TyPosition, Type, TypeDef, TypeId,
};
use std::borrow::Cow;
use std::fmt::Write;

impl<'tcx> super::CContext<'tcx> {
    pub fn gen_ty(&self, id: TypeId, ty: TypeDef<'tcx>) {
        if ty.attrs().disable {
            // Skip type if disabled
            return;
        }
        if let TypeDef::Struct(s) = ty {
            if s.fields.is_empty() {
                // Skip ZST
                return;
            }
        }
        if let TypeDef::OutStruct(s) = ty {
            if s.fields.is_empty() {
                // Skip ZST
                return;
            }
        }

        let decl_header_path = self.formatter.fmt_decl_header_path(id);
        let impl_header_path = self.formatter.fmt_impl_header_path(id);

        let _guard = self.errors.set_context_ty(ty.name().as_str().into());
        let tygen = self.ty_gen_context(id, decl_header_path, impl_header_path);

        let decl_header = match ty {
            TypeDef::Enum(e) => tygen.gen_enum_def(e),
            TypeDef::Opaque(o) => tygen.gen_opaque_def(o),
            TypeDef::Struct(s) => tygen.gen_struct_def(s),
            TypeDef::OutStruct(s) => tygen.gen_struct_def(s),
            _ => unreachable!("unknown AST/HIR variant"),
        };

        let impl_header = tygen.gen_impl(ty);

        self.files
            .add_file(tygen.decl_header_path, decl_header.to_string());
        self.files
            .add_file(tygen.impl_header_path, impl_header.to_string());
    }

    pub(crate) fn ty_gen_context<'cx>(
        &'cx self,
        id: TypeId,
        decl_header_path: String,
        impl_header_path: String,
    ) -> TyGenContext<'cx, 'tcx> {
        TyGenContext {
            tcx: self.tcx,
            formatter: &self.formatter,
            errors: &self.errors,
            is_for_cpp: self.is_for_cpp,
            id,
            decl_header_path,
            impl_header_path,
        }
    }
}

#[derive(Template)]
#[template(path = "c2/enum.h.jinja", escape = "none")]
struct EnumTemplate<'a> {
    ty: &'a hir::EnumDef,
    fmt: &'a CFormatter<'a>,
    ty_name: &'a str,
    is_for_cpp: bool,
}

#[derive(Template)]
#[template(path = "c2/struct.h.jinja", escape = "none")]
struct StructTemplate<'a> {
    ty_name: Cow<'a, str>,
    fields: Vec<(Cow<'a, str>, Cow<'a, str>)>,
    is_for_cpp: bool,
}

#[derive(Template)]
#[template(path = "c2/opaque.h.jinja", escape = "none")]
struct OpaqueTemplate<'a> {
    ty_name: Cow<'a, str>,
    is_for_cpp: bool,
}

#[derive(Template)]
#[template(path = "c2/impl.h.jinja", escape = "none")]
struct ImplTemplate<'a> {
    methods: Vec<MethodTemplate<'a>>,
    is_for_cpp: bool,
    ty_name: Cow<'a, str>,
    dtor_name: Option<String>,
}

struct MethodTemplate<'a> {
    return_ty: Cow<'a, str>,
    params: String,
    params_inv: String,
    name: Cow<'a, str>,
    abi_name: Cow<'a, str>,
    maybe_result_typedef: String,
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
    pub(crate) decl_header_path: String,
    pub(crate) impl_header_path: String,
}

impl<'cx, 'tcx> TyGenContext<'cx, 'tcx> {
    pub fn gen_enum_def(&self, def: &'tcx hir::EnumDef) -> Header {
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

    pub fn gen_opaque_def(&self, _def: &'tcx hir::OpaqueDef) -> Header {
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

    pub fn gen_struct_def<P: TyPosition>(&self, def: &'tcx hir::StructDef<P>) -> Header {
        let mut decl_header = Header::new(self.decl_header_path.clone(), self.is_for_cpp);
        let ty_name = self.formatter.fmt_type_name(self.id);
        let mut fields = vec![];
        for field in def.fields.iter() {
            self.gen_ty_decl(
                &field.ty,
                field.name.as_str(),
                true,
                &mut decl_header,
                &mut fields,
            );
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

    pub fn gen_impl(&self, ty: hir::TypeDef<'tcx>) -> Header {
        let mut impl_header = Header::new(self.impl_header_path.clone(), self.is_for_cpp);
        let mut methods = vec![];
        for method in ty.methods() {
            if method.attrs.disable {
                // Skip method if disabled
                continue;
            }
            let _guard = self.errors.set_context_method(
                self.formatter.fmt_type_name_diagnostics(self.id),
                method.name.as_str().into(),
            );
            methods.push(self.gen_method(method, &mut impl_header));
        }

        let ty_name = self.formatter.fmt_type_name(self.id);

        let dtor_name = if let TypeDef::Opaque(_) = ty {
            Some(self.formatter.fmt_dtor_name(self.id))
        } else {
            None
        };
        ImplTemplate {
            ty_name,
            methods,
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
        impl_header.includes.remove(&self.impl_header_path);
        impl_header.includes.remove(&self.decl_header_path);

        impl_header
    }

    fn gen_method(&self, method: &'tcx hir::Method, header: &mut Header) -> MethodTemplate<'tcx> {
        use diplomat_core::hir::{ReturnType, SuccessType};
        let abi_method_name = self.formatter.fmt_method_name_for_abi(self.id, method);
        let method_name = if !self.is_for_cpp {
            self.formatter.fmt_method_name(self.id, method)
        } else {
            abi_method_name.clone()
        };
        let mut maybe_result_typedef = String::new();
        let mut param_decls = Vec::new();
        if let Some(ref self_ty) = method.param_self {
            let self_ty = self_ty.ty.clone().into();
            self.gen_ty_decl(&self_ty, "self", false, header, &mut param_decls);
        }

        for param in &method.params {
            self.gen_ty_decl(
                &param.ty,
                param.name.as_str(),
                false,
                header,
                &mut param_decls,
            );
        }

        let return_ty: Cow<str> = match method.output {
            ReturnType::Infallible(SuccessType::Unit) => "void".into(),
            ReturnType::Infallible(SuccessType::Write) => {
                param_decls.push((self.formatter.fmt_diplomat_write().into(), "write".into()));
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
                        param_decls
                            .push((self.formatter.fmt_diplomat_write().into(), "write".into()));
                        None
                    }
                    SuccessType::Unit => None,
                    SuccessType::OutType(o) => Some(o),
                    _ => unreachable!("unknown AST/HIR variant"),
                };
                self.gen_result_ty(&mut maybe_result_typedef, &method_name, ok_ty, err, header)
                    .into()
            }
            _ => unreachable!("unknown AST/HIR variant"),
        };

        let mut params = String::new();
        let mut params_inv = String::new();
        let mut first = true;
        for (decl_ty, decl_name) in param_decls {
            let comma = if first {
                first = false;
                ""
            } else {
                ", "
            };
            write!(&mut params, "{comma}{decl_ty} {decl_name}").unwrap();
            write!(&mut params_inv, "{comma}{decl_name}").unwrap();
        }

        MethodTemplate {
            name: method_name.into(),
            abi_name: abi_method_name.into(),
            return_ty,
            params,
            params_inv,
            maybe_result_typedef,
        }
    }

    pub fn gen_result_ty(
        &self,
        typedef: &mut String,
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
        writeln!(
            typedef,
            "typedef struct {fn_name}_result {{{union_def} bool is_ok;}} {fn_name}_result;"
        )
        .unwrap();
        format!("{fn_name}_result")
    }

    /// Generates a list of decls for a given type, returned as (type, name)
    ///
    /// Might return multiple in the case of slices and strings. The `is_struct` parameter
    /// affects whether the decls are generated for a struct field or method
    pub fn gen_ty_decl<'a, P: TyPosition>(
        &self,
        ty: &Type<P>,
        ident: &'a str,
        is_struct: bool,
        header: &mut Header,
        out: &mut Vec<(Cow<'tcx, str>, Cow<'a, str>)>,
    ) {
        let param_name = self.formatter.fmt_param_name(ident);
        match ty {
            Type::Slice(hir::Slice::Str(
                _,
                hir::StringEncoding::UnvalidatedUtf8 | hir::StringEncoding::Utf8,
            )) if !is_struct => {
                out.push(("const char*".into(), format!("{param_name}_data").into()));
                out.push(("size_t".into(), format!("{param_name}_len").into()));
            }
            Type::Slice(hir::Slice::Str(_, hir::StringEncoding::UnvalidatedUtf16))
                if !is_struct =>
            {
                out.push((
                    "const char16_t*".into(),
                    format!("{param_name}_data").into(),
                ));
                out.push(("size_t".into(), format!("{param_name}_len").into()));
            }
            Type::Slice(hir::Slice::Primitive(b, p)) if !is_struct => {
                let prim = self.formatter.fmt_primitive_as_c(*p);
                let ptr_type = self.formatter.fmt_ptr(
                    &prim,
                    b.map(|b| b.mutability).unwrap_or(hir::Mutability::Mutable),
                );
                out.push((
                    format!("{ptr_type}").into(),
                    format!("{param_name}_data").into(),
                ));
                out.push(("size_t".into(), format!("{param_name}_len").into()));
            }
            Type::Slice(hir::Slice::Strs(encoding)) => {
                out.push((
                    match encoding {
                        hir::StringEncoding::UnvalidatedUtf16 => "DiplomatStrings16View*",
                        _ => "DiplomatStringsView*",
                    }
                    .into(),
                    format!("{param_name}_data").into(),
                ));
                out.push(("size_t".into(), format!("{param_name}_len").into()));
            }
            _ => {
                let ty = self.gen_ty_name(ty, header);
                out.push((ty, param_name));
            }
        }
    }

    // Generate the C code for referencing a particular type.
    // Handles adding imports and such as necessary
    fn gen_ty_name<P: TyPosition>(&self, ty: &Type<P>, header: &mut Header) -> Cow<'tcx, str> {
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
                hir::Slice::Primitive(borrow, prim) => self
                    .formatter
                    .fmt_primitive_slice_name(*borrow, *prim)
                    .into(),
                hir::Slice::Str(_, hir::StringEncoding::UnvalidatedUtf16) => {
                    "DiplomatString16View".into()
                }
                hir::Slice::Str(_, _) => "DiplomatStringView".into(),
                hir::Slice::Strs(hir::StringEncoding::UnvalidatedUtf16) => {
                    "DiplomatStrings16View".into()
                }
                hir::Slice::Strs(_) => "DiplomatStringsView".into(),
                &_ => unreachable!("unknown AST/HIR variant"),
            },
            _ => unreachable!("unknown AST/HIR variant"),
        };

        ty_name
    }
}
