//! Rejection rules: which HIR shapes this backend refuses to generate for.

use std::borrow::Cow;
use std::cell::Cell;
use std::collections::HashSet;

use diplomat_core::hir::{
    self, OutType, ReturnType, ReturnableStructPath, StructPathLike, SuccessType, Type,
    TypeContext, TypeDef,
};

use super::formatter::{
    enum_variant_name, field_name, method_name, opaque_module_name, type_def_name, valid_rust_ident,
};
use super::type_map::{is_lifetime_struct, is_owned_slice, is_supported_slice, primitive_name};
use crate::{ErrorContextGuard, ErrorStore};

pub(super) struct Reporter<'a, 'tcx> {
    errors: &'a ErrorStore<'tcx, String>,
    invalid: &'a Cell<bool>,
}

impl<'a, 'tcx> Reporter<'a, 'tcx> {
    pub(super) fn new(errors: &'a ErrorStore<'tcx, String>, invalid: &'a Cell<bool>) -> Self {
        Self { errors, invalid }
    }

    /// Record a validation failure, marking the whole run as ungeneratable.
    fn reject(&self, message: impl Into<String>) {
        self.invalid.set(true);
        self.errors.push_error(message.into());
    }

    /// Whether any failure has been recorded so far.
    pub(super) fn is_invalid(&self) -> bool {
        self.invalid.get()
    }

    /// Set the current error context to a named type; see [`ErrorStore::set_context_ty`].
    fn set_context_ty(&self, ty: Cow<'tcx, hir::LocIdent>) -> ErrorContextGuard<'_, 'tcx, String> {
        self.errors.set_context_ty(ty)
    }

    /// Set the current error context to a named method; see
    /// [`ErrorStore::set_context_method`].
    fn set_context_method(
        &self,
        method: Cow<'tcx, hir::LocIdent>,
    ) -> ErrorContextGuard<'_, 'tcx, String> {
        self.errors.set_context_method(method)
    }
}

pub(super) fn validate<'tcx>(tcx: &'tcx TypeContext, reporter: &Reporter<'_, 'tcx>) {
    let mut generated_names = HashSet::new();
    let mut module_names = HashSet::new();

    for (_, def) in tcx.all_types() {
        if def.attrs().disable {
            continue;
        }
        let _type_guard = reporter.set_context_ty(def.name_with_span().into());
        let name = type_def_name(def);
        let mut names = vec![name.clone()];
        if let TypeDef::Opaque(opaque) = def {
            let module = opaque_module_name(opaque);
            if !valid_rust_ident(&module) {
                reporter.reject(format!(
                    "[Rust backend] `{module}` is not a valid generated Rust module name"
                ));
            } else if !module_names.insert(module.clone()) {
                reporter.reject(format!(
                    "[Rust backend] generated module name collision for `{module}` (from `{name}`)"
                ));
            }
        }
        if matches!(def, TypeDef::Opaque(_)) {
            names.extend([
                format!("{name}Ref"),
                format!("{name}RefMut"),
                format!("{name}SharedArg"),
                format!("{name}MutArg"),
            ]);
        }
        for generated in names {
            if !valid_rust_ident(&generated) {
                reporter.reject(format!(
                    "[Rust backend] `{generated}` is not a valid Rust identifier"
                ));
            } else if !generated_names.insert(generated.clone()) {
                reporter.reject(format!(
                    "[Rust backend] generated name collision for `{generated}`"
                ));
            }
        }

        match def {
            TypeDef::Opaque(opaque) => {
                if !valid_rust_ident(opaque.dtor_abi_name.as_str()) {
                    reporter.reject(format!(
                        "[Rust backend] ABI destructor name `{}` is not a valid Rust identifier",
                        opaque.dtor_abi_name
                    ));
                }
                validate_methods(
                    tcx,
                    &opaque.methods,
                    opaque.lifetimes.num_lifetimes(),
                    reporter,
                );
            }
            TypeDef::Struct(strct) => {
                let mut field_names = HashSet::new();
                for field in &strct.fields {
                    if !valid_rust_ident(&field_name(field)) {
                        reporter.reject(format!(
                            "[Rust backend] `{}` is not a valid Rust field identifier",
                            field_name(field)
                        ));
                    }
                    if !field_names.insert(field_name(field)) {
                        reporter.reject("[Rust backend] generated struct field name collision");
                    }
                    if !supported_struct_field(&field.ty, tcx) {
                        reporter.reject(
                            "[Rust backend] structs may contain only supported primitives, enums, and borrowed slices",
                        );
                    }
                }
                if !strct.methods.is_empty() {
                    reporter.reject(
                        "[Rust backend] methods on value structs are not supported in this experiment",
                    );
                }
            }
            TypeDef::OutStruct(_) => {
                reporter.reject("[Rust backend] output-only structs are unsupported")
            }
            TypeDef::Enum(enm) => {
                let mut variant_names = HashSet::new();
                for variant in &enm.variants {
                    let name = enum_variant_name(variant);
                    if !valid_rust_ident(&name) {
                        reporter.reject(format!(
                            "[Rust backend] `{name}` is not a valid Rust enum variant"
                        ));
                    }
                    if !variant_names.insert(name) {
                        reporter.reject("[Rust backend] generated enum variant name collision");
                    }
                }
                if !enm.methods.is_empty() {
                    reporter.reject(
                        "[Rust backend] methods on enums are not supported in this experiment",
                    );
                }
            }
            _ => reporter.reject("[Rust backend] unsupported type definition"),
        }
    }

    for (_, method) in tcx.all_free_functions() {
        let _guard = reporter.set_context_method((&method.name).into());
        reporter.reject(
            "[Rust backend] free functions are unsupported; place the function on an opaque type",
        );
    }
    for (_, trt) in tcx.all_traits() {
        let _guard = reporter.set_context_ty((&trt.name).into());
        reporter.reject("[Rust backend] traits and callbacks are unsupported");
    }
}

pub(super) fn validate_methods<'tcx>(
    tcx: &'tcx TypeContext,
    methods: &'tcx [hir::Method],
    type_lifetime_count: usize,
    reporter: &Reporter<'_, 'tcx>,
) {
    let mut names = HashSet::new();
    for method in methods {
        if method.attrs.disable {
            continue;
        }
        let _method_guard = reporter.set_context_method((&method.name).into());
        let method_name = method_name(method);
        if !valid_rust_ident(method.abi_name.as_str()) {
            reporter.reject(format!(
                "[Rust backend] ABI symbol `{}` is not a valid Rust identifier",
                method.abi_name
            ));
        }
        if !valid_rust_ident(&method_name) {
            reporter.reject(format!(
                "[Rust backend] `{method_name}` is not a valid Rust method identifier"
            ));
        } else if !names.insert(method_name.clone()) {
            reporter.reject(format!(
                "[Rust backend] generated method name collision for `{method_name}`"
            ));
        }

        for param in &method.params {
            if !valid_rust_ident(param.name.as_str()) {
                reporter.reject(format!(
                    "[Rust backend] `{}` is not a valid Rust parameter identifier",
                    param.name
                ));
            }
            if let Some(disabled) = disabled_type_name(&param.ty, tcx) {
                reporter.reject(format!(
                    "[Rust backend] found usage of disabled type `{disabled}` as parameter `{}`",
                    param.name
                ));
            } else if !is_input_type(&param.ty, tcx) {
                reporter.reject(format!(
                    "[Rust backend] unsupported parameter type for `{}`",
                    param.name
                ));
            }
        }
        let disabled_output = match &method.output {
            ReturnType::Infallible(SuccessType::OutType(ty))
            | ReturnType::Nullable(SuccessType::OutType(ty)) => disabled_type_name(ty, tcx),
            _ => None,
        };
        if let Some(disabled) = disabled_output {
            reporter.reject(format!(
                "[Rust backend] found usage of disabled type `{disabled}` in the return type"
            ));
        } else if !is_return_type(&method.output, tcx) {
            reporter.reject("[Rust backend] unsupported return type");
        }

        let used = method.output.used_method_lifetimes();
        // Lifetimes introduced by the enclosing opaque's type parameters are not
        // call-local borrows; they are declared on the generated impl.
        let type_lifetimes: Vec<_> = method
            .lifetime_env
            .all_lifetimes()
            .take(type_lifetime_count)
            .collect();
        let borrowed_outputs: Vec<_> = used
            .iter()
            .copied()
            .filter(|lifetime| !type_lifetimes.contains(lifetime))
            .collect();
        if !borrowed_outputs.is_empty() {
            if borrowed_outputs.len() != 1 {
                reporter
                    .reject("[Rust backend] borrowed returns may use exactly one output lifetime");
                continue;
            }
            let mut visitor = method.borrowing_param_visitor(tcx, false);
            if let Some(param_self) = &method.param_self {
                visitor.visit_param(&param_self.ty.clone().into(), "self");
            }
            for param in &method.params {
                visitor.visit_param(&param.ty, param.name.as_str());
            }
            let map = visitor.borrow_map();
            let Some(info) = map.get(&borrowed_outputs[0]) else {
                reporter.reject("[Rust backend] borrowed return has no lifetime edge");
                continue;
            };
            if info.incoming_edges.is_empty() {
                reporter.reject("[Rust backend] borrowed return has no owning input");
            }
            if info.incoming_edges.iter().any(|edge| {
                !matches!(
                    edge.kind,
                    hir::borrowing_param::LifetimeEdgeKind::OpaqueParam
                        | hir::borrowing_param::LifetimeEdgeKind::SliceParam
                )
            }) {
                reporter.reject(
                    "[Rust backend] borrowed returns may only borrow directly from opaque or slice inputs",
                );
            }
        }
    }
}

/// The declared name of a type the provider disabled for this backend with
/// `#[diplomat::attr(rust, disable)]`, if `ty` names one.
///
/// Disabled definitions are never emitted, so a signature that mentions one would
/// otherwise generate references to types that do not exist. Other backends report
/// the same situation as "Found usage of disabled type".
pub(super) fn disabled_type_name<P: hir::TyPosition>(
    ty: &Type<P>,
    tcx: &TypeContext,
) -> Option<String> {
    let def = match ty {
        Type::Enum(path) => tcx.resolve_type(path.tcx_id.into()),
        Type::Opaque(path) => tcx.resolve_type(path.tcx_id.into()),
        Type::Struct(path) => tcx.resolve_type(path.id()),
        // `Option<T>` reaches generation as a nested payload; unwrap it here so the
        // report names the disabled type instead of falling back to the generic
        // "unsupported parameter type".
        Type::DiplomatOption(inner) => return disabled_type_name(inner.as_ref(), tcx),
        _ => return None,
    };
    def.attrs().disable.then(|| type_def_name(def))
}

pub(super) fn is_value_type<P: hir::TyPosition>(ty: &Type<P>, tcx: &TypeContext) -> bool {
    match ty {
        Type::Primitive(p) => primitive_name(*p).is_some(),
        Type::Enum(path) => !path.resolve(tcx).attrs.disable,
        Type::Struct(path) => {
            let def = tcx.resolve_type(path.id());
            match def {
                TypeDef::Struct(def) => {
                    !def.attrs.disable
                        && def.lifetimes.num_lifetimes() == 0
                        && def.fields.iter().all(|field| is_value_type(&field.ty, tcx))
                }
                _ => false,
            }
        }
        _ => false,
    }
}

pub(super) fn is_input_type(ty: &Type<hir::InputOnly>, tcx: &TypeContext) -> bool {
    match ty {
        Type::Opaque(path) => !path.is_optional() && !path.resolve(tcx).attrs.disable,
        Type::DiplomatOption(inner) => is_value_type(inner.as_ref(), tcx),
        Type::Slice(slice) => is_supported_slice(slice),
        Type::Struct(path) => {
            matches!(
                tcx.resolve_type(path.id()),
                TypeDef::Struct(strct) if is_lifetime_struct(strct)
            ) || is_value_type(ty, tcx)
        }
        _ => is_value_type(ty, tcx),
    }
}

pub(super) fn is_output_type(ty: &OutType, tcx: &TypeContext) -> bool {
    match ty {
        Type::Opaque(path) => !path.resolve(tcx).attrs.disable,
        Type::DiplomatOption(inner) => is_value_type(inner.as_ref(), tcx),
        Type::Struct(ReturnableStructPath::Struct(path)) => {
            let def = path.resolve(tcx);
            if is_lifetime_struct(def) {
                def.fields
                    .iter()
                    .all(|field| supported_struct_field(&field.ty, tcx))
            } else {
                def.lifetimes.num_lifetimes() == 0
                    && def.fields.iter().all(|field| is_value_type(&field.ty, tcx))
            }
        }
        Type::Struct(ReturnableStructPath::OutStruct(_)) => false,
        Type::Slice(slice) => is_supported_slice(slice) || is_owned_slice(slice),
        _ => is_value_type(ty, tcx),
    }
}

pub(super) fn is_return_type(ret: &ReturnType, tcx: &TypeContext) -> bool {
    match ret {
        ReturnType::Infallible(SuccessType::Unit) => true,
        ReturnType::Infallible(SuccessType::OutType(ty)) => is_output_type(ty, tcx),
        ReturnType::Nullable(SuccessType::OutType(ty)) => {
            !matches!(ty, Type::Opaque(_)) && is_value_type(ty, tcx)
        }
        ReturnType::Fallible(..) | ReturnType::Nullable(_) | ReturnType::Infallible(_) => false,
    }
}

/// A field type that this backend can place in a generated struct.
pub(super) fn supported_struct_field<P: hir::TyPosition>(ty: &Type<P>, tcx: &TypeContext) -> bool {
    match ty {
        Type::Primitive(primitive) => primitive_name(*primitive).is_some(),
        Type::Enum(path) => !path.resolve(tcx).attrs.disable,
        Type::Slice(slice) => is_supported_slice(slice),
        _ => false,
    }
}
