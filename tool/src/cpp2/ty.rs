use super::header::{Forward, Header};
use super::Cpp2Context;
use super::Cpp2Formatter;
use askama::Template;
use diplomat_core::hir::{
    self, Mutability, OpaqueOwner, ReturnType, SelfType, SuccessType, TyPosition, Type, TypeDef,
    TypeId,
};
use std::borrow::Cow;

impl<'tcx> super::Cpp2Context<'tcx> {
    pub fn gen_ty(&self, id: TypeId, ty: TypeDef<'tcx>) {
        if ty.attrs().disable {
            // Skip type if disabled
            return;
        }
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
        let guard = self.errors.set_context_ty(ty.name().as_str().into());
        match ty {
            TypeDef::Enum(o) => context.gen_enum_def(o, id),
            TypeDef::Opaque(o) => context.gen_opaque_def(o, id),
            TypeDef::Struct(s) => context.gen_struct_def(s, id),
            TypeDef::OutStruct(s) => context.gen_struct_def(s, id),
            _ => unreachable!("unknown AST/HIR variant"),
        }
        drop(guard);

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

/// An expression associated with a variable name having the given suffix.
struct PartiallyNamedExpression<'a> {
    suffix: Cow<'a, str>,
    expression: Cow<'a, str>,
}

/// A type name with a corresponding variable name, such as a struct field or a function parameter.
struct NamedType<'a> {
    var_name: Cow<'a, str>,
    type_name: Cow<'a, str>,
}

/// Everything needed for rendering a method.
struct MethodInfo<'a> {
    /// HIR of the method being rendered
    method: &'a hir::Method,
    /// The C++ return type
    return_ty: Cow<'a, str>,
    /// The C++ method name
    method_name: Cow<'a, str>,
    /// The C method name
    c_method_name: Cow<'a, str>,
    /// Qualifiers for the function that come before the declaration (like "static")
    pre_qualifiers: Vec<Cow<'a, str>>,
    /// Qualifiers for the function that come after the declaration (like "const")
    post_qualifiers: Vec<Cow<'a, str>>,
    /// Type declarations for the C++ parameters
    param_decls: Vec<NamedType<'a>>,
    /// C++ conversion code for each parameter of the C function
    cpp_to_c_params: Vec<Cow<'a, str>>,
    /// If the function has a return value, the C++ code for the conversion. Assumes that
    /// the C function return value is saved to a variable named `result` or that the
    /// writeable, if present, is saved to a variable named `output`.
    c_to_cpp_return_expression: Option<Cow<'a, str>>,
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

        let methods = ty
            .methods
            .iter()
            .flat_map(|method| self.gen_method_info(id, method))
            .collect::<Vec<_>>();

        #[derive(Template)]
        #[template(path = "cpp2/enum_decl.h.jinja", escape = "none")]
        struct DeclTemplate<'a> {
            ty: &'a hir::EnumDef,
            fmt: &'a Cpp2Formatter<'a>,
            type_name: &'a str,
            ctype: &'a str,
            methods: &'a [MethodInfo<'a>],
        }

        DeclTemplate {
            ty,
            fmt: &self.cx.formatter,
            type_name: &type_name,
            ctype: &ctype,
            methods: methods.as_slice(),
        }
        .render_into(self.decl_header)
        .unwrap();

        #[derive(Template)]
        #[template(path = "cpp2/enum_impl.h.jinja", escape = "none")]
        struct ImplTemplate<'a> {
            ty: &'a hir::EnumDef,
            fmt: &'a Cpp2Formatter<'a>,
            type_name: &'a str,
            ctype: &'a str,
            methods: &'a [MethodInfo<'a>],
        }

        ImplTemplate {
            ty,
            fmt: &self.cx.formatter,
            type_name: &type_name,
            ctype: &ctype,
            methods: methods.as_slice(),
        }
        .render_into(self.impl_header)
        .unwrap();

        self.decl_header
            .includes
            .insert(self.cx.formatter.fmt_c_decl_header_path(id));
    }

    pub fn gen_opaque_def(&mut self, ty: &'tcx hir::OpaqueDef, id: TypeId) {
        let type_name = self.cx.formatter.fmt_type_name(id);
        let ctype = self.cx.formatter.fmt_c_name(&type_name);

        let methods = ty
            .methods
            .iter()
            .flat_map(|method| self.gen_method_info(id, method))
            .collect::<Vec<_>>();

        #[derive(Template)]
        #[template(path = "cpp2/opaque_decl.h.jinja", escape = "none")]
        struct DeclTemplate<'a> {
            // ty: &'a hir::OpaqueDef,
            fmt: &'a Cpp2Formatter<'a>,
            type_name: &'a str,
            ctype: &'a str,
            methods: &'a [MethodInfo<'a>],
        }

        DeclTemplate {
            // ty,
            fmt: &self.cx.formatter,
            type_name: &type_name,
            ctype: &ctype,
            methods: methods.as_slice(),
        }
        .render_into(self.decl_header)
        .unwrap();

        #[derive(Template)]
        #[template(path = "cpp2/opaque_impl.h.jinja", escape = "none")]
        struct ImplTemplate<'a> {
            // ty: &'a hir::OpaqueDef,
            fmt: &'a Cpp2Formatter<'a>,
            type_name: &'a str,
            ctype: &'a str,
            methods: &'a [MethodInfo<'a>],
        }

        ImplTemplate {
            // ty,
            fmt: &self.cx.formatter,
            type_name: &type_name,
            ctype: &ctype,
            methods: methods.as_slice(),
        }
        .render_into(self.impl_header)
        .unwrap();

        self.decl_header
            .includes
            .insert(self.cx.formatter.fmt_c_decl_header_path(id));
    }

    pub fn gen_struct_def<P: TyPosition>(&mut self, def: &'tcx hir::StructDef<P>, id: TypeId) {
        let type_name = self.cx.formatter.fmt_type_name(id);
        let ctype = self.cx.formatter.fmt_c_name(&type_name);

        let field_decls = def
            .fields
            .iter()
            .map(|field| self.gen_ty_decl(&field.ty, field.name.as_str()))
            .collect::<Vec<_>>();

        let cpp_to_c_fields = def
            .fields
            .iter()
            .flat_map(|field| self.gen_cpp_to_c_for_field("", field))
            .collect::<Vec<_>>();

        let c_to_cpp_fields = def
            .fields
            .iter()
            .map(|field| self.gen_c_to_cpp_for_field("c_struct.", field))
            .collect::<Vec<_>>();

        let methods = def
            .methods
            .iter()
            .flat_map(|method| self.gen_method_info(id, method))
            .collect::<Vec<_>>();

        #[derive(Template)]
        #[template(path = "cpp2/struct_decl.h.jinja", escape = "none")]
        struct DeclTemplate<'a> {
            // ty: &'a hir::OpaqueDef,
            // fmt: &'a Cpp2Formatter<'a>,
            type_name: &'a str,
            ctype: &'a str,
            fields: &'a [NamedType<'a>],
            methods: &'a [MethodInfo<'a>],
        }

        DeclTemplate {
            // ty,
            // fmt: &self.cx.formatter,
            type_name: &type_name,
            ctype: &ctype,
            fields: field_decls.as_slice(),
            methods: methods.as_slice(),
        }
        .render_into(self.decl_header)
        .unwrap();

        #[derive(Template)]
        #[template(path = "cpp2/struct_impl.h.jinja", escape = "none")]
        struct ImplTemplate<'a> {
            // ty: &'a hir::OpaqueDef,
            // fmt: &'a Cpp2Formatter<'a>,
            type_name: &'a str,
            ctype: &'a str,
            cpp_to_c_fields: &'a [NamedExpression<'a>],
            c_to_cpp_fields: &'a [NamedExpression<'a>],
            methods: &'a [MethodInfo<'a>],
        }

        ImplTemplate {
            // ty,
            // fmt: &self.cx.formatter,
            type_name: &type_name,
            ctype: &ctype,
            cpp_to_c_fields: cpp_to_c_fields.as_slice(),
            c_to_cpp_fields: c_to_cpp_fields.as_slice(),
            methods: methods.as_slice(),
        }
        .render_into(self.impl_header)
        .unwrap();
    }

    fn gen_method_info(
        &mut self,
        id: TypeId,
        method: &'tcx hir::Method,
    ) -> Option<MethodInfo<'ccx>> {
        if method.attrs.disable {
            return None;
        }
        let _guard = self.cx.errors.set_context_method(
            self.cx.formatter.fmt_type_name_diagnostics(id),
            method.name.as_str().into(),
        );
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
            let conversions = self.gen_cpp_to_c_for_type(&param.ty, param.name.as_str().into());
            cpp_to_c_params.extend(
                conversions
                    .into_iter()
                    .map(|PartiallyNamedExpression { expression, .. }| expression),
            );
        }

        if method.is_writeable() {
            cpp_to_c_params.push("&writeable".into());
        }

        let return_ty = self.gen_cpp_return_type_name(&method.output);

        let c_to_cpp_return_expression: Option<Cow<str>> =
            self.gen_c_to_cpp_for_return_type(&method.output, "result".into());

        let pre_qualifiers = if method.param_self.is_none() {
            vec!["static".into()]
        } else {
            vec![]
        };

        let post_qualifiers = match &method.param_self {
            Some(param_self) if param_self.ty.is_immutably_borrowed() => vec!["const".into()],
            Some(_) => vec![],
            None => vec![],
        };

        Some(MethodInfo {
            method,
            return_ty,
            method_name,
            c_method_name,
            pre_qualifiers,
            post_qualifiers,
            param_decls,
            cpp_to_c_params,
            c_to_cpp_return_expression,
        })
    }

    /// Generates C++ code for referencing a particular type with a given name.
    fn gen_ty_decl<'a, P: TyPosition>(&mut self, ty: &Type<P>, var_name: &'a str) -> NamedType<'a>
    where
        'ccx: 'a,
    {
        let var_name = self.cx.formatter.fmt_param_name(var_name);
        let type_name = self.gen_type_name(ty);

        NamedType {
            var_name,
            type_name,
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

                if self.cx.tcx.resolve_type(op_id).attrs().disable {
                    self.cx
                        .errors
                        .push_error(format!("Found usage of disabled type {type_name}"))
                }
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
                let type_name = self.cx.formatter.fmt_type_name(id);
                if self.cx.tcx.resolve_type(id).attrs().disable {
                    self.cx
                        .errors
                        .push_error(format!("Found usage of disabled type {type_name}"))
                }
                self.decl_header.forwards.insert(Forward::Struct(
                    self.cx.formatter.fmt_type_name(id).into_owned(),
                ));
                self.decl_header
                    .includes
                    .insert(self.cx.formatter.fmt_decl_header_path(id));
                self.impl_header
                    .includes
                    .insert(self.cx.formatter.fmt_impl_header_path(id));
                type_name
            }
            Type::Enum(ref e) => {
                let id = e.tcx_id.into();
                let type_name = self.cx.formatter.fmt_type_name(id);
                if self.cx.tcx.resolve_type(id).attrs().disable {
                    self.cx
                        .errors
                        .push_error(format!("Found usage of disabled type {type_name}"))
                }
                self.decl_header.forwards.insert(Forward::EnumStruct(
                    self.cx.formatter.fmt_type_name(id).into_owned(),
                ));
                self.decl_header
                    .includes
                    .insert(self.cx.formatter.fmt_decl_header_path(id));
                self.impl_header
                    .includes
                    .insert(self.cx.formatter.fmt_impl_header_path(id));
                type_name
            }
            Type::Slice(hir::Slice::Str(
                _,
                hir::StringEncoding::UnvalidatedUtf8 | hir::StringEncoding::Utf8,
            )) => self.cx.formatter.fmt_borrowed_utf8_str(),
            Type::Slice(hir::Slice::Str(_, hir::StringEncoding::UnvalidatedUtf16)) => {
                self.cx.formatter.fmt_borrowed_utf16_str()
            }
            Type::Slice(hir::Slice::Primitive(b, p)) => {
                let ret = self.cx.formatter.fmt_primitive_as_c(p);
                let ret = self.cx.formatter.fmt_borrowed_slice(&ret, b.mutability);
                ret.into_owned().into()
            }
            _ => unreachable!("unknown AST/HIR variant"),
        }
    }

    /// Generates a C++ expression that converts from the C++ self type to the corresponding C self type.
    fn gen_cpp_to_c_self(&self, ty: &SelfType) -> Cow<'static, str> {
        match *ty {
            SelfType::Opaque(..) => "this->AsFFI()".into(),
            SelfType::Struct(..) => "this->AsFFI()".into(),
            SelfType::Enum(..) => "this->AsFFI()".into(),
            _ => unreachable!("unknown AST/HIR variant"),
        }
    }

    /// Generates one or two C++ expressions that convert from a C++ field to the corresponding C field.
    ///
    /// Returns `NamedExpression`s whose `var_name` corresponds to the field of the C struct.
    ///
    /// `cpp_struct_access` should be code for referencing a field of the C++ struct.
    fn gen_cpp_to_c_for_field<'a, P: TyPosition>(
        &self,
        cpp_struct_access: &str,
        field: &'a hir::StructField<P>,
    ) -> Vec<NamedExpression<'a>> {
        let var_name = self.cx.formatter.fmt_param_name(field.name.as_str());
        let field_getter = format!("{cpp_struct_access}{var_name}");
        self.gen_cpp_to_c_for_type(&field.ty, field_getter.into())
            .into_iter()
            .map(
                |PartiallyNamedExpression { suffix, expression }| NamedExpression {
                    var_name: format!("{var_name}{suffix}").into(),
                    expression,
                },
            )
            .collect()
    }

    /// Generates one or two C++ expressions that convert from a C++ type to the corresponding C type.
    ///
    /// Returns `PartiallyNamedExpression`s whose `suffix` is either empty, `_data`, or `_size` for
    /// referencing fields of the C struct.
    fn gen_cpp_to_c_for_type<'a, P: TyPosition>(
        &self,
        ty: &Type<P>,
        cpp_name: Cow<'a, str>,
    ) -> Vec<PartiallyNamedExpression<'a>> {
        match *ty {
            Type::Primitive(..) => {
                vec![PartiallyNamedExpression {
                    suffix: "".into(),
                    expression: cpp_name.clone(),
                }]
            }
            Type::Opaque(ref op) if op.is_optional() => {
                vec![PartiallyNamedExpression {
                    suffix: "".into(),
                    expression: format!("{cpp_name} ? {cpp_name}->AsFFI() : nullptr").into(),
                }]
            }
            Type::Opaque(..) => {
                vec![PartiallyNamedExpression {
                    suffix: "".into(),
                    expression: format!("{cpp_name}.AsFFI()").into(),
                }]
            }
            Type::Struct(..) => {
                vec![PartiallyNamedExpression {
                    suffix: "".into(),
                    expression: format!("{cpp_name}.AsFFI()").into(),
                }]
            }
            Type::Enum(..) => {
                vec![PartiallyNamedExpression {
                    suffix: "".into(),
                    expression: format!("{cpp_name}.AsFFI()").into(),
                }]
            }
            Type::Slice(hir::Slice::Str(..)) | Type::Slice(hir::Slice::Primitive(..)) => {
                vec![
                    PartiallyNamedExpression {
                        suffix: "_data".into(),
                        expression: format!("{cpp_name}.data()").into(),
                    },
                    PartiallyNamedExpression {
                        suffix: "_size".into(),
                        expression: format!("{cpp_name}.size()").into(),
                    },
                ]
            }
            _ => unreachable!("unknown AST/HIR variant"),
        }
    }

    /// Generates the C++ type name of a return type.
    fn gen_cpp_return_type_name(&mut self, result_ty: &ReturnType) -> Cow<'ccx, str> {
        match *result_ty {
            ReturnType::Infallible(None) => "void".into(),
            ReturnType::Infallible(Some(ref ty)) => match ty {
                SuccessType::Writeable => self.cx.formatter.fmt_owned_str(),
                SuccessType::OutType(o) => self.gen_type_name(o),
                &_ => unreachable!("unknown AST/HIR variant"),
            },
            ReturnType::Fallible(ref ok, ref err) => {
                let ok_type_name = match ok {
                    Some(SuccessType::Writeable) => self.cx.formatter.fmt_owned_str(),
                    None => "std::monostate".into(),
                    Some(SuccessType::OutType(o)) => self.gen_type_name(o),
                    &Some(_) => unreachable!("unknown AST/HIR variant"),
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

    /// Generates a C++ expression that converts from a C field to the corresponding C++ field.
    ///
    /// `c_struct_access` should be code for referencing a field of the C struct.
    fn gen_c_to_cpp_for_field<'a, P: TyPosition>(
        &self,
        c_struct_access: &str,
        field: &'a hir::StructField<P>,
    ) -> NamedExpression<'a> {
        let var_name = self.cx.formatter.fmt_param_name(field.name.as_str());
        let field_getter = format!("{c_struct_access}{var_name}");
        let expression = self.gen_c_to_cpp_for_type(&field.ty, field_getter.into());
        NamedExpression {
            var_name,
            expression,
        }
    }

    /// Generates a C++ expression that converts from a C type to the corresponding C++ type.
    ///
    /// If the type is a slice, this function assumes that `{var_name}_data` and `{var_name}_size` resolve
    /// to valid expressions referencing the two different C variables for the pointer and the length.
    fn gen_c_to_cpp_for_type<'a, P: TyPosition>(
        &self,
        ty: &Type<P>,
        var_name: Cow<'a, str>,
    ) -> Cow<'a, str> {
        match *ty {
            Type::Primitive(..) => var_name,
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
            Type::Slice(hir::Slice::Str(
                _,
                hir::StringEncoding::UnvalidatedUtf8 | hir::StringEncoding::Utf8,
            )) => {
                let string_view = self.cx.formatter.fmt_borrowed_utf8_str();
                format!("{string_view}({var_name}_data, {var_name}_size)").into()
            }
            Type::Slice(hir::Slice::Str(_, hir::StringEncoding::UnvalidatedUtf16)) => {
                let string_view = self.cx.formatter.fmt_borrowed_utf16_str();
                format!("{string_view}({var_name}_data, {var_name}_size)").into()
            }
            Type::Slice(hir::Slice::Primitive(b, p)) => {
                let prim_name = self.cx.formatter.fmt_primitive_as_c(p);
                let span = self
                    .cx
                    .formatter
                    .fmt_borrowed_slice(&prim_name, b.mutability);
                format!("{span}({var_name}_data, {var_name}_size)").into()
            }
            _ => unreachable!("unknown AST/HIR variant"),
        }
    }

    /// Generates a C++ expression that converts from a C return type to the corresponding C++ return type.
    ///
    /// If the type is `Writeable`, this function assumes that there is a variable named `output` in scope.
    fn gen_c_to_cpp_for_return_type<'a>(
        &mut self,
        result_ty: &ReturnType,
        var_name: Cow<'a, str>,
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
                    &Some(_) => unreachable!("unknown AST/HIR variant"),
                };
                let err_type_name = match err {
                    Some(o) => self.gen_type_name(o),
                    None => "std::monostate".into(),
                };
                let ok_conversion = match ok {
                    // Note: the `output` variable is a string initialized in the template
                    Some(SuccessType::Writeable) => "std::move(output)".into(),
                    None => "".into(),
                    Some(SuccessType::OutType(o)) => self.gen_c_to_cpp_for_type(o, ok_path.into()),
                    &Some(_) => unreachable!("unknown AST/HIR variant"),
                };
                let err_conversion = match err {
                    Some(o) => self.gen_c_to_cpp_for_type(o, err_path.into()),
                    None => "".into(),
                };
                Some(
                    format!("{var_name}.is_ok ? diplomat::result<{ok_type_name}, {err_type_name}>(diplomat::Ok<{ok_type_name}>({ok_conversion})) : diplomat::result<{ok_type_name}, {err_type_name}>(diplomat::Err<{err_type_name}>({err_conversion}))").into()
                )
            }
            ReturnType::Infallible(Some(_)) => unreachable!("unknown AST/HIR variant"),
        }
    }
}
