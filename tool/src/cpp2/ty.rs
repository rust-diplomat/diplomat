use super::header::Header;
use super::Cpp2Context;
use diplomat_core::hir::{
    self, Mutability, OpaqueOwner, ParamSelf, SelfType, TyPosition, Type, TypeDef, TypeId,
};
use std::borrow::Cow;
use std::fmt::Write;

impl<'tcx> super::Cpp2Context<'tcx> {
    pub fn gen_ty(&self, id: TypeId, ty: TypeDef<'tcx>) {
        let type_name = self.formatter.fmt_type_name(id);
        let decl_header_path = self.formatter.fmt_decl_header_path(id);
        let mut decl_header = Header::new(decl_header_path.clone());
        let impl_header_path = self.formatter.fmt_impl_header_path(id);
        let mut impl_header = Header::new(impl_header_path.clone());

        let mut context = TyGenContext {
            cx: self,
            decl_header: &mut decl_header,
            impl_header: &mut impl_header,
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
        context.decl_header.forward_classes.remove(&*type_name);
        context.decl_header.forward_structs.remove(&*type_name);
        context.decl_header.includes.remove(&*decl_header_path);
        // TODO: Do this for impl_header too?

        context
            .impl_header
            .includes
            .insert(decl_header_path.clone());
        let c_impl_header_path = self.formatter.fmt_c_impl_header_path(id);
        context.impl_header.includes.insert(c_impl_header_path);

        self.files
            .add_file(decl_header_path, decl_header.to_string());
        self.files
            .add_file(impl_header_path, impl_header.to_string());
    }
}

/// Context for generating a particular type's header
pub struct TyGenContext<'ccx, 'tcx, 'header> {
    pub cx: &'ccx Cpp2Context<'tcx>,
    pub impl_header: &'header mut Header,
    pub decl_header: &'header mut Header,
}

impl<'ccx, 'tcx: 'ccx, 'header> TyGenContext<'ccx, 'tcx, 'header> {
    pub fn gen_enum_def(&mut self, ty: &'tcx hir::EnumDef, id: TypeId) {
        let type_name = self.cx.formatter.fmt_type_name(id);
        writeln!(self.decl_header, "enum struct {type_name} {{").unwrap();
        for variant in ty.variants.iter() {
            writeln!(
                self.decl_header,
                "\t{} = {},",
                variant.name.as_str(),
                variant.discriminant
            )
            .unwrap();
        }
        writeln!(self.decl_header, "}};").unwrap();
    }

    pub fn gen_opaque_def(&mut self, ty: &'tcx hir::OpaqueDef, id: TypeId) {
        let type_name = self.cx.formatter.fmt_type_name(id);
        let ctype = self.cx.formatter.fmt_c_name(&type_name);
        let const_cptr = self.cx.formatter.fmt_c_ptr(&ctype, Mutability::Immutable);
        let mut_cptr = self.cx.formatter.fmt_c_ptr(&ctype, Mutability::Mutable);
        self.decl_header
            .includes
            .insert(self.cx.formatter.fmt_c_decl_header_path(id));
        write!(
            self.decl_header,
            "class {type_name} {{
public:
"
        )
        .unwrap();
        for method in ty.methods.iter() {
            self.gen_method(id, method);
        }
        write!(
            self.decl_header,
            "
\tinline {const_cptr} AsFFI() const;
\tinline {mut_cptr} AsFFI();
\tinline ~{type_name}();
private:
\t{type_name}() = delete;
}};
"
        )
        .unwrap();
        write!(
            self.impl_header,
            "
inline {const_cptr} {type_name}::AsFFI() const {{
\treturn reinterpret_cast<{const_cptr}>(this);
}}
inline {mut_cptr} {type_name}::AsFFI() {{
\treturn reinterpret_cast<{mut_cptr}>(this);
}}
inline {type_name}::~{type_name}() {{
\t{ctype}_destroy(AsFFI());
}}
"
        )
        .unwrap();
    }

    pub fn gen_struct_def<P: TyPosition>(&mut self, def: &'tcx hir::StructDef<P>, id: TypeId) {
        let type_name = self.cx.formatter.fmt_type_name(id);
        writeln!(self.decl_header, "struct {type_name} {{").unwrap();
        for field in def.fields.iter() {
            let decls = self.gen_ty_decl(&field.ty, field.name.as_str());
            for (decl_ty, decl_name) in decls {
                writeln!(self.decl_header, "\t{decl_ty} {decl_name};").unwrap();
            }
        }
        // reborrow to avoid borrowing across mutation
        writeln!(self.decl_header, "}};").unwrap();
    }

    pub fn gen_method(&mut self, id: TypeId, method: &'tcx hir::Method) {
        use diplomat_core::hir::{ReturnFallability, ReturnType};
        let type_name = self.cx.formatter.fmt_type_name(id);
        let method_name = self.cx.formatter.fmt_method_name(method);
        let mut param_decls = Vec::new();

        for param in &method.params {
            let decls = self.gen_ty_decl(&param.ty, param.name.as_str());
            param_decls.extend(decls);
        }

        let return_ty: Cow<str> = match method.output {
            ReturnFallability::Infallible(None) => "void".into(),
            ReturnFallability::Infallible(Some(ref ty)) => match ty {
                ReturnType::Writeable => self.cx.formatter.fmt_owned_str(),
                ReturnType::OutType(o) => self.gen_type_name(o),
            },
            ReturnFallability::Fallible(ref ok, ref err) => {
                let ok_type_name = match ok {
                    Some(ReturnType::Writeable) => self.cx.formatter.fmt_owned_str(),
                    None => "std::monostate".into(),
                    Some(ReturnType::OutType(o)) => self.gen_type_name(o),
                };
                let err_type_name = match err {
                    Some(o) => self.gen_type_name(o),
                    None => "std::monostate".into(),
                };
                let ret: Cow<str> =
                    format!("DiplomatResult<{ok_type_name}, {err_type_name}>").into();
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

        write!(
            self.decl_header,
            "
\tinline {maybe_static}{return_ty} {method_name}({params}){qualifiers};
"
        )
        .unwrap();

        write!(
            self.impl_header,
            "
inline {return_ty} {type_name}::{method_name}({params}){qualifiers} {{
\t// TODO
}}
"
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
    ) -> Vec<(Cow<'ccx, str>, Cow<'a, str>)> {
        let param_name = self.cx.formatter.fmt_param_name(ident);
        let ty = self.gen_type_name(ty);
        vec![(ty, param_name)]
    }

    // Generate the C++ code for referencing a particular type.
    // Handles adding imports and such as necessary
    fn gen_type_name<P: TyPosition>(&mut self, ty: &Type<P>) -> Cow<'ccx, str> {
        match *ty {
            Type::Primitive(prim) => self.cx.formatter.fmt_primitive_as_c(prim),
            Type::Opaque(ref op) => {
                let op_id = op.tcx_id.into();
                let type_name = self.cx.formatter.fmt_type_name(op_id);
                let mutability = op.owner.mutability().unwrap_or(hir::Mutability::Mutable);
                let ret = if op.owner.is_owned() {
                    self.cx.formatter.fmt_owned(&type_name)
                } else if op.is_optional() {
                    self.cx
                        .formatter
                        .fmt_optional_borrowed(&type_name, mutability)
                } else {
                    self.cx.formatter.fmt_borrowed(&type_name, mutability)
                };
                let ret = ret.into_owned().into();

                self.decl_header
                    .forward_classes
                    .insert(type_name.into_owned());
                ret
            }
            Type::Struct(ref st) => {
                let id = P::id_for_path(st);
                // TODO: Make these forward declarations instead of includes
                self.decl_header
                    .includes
                    .insert(self.cx.formatter.fmt_decl_header_path(id));
                self.cx.formatter.fmt_type_name(id)
            }
            Type::Enum(ref e) => {
                let id = e.tcx_id.into();
                // TODO: Make these forward declarations instead of includes
                self.decl_header
                    .includes
                    .insert(self.cx.formatter.fmt_decl_header_path(id));
                self.cx.formatter.fmt_type_name(id)
            }
            Type::Slice(hir::Slice::Str(_lifetime)) => self.cx.formatter.fmt_borrowed_str(),
            Type::Slice(hir::Slice::Primitive(b, p)) => {
                let ret = self.cx.formatter.fmt_primitive_as_c(p);
                let ret = self.cx.formatter.fmt_borrowed_slice(&ret, b.mutability);
                ret.into_owned().into()
            }
        }
    }
}
