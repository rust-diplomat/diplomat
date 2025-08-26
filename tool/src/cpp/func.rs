use std::borrow::Cow;

use askama::Template;
use diplomat_core::hir::{self, SelfType, StructPathLike, SymbolId, Type};

use crate::c::Header as C2Header;
use crate::c::CAPI_NAMESPACE;
use crate::cpp::{header::Header, ty::NamedType, TyGenContext};
use crate::filters;

/// We generate a pair of methods for writeables, one which returns a std::string
/// and one which operates on a WriteTrait
pub(super) struct MethodWriteableInfo<'a> {
    /// The method name. Usually `{}_write()`, but could potentially
    /// be made customizeable
    pub(super) method_name: Cow<'a, str>,
    /// The return type for the method without the std::string
    pub(super) return_ty: Cow<'a, str>,
    pub(super) c_to_cpp_return_expression: Option<Cow<'a, str>>,
}

/// Everything needed for rendering a method.
pub(super) struct MethodInfo<'a> {
    /// HIR of the method being rendered
    pub(super) method: &'a hir::Method,
    /// The C++ return type
    pub(super) return_ty: Cow<'a, str>,
    /// The C++ method name
    pub(super) method_name: Cow<'a, str>,
    /// The C method name
    pub(super) abi_name: String,
    /// Qualifiers for the function that come before the declaration (like "static")
    pub(super) pre_qualifiers: Vec<Cow<'a, str>>,
    /// Qualifiers for the function that come after the declaration (like "const")
    pub(super) post_qualifiers: Vec<Cow<'a, str>>,
    /// Type declarations for the C++ parameters
    pub(super) param_decls: Vec<NamedType<'a>>,
    /// Parameter validations, such as string checks
    pub(super) param_validations: Vec<String>,
    /// Conversion code from C++ to C, used to fill out cpp_to_c_params before a call. Used for converting clones of structs to references.
    pub(super) param_pre_conversions: Vec<String>,
    /// C++ conversion code for each parameter of the C function
    pub(super) cpp_to_c_params: Vec<Cow<'a, str>>,
    /// Conversion code of params from C to C++, grabbing the results of cpp_to_c_params and converting them into something C++ friendly. Used for converting references to clones of structs.
    pub(super) param_post_conversions: Vec<String>,
    /// If the function has a return value, the C++ code for the conversion. Assumes that
    /// the C function return value is saved to a variable named `result` or that the
    /// DiplomatWrite, if present, is saved to a variable named `output`.
    pub(super) c_to_cpp_return_expression: Option<Cow<'a, str>>,

    /// If the method returns a writeable, the info for that
    pub(super) writeable_info: Option<MethodWriteableInfo<'a>>,
    pub(super) docs: String,
}

#[derive(Template, Default)]
#[template(path = "cpp/free_functions/func_block_impl.h.jinja", escape = "none")]
/// Header for the implementation of a block of functions.
struct ImplTemplate {
    namespace: Option<String>,
    methods: Vec<String>,
    c_header: C2Header,
}

#[derive(Template, Default)]
#[template(path = "cpp/free_functions/func_block_decl.h.jinja", escape = "none")]
/// Header for the definition of a block of function.s
struct DeclTemplate {
    namespace: Option<String>,
    methods: Vec<String>,
    c_header: C2Header,
}

/// Helper for rendering function block information to [`Header`]s
/// Used either for creating blocks of functions that belong to structs, or for free functions that belong to no structs.
pub struct FuncGenContext<'tcx> {
    pub impl_header: Header,
    pub decl_header: Header,
    c: crate::c::FuncGenContext<'tcx>,
    impl_template: ImplTemplate,
    decl_template: DeclTemplate,
}

impl<'tcx> FuncGenContext<'tcx> {
    pub fn new(
        impl_header_path: String,
        decl_header_path: String,
        namespace: Option<String>,
        is_for_cpp: bool,
    ) -> Self {
        let decl_c_header = crate::c::Header::new(decl_header_path.clone(), is_for_cpp);
        FuncGenContext {
            c: crate::c::FuncGenContext::new(decl_c_header, is_for_cpp),
            impl_header: Header::new(impl_header_path),
            decl_header: Header::new(decl_header_path.clone()),
            impl_template: ImplTemplate {
                namespace: namespace.clone(),
                ..Default::default()
            },
            decl_template: DeclTemplate {
                c_header: crate::c::Header::new(decl_header_path.clone(), is_for_cpp),
                namespace,
                ..Default::default()
            },
        }
    }

    /// Generate a free function and prepare it for rendering to [`DeclTemplate`] and [`ImplTemplate`].
    pub fn generate_function<'b>(
        &mut self,
        func_id: hir::FunctionId,
        func: &'tcx hir::Method,
        context: &mut TyGenContext<'b, 'tcx, '_>,
    ) {
        let info = Self::gen_method_info(func_id.into(), func, context);

        #[derive(Template)]
        #[template(
            path = "cpp/function_defs/func_block_function.h.jinja",
            escape = "none"
        )]
        struct FunctionImpl<'a> {
            m: &'a MethodInfo<'a>,
            namespace: Option<String>,
        }

        #[derive(Template)]
        #[template(
            path = "cpp/function_defs/func_block_function_decl.h.jinja",
            escape = "none"
        )]
        struct FunctionDecl<'a> {
            m: &'a MethodInfo<'a>,
        }

        if let Some(m) = &info {
            let impl_bl = FunctionImpl {
                m,
                namespace: func.attrs.namespace.clone(),
            };
            self.impl_template.methods.push(impl_bl.to_string());

            let decl_bl = FunctionDecl { m };
            self.decl_template.methods.push(decl_bl.to_string());

            self.c.gen_method(func, &context.c);
        }
    }

    pub fn render(&mut self) -> Result<(), askama::Error> {
        self.c
            .render_into(None, None, &mut self.impl_template.c_header)
            .unwrap();
        self.impl_template.render_into(&mut self.impl_header)?;
        self.decl_template.render_into(&mut self.decl_header)?;
        Ok(())
    }

    pub(super) fn gen_method_info<'a, 'b>(
        id: SymbolId,
        method: &'a hir::Method,
        context: &mut TyGenContext<'b, 'a, '_>,
    ) -> Option<MethodInfo<'b>> {
        if method.attrs.disable {
            return None;
        }
        let _guard = context.errors.set_context_method(
            context.c.tcx.fmt_symbol_name_diagnostics(id),
            method.name.as_str().into(),
        );
        let method_name = context.formatter.fmt_method_name(method);
        let abi_name = match id {
            SymbolId::FunctionId(..) => context.formatter.namespace_func_name(method),
            SymbolId::TypeId(ty) => context
                .formatter
                .namespace_ty_name(ty, method.abi_name.as_str()),
            _ => panic!("Unsupported method generation for symbol ID {id:?}"),
        };
        let mut param_decls = Vec::new();
        let mut cpp_to_c_params = Vec::new();

        let mut param_pre_conversions = Vec::new();
        let mut param_post_conversions = Vec::new();

        if let Some(param_self) = method.param_self.as_ref() {
            // Convert the self parameter as normal:
            let conversion = context.gen_cpp_to_c_self(&param_self.ty);
            // If we happen to be a reference to a struct (and we can't just do a reinterpret_cast on the pointer),
            // Then we need to add some pre- and post- function call conversions to:
            // 1. Create `thisDiplomatRefClone` as the converted FFI friendly struct.
            // 2. Pass in the reference to `thisDiplomatRefClone`
            // 3. Assign `*this` to the value of `thisDiplomatRefClone`
            let conversion = if let hir::ParamSelf {
                ty: SelfType::Struct(ref s),
                ..
            } = param_self
            {
                let attrs = &s.resolve(context.c.tcx).attrs;
                if !s.owner.is_owned() && !attrs.abi_compatible {
                    param_pre_conversions
                        .push(format!("auto thisDiplomatRefClone = {conversion};"));

                    if s.owner.mutability().is_mutable() {
                        param_post_conversions.push(format!(
                            "*this = {}::FromFFI(thisDiplomatRefClone);",
                            context.formatter.fmt_type_name(s.id())
                        ));
                    }
                    "&thisDiplomatRefClone".to_string().into()
                } else {
                    conversion
                }
            } else {
                conversion
            };

            cpp_to_c_params.push(conversion);
        }

        let mut param_validations = Vec::new();
        let mut returns_utf8_err = false;

        let namespace = match id {
            SymbolId::FunctionId(..) => method.attrs.namespace.clone(),
            SymbolId::TypeId(ty) => context.c.tcx.resolve_type(ty).attrs().namespace.clone(),
            _ => panic!("Unsupported SymbolId: {id:?}"),
        };

        for param in method.params.iter() {
            let decls = context.gen_ty_decl(&param.ty, param.name.as_str());
            let param_name = decls.var_name.clone();
            param_decls.push(decls);
            if matches!(
                param.ty,
                Type::Slice(hir::Slice::Str(_, hir::StringEncoding::Utf8))
            ) {
                param_validations.push(format!(
                    "if (!diplomat::capi::diplomat_is_str({param_name}.data(), {param_name}.size())) {{\n  return diplomat::Err<diplomat::Utf8Error>();\n}}",
                ));
                returns_utf8_err = true;
            }

            let conversion = context.gen_cpp_to_c_for_type(
                &param.ty,
                param_name,
                Some(method.abi_name.to_string()),
                namespace.clone(),
            );
            // If we happen to be a reference to a struct (and we can't just do a reinterpret_cast on the pointer),
            // Then we need to add some pre- and post- function call conversions to:
            // 1. Create `varNameDiplomatRefClone` as the converted FFI friendly struct.
            // 2. Pass in the reference to `varNameDiplomatRefClone`
            // 3. Assign `varName` to the value of `varNameDiplomatRefClone`
            let conversion = if let hir::Param {
                ty: hir::Type::Struct(ref s),
                ..
            } = param
            {
                let attrs = &s.resolve(context.c.tcx).attrs;
                if !s.owner.is_owned() && !attrs.abi_compatible {
                    param_pre_conversions.push(format!(
                        "auto {}DiplomatRefClone = {};",
                        param.name, conversion
                    ));

                    if s.owner.mutability().is_mutable() {
                        param_post_conversions.push(format!(
                            "{} = {}::FromFFI({}DiplomatRefClone);",
                            param.name,
                            context.formatter.fmt_type_name(s.id()),
                            param.name
                        ));
                    }
                    format!("&{}DiplomatRefClone", param.name).into()
                } else {
                    conversion
                }
            } else {
                conversion
            };

            cpp_to_c_params.push(conversion);
        }

        /// The UTF8 errors are added in by the C++ backend when converting from C++
        /// types. We wrap them in another layer of diplomat::result.
        fn wrap_return_ty_and_expr_for_utf8(
            return_ty: &mut Cow<str>,
            c_to_cpp_return_expression: &mut Option<Cow<str>>,
        ) {
            if let Some(return_expr) = c_to_cpp_return_expression {
                *c_to_cpp_return_expression =
                    Some(format!("diplomat::Ok<{return_ty}>({return_expr})").into());
                *return_ty = format!("diplomat::result<{return_ty}, diplomat::Utf8Error>").into();
            } else {
                *c_to_cpp_return_expression = Some("diplomat::Ok<std::monostate>()".into());
                *return_ty = "diplomat::result<std::monostate, diplomat::Utf8Error>".into();
            }
        }

        let mut return_ty = context.gen_cpp_return_type_name(&method.output, false);

        let mut c_to_cpp_return_expression =
            context.gen_c_to_cpp_for_return_type(&method.output, "result".into(), false);

        if returns_utf8_err {
            wrap_return_ty_and_expr_for_utf8(&mut return_ty, &mut c_to_cpp_return_expression)
        };

        // If the return expression is a std::move, unwrap that, because the linter doesn't like it
        c_to_cpp_return_expression = c_to_cpp_return_expression.map(|expr| {
            if expr.starts_with("std::move") {
                expr["std::move(".len()..(expr.len() - 1)].to_owned().into()
            } else {
                expr
            }
        });

        // Writeable methods generate a `std::string foo()` and a
        // `template<typename W> void foo_write(W& writeable)` where `W` is a `WriteTrait`
        // implementor. The generic method needs its own return type and conversion code.
        let mut writeable_info = None;
        if method.output.is_write() {
            cpp_to_c_params.push("&write".into());
            let mut return_ty = context.gen_cpp_return_type_name(&method.output, true);

            let mut c_to_cpp_return_expression =
                context.gen_c_to_cpp_for_return_type(&method.output, "result".into(), true);
            if returns_utf8_err {
                wrap_return_ty_and_expr_for_utf8(&mut return_ty, &mut c_to_cpp_return_expression)
            };
            writeable_info = Some(MethodWriteableInfo {
                method_name: format!("{method_name}_write").into(),
                return_ty,
                c_to_cpp_return_expression,
            });
        }

        let pre_qualifiers =
            if method.param_self.is_none() && !matches!(id, SymbolId::FunctionId(..)) {
                vec!["static".into()]
            } else {
                vec![]
            };

        let post_qualifiers = match &method.param_self {
            Some(param_self)
                if param_self.ty.is_immutably_borrowed() || param_self.ty.is_consuming() =>
            {
                vec!["const".into()]
            }
            Some(_) => vec![],
            None => vec![],
        };

        Some(MethodInfo {
            method,
            return_ty,
            method_name,
            abi_name,
            pre_qualifiers,
            post_qualifiers,
            param_decls,
            param_pre_conversions,
            param_validations,
            param_post_conversions,
            cpp_to_c_params,
            c_to_cpp_return_expression,
            writeable_info,
            docs: context.formatter.fmt_docs(&method.docs),
        })
    }
}
