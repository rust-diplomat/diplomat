//! Rendering of a single method: the safe signature, the ABI call, and the
//! conversion of its result back into the safe public type.

use std::fmt::Write as _;

use diplomat_core::hir::{
    self, DocsUrlGenerator, MaybeOwn, Mutability, OutType, PrimitiveType, ReturnType,
    ReturnableStructPath, SelfType, Slice, StringEncoding, StructPathLike, SuccessType, Type,
    TypeContext, TypeDef,
};

use super::opaque::opaque_return_expr;
use crate::r#rust::formatter::{emit_docs, method_name, opaque_name};
use crate::r#rust::lifetimes::{lifetime_prefix, method_generics};
use crate::r#rust::type_map::{
    ffi_borrowed_slice_expr, is_owned_slice, safe_input_type, safe_return_type, struct_input_expr,
    struct_needs_abi_mirror, struct_output_expr,
};

pub(super) fn emit_method(
    out: &mut String,
    type_lifetimes: usize,
    method: &hir::Method,
    tcx: &TypeContext,
    docs_url_gen: &DocsUrlGenerator,
) {
    emit_docs(out, &method.docs, docs_url_gen, "    ");
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
    writeln!(
        out,
        "    pub fn {name}{generics}({}){} {{",
        params.join(", "),
        if return_ty == "()" {
            String::new()
        } else {
            format!(" -> {return_ty}")
        }
    )
    .unwrap();
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
                if struct_needs_abi_mirror(strct) {
                    match path.owner {
                        MaybeOwn::Own => args.push(struct_input_expr("self", strct)),
                        MaybeOwn::Borrow(borrow) if borrow.mutability == Mutability::Mutable => {
                            pre.push(format!(
                                "        let mut __abi = {};",
                                struct_input_expr("self", strct)
                            ));
                            args.push("&mut __abi as *mut _".into());
                            post.push(format!(
                                "        *self = {};",
                                struct_output_expr("__abi", strct)
                            ));
                        }
                        MaybeOwn::Borrow(_) => {
                            pre.push(format!(
                                "        let __abi = {};",
                                struct_input_expr("self", strct)
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
    writeln!(out, "        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.").unwrap();
    for line in &pre {
        writeln!(out, "{line}").unwrap();
    }
    if method.output.is_write() {
        emit_write_body(out, method, tcx, &args);
        for line in &post {
            writeln!(out, "{line}").unwrap();
        }
    } else if return_ty == "()" {
        writeln!(
            out,
            "        unsafe {{ ffi::{}({}) }};",
            method.abi_name,
            args.join(", ")
        )
        .unwrap();
        for line in &post {
            writeln!(out, "{line}").unwrap();
        }
    } else {
        let expression = return_expr(&method.output, method, tcx);
        if expression == "result" && post.is_empty() {
            // Avoid a clippy `let_and_return` in the common pass-through case.
            writeln!(
                out,
                "        unsafe {{ ffi::{}({}) }}",
                method.abi_name,
                args.join(", ")
            )
            .unwrap();
        } else {
            writeln!(
                out,
                "        let result = unsafe {{ ffi::{}({}) }};",
                method.abi_name,
                args.join(", ")
            )
            .unwrap();
            for line in &post {
                writeln!(out, "{line}").unwrap();
            }
            writeln!(out, "        {expression}").unwrap();
        }
    }
    out.push_str("    }\n");
}

/// Build a `DiplomatWrite`, pass it as the trailing ABI argument, and return the
/// written UTF-8. The public signature never names the writer.
fn emit_write_body(out: &mut String, method: &hir::Method, tcx: &TypeContext, args: &[String]) {
    let mut abi_args = args.to_vec();
    abi_args.push("write".into());
    let abi_args = abi_args.join(", ");
    match &method.output {
        ReturnType::Infallible(_) => {
            writeln!(
                out,
                "        crate::private::with_write(|write| {{\n            unsafe {{ ffi::{}({abi_args}) }};\n        }}).1",
                method.abi_name
            )
            .unwrap();
        }
        ReturnType::Fallible(_, err) => {
            writeln!(
                out,
                "        let (result, text) = crate::private::with_write(|write| {{\n            unsafe {{ ffi::{}({abi_args}) }}\n        }});",
                method.abi_name
            )
            .unwrap();
            let err = error_expr(err, method, tcx);
            if err == "result" {
                writeln!(out, "        Result::from(result).map(|()| text)").unwrap();
            } else {
                writeln!(
                    out,
                    "        match Result::from(result) {{ Ok(()) => Ok(text), Err(result) => Err({err}) }}"
                )
                .unwrap();
            }
        }
        ReturnType::Nullable(_) => {
            writeln!(
                out,
                "        let (result, text) = crate::private::with_write(|write| {{\n            unsafe {{ ffi::{}({abi_args}) }}\n        }});\n        Option::from(result).map(|()| text)",
                method.abi_name
            )
            .unwrap();
        }
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
        Type::DiplomatOption(_) => format!("ffi::DiplomatOption::from({})", param.name),
        Type::Slice(slice) => ffi_borrowed_slice_expr(slice, param.name.as_str()),
        Type::Struct(path) => {
            let def = tcx.resolve_type(path.id());
            match def {
                TypeDef::Struct(strct) if struct_needs_abi_mirror(strct) => {
                    struct_input_expr(param.name.as_str(), strct)
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
        // `DiplomatOption<T>` maps straight onto `Option<T>`.
        ReturnType::Nullable(SuccessType::OutType(_)) => "result.into()".into(),
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
            if struct_needs_abi_mirror(strct) {
                struct_output_expr("result", strct)
            } else {
                "result".into()
            }
        }
        SuccessType::OutType(Type::Primitive(PrimitiveType::Char)) => {
            "crate::private::char_from_u32(result)".into()
        }
        SuccessType::OutType(Type::DiplomatOption(_)) => "result.into()".into(),
        SuccessType::OutType(_) => "result".into(),
        // `Write` is rejected by validation, and `SuccessType` is `#[non_exhaustive]`, so
        // anything reaching here is a shape codegen has never been taught.
        _ => unreachable!("validated success shape"),
    }
}

/// The expression that turns an ABI error payload (bound to `result`) into its safe public
/// type. `None` is `Result<T, ()>`, whose payload is the zero-sized value itself.
fn error_expr(err: &Option<OutType>, method: &hir::Method, tcx: &TypeContext) -> String {
    match err {
        None => "result".into(),
        // An owned opaque error becomes the owning wrapper, so its `Drop` frees the
        // provider's allocation even when the caller discards the error with `?`.
        Some(Type::Opaque(path)) => opaque_return_expr(path, method, tcx),
        Some(_) => "result".into(),
    }
}
