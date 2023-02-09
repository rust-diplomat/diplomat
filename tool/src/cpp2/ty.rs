use super::header::{Forward, Header};
use super::Cpp2Context;
use diplomat_core::hir::{
    self, Mutability, OpaqueOwner, ParamSelf, ReturnFallability, ReturnType, SelfType,
    TyPosition, Type, TypeDef, TypeId,
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
            impl_header: &mut impl_header,
        };
        match ty {
            TypeDef::Enum(o) => context.gen_enum_def(o, id),
            TypeDef::Opaque(o) => context.gen_opaque_def(o, id),
            TypeDef::Struct(s) => context.gen_struct_def(s, id),
            TypeDef::OutStruct(s) => context.gen_struct_def(s, id),
        }

        // In some cases like generating decls for `self` parameters,
        // a header will get its own forwards and includes. Instead of
        // trying to avoid pushing them, it's cleaner to just pull them out
        // once done
        context.decl_header.forwards.remove(&*ty_name);
        context.impl_header.forwards.remove(&*ty_name);
        context.decl_header.includes.remove(&*decl_header_path);
        context.impl_header.includes.remove(&*impl_header_path);
        context.impl_header.includes.remove(&*decl_header_path);

        context.impl_header.decl_include = Some(decl_header_path.clone());

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
        let ty_name = self.cx.formatter.fmt_type_name(id);
        let ctype = self.cx.formatter.fmt_c_name(&ty_name);
        self.decl_header
            .includes
            .insert(self.cx.formatter.fmt_c_decl_header_path(id));
        write!(self.decl_header, "class {ty_name} {{
\t{ctype} value;

public:
\tenum Value {{
").unwrap();
write!(self.impl_header, "inline {ty_name}::{ty_name}({ty_name}::Value cpp_value) {{
\tswitch (cpp_value) {{
").unwrap();
        for variant in ty.variants.iter() {
            let enum_variant = self.cx.formatter.fmt_enum_variant(variant);
            let c_enum_variant = self.cx.formatter.fmt_c_enum_variant(&ty_name, variant);
            writeln!(self.decl_header, "\t\t{enum_variant},").unwrap();
            write!(self.impl_header, "\t\tcase {enum_variant}:
\t\t\tvalue = {c_enum_variant};
\t\t\tbreak;
").unwrap();
        }
        write!(self.decl_header, "\t}};

\tinline {ty_name}({ty_name}::Value cpp_value);
\tinline {ty_name}({ctype} c_enum) : value(c_enum) {{}};
");
        write!(self.impl_header, "\t\tdefault:
\t\t\tabort();
\t}}
}}
").unwrap();
        for method in ty.methods.iter() {
            self.gen_method(id, method);
        }
        write!(self.decl_header, "
\tinline {ctype} AsFFI() const;
\tinline static {ty_name} FromFFI({ctype} c_enum);
}};\n\n").unwrap();
        write!(self.impl_header, "
inline {ctype} {ty_name}::AsFFI() const {{
\treturn value;
}}

inline {ty_name} {ty_name}::FromFFI({ctype} c_enum) {{
\treturn {ty_name}(c_enum);
}}
").unwrap();
    }

    pub fn gen_opaque_def(&mut self, ty: &'tcx hir::OpaqueDef, id: TypeId) {
        let ty_name = self.cx.formatter.fmt_type_name(id);
        let const_ptr = self.cx.formatter.fmt_c_ptr(&ty_name, Mutability::Immutable);
        let mut_ptr = self.cx.formatter.fmt_c_ptr(&ty_name, Mutability::Mutable);
        let ctype = self.cx.formatter.fmt_c_name(&ty_name);
        let const_cptr = self.cx.formatter.fmt_c_ptr(&ctype, Mutability::Immutable);
        let mut_cptr = self.cx.formatter.fmt_c_ptr(&ctype, Mutability::Mutable);
        self.decl_header
            .includes
            .insert(self.cx.formatter.fmt_c_decl_header_path(id));
        write!(
            self.decl_header,
            "class {ty_name} {{
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
\tinline static {const_ptr} FromFFI({const_cptr} ptr);
\tinline static {mut_ptr} FromFFI({mut_cptr} ptr);
\tinline ~{ty_name}();
private:
\t{ty_name}() = delete;
}};

"
        )
        .unwrap();
        write!(
            self.impl_header,
            "inline {const_cptr} {ty_name}::AsFFI() const {{
\treturn reinterpret_cast<{const_cptr}>(this);
}}

inline {mut_cptr} {ty_name}::AsFFI() {{
\treturn reinterpret_cast<{mut_cptr}>(this);
}}

inline {const_ptr} {ty_name}::FromFFI({const_cptr} ptr) {{
\treturn reinterpret_cast<{const_ptr}>(ptr);
}}

inline {mut_ptr} {ty_name}::FromFFI({mut_cptr} ptr) {{
\treturn reinterpret_cast<{mut_ptr}>(ptr);
}}

inline {ty_name}::~{ty_name}() {{
\t{ctype}_destroy(AsFFI());
}}

"
        )
        .unwrap();
    }

    pub fn gen_struct_def<P: TyPosition>(&mut self, def: &'tcx hir::StructDef<P>, id: TypeId) {
        let ty_name = self.cx.formatter.fmt_type_name(id);
        let ctype = self.cx.formatter.fmt_c_name(&ty_name);
        writeln!(self.decl_header, "struct {ty_name} {{").unwrap();
        for field in def.fields.iter() {
            let (decl_ty, decl_name) = self.gen_ty_decl(&field.ty, field.name.as_str());
            writeln!(self.decl_header, "\t{decl_ty} {decl_name};").unwrap();
        }
        for method in def.methods.iter() {
            self.gen_method(id, method);
        }
        write!(self.decl_header, "
\tinline {ctype} AsFFI() const;
\tinline static {ty_name} FromFFI({ctype} c_struct);
}};\n\n").unwrap();
    write!(self.impl_header,
    "
inline {ctype} {ty_name}::AsFFI() const {{
\treturn {ctype} {{
").unwrap();
    for field in def.fields.iter() {
        let (decl_ty, decl_name) = self.gen_ty_decl(&field.ty, field.name.as_str());
        for (c_name, conversion) in self.gen_cpp_to_c(&field.ty, &decl_name) {
            writeln!(self.impl_header, "\t\t.{c_name} = {conversion},").unwrap();
        }
    }
write!(self.impl_header, "\t}};
}}

inline {ty_name} {ty_name}::FromFFI({ctype} c_struct) {{
\treturn {ty_name} {{
").unwrap();
        for field in def.fields.iter() {
            let (decl_ty, decl_name) = self.gen_ty_decl(&field.ty, field.name.as_str());
            let field_getter = format!("c_struct.{decl_name}");
            let conversion = self.gen_c_to_cpp(&field.ty, &field_getter);
            writeln!(self.impl_header, "\t\t.{decl_name} = {conversion},").unwrap();
        }
    write!(self.impl_header, "\t}};
}}

"
)
.unwrap();
    }

    pub fn gen_method(&mut self, id: TypeId, method: &'tcx hir::Method) {
        let ty_name = self.cx.formatter.fmt_type_name(id);
        let method_name = self.cx.formatter.fmt_method_name(method);
        let c_method_name = self.cx.formatter.fmt_c_method_name(id, method);
        let mut param_decls = Vec::new();
        let mut cpp_to_c_params = Vec::new();

        if let Some(param_self) = method.param_self.as_ref() {
            cpp_to_c_params.push(self.gen_cpp_to_c_self(&param_self.ty));
        }

        for param in method.params.iter() {
            let decls = self.gen_ty_decl(&param.ty, param.name.as_str());
            param_decls.push(decls);
            let conversions = self.gen_cpp_to_c(&param.ty, param.name.as_str());
            cpp_to_c_params.extend(conversions.into_iter().map(|(_, cnv)| cnv));
        }

        if method.is_writeable() {
            cpp_to_c_params.push("&writeable".into());
        }

        let return_ty = self.gen_return_ty_name(&method.output);

        let return_statement: Cow<str> = self.gen_fallible_c_to_cpp(&method.output, "result").map(|s| format!("\n\treturn {s};").into()).unwrap_or("".into());

        let return_prefix = if return_statement.is_empty() {
            ""
        } else {
            "auto result = "
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

        let mut c_params = String::new();
        let mut first = true;
        for conversion in cpp_to_c_params {
            let comma = if first {
                first = false;
                ""
            } else {
                ",\n\t\t"
            };
            write!(&mut c_params, "{comma}{conversion}").unwrap();
        }

        let writeable_prefix = if method.is_writeable() {
            "std::string output;
\tcapi::DiplomatWriteable writeable = diplomat::WriteableFromString(output);
\t"
        } else {
            ""
        };

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
            "inline {return_ty} {ty_name}::{method_name}({params}){qualifiers} {{
\t{writeable_prefix}{return_prefix}{c_method_name}({c_params});{return_statement}
}}

"
        )
        .unwrap();
    }

    /// Generates a parameter decl for a given type, returned as (type, param_name)
    pub fn gen_ty_decl<'a, P: TyPosition>(
        &mut self,
        ty: &Type<P>,
        ident: &'a str,
    ) -> (Cow<'ccx, str>, Cow<'a, str>) {
        let param_name = self.cx.formatter.fmt_param_name(ident);
        let ty = self.gen_ty_name(ty);
        (ty, param_name)
    }

    // Generate the C++ code for referencing a particular type.
    // Handles adding imports and such as necessary
    fn gen_ty_name<P: TyPosition>(&mut self, ty: &Type<P>) -> Cow<'ccx, str> {
        match *ty {
            Type::Primitive(prim) => self.cx.formatter.fmt_primitive_as_c(prim),
            Type::Opaque(ref op) => {
                let op_id = op.tcx_id.into();
                let ty_name = self.cx.formatter.fmt_type_name(op_id);
                let mutability = op.owner.mutability().unwrap_or(hir::Mutability::Mutable);
                let ret = match (op.owner.is_owned(), op.is_optional()) {
                    // unique_ptr is nullable
                    (true, _) => self.cx.formatter.fmt_owned(&ty_name),
                    (false, true) => self
                        .cx
                        .formatter
                        .fmt_optional_borrowed(&ty_name, mutability),
                    (false, false) => self.cx.formatter.fmt_borrowed(&ty_name, mutability),
                };
                let ret = ret.into_owned().into();

                self.decl_header
                    .forwards
                    .insert(Forward::Class(ty_name.into_owned()));
                self.impl_header
                    .includes
                    .insert(self.cx.formatter.fmt_decl_header_path(op_id));
                ret
            }
            Type::Struct(ref st) => {
                let id = P::id_for_path(st);
                self.decl_header.forwards.insert(Forward::Struct(
                    self.cx.formatter.fmt_type_name(id).into_owned(),
                ));
                self.decl_header
                    .includes
                    .insert(self.cx.formatter.fmt_decl_header_path(id));
                self.cx.formatter.fmt_type_name(id)
            }
            Type::Enum(ref e) => {
                let id = e.tcx_id.into();
                self.decl_header.forwards.insert(Forward::EnumStruct(
                    self.cx.formatter.fmt_type_name(id).into_owned(),
                ));
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

    fn gen_cpp_to_c_self(&self, ty: &SelfType) -> Cow<'static, str> {
        match *ty {
            SelfType::Opaque(..) => "this->AsFFI()".into(),
            SelfType::Struct(..) => todo!(),
            SelfType::Enum(..) => todo!(),
        }
    }

    fn gen_cpp_to_c<'a, P: TyPosition>(
        &self,
        ty: &Type<P>,
        param_name: &'a str,
    ) -> Vec<(Cow<'a, str>, Cow<'a, str>)> {
        match *ty {
            Type::Primitive(..) => {
                vec![(param_name.into(), param_name.into())]
            }
            Type::Opaque(ref op) if op.is_optional() => {
                vec![(param_name.into(), format!("{param_name} ? {param_name}->AsFFI() : nullptr").into())]
            }
            Type::Opaque(..) => {
                vec![(param_name.into(), format!("{param_name}.AsFFI()").into())]
            }
            Type::Struct(..) => {
                vec![(param_name.into(), format!("{param_name}.AsFFI()").into())]
            }
            Type::Enum(..) => {
                vec![(param_name.into(), format!("{param_name}.AsFFI()").into())]
            }
            Type::Slice(hir::Slice::Str(..)) => {
                // TODO: This needs to change if an abstraction other than std::string_view is used
                vec![
                    (format!("{}_data", param_name).into(), format!("{param_name}.data()").into()),
                    (format!("{}_size", param_name).into(), format!("{param_name}.size()").into()),
                ]
            }
            Type::Slice(hir::Slice::Primitive(..)) => {
                // TODO: This needs to change if an abstraction other than std::span is used
                vec![
                    (format!("{}_data", param_name).into(), format!("{param_name}.data()").into()),
                    (format!("{}_size", param_name).into(), format!("{param_name}.size()").into()),
                ]
            }
        }
    }

    fn gen_return_ty_name(&mut self, ty: &ReturnFallability) -> Cow<'ccx, str> {
        match *ty {
            ReturnFallability::Infallible(None) => "void".into(),
            ReturnFallability::Infallible(Some(ref ty)) => match ty {
                ReturnType::Writeable => self.cx.formatter.fmt_owned_str(),
                ReturnType::OutType(o) => self.gen_ty_name(o),
            },
            ReturnFallability::Fallible(ref ok, ref err) => {
                let ok_type_name = match ok {
                    Some(ReturnType::Writeable) => self.cx.formatter.fmt_owned_str(),
                    None => "std::monostate".into(),
                    Some(ReturnType::OutType(o)) => self.gen_ty_name(o),
                };
                let err_type_name = match err {
                    Some(o) => self.gen_ty_name(o),
                    None => "std::monostate".into(),
                };
                let ret: Cow<str> =
                    format!("diplomat::result<{ok_type_name}, {err_type_name}>").into();
                ret
            }
        }
    }

    fn gen_c_to_cpp<'a, P: TyPosition>(&self, ty: &Type<P>, var_name: &'a str) -> Cow<'a, str> {
        match *ty {
            Type::Primitive(..) => var_name.into(),
            Type::Opaque(ref op) if op.owner.is_owned() => {
                let op_id = op.tcx_id.into();
                let ty_name = self.cx.formatter.fmt_type_name(op_id);
                // TODO: Add imports?
                format!("std::unique_ptr<{ty_name}>({ty_name}::FromFFI({var_name}))").into()
            }
            Type::Opaque(ref op) if op.is_optional() => {
                let op_id = op.tcx_id.into();
                let ty_name = self.cx.formatter.fmt_type_name(op_id);
                // TODO: Add imports?
                format!("{var_name} ? {{ *{ty_name}::FromFFI({var_name}) }} : std::nullopt")
                    .into()
            }
            Type::Opaque(ref op) => {
                let op_id = op.tcx_id.into();
                let ty_name = self.cx.formatter.fmt_type_name(op_id);
                // TODO: Add imports?
                format!("*{ty_name}::FromFFI({var_name})").into()
            }
            Type::Struct(ref st) => {
                let id = P::id_for_path(&st);
                let ty_name = self.cx.formatter.fmt_type_name(id);
                // TODO: Add imports?
                format!("{ty_name}::FromFFI({var_name})").into()
            }
            Type::Enum(ref e) => {
                let id = e.tcx_id.into();
                let ty_name = self.cx.formatter.fmt_type_name(id);
                // TODO: Add imports?
                format!("{ty_name}::FromFFI({var_name})").into()
            }
            Type::Slice(hir::Slice::Str(..)) => {
                // TODO: This needs to change if an abstraction other than std::string_view is used
                let string_view = self.cx.formatter.fmt_borrowed_str();
                format!("{string_view}({var_name}_data, {var_name}_size)").into()
            }
            Type::Slice(hir::Slice::Primitive(b, p)) => {
                // TODO: This needs to change if an abstraction other than std::span is used
                let prim_name = self.cx.formatter.fmt_primitive_as_c(p);
                let span = self.cx.formatter.fmt_borrowed_slice(&prim_name, b.mutability);
                format!("{span}({var_name}_data, {var_name}_size)").into()
            }
        }
    }

    fn gen_fallible_c_to_cpp<'a>(&mut self, result_ty: &ReturnFallability, var_name: &'a str) -> Option<Cow<'a, str>> {
        match *result_ty {
            ReturnFallability::Infallible(None) => None,
            ReturnFallability::Infallible(Some(ReturnType::Writeable)) => {
                Some(
                    "/* TODO: Writeable conversion */".into()
                )
            },
            ReturnFallability::Infallible(Some(ReturnType::OutType(ref out_ty))) => {
                Some(
                    self.gen_c_to_cpp(out_ty, var_name)
                )
            },
            ReturnFallability::Fallible(ref ok, ref err) => {
                let ok_path = format!("{var_name}.ok");
                let err_path = format!("{var_name}.err");
                let ok_ty_name = match ok {
                    Some(ReturnType::Writeable) => self.cx.formatter.fmt_owned_str(),
                    None => "std::monostate".into(),
                    Some(ReturnType::OutType(o)) => self.gen_ty_name(o),
                };
                let err_ty_name = match err {
                    Some(o) => self.gen_ty_name(o),
                    None => "std::monostate".into(),
                };
                let ok_conversion = match ok {
                    Some(ReturnType::Writeable) => "/* TODO: Writeable conversion */".into(),
                    None => "".into(),
                    Some(ReturnType::OutType(o)) => self.gen_c_to_cpp(o, &ok_path),
                };
                let err_conversion = match err {
                    Some(o) => self.gen_c_to_cpp(o, &err_path),
                    None => "".into(),
                };
                Some(
                    format!("{var_name}.is_ok ? diplomat::result<{ok_ty_name}, {err_ty_name}>(diplomat::Ok<{ok_ty_name}>({ok_conversion})) : diplomat::result<{ok_ty_name}, {err_ty_name}>(diplomat::Err<{err_ty_name}>({err_conversion}))").into()
                )
            }
        }
    }
}
