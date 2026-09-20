//! Rendering of a single method: the safe signature, the ABI call, and the
//! conversion of its result back into the safe public type.

use std::fmt::Write as _;

use diplomat_core::hir::{
    self, DocsUrlGenerator, Mutability, ReturnType, ReturnableStructPath, SelfType, Slice,
    StringEncoding, StructPathLike, SuccessType, Type, TypeContext, TypeDef,
};

use super::opaque::opaque_return_expr;
use crate::r#rust::formatter::{emit_docs, method_name, opaque_name};
use crate::r#rust::lifetimes::{lifetime_prefix, method_generics};
use crate::r#rust::type_map::{
    ffi_borrowed_slice_expr, is_lifetime_struct, is_owned_slice, safe_input_type, safe_return_type,
    struct_input_expr, struct_output_expr,
};

pub(super) fn emit_method(
    out: &mut String,
    opaque: &hir::OpaqueDef,
    method: &hir::Method,
    tcx: &TypeContext,
    docs_url_gen: &DocsUrlGenerator,
) {
    emit_docs(out, &method.docs, docs_url_gen, "    ");
    let name = method_name(method);
    let generics = method_generics(method, opaque.lifetimes.num_lifetimes(), tcx);
    let mut params = Vec::new();
    if let Some(param_self) = &method.param_self {
        let borrow = match &param_self.ty {
            SelfType::Opaque(path) => path.borrowed(),
            _ => unreachable!("only opaque impl methods reach Safe Rust codegen"),
        };
        params.push(match borrow.mutability {
            Mutability::Immutable => format!("&{}self", lifetime_prefix(borrow.lifetime, method)),
            Mutability::Mutable => format!("&{}mut self", lifetime_prefix(borrow.lifetime, method)),
        });
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
    if let Some(param_self) = &method.param_self {
        args.push(match param_self.get_mutability() {
            Mutability::Immutable => "self.inner.as_ptr() as *const _".into(),
            Mutability::Mutable => "self.inner.as_ptr()".into(),
        });
    }
    args.extend(method.params.iter().map(|param| input_expr(param, tcx)));
    writeln!(out, "        // SAFETY: generated arguments preserve the ownership, mutability, and lifetime constraints encoded by HIR.").unwrap();
    if return_ty == "()" {
        writeln!(
            out,
            "        unsafe {{ ffi::{}({}) }};",
            method.abi_name,
            args.join(", ")
        )
        .unwrap();
    } else {
        let expression = return_expr(&method.output, method, tcx);
        if expression == "result" {
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
            writeln!(out, "        {expression}").unwrap();
        }
    }
    out.push_str("    }\n");
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
        Type::DiplomatOption(_) => format!("ffi::DiplomatOption::from({})", param.name),
        Type::Slice(slice) => ffi_borrowed_slice_expr(slice, param.name.as_str()),
        Type::Struct(path) => {
            let def = tcx.resolve_type(path.id());
            match def {
                TypeDef::Struct(strct) if is_lifetime_struct(strct) => {
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
        ReturnType::Infallible(SuccessType::OutType(Type::Opaque(path))) => {
            opaque_return_expr(path, method, tcx)
        }
        ReturnType::Infallible(SuccessType::OutType(Type::Slice(slice))) => {
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
        ReturnType::Infallible(SuccessType::OutType(Type::Struct(
            ReturnableStructPath::Struct(path),
        ))) => {
            let strct = path.resolve(tcx);
            if is_lifetime_struct(strct) {
                struct_output_expr("result", strct)
            } else {
                "result".into()
            }
        }
        ReturnType::Infallible(SuccessType::OutType(Type::DiplomatOption(_)))
        | ReturnType::Nullable(SuccessType::OutType(_)) => "result.into()".into(),
        ReturnType::Infallible(SuccessType::OutType(_)) => "result".into(),
        _ => unreachable!("validated return shape"),
    }
}
