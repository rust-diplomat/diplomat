use super::header::Header;
use super::Cpp2Context;
use diplomat_core::hir::{
    self, Mutability, OpaqueOwner, ParamSelf, SelfType, TyPosition, Type, TypeDef, TypeId,
};
use std::borrow::Cow;
use std::fmt::Write;

impl<'tcx> super::Cpp2Context<'tcx> {
    pub fn gen_ty(&self, id: TypeId, ty: TypeDef<'tcx>) {
        let ty_name = self.formatter.fmt_type_name(id);
        let decl_header_path = self.formatter.fmt_decl_header_path(id);
        let mut decl_header = Header::new(decl_header_path.clone());
        let impl_header_path = self.formatter.fmt_impl_header_path(id);
        let mut impl_header = Header::new(impl_header_path.clone());

        let mut context = TyGenContext {
            cx: self,
            decl_header: &mut decl_header,
            impl_header: &mut impl_header
        };
        match ty {
            TypeDef::Enum(o) => context.gen_enum_def(o, id),
            TypeDef::Opaque(o) => context.gen_opaque_def(o, id),
            TypeDef::Struct(s) => context.gen_struct_def(s, id),
            TypeDef::OutStruct(s) => context.gen_struct_def(s, id),
        }

        context.decl_header.body += "\n\n\n";

        // In some cases like generating decls for `self` parameters,
        // a header will get its own forwards and includes. Instead of
        // trying to avoid pushing them, it's cleaner to just pull them out
        // once done
        context.decl_header.forward_classes.remove(&*ty_name);
        context.decl_header.forward_structs.remove(&*ty_name);
        context.decl_header.includes.remove(&*decl_header_path);
        // TODO: Do this for impl_header too?

        context.impl_header.includes.insert(decl_header_path.clone());
        let c_impl_header_path = self.formatter.fmt_c_impl_header_path(id);
        context.impl_header.includes.insert(c_impl_header_path);

        self.files.add_file(decl_header_path, decl_header.to_string());
        self.files.add_file(impl_header_path, impl_header.to_string());
    }
}
/// Simple wrapper type representing the return type of a fallible function
pub type ResultType<'tcx> = (Option<&'tcx hir::OutType>, Option<&'tcx hir::OutType>);

/// Context for generating a particular type's header
pub struct TyGenContext<'ccx, 'tcx, 'header> {
    pub cx: &'ccx Cpp2Context<'tcx>,
    pub impl_header: &'header mut Header,
    pub decl_header: &'header mut Header,
}

impl<'ccx, 'tcx: 'ccx, 'header> TyGenContext<'ccx, 'tcx, 'header> {
    pub fn gen_enum_def(&mut self, ty: &'tcx hir::EnumDef, id: TypeId) {
        let ty_name = self.cx.formatter.fmt_type_name(id);
        writeln!(&mut self.decl_header.body, "enum struct {ty_name} {{");
        for variant in ty.variants.iter() {
            writeln!(&mut self.decl_header.body, "\t{} = {},", variant.name.as_str(), variant.discriminant);
        }
        writeln!(&mut self.decl_header.body, "}};");
    }

    pub fn gen_opaque_def(&mut self, ty: &'tcx hir::OpaqueDef, id: TypeId) {
        let ty_name = self.cx.formatter.fmt_type_name(id);
        let ctype = self.cx.formatter.fmt_c_name(&ty_name);
        let cptr = self.cx.formatter.fmt_c_ptr(&ctype);
        self.decl_header.includes.insert(self.cx.formatter.fmt_c_decl_header_path(id));
        writeln!(&mut self.decl_header.body, "class {ty_name} {{").unwrap();
        writeln!(&mut self.decl_header.body, "public:");
        for method in ty.methods.iter() {
            self.gen_method(id, method);
            writeln!(&mut self.decl_header.body);
        }
        writeln!(&mut self.decl_header.body, "\tinline {cptr} AsFFI();");
        writeln!(&mut self.impl_header.body, "inline {cptr} {ty_name}::AsFFI() {{");
        writeln!(
            &mut self.impl_header.body,
            "\treturn reinterpret_cast<{cptr}>(this);"
        );
        writeln!(&mut self.impl_header.body, "}}").unwrap();
        writeln!(&mut self.decl_header.body);
        self.gen_dtor(id);
        writeln!(&mut self.decl_header.body);
        writeln!(&mut self.decl_header.body, "private:");
        writeln!(&mut self.decl_header.body, "\t{ty_name}() = delete;");
        writeln!(&mut self.decl_header.body, "}};").unwrap();
    }

    pub fn gen_struct_def<P: TyPosition>(&mut self, def: &'tcx hir::StructDef<P>, id: TypeId) {
        let ty_name = self.cx.formatter.fmt_type_name(id);
        writeln!(&mut self.decl_header.body, "struct {ty_name} {{").unwrap();
        for field in def.fields.iter() {
            let decls = self.gen_ty_decl(&field.ty, field.name.as_str(), true);
            for (decl_ty, decl_name) in decls {
                writeln!(&mut self.decl_header.body, "\t{decl_ty} {decl_name};").unwrap();
            }
        }
        // reborrow to avoid borrowing across mutation
        writeln!(&mut self.decl_header.body, "}};").unwrap();
    }

    pub fn gen_method(&mut self, id: TypeId, method: &'tcx hir::Method) {
        use diplomat_core::hir::{ReturnFallability, ReturnType};
        let ty_name = self.cx.formatter.fmt_type_name(id);
        let method_name = self.cx.formatter.fmt_method_name(id, method);
        let mut param_decls = Vec::new();

        for param in &method.params {
            let decls = self.gen_ty_decl(&param.ty, param.name.as_str(), false);
            param_decls.extend(decls);
        }

        let return_ty: Cow<str> = match method.output {
            ReturnFallability::Infallible(None) => "void".into(),
            ReturnFallability::Infallible(Some(ref ty)) => match ty {
                ReturnType::Writeable => self.cx.formatter.fmt_owned_str(),
                ReturnType::OutType(o) => self.gen_ty_name(o),
            },
            ReturnFallability::Fallible(ref ok, ref err) => {
                let (ok_ty_name, ok_ty) = match ok {
                    Some(ReturnType::Writeable) => (self.cx.formatter.fmt_owned_str(), None),
                    None => ("std::monostate".into(), None),
                    Some(ReturnType::OutType(o)) => (self.gen_ty_name(o), Some(o)),
                };
                let err_ty_name = match err {
                    Some(o) => self.gen_ty_name(o),
                    None => "std::monostate".into(),
                };
                let ret: Cow<str> = format!("DiplomatResult<{ok_ty_name}, {err_ty_name}>").into();
                ret
            }
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

        let maybe_static = if method.param_self.is_none() {
            "static "
        } else {
            ""
        };

        let qualifiers = match &method.param_self {
            Some(ParamSelf {
                ty: SelfType::Opaque(opaque_path),
            }) if opaque_path.owner.mutability == Mutability::Immutable => " const",
            Some(_) => "",
            None => "",
        };

        writeln!(
            self.decl_header.body,
            "\tinline {maybe_static}{return_ty} {method_name}({params}){qualifiers};"
        )
        .unwrap();


        writeln!(
            self.impl_header.body,
            "inline {return_ty} {ty_name}::{method_name}({params}){qualifiers} {{"
        );
        writeln!(
            self.impl_header.body,
            "\t// TODO"
        );
        writeln!(
            self.impl_header.body,
            "}}"
        );
    }

    pub fn gen_dtor(&mut self, id: TypeId) {
        let ty_name = self.cx.formatter.fmt_type_name(id);
        let ctype = self.cx.formatter.fmt_c_name(&ty_name);
        writeln!(self.decl_header.body, "\tinline ~{ty_name}();").unwrap();
        writeln!(self.impl_header.body, "inline {ty_name}::~{ty_name}() {{").unwrap();
        writeln!(self.impl_header.body, "\t{ctype}_destroy(AsFFI());").unwrap();
        writeln!(self.impl_header.body, "}}").unwrap();
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
    ) -> Vec<(Cow<'ccx, str>, Cow<'a, str>)> {
        let param_name = self.cx.formatter.fmt_param_name(ident);
        let ty = self.gen_ty_name(ty);
        vec![(ty, param_name)]
    }

    // Generate the C++ code for referencing a particular type.
    // Handles adding imports and such as necessary
    fn gen_ty_name<P: TyPosition>(&mut self, ty: &Type<P>) -> Cow<'ccx, str> {
        match *ty {
            Type::Primitive(prim) => self.cx.formatter.fmt_primitive_as_c(prim),
            Type::Opaque(ref op) => {
                let op_id = op.tcx_id.into();
                let name = self.cx.formatter.fmt_type_name(op_id);
                let ret = if op.owner.is_owned() {
                    self.cx.formatter.fmt_owned(&name)
                } else if op.is_optional() {
                    self.cx.formatter.fmt_optional_borrowed(&name)
                } else {
                    self.cx.formatter.fmt_borrowed(&name)
                };
                let mutability = op.owner.mutability().unwrap_or(hir::Mutability::Mutable);
                let ret = self.cx.formatter.fmt_constness(&ret, mutability);
                let ret = ret.into_owned().into();

                self.decl_header.forward_classes.insert(name.into_owned());
                ret
            }
            Type::Struct(ref st) => {
                let id = P::id_for_path(st);
                let ret = self.cx.formatter.fmt_type_name(id);
                let header_path = self.cx.formatter.fmt_decl_header_path(id);
                // TODO: Make these forward declarations instead of includes
                self.decl_header.includes.insert(header_path.into());
                ret
            }
            Type::Enum(ref e) => {
                let id = e.tcx_id.into();
                let ret = self.cx.formatter.fmt_type_name(id);
                let header_path = self.cx.formatter.fmt_decl_header_path(id);
                // TODO: Make these forward declarations instead of includes
                self.decl_header.includes.insert(header_path.into());
                ret
            }
            Type::Slice(hir::Slice::Str(..)) => {
                let ret = self.cx.formatter.fmt_borrowed_str();
                ret.into_owned().into()
            }
            Type::Slice(hir::Slice::Primitive(b, p)) => {
                let ret = self.cx.formatter.fmt_primitive_as_c(p);
                let ret = self.cx.formatter.fmt_borrowed_slice(&ret);
                let ret = self.cx.formatter.fmt_constness(&ret, b.mutability);
                ret.into_owned().into()
            }
        }
    }
}
