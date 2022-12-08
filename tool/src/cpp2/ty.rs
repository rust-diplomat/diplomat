use super::header::Header;
use super::Cpp2Context;
use diplomat_core::hir::{
    self, Mutability, OpaqueOwner, ParamSelf, SelfType, TyPosition, Type, TypeDef, TypeId,
};
use std::borrow::Cow;
use std::fmt::Write;

impl<'tcx> super::Cpp2Context<'tcx> {
    pub fn gen_ty(&self, id: TypeId, ty: TypeDef<'tcx>) {
        let header_name = self.formatter.fmt_header_name(id);
        let header_path = format!("{header_name}.y");
        let mut header = Header::new(header_name.clone().into());

        let mut context = TyGenContext::new(self, &mut header);
        match ty {
            TypeDef::Enum(o) => context.gen_enum_def(o, id),
            TypeDef::Opaque(o) => context.gen_opaque_def(o, id),
            TypeDef::Struct(s) => context.gen_struct_def(s, id),
            TypeDef::OutStruct(s) => context.gen_struct_def(s, id),
        }

        context.header.body += "\n\n\n";

        // In some cases like generating decls for `self` parameters,
        // a header will get its own forwards and includes. Instead of
        // trying to avoid pushing them, it's cleaner to just pull them out
        // once done
        let ty_name = context.cx.formatter.fmt_type_name(id);
        context.header.forward_classes.remove(&*ty_name);
        context.header.forward_structs.remove(&*ty_name);
        context.header.includes.remove(&*header_name);

        self.files.add_file(header_path, header.to_string());
    }
}
/// Simple wrapper type representing the return type of a fallible function
pub type ResultType<'tcx> = (Option<&'tcx hir::OutType>, Option<&'tcx hir::OutType>);

/// Context for generating a particular type's header
pub struct TyGenContext<'ccx, 'tcx, 'header> {
    cx: &'ccx Cpp2Context<'tcx>,
    header: &'header mut Header,
}

impl<'ccx, 'tcx: 'ccx, 'header> TyGenContext<'ccx, 'tcx, 'header> {
    pub fn new(cx: &'ccx Cpp2Context<'tcx>, header: &'header mut Header) -> Self {
        TyGenContext { cx, header }
    }

    pub fn gen_enum_def(&mut self, ty: &'tcx hir::EnumDef, id: TypeId) {
        let ty_name = self.cx.formatter.fmt_type_name(id);
        writeln!(&mut self.header.body, "enum struct {ty_name} {{");
        for variant in ty.variants.iter() {
            writeln!(&mut self.header.body, "\t{} = {},", variant.name.as_str(), variant.discriminant);
        }
        writeln!(&mut self.header.body, "}};");
    }

    pub fn gen_opaque_def(&mut self, ty: &'tcx hir::OpaqueDef, id: TypeId) {
        let ty_name = self.cx.formatter.fmt_type_name(id);
        let ctype = self.cx.formatter.fmt_c_name(&ty_name);
        writeln!(&mut self.header.body, "class {ty_name} {{").unwrap();
        writeln!(&mut self.header.body, "public:");
        for method in ty.methods.iter() {
            self.gen_method(id, method);
            writeln!(&mut self.header.body);
        }
        writeln!(&mut self.header.body, "\tinline {ctype} AsFFI() {{");
        writeln!(
            &mut self.header.body,
            "\t\treturn reinterpret_cast::<{ctype}>(this);"
        );
        writeln!(&mut self.header.body, "\t}}").unwrap();
        writeln!(&mut self.header.body);
        self.gen_dtor(id);
        writeln!(&mut self.header.body);
        writeln!(&mut self.header.body, "private:");
        writeln!(&mut self.header.body, "\t{ty_name}() = delete;");
        writeln!(&mut self.header.body, "}}").unwrap();
    }

    pub fn gen_struct_def<P: TyPosition>(&mut self, def: &'tcx hir::StructDef<P>, id: TypeId) {
        let ty_name = self.cx.formatter.fmt_type_name(id);
        writeln!(&mut self.header.body, "struct {ty_name} {{").unwrap();
        for field in def.fields.iter() {
            let decls = self.gen_ty_decl(&field.ty, field.name.as_str(), true);
            for (decl_ty, decl_name) in decls {
                writeln!(&mut self.header.body, "\t{decl_ty} {decl_name};").unwrap();
            }
        }
        // reborrow to avoid borrowing across mutation
        writeln!(&mut self.header.body, "}};").unwrap();
    }

    pub fn gen_method(&mut self, id: TypeId, method: &'tcx hir::Method) {
        use diplomat_core::hir::{ReturnFallability, ReturnType};
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
            self.header.body,
            "\t{maybe_static}{return_ty} {method_name}({params}){qualifiers};"
        )
        .unwrap();
    }

    pub fn gen_dtor(&mut self, id: TypeId) {
        let ty_name = self.cx.formatter.fmt_type_name(id);
        writeln!(self.header.body, "\t~{ty_name}() {{").unwrap();
        writeln!(self.header.body, "\t\t{ty_name}_destroy(AsFFI());").unwrap();
        writeln!(self.header.body, "\t}}").unwrap();
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
        match ty {
            Type::Slice(hir::Slice::Str(..)) => {
                let ty = self.cx.formatter.fmt_borrowed_str();
                vec![(ty, param_name)]
            }
            Type::Slice(hir::Slice::Primitive(b, p)) => {
                let ty = self.cx.formatter.fmt_primitive_as_c(*p);
                let ty = self.cx.formatter.fmt_borrowed_slice(&ty);
                let ty = self.cx.formatter.fmt_constness(&ty, b.mutability);
                vec![(ty.into_owned().into(), param_name)]
            }
            _ => {
                let ty = self.gen_ty_name(ty);
                vec![(ty, param_name)]
            }
        }
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

                self.header.forward_classes.insert(name.to_string());
                ret.to_string().into()
            }
            Type::Struct(ref st) => {
                let id = P::id_for_path(st);
                let ret = self.cx.formatter.fmt_type_name(id);
                let header_name = self.cx.formatter.fmt_header_name(id);
                self.header.includes.insert(header_name.into());
                ret
            }
            Type::Enum(ref e) => {
                let id = e.tcx_id.into();
                let ret = self.cx.formatter.fmt_type_name(id);
                let header_name = self.cx.formatter.fmt_header_name(id);
                self.header.includes.insert(header_name.into());
                ret
            }
            Type::Slice(ref s) => match s {
                // only reachable for structs, not methods
                hir::Slice::Str(..) => "sssssss".into(),
                hir::Slice::Primitive(_, p) => panic!("Attempted to gen_ty_name for slice of {}, should have been handled by gen_ty_decl", p.as_str())
            },
        }
    }
}
