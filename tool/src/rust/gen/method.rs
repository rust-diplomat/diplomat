//! Rendering of one safe Rust method through an Askama template.
//!
//! HIR lowering and ABI conversion remain in Rust. This module builds the small
//! render model consumed by `templates/rust/method.rs.jinja`; it never writes
//! generated source directly.

use askama::Template;
use diplomat_core::hir::{
    self, DocsUrlGenerator, MaybeOwn, Mutability, OutType, PrimitiveType, ReturnType,
    ReturnableStructPath, SelfType, Slice, StringEncoding, StructPathLike, SuccessType, Type,
    TypeContext, TypeDef,
};

use super::opaque::opaque_return_expr;
use crate::r#rust::formatter::{method_name, opaque_name, render_docs};
use crate::r#rust::lifetimes::{lifetime_prefix, method_generics};
use crate::r#rust::type_map::{
    convert_from_ffi, convert_to_ffi, ffi_borrowed_slice_expr, is_owned_slice, safe_input_type,
    safe_return_type, struct_input_expr, struct_needs_abi_mirror, struct_output_expr,
};

#[derive(Template)]
#[template(path = "rust/method.rs.jinja", escape = "none")]
struct MethodTemplate<'a> {
    method: &'a MethodView,
}

/// All syntax needed to render one public method.
///
/// The strings in this view are already validated/lowered Rust fragments. The
/// template only controls their placement and indentation.
pub(super) struct MethodView {
    docs: String,
    name: String,
    generics: String,
    params: String,
    return_suffix: String,
    body: Vec<String>,
}

impl MethodView {
    pub(super) fn render(&self) -> String {
        MethodTemplate { method: self }
            .render()
            .expect("Rust method template rendering cannot fail")
    }
}

pub(super) fn render_method(
    type_lifetimes: usize,
    method: &hir::Method,
    tcx: &TypeContext,
    docs_url_gen: &DocsUrlGenerator,
) -> String {
    build_method_view(type_lifetimes, method, tcx, docs_url_gen).render()
}

fn build_method_view(
    type_lifetimes: usize,
    method: &hir::Method,
    tcx: &TypeContext,
    docs_url_gen: &DocsUrlGenerator,
) -> MethodView {
    let name = method_name(method);
    let generics = method_generics(method, type_lifetimes, tcx);
    let mut params = Vec::new();
    if let Some(param_self) = &method.param_self {
        // An opaque receiver is always a borrow of the handle. A value-struct receiver
        // is the value itself, or a borrow of it, and an enum receiver is always by
        // value — a value type has no indirection to describe.
        let self_param = match &param_self.ty {
            SelfType::Opaque(path) => {
                let borrow = path.borrowed();
                match borrow.mutability {
                    Mutability::Immutable => {
                        format!("&{}self", lifetime_prefix(borrow.lifetime, method))
                    }
                    Mutability::Mutable => {
                        format!("&{}mut self", lifetime_prefix(borrow.lifetime, method))
                    }
                }
            }
            SelfType::Struct(path) => match path.owner {
                MaybeOwn::Own => "self".to_string(),
                MaybeOwn::Borrow(borrow) => match borrow.mutability {
                    Mutability::Immutable => {
                        format!("&{}self", lifetime_prefix(borrow.lifetime, method))
                    }
                    Mutability::Mutable => {
                        format!("&{}mut self", lifetime_prefix(borrow.lifetime, method))
                    }
                },
            },
            SelfType::Enum(_) => "self".to_string(),
            _ => unreachable!("validated method receiver"),
        };
        params.push(self_param);
    }
    params.extend(method.params.iter().map(|param| {
        format!(
            "{}: {}",
            param.name,
            safe_input_type(&param.ty, method, tcx)
        )
    }));
    let return_ty = safe_return_type(&method.output, method, tcx);
    let return_suffix = if return_ty == "()" {
        String::new()
    } else {
        format!(" -> {return_ty}")
    };

    let mut args = Vec::new();
    let mut pre = Vec::new();
    let mut post = Vec::new();
    if let Some(param_self) = &method.param_self {
        match &param_self.ty {
            SelfType::Opaque(_) => args.push(match param_self.get_mutability() {
                Mutability::Immutable => "self.inner.as_ptr() as *const _".into(),
                Mutability::Mutable => "self.inner.as_ptr()".into(),
            }),
            SelfType::Struct(path) => {
                let strct = path.resolve(tcx);
                if struct_needs_abi_mirror(strct, tcx) {
                    match path.owner {
                        MaybeOwn::Own => args.push(struct_input_expr("self", strct, tcx)),
                        MaybeOwn::Borrow(borrow) if borrow.mutability == Mutability::Mutable => {
                            pre.push(format!(
                                "        let mut __abi = {};",
                                struct_input_expr("self", strct, tcx)
                            ));
                            args.push("&mut __abi as *mut _".into());
                            post.push(format!(
                                "        *self = {};",
                                struct_output_expr("__abi", strct, tcx)
                            ));
                        }
                        MaybeOwn::Borrow(_) => {
                            pre.push(format!(
                                "        let __abi = {};",
                                struct_input_expr("self", strct, tcx)
                            ));
                            args.push("&__abi as *const _".into());
                        }
                    }
                } else {
                    args.push("self".to_string());
                }
            }
            // An enum receiver is the value itself.
            _ => args.push("self".to_string()),
        }
    }
    args.extend(method.params.iter().map(|param| input_expr(param, tcx)));

    let mut body = vec![
        "        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.".into(),
    ];
    body.extend(pre);
    if method.output.is_write() {
        body.extend(write_body(method, tcx, &args));
        body.extend(post);
    } else if return_ty == "()" {
        body.push(format!(
            "        unsafe {{ ffi::{}({}) }};",
            method.abi_name,
            args.join(", ")
        ));
        body.extend(post);
    } else {
        let expression = return_expr(&method.output, method, tcx);
        if expression == "result" && post.is_empty() {
            // Avoid a clippy `let_and_return` in the common pass-through case.
            body.push(format!(
                "        unsafe {{ ffi::{}({}) }}",
                method.abi_name,
                args.join(", ")
            ));
        } else {
            body.push(format!(
                "        let result = unsafe {{ ffi::{}({}) }};",
                method.abi_name,
                args.join(", ")
            ));
            body.extend(post);
            body.push(format!("        {expression}"));
        }
    }

    MethodView {
        docs: render_docs(&method.docs, docs_url_gen, "    "),
        name,
        generics,
        params: params.join(", "),
        return_suffix,
        body,
    }
}

/// Build a `DiplomatWrite`, pass it as the trailing ABI argument, and return the
/// written UTF-8. The public signature never names the writer.
fn write_body(method: &hir::Method, tcx: &TypeContext, args: &[String]) -> Vec<String> {
    let mut abi_args = args.to_vec();
    abi_args.push("write".into());
    let abi_args = abi_args.join(", ");
    match &method.output {
        ReturnType::Infallible(_) => vec![
            "        crate::private::with_write(|write| {".into(),
            format!(
                "            unsafe {{ ffi::{}({abi_args}) }};",
                method.abi_name
            ),
            "        }).1".into(),
        ],
        ReturnType::Fallible(_, err) => {
            let mut lines = vec![
                "        let (result, text) = crate::private::with_write(|write| {".into(),
                format!(
                    "            unsafe {{ ffi::{}({abi_args}) }}",
                    method.abi_name
                ),
                "        });".into(),
            ];
            let err = error_expr(err, method, tcx);
            if err == "result" {
                lines.push("        Result::from(result).map(|()| text)".into());
            } else {
                lines.push(format!(
                    "        match Result::from(result) {{ Ok(()) => Ok(text), Err(result) => Err({err}) }}"
                ));
            }
            lines
        }
        ReturnType::Nullable(_) => vec![
            "        let (result, text) = crate::private::with_write(|write| {".into(),
            format!(
                "            unsafe {{ ffi::{}({abi_args}) }}",
                method.abi_name
            ),
            "        });".into(),
            "        Option::from(result).map(|()| text)".into(),
        ],
    }
}

pub(super) fn input_expr(param: &hir::Param, tcx: &TypeContext) -> String {
    match &param.ty {
        Type::Opaque(path) => {
            let name = opaque_name(path.tcx_id, tcx);
            match path.owner.mutability {
                Mutability::Immutable => format!(
                    "crate::private::{name}SharedSealed::__as_const_ptr({})",
                    param.name
                ),
                Mutability::Mutable => {
                    format!(
                        "crate::private::{name}MutSealed::__as_mut_ptr({})",
                        param.name
                    )
                }
            }
        }
        Type::Primitive(PrimitiveType::Char) => format!("{} as u32", param.name),
        Type::DiplomatOption(_) => convert_to_ffi(&param.ty, param.name.as_str(), tcx),
        Type::Slice(slice) => ffi_borrowed_slice_expr(slice, param.name.as_str()),
        Type::Struct(path) => {
            let def = tcx.resolve_type(path.id());
            match def {
                TypeDef::Struct(strct) if struct_needs_abi_mirror(strct, tcx) => {
                    struct_input_expr(param.name.as_str(), strct, tcx)
                }
                _ => param.name.to_string(),
            }
        }
        _ => param.name.to_string(),
    }
}

pub(super) fn return_expr(ret: &ReturnType, method: &hir::Method, tcx: &TypeContext) -> String {
    match ret {
        ReturnType::Infallible(success) => success_expr(success, method, tcx),
        ReturnType::Nullable(SuccessType::OutType(ty)) => {
            let inner = convert_from_ffi(ty, "__v", tcx);
            if inner == "__v" {
                "result.into()".into()
            } else if let Some(func) = inner.strip_suffix("(__v)") {
                format!("result.into_option().map({func})")
            } else {
                format!("result.into_option().map(|__v| {inner})")
            }
        }
        // The ABI returns one `DiplomatResult`; the runtime owns the tag check, so the
        // generated code only has to convert each payload in its own arm. Both arms bind
        // the payload as `result`, which is the name the conversions below expect.
        ReturnType::Fallible(success, err) => {
            let ok = success_expr(success, method, tcx);
            let err = error_expr(err, method, tcx);
            if ok == "result" && err == "result" {
                // Both payloads are already the safe types, so the runtime's own
                // conversion does the whole job and no hand-written match is needed.
                "result.into()".into()
            } else {
                format!(
                    "match Result::from(result) {{ Ok(result) => Ok({ok}), Err(result) => Err({err}) }}"
                )
            }
        }
        ReturnType::Nullable(_) => unreachable!("validated return shape"),
    }
}

/// The expression that turns an ABI success payload (bound to `result`) into its safe
/// public type.
fn success_expr(success: &SuccessType, method: &hir::Method, tcx: &TypeContext) -> String {
    match success {
        SuccessType::Unit => "result".into(),
        SuccessType::OutType(Type::Opaque(path)) => opaque_return_expr(path, method, tcx),
        SuccessType::OutType(Type::Slice(slice)) => {
            if is_owned_slice(slice) {
                "Box::from(result)".into()
            } else if matches!(slice, Slice::Str(_, StringEncoding::Utf8)) {
                "unsafe { crate::private::utf8_str_from_slice(result) }".into()
            } else {
                // `DiplomatSlice`/`DiplomatSliceMut` convert into the safe slice type
                // the HIR-selected method lifetime calls for.
                "result.into()".into()
            }
        }
        SuccessType::OutType(Type::Struct(ReturnableStructPath::Struct(path))) => {
            let strct = path.resolve(tcx);
            if struct_needs_abi_mirror(strct, tcx) {
                struct_output_expr("result", strct, tcx)
            } else {
                "result".into()
            }
        }
        SuccessType::OutType(Type::Primitive(PrimitiveType::Char)) => {
            "crate::private::char_from_u32(result)".into()
        }
        SuccessType::OutType(ty @ Type::DiplomatOption(_)) => convert_from_ffi(ty, "result", tcx),
        SuccessType::OutType(_) => "result".into(),
        // `Write` is rejected by validation, and `SuccessType` is `#[non_exhaustive]`, so
        // anything reaching here is a shape codegen has never been taught.
        _ => unreachable!("validated success shape"),
    }
}

/// The expression that turns an ABI error payload (bound to `result`) into its safe
/// public type. `None` is `Result<T, ()>`, whose payload is the zero-sized value itself.
fn error_expr(err: &Option<OutType>, method: &hir::Method, tcx: &TypeContext) -> String {
    match err {
        None => "result".into(),
        // An owned opaque error becomes the owning wrapper, so its `Drop` frees the
        // provider's allocation even when the caller discards the error with `?`.
        Some(Type::Opaque(path)) => opaque_return_expr(path, method, tcx),
        // Primitive `char` and any value struct that needs an ABI mirror have
        // different raw/public payload types. Convert the error arm just like
        // the success arm; ordinary primitives and enums remain identity.
        Some(ty) => convert_from_ffi(ty, "result", tcx),
    }
}
