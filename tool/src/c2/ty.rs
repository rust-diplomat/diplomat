use super::formatter::CFormatter;
use super::header::Header;
use super::CContext;
use askama::Template;
use diplomat_core::hir::{
    self, FloatType, IntSizeType, IntType, OpaqueOwner, StructPathLike, TyPosition, Type, TypeDef,
    TypeId,
};
use std::borrow::Cow;
use std::fmt::Write;

impl<'tcx> super::CContext<'tcx> {
    pub fn gen_ty(&self, id: TypeId, ty: TypeDef<'tcx>) {
        if ty.attrs().disable {
            // Skip type if disabled
            return;
        }
        let decl_header_path = self.formatter.fmt_decl_header_path(id);
        let mut decl_header = Header::new(decl_header_path.clone());
        let impl_header_path = self.formatter.fmt_impl_header_path(id);
        let mut impl_header = Header::new(impl_header_path.clone());

        let mut context = TyGenContext {
            cx: self,
            decl_header: &mut decl_header,
            impl_header: &mut impl_header,
        };

        let _guard = self.errors.set_context_ty(ty.name().as_str().into());
        match ty {
            TypeDef::Enum(e) => context.gen_enum_def(e, id),
            TypeDef::Opaque(o) => context.gen_opaque_def(o, id),
            TypeDef::Struct(s) => context.gen_struct_def(s, id),
            TypeDef::OutStruct(s) => context.gen_struct_def(s, id),
            _ => unreachable!("unknown AST/HIR variant"),
        }
        for method in ty.methods() {
            if method.attrs.disable {
                // Skip method if disabled
                continue;
            }
            let _guard = self.errors.set_context_method(
                self.formatter.fmt_type_name_diagnostics(id),
                method.name.as_str().into(),
            );
            context.gen_method(id, method);
        }

        if let TypeDef::Opaque(_) = ty {
            context.gen_dtor(id);
        }

        // In some cases like generating decls for `self` parameters,
        // a header will get its own includes. Instead of
        // trying to avoid pushing them, it's cleaner to just pull them out
        // once done
        context.decl_header.includes.remove(&*decl_header_path);
        context.impl_header.includes.remove(&*impl_header_path);
        context.impl_header.includes.remove(&*decl_header_path);

        context.impl_header.decl_include = Some(decl_header_path.clone());

        self.files
            .add_file(decl_header_path, decl_header.to_string());
        self.files
            .add_file(impl_header_path, impl_header.to_string());
    }

    pub fn gen_result(&self, name: &str, ty: ResultType) {
        let _guard = self
            .errors
            .set_context_ty(self.formatter.fmt_result_for_diagnostics(ty).into());
        let header_path = self.formatter.fmt_result_header_path(name);
        let mut header = Header::new(header_path.clone());
        let mut dummy_header = Header::new("".to_string());
        let mut context = TyGenContext {
            cx: self,
            // NOTE: Only one header for results
            decl_header: &mut header,
            impl_header: &mut dummy_header,
        };
        context.gen_result(name, ty);
        self.files.add_file(header_path, header.to_string());
    }
}
/// Simple wrapper type representing the return type of a fallible function
pub type ResultType<'tcx> = (Option<&'tcx hir::OutType>, Option<&'tcx hir::OutType>);

/// Context for generating a particular type's header
pub struct TyGenContext<'ccx, 'tcx, 'header> {
    pub cx: &'ccx CContext<'tcx>,
    pub decl_header: &'header mut Header,
    pub impl_header: &'header mut Header,
}

#[derive(Template)]
#[template(path = "c2/enum.h.jinja", escape = "none")]
struct EnumTemplate<'a> {
    ty: &'a hir::EnumDef,
    fmt: &'a CFormatter<'a>,
    ty_name: &'a str,
}

#[derive(Template)]
#[template(path = "c2/struct.h.jinja", escape = "none")]
struct StructTemplate<'a> {
    ty_name: Cow<'a, str>,
    fields: Vec<(Cow<'a, str>, Cow<'a, str>)>,
}

#[derive(Template)]
#[template(path = "c2/opaque.h.jinja", escape = "none")]
struct OpaqueTemplate<'a> {
    ty_name: Cow<'a, str>,
}

impl<'ccx, 'tcx: 'ccx, 'header> TyGenContext<'ccx, 'tcx, 'header> {
    pub fn gen_enum_def(&mut self, def: &'tcx hir::EnumDef, id: TypeId) {
        let ty_name = self.cx.formatter.fmt_type_name(id);
        EnumTemplate {
            ty: def,
            fmt: &self.cx.formatter,
            ty_name: &ty_name,
        }
        .render_into(self.decl_header)
        .unwrap();
    }

    pub fn gen_opaque_def(&mut self, _def: &'tcx hir::OpaqueDef, id: TypeId) {
        let ty_name = self.cx.formatter.fmt_type_name(id);
        OpaqueTemplate { ty_name }
            .render_into(self.decl_header)
            .unwrap();
    }

    pub fn gen_struct_def<P: TyPosition>(&mut self, def: &'tcx hir::StructDef<P>, id: TypeId) {
        let ty_name = self.cx.formatter.fmt_type_name(id);
        let mut fields = vec![];
        for field in def.fields.iter() {
            self.gen_ty_decl(&field.ty, field.name.as_str(), true, &mut fields);
        }

        StructTemplate { ty_name, fields }
            .render_into(self.decl_header)
            .unwrap();
    }

    pub fn gen_method(&mut self, id: TypeId, method: &'tcx hir::Method) {
        use diplomat_core::hir::{ReturnType, SuccessType};
        let method_name = self.cx.formatter.fmt_method_name(id, method);
        let mut param_decls = Vec::new();
        if let Some(ref self_ty) = method.param_self {
            let self_ty = self_ty.ty.clone().into();
            self.gen_ty_decl(&self_ty, "self", false, &mut param_decls);
        }

        for param in &method.params {
            self.gen_ty_decl(&param.ty, param.name.as_str(), false, &mut param_decls);
        }

        let return_ty: Cow<str> = match method.output {
            ReturnType::Infallible(SuccessType::Unit) => "void".into(),
            ReturnType::Infallible(SuccessType::Write) => {
                param_decls.push(("DiplomatWrite*".into(), "write".into()));
                "void".into()
            }
            ReturnType::Infallible(SuccessType::OutType(ref o)) => self.gen_ty_name(o, false),
            ReturnType::Fallible(ref ok, _) | ReturnType::Nullable(ref ok) => {
                // Result<T, ()> and Option<T> are the same on the ABI
                let err = if let ReturnType::Fallible(_, Some(ref e)) = method.output {
                    Some(e)
                } else {
                    None
                };
                let (ok_type_name, ok_ty) = match ok {
                    SuccessType::Write => {
                        param_decls.push(("DiplomatWrite*".into(), "write".into()));
                        ("void".into(), None)
                    }
                    SuccessType::Unit => ("void".into(), None),
                    SuccessType::OutType(o) => {
                        (self.cx.formatter.fmt_type_name_uniquely(o), Some(o))
                    }
                    _ => unreachable!("unknown AST/HIR variant"),
                };
                let err_type_name = match err {
                    Some(o) => self.cx.formatter.fmt_type_name_uniquely(o),
                    None => "void".into(),
                };
                // todo push to results set
                let result_name = self
                    .cx
                    .formatter
                    .fmt_result_name(&ok_type_name, &err_type_name);
                self.impl_header
                    .includes
                    .insert(self.cx.formatter.fmt_result_header_path(&result_name));
                self.cx
                    .result_store
                    .borrow_mut()
                    .insert(result_name.clone(), (ok_ty, err));
                result_name.into()
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

        write!(self.impl_header, "{return_ty} {method_name}({params});\n\n").unwrap();
    }

    pub fn gen_dtor(&mut self, id: TypeId) {
        let ty_name = self.cx.formatter.fmt_type_name(id);
        let dtor_name = self.cx.formatter.fmt_dtor_name(id);
        write!(self.impl_header, "void {dtor_name}({ty_name}* self);\n\n").unwrap();
    }

    pub fn gen_result(&mut self, name: &str, ty: ResultType) {
        let ok_line = if let Some(ok) = ty.0 {
            let ok_name = self.gen_ty_name(ok, true);
            format!("\t\t{ok_name} ok;\n")
        } else {
            "".into()
        };
        let err_line = if let Some(err) = ty.1 {
            let err_name = self.gen_ty_name(err, true);
            format!("\t\t{err_name} err;\n")
        } else {
            "".into()
        };

        let union_def = if ty.0.is_some() || ty.1.is_some() {
            format!("\tunion {{\n{ok_line}{err_line}\t}};\n")
        } else {
            "".into()
        };

        writeln!(
            self.decl_header,
            "typedef struct {name} {{\n{union_def}\tbool is_ok;\n}} {name};"
        )
        .unwrap();
    }

    /// Generates a list of decls for a given type, returned as (type, name)
    ///
    /// Might return multiple in the case of slices and strings. The `is_struct` parameter
    /// affects whether the decls are generated for a struct field or method
    pub fn gen_ty_decl<'a, P: TyPosition>(
        &mut self,
        ty: &Type<P>,
        ident: &'a str,
        is_struct: bool,
        out: &mut Vec<(Cow<'ccx, str>, Cow<'a, str>)>,
    ) {
        let param_name = self.cx.formatter.fmt_param_name(ident);
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
                let prim = self.cx.formatter.fmt_primitive_as_c(*p);
                let ptr_type = self.cx.formatter.fmt_ptr(
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
                let ty = self.gen_ty_name(ty, is_struct);
                out.push((ty, param_name));
            }
        }
    }

    // Generate the C code for referencing a particular type.
    // Handles adding imports and such as necessary
    fn gen_ty_name<P: TyPosition>(&mut self, ty: &Type<P>, is_decl: bool) -> Cow<'ccx, str> {
        let header = if is_decl {
            &mut self.decl_header
        } else {
            &mut self.impl_header
        };
        let (id, ty_name) = match *ty {
            Type::Primitive(prim) => (None, self.cx.formatter.fmt_primitive_as_c(prim)),
            Type::Opaque(ref op) => {
                let op_id = op.tcx_id.into();
                let ty_name = self.cx.formatter.fmt_type_name(op_id);
                if self.cx.tcx.resolve_type(op_id).attrs().disable {
                    self.cx
                        .errors
                        .push_error(format!("Found usage of disabled type {ty_name}"))
                }
                // unwrap_or(mut) since owned pointers need to not be const
                let mutability = op.owner.mutability().unwrap_or(hir::Mutability::Mutable);
                let ret = self.cx.formatter.fmt_ptr(&ty_name, mutability);
                header
                    .includes
                    .insert(self.cx.formatter.fmt_decl_header_path(op_id));
                (Some(op_id), ret.into_owned().into())
            }
            Type::Struct(ref st) => {
                let st_id = st.id();
                let ty_name = self.cx.formatter.fmt_type_name(st_id);
                if self.cx.tcx.resolve_type(st_id).attrs().disable {
                    self.cx
                        .errors
                        .push_error(format!("Found usage of disabled type {ty_name}"))
                }
                let ret = ty_name.clone();
                let header_path = self.cx.formatter.fmt_decl_header_path(st_id);
                header.includes.insert(header_path);
                (Some(st_id), ret)
            }
            Type::Enum(ref e) => {
                let id = e.tcx_id.into();
                let ty_name = self.cx.formatter.fmt_type_name(id);
                if self.cx.tcx.resolve_type(id).attrs().disable {
                    self.cx
                        .errors
                        .push_error(format!("Found usage of disabled type {ty_name}"))
                }
                let header_path = self.cx.formatter.fmt_decl_header_path(id);
                header.includes.insert(header_path);
                (Some(id), ty_name)
            }
            Type::Slice(ref s) => (
                None,
                match s {
                    hir::Slice::Primitive(
                        _,
                        hir::PrimitiveType::Int(IntType::U8) | hir::PrimitiveType::Byte,
                    ) => "DiplomatU8View",
                    hir::Slice::Primitive(_, hir::PrimitiveType::Int(IntType::U16)) => {
                        "DiplomatU16View"
                    }
                    hir::Slice::Primitive(_, hir::PrimitiveType::Int(IntType::U32)) => {
                        "DiplomatU32View"
                    }
                    hir::Slice::Primitive(_, hir::PrimitiveType::Int(IntType::U64)) => {
                        "DiplomatU64View"
                    }
                    hir::Slice::Primitive(_, hir::PrimitiveType::Int(IntType::I8)) => {
                        "DiplomatI8View"
                    }
                    hir::Slice::Primitive(_, hir::PrimitiveType::Int(IntType::I16)) => {
                        "DiplomatI16View"
                    }
                    hir::Slice::Primitive(_, hir::PrimitiveType::Int(IntType::I32)) => {
                        "DiplomatI32View"
                    }
                    hir::Slice::Primitive(_, hir::PrimitiveType::Int(IntType::I64)) => {
                        "DiplomatI64View"
                    }
                    hir::Slice::Primitive(_, hir::PrimitiveType::IntSize(IntSizeType::Usize)) => {
                        "DiplomatUsizeView"
                    }
                    hir::Slice::Primitive(_, hir::PrimitiveType::IntSize(IntSizeType::Isize)) => {
                        "DiplomatIsizeView"
                    }
                    hir::Slice::Primitive(_, hir::PrimitiveType::Bool) => "DiplomatBoolView",
                    hir::Slice::Primitive(_, hir::PrimitiveType::Float(FloatType::F32)) => {
                        "DiplomatF32View"
                    }
                    hir::Slice::Primitive(_, hir::PrimitiveType::Float(FloatType::F64)) => {
                        "DiplomatF64View"
                    }
                    hir::Slice::Primitive(_, hir::PrimitiveType::Char) => "DiplomatCharView",
                    hir::Slice::Str(_, hir::StringEncoding::UnvalidatedUtf16) => {
                        "DiplomatString16View"
                    }
                    hir::Slice::Str(_, _) => "DiplomatStringView",
                    hir::Slice::Strs(hir::StringEncoding::UnvalidatedUtf16) => {
                        "DiplomatStrings16View"
                    }
                    hir::Slice::Strs(_) => "DiplomatStringsView",
                    &_ => unreachable!("unknown AST/HIR variant"),
                }
                .into(),
            ),
            _ => unreachable!("unknown AST/HIR variant"),
        };
        // Todo(breaking): We can remove this requirement
        // and users will be forced to import more types
        if let Some(id) = id {
            if !is_decl {
                header
                    .includes
                    .insert(self.cx.formatter.fmt_impl_header_path(id));
            }
        }
        ty_name
    }
}
