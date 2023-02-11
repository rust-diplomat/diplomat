use super::header::{Forward, Header};
use super::Cpp2Context;
use diplomat_core::hir::{
    self, Mutability, OpaqueOwner, ParamSelf, ReturnType, SelfType, SuccessType, TyPosition, Type,
    TypeDef, TypeId,
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

        // In some cases like generating decls for `self` parameters,
        // a header will get its own forwards and includes. Instead of
        // trying to avoid pushing them, it's cleaner to just pull them out
        // once done
        context.decl_header.forwards.remove(&*type_name);
        context.impl_header.forwards.remove(&*type_name);
        context.decl_header.includes.remove(&*decl_header_path);
        context.impl_header.includes.remove(&*impl_header_path);
        context.impl_header.includes.remove(&*decl_header_path);

        context.impl_header.decl_include = Some(decl_header_path.clone());

        let c_decl_header_path = self.formatter.fmt_c_decl_header_path(id);
        context.decl_header.includes.insert(c_decl_header_path);

        let c_impl_header_path = self.formatter.fmt_c_impl_header_path(id);
        context.impl_header.includes.insert(c_impl_header_path);

        self.files
            .add_file(decl_header_path, decl_header.to_string());
        self.files
            .add_file(impl_header_path, impl_header.to_string());
    }
}

/// An expression with a corresponding variable name, such as a struct field or a function parameter.
struct NamedExpression<'a> {
    var_name: Cow<'a, str>,
    expression: Cow<'a, str>,
}

/// A type name with a corresponding variable name, such as a struct field or a function parameter.
struct NamedType<'a> {
    var_name: Cow<'a, str>,
    type_name: Cow<'a, str>,
}

/// Context for generating a particular type's header
pub struct TyGenContext<'ccx, 'tcx, 'header> {
    pub cx: &'ccx Cpp2Context<'tcx>,
    pub impl_header: &'header mut Header,
    pub decl_header: &'header mut Header,
}

impl<'ccx, 'tcx: 'ccx, 'header> TyGenContext<'ccx, 'tcx, 'header> {
    /// Adds an enum definition to the current decl and impl headers.
    ///
    /// The enum is defined in C++ using a `class` with a single private field that is the
    /// C enum type. This enables us to add methods to the enum and generally make the enum
    /// behave more like an upgraded C++ type. We don't use `enum class` because methods
    /// cannot be added to it.
    pub fn gen_enum_def(&mut self, ty: &'tcx hir::EnumDef, id: TypeId) {
        let type_name = self.cx.formatter.fmt_type_name(id);
        let ctype = self.cx.formatter.fmt_c_name(&type_name);
        self.decl_header
            .includes
            .insert(self.cx.formatter.fmt_c_decl_header_path(id));
        write!(
            self.decl_header,
            "class {type_name} {{
\t{ctype} value;

public:
\tenum Value {{
"
        )
        .unwrap();
        write!(
            self.impl_header,
            "inline {type_name}::{type_name}({type_name}::Value cpp_value) {{
\tswitch (cpp_value) {{
"
        )
        .unwrap();
        for variant in ty.variants.iter() {
            let enum_variant = self.cx.formatter.fmt_enum_variant(variant);
            let c_enum_variant = self.cx.formatter.fmt_c_enum_variant(&type_name, variant);
            writeln!(self.decl_header, "\t\t{enum_variant},").unwrap();
            write!(
                self.impl_header,
                "\t\tcase {enum_variant}:
\t\t\tvalue = {c_enum_variant};
\t\t\tbreak;
"
            )
            .unwrap();
        }
        write!(
            self.decl_header,
            "\t}};

\tinline {type_name}({type_name}::Value cpp_value);
\tinline {type_name}({ctype} c_enum) : value(c_enum) {{}};
"
        )
        .unwrap();
        write!(
            self.impl_header,
            "\t\tdefault:
\t\t\tabort();
\t}}
}}
"
        )
        .unwrap();
        for method in ty.methods.iter() {
            self.gen_method(id, method);
        }
        write!(
            self.decl_header,
            "
\tinline {ctype} AsFFI() const;
\tinline static {type_name} FromFFI({ctype} c_enum);
}};\n\n"
        )
        .unwrap();
        write!(
            self.impl_header,
            "
inline {ctype} {type_name}::AsFFI() const {{
\treturn value;
}}

inline {type_name} {type_name}::FromFFI({ctype} c_enum) {{
\treturn {type_name}(c_enum);
}}
"
        )
        .unwrap();
    }

    pub fn gen_opaque_def(&mut self, ty: &'tcx hir::OpaqueDef, id: TypeId) {
        let type_name = self.cx.formatter.fmt_type_name(id);
        let const_ptr = self
            .cx
            .formatter
            .fmt_c_ptr(&type_name, Mutability::Immutable);
        let mut_ptr = self.cx.formatter.fmt_c_ptr(&type_name, Mutability::Mutable);
        let ctype = self.cx.formatter.fmt_c_name(&type_name);
        let const_cptr = self.cx.formatter.fmt_c_ptr(&ctype, Mutability::Immutable);
        let mut_cptr = self.cx.formatter.fmt_c_ptr(&ctype, Mutability::Mutable);
        let const_ref = self
            .cx
            .formatter
            .fmt_borrowed(&type_name, Mutability::Immutable);
        let move_ref = self.cx.formatter.fmt_move_ref(&type_name);
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
\tinline static {const_ptr} FromFFI({const_cptr} ptr);
\tinline static {mut_ptr} FromFFI({mut_cptr} ptr);
\tinline static void operator delete(void* ptr);
private:
\t{type_name}() = delete;
\t{type_name}({const_ref}) = delete;
\t{type_name}({move_ref}) noexcept = delete;
\t{type_name} operator=({const_ref}) = delete;
\t{type_name} operator=({move_ref}) noexcept = delete;
\tstatic void operator delete[](void*, size_t) = delete;
}};

"
        )
        .unwrap();
        write!(
            self.impl_header,
            "inline {const_cptr} {type_name}::AsFFI() const {{
\treturn reinterpret_cast<{const_cptr}>(this);
}}

inline {mut_cptr} {type_name}::AsFFI() {{
\treturn reinterpret_cast<{mut_cptr}>(this);
}}

inline {const_ptr} {type_name}::FromFFI({const_cptr} ptr) {{
\treturn reinterpret_cast<{const_ptr}>(ptr);
}}

inline {mut_ptr} {type_name}::FromFFI({mut_cptr} ptr) {{
\treturn reinterpret_cast<{mut_ptr}>(ptr);
}}

inline void {type_name}::operator delete(void* ptr) {{
\t{ctype}_destroy(reinterpret_cast<{mut_cptr}>(ptr));
}}

"
        )
        .unwrap();
    }

    pub fn gen_struct_def<P: TyPosition>(&mut self, def: &'tcx hir::StructDef<P>, id: TypeId) {
        let type_name = self.cx.formatter.fmt_type_name(id);
        let ctype = self.cx.formatter.fmt_c_name(&type_name);
        writeln!(self.decl_header, "struct {type_name} {{").unwrap();
        for field in def.fields.iter() {
            let NamedType {
                var_name,
                type_name,
            } = self.gen_ty_decl(&field.ty, field.name.as_str());
            writeln!(self.decl_header, "\t{type_name} {var_name};").unwrap();
        }
        for method in def.methods.iter() {
            self.gen_method(id, method);
        }
        write!(
            self.decl_header,
            "
\tinline {ctype} AsFFI() const;
\tinline static {type_name} FromFFI({ctype} c_struct);
}};\n\n"
        )
        .unwrap();
        write!(
            self.impl_header,
            "
inline {ctype} {type_name}::AsFFI() const {{
\treturn {ctype} {{
"
        )
        .unwrap();
        for field in def.fields.iter() {
            let param_name = self.cx.formatter.fmt_param_name(field.name.as_str());
            for NamedExpression {
                var_name,
                expression,
            } in self.gen_cpp_to_c_for_type(&field.ty, &param_name)
            {
                writeln!(self.impl_header, "\t\t.{var_name} = {expression},").unwrap();
            }
        }
        write!(
            self.impl_header,
            "\t}};
}}

inline {type_name} {type_name}::FromFFI({ctype} c_struct) {{
\treturn {type_name} {{
"
        )
        .unwrap();
        for field in def.fields.iter() {
            let param_name = self.cx.formatter.fmt_param_name(field.name.as_str());
            let field_getter = format!("c_struct.{param_name}");
            let conversion = self.gen_c_to_cpp_for_type(&field.ty, &field_getter);
            writeln!(self.impl_header, "\t\t.{param_name} = {conversion},").unwrap();
        }
        write!(
            self.impl_header,
            "\t}};
}}

"
        )
        .unwrap();
    }

    pub fn gen_method(&mut self, id: TypeId, method: &'tcx hir::Method) {
        let type_name = self.cx.formatter.fmt_type_name(id);
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
            let conversions = self.gen_cpp_to_c_for_type(&param.ty, param.name.as_str());
            cpp_to_c_params.extend(
                conversions
                    .into_iter()
                    .map(|named_expression| named_expression.expression),
            );
        }

        if method.is_writeable() {
            cpp_to_c_params.push("&writeable".into());
        }

        let return_ty = self.gen_cpp_return_type_name(&method.output);

        let return_statement: Cow<str> = self
            .gen_c_to_cpp_for_return_type(&method.output, "result")
            .map(|s| format!("\n\treturn {s};").into())
            .unwrap_or_else(|| "".into());

        let return_prefix = if method.output.returns_value() {
            "auto result = "
        } else {
            ""
        };

        let mut params = String::new();
        let mut first = true;
        for NamedType {
            var_name,
            type_name,
        } in param_decls
        {
            let comma = if first {
                first = false;
                ""
            } else {
                ", "
            };
            write!(&mut params, "{comma}{type_name} {var_name}").unwrap();
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
            "inline {return_ty} {type_name}::{method_name}({params}){qualifiers} {{
\t{writeable_prefix}{return_prefix}{c_method_name}({c_params});{return_statement}
}}

"
        )
        .unwrap();
    }

    /// Generates C++ code for referencing a particular type with a given name.
    fn gen_ty_decl<'a, P: TyPosition>(&mut self, ty: &Type<P>, var_name: &'a str) -> NamedType<'a>
    where
        'ccx: 'a,
    {
        let param_name = self.cx.formatter.fmt_param_name(var_name);
        let ty = self.gen_type_name(ty);
        NamedType {
            var_name: param_name,
            type_name: ty,
        }
    }

    /// Generates C++ code for referencing a particular type.
    ///
    /// This function adds the necessary type imports to the decl and impl files.
    fn gen_type_name<P: TyPosition>(&mut self, ty: &Type<P>) -> Cow<'ccx, str> {
        match *ty {
            Type::Primitive(prim) => self.cx.formatter.fmt_primitive_as_c(prim),
            Type::Opaque(ref op) => {
                let op_id = op.tcx_id.into();
                let type_name = self.cx.formatter.fmt_type_name(op_id);
                let mutability = op.owner.mutability().unwrap_or(hir::Mutability::Mutable);
                let ret = match (op.owner.is_owned(), op.is_optional()) {
                    // unique_ptr is nullable
                    (true, _) => self.cx.formatter.fmt_owned(&type_name),
                    (false, true) => self
                        .cx
                        .formatter
                        .fmt_optional_borrowed(&type_name, mutability),
                    (false, false) => self.cx.formatter.fmt_borrowed(&type_name, mutability),
                };
                let ret = ret.into_owned().into();

                self.decl_header
                    .forwards
                    .insert(Forward::Class(type_name.into_owned()));
                self.impl_header
                    .includes
                    .insert(self.cx.formatter.fmt_impl_header_path(op_id));
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
                self.impl_header
                    .includes
                    .insert(self.cx.formatter.fmt_impl_header_path(id));
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
                self.impl_header
                    .includes
                    .insert(self.cx.formatter.fmt_impl_header_path(id));
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

    /// Generates a C++ expression that converts from the C++ self type to the corresponding C self type.
    fn gen_cpp_to_c_self(&self, ty: &SelfType) -> Cow<'static, str> {
        match *ty {
            SelfType::Opaque(..) => "this->AsFFI()".into(),
            SelfType::Struct(..) => todo!(),
            SelfType::Enum(..) => todo!(),
        }
    }

    /// Generates one or two C++ expressions that convert from a C++ type to the corresponding C type.
    ///
    /// If the type is a slice, this function assumes that `{var_name}_data` and `{var_name}_size` resolve
    /// to valid expressions referencing the two different C variables for the pointer and the length.
    fn gen_cpp_to_c_for_type<'a, P: TyPosition>(
        &self,
        ty: &Type<P>,
        var_name: &'a str,
    ) -> Vec<NamedExpression<'a>> {
        match *ty {
            Type::Primitive(..) => {
                vec![NamedExpression {
                    var_name: var_name.into(),
                    expression: var_name.into(),
                }]
            }
            Type::Opaque(ref op) if op.is_optional() => {
                vec![NamedExpression {
                    var_name: var_name.into(),
                    expression: format!("{var_name} ? {var_name}->AsFFI() : nullptr").into(),
                }]
            }
            Type::Opaque(..) => {
                vec![NamedExpression {
                    var_name: var_name.into(),
                    expression: format!("{var_name}.AsFFI()").into(),
                }]
            }
            Type::Struct(..) => {
                vec![NamedExpression {
                    var_name: var_name.into(),
                    expression: format!("{var_name}.AsFFI()").into(),
                }]
            }
            Type::Enum(..) => {
                vec![NamedExpression {
                    var_name: var_name.into(),
                    expression: format!("{var_name}.AsFFI()").into(),
                }]
            }
            Type::Slice(hir::Slice::Str(..)) => {
                // TODO: This needs to change if an abstraction other than std::string_view is used
                vec![
                    NamedExpression {
                        var_name: format!("{var_name}_data").into(),
                        expression: format!("{var_name}.data()").into(),
                    },
                    NamedExpression {
                        var_name: format!("{var_name}_size").into(),
                        expression: format!("{var_name}.size()").into(),
                    },
                ]
            }
            Type::Slice(hir::Slice::Primitive(..)) => {
                // TODO: This needs to change if an abstraction other than std::span is used
                vec![
                    NamedExpression {
                        var_name: format!("{var_name}_data").into(),
                        expression: format!("{var_name}.data()").into(),
                    },
                    NamedExpression {
                        var_name: format!("{var_name}_size").into(),
                        expression: format!("{var_name}.size()").into(),
                    },
                ]
            }
        }
    }

    /// Generates the C++ type name of a return type.
    fn gen_cpp_return_type_name(&mut self, result_ty: &ReturnType) -> Cow<'ccx, str> {
        match *result_ty {
            ReturnType::Infallible(None) => "void".into(),
            ReturnType::Infallible(Some(ref ty)) => match ty {
                SuccessType::Writeable => self.cx.formatter.fmt_owned_str(),
                SuccessType::OutType(o) => self.gen_type_name(o),
            },
            ReturnType::Fallible(ref ok, ref err) => {
                let ok_type_name = match ok {
                    Some(SuccessType::Writeable) => self.cx.formatter.fmt_owned_str(),
                    None => "std::monostate".into(),
                    Some(SuccessType::OutType(o)) => self.gen_type_name(o),
                };
                let err_type_name = match err {
                    Some(o) => self.gen_type_name(o),
                    None => "std::monostate".into(),
                };
                let ret: Cow<str> =
                    format!("diplomat::result<{ok_type_name}, {err_type_name}>").into();
                ret
            }
        }
    }

    /// Generates a C++ expression that converts from a C type to the corresponding C++ type.
    ///
    /// If the type is a slice, this function assumes that `{var_name}_data` and `{var_name}_size` resolve
    /// to valid expressions referencing the two different C variables for the pointer and the length.
    fn gen_c_to_cpp_for_type<'a, P: TyPosition>(
        &self,
        ty: &Type<P>,
        var_name: &'a str,
    ) -> Cow<'a, str> {
        match *ty {
            Type::Primitive(..) => var_name.into(),
            Type::Opaque(ref op) if op.owner.is_owned() => {
                let id = op.tcx_id.into();
                let type_name = self.cx.formatter.fmt_type_name(id);
                // Note: The impl file is imported in gen_type_name().
                format!("std::unique_ptr<{type_name}>({type_name}::FromFFI({var_name}))").into()
            }
            Type::Opaque(ref op) if op.is_optional() => {
                let id = op.tcx_id.into();
                let type_name = self.cx.formatter.fmt_type_name(id);
                // Note: The impl file is imported in gen_type_name().
                format!("{var_name} ? {{ *{type_name}::FromFFI({var_name}) }} : std::nullopt")
                    .into()
            }
            Type::Opaque(ref op) => {
                let id = op.tcx_id.into();
                let type_name = self.cx.formatter.fmt_type_name(id);
                // Note: The impl file is imported in gen_type_name().
                format!("*{type_name}::FromFFI({var_name})").into()
            }
            Type::Struct(ref st) => {
                let id = P::id_for_path(st);
                let type_name = self.cx.formatter.fmt_type_name(id);
                // Note: The impl file is imported in gen_type_name().
                format!("{type_name}::FromFFI({var_name})").into()
            }
            Type::Enum(ref e) => {
                let id = e.tcx_id.into();
                let type_name = self.cx.formatter.fmt_type_name(id);
                // Note: The impl file is imported in gen_type_name().
                format!("{type_name}::FromFFI({var_name})").into()
            }
            Type::Slice(hir::Slice::Str(..)) => {
                // TODO: This needs to change if an abstraction other than std::string_view is used
                let string_view = self.cx.formatter.fmt_borrowed_str();
                format!("{string_view}({var_name}_data, {var_name}_size)").into()
            }
            Type::Slice(hir::Slice::Primitive(b, p)) => {
                // TODO: This needs to change if an abstraction other than std::span is used
                let prim_name = self.cx.formatter.fmt_primitive_as_c(p);
                let span = self
                    .cx
                    .formatter
                    .fmt_borrowed_slice(&prim_name, b.mutability);
                format!("{span}({var_name}_data, {var_name}_size)").into()
            }
        }
    }

    /// Generates a C++ expression that converts from a C return type to the corresponding C++ return type.
    ///
    /// If the type is `Writeable`, this function assumes that there is a variable named `output` in scope.
    ///
    /// If the type is a slice, this function assumes that `{var_name}_data` and `{var_name}_size` resolve
    /// to valid expressions referencing the two different C variables for the pointer and the length.
    fn gen_c_to_cpp_for_return_type<'a>(
        &mut self,
        result_ty: &ReturnType,
        var_name: &'a str,
    ) -> Option<Cow<'a, str>> {
        match *result_ty {
            ReturnType::Infallible(None) => None,
            ReturnType::Infallible(Some(SuccessType::Writeable)) => Some("output".into()),
            ReturnType::Infallible(Some(SuccessType::OutType(ref out_ty))) => {
                Some(self.gen_c_to_cpp_for_type(out_ty, var_name))
            }
            ReturnType::Fallible(ref ok, ref err) => {
                let ok_path = format!("{var_name}.ok");
                let err_path = format!("{var_name}.err");
                let ok_type_name = match ok {
                    Some(SuccessType::Writeable) => self.cx.formatter.fmt_owned_str(),
                    None => "std::monostate".into(),
                    Some(SuccessType::OutType(o)) => self.gen_type_name(o),
                };
                let err_type_name = match err {
                    Some(o) => self.gen_type_name(o),
                    None => "std::monostate".into(),
                };
                let ok_conversion = match ok {
                    // Note: the `output` variable is a string initialized in gen_method
                    Some(SuccessType::Writeable) => "std::move(output)".into(),
                    None => "".into(),
                    Some(SuccessType::OutType(o)) => self.gen_c_to_cpp_for_type(o, &ok_path),
                };
                let err_conversion = match err {
                    Some(o) => self.gen_c_to_cpp_for_type(o, &err_path),
                    None => "".into(),
                };
                Some(
                    format!("{var_name}.is_ok ? diplomat::result<{ok_type_name}, {err_type_name}>(diplomat::Ok<{ok_type_name}>({ok_conversion})) : diplomat::result<{ok_type_name}, {err_type_name}>(diplomat::Err<{err_type_name}>({err_conversion}))").into()
                )
            }
        }
    }
}
