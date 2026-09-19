//! Experimental Safe Rust client backend over Diplomat's native C ABI.

use std::borrow::Cow;
use std::cell::Cell;
use std::collections::HashSet;
use std::fmt::Write as _;

use diplomat_core::hir::{
    self, BackendAttrSupport, DocsUrlGenerator, MaybeOwn, MaybeStatic, Mutability, OutType,
    PrimitiveType, ReturnType, ReturnableStructPath, SelfType, Slice, StringEncoding,
    StructPathLike, SuccessType, Type, TypeContext, TypeDef,
};
use heck::ToSnakeCase;
use serde::{Deserialize, Serialize};

use crate::{Config, ErrorContextGuard, ErrorStore, FileMap};

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct RustConfig {
    /// Cargo package name for the generated bindings.
    pub crate_name: Option<String>,
    /// Native library name passed to Rust's `#[link]` attribute.
    pub dylib_name: Option<String>,
}

impl RustConfig {
    pub fn set(&mut self, key: &str, value: toml::Value) {
        let value = value
            .as_str()
            .unwrap_or_else(|| panic!("Rust config key `{key}` must be a string"))
            .to_string();
        match key {
            "crate_name" => self.crate_name = Some(value),
            "dylib_name" => self.dylib_name = Some(value),
            _ => panic!("Unrecognized Rust config key: {key}"),
        }
    }
}

pub(crate) fn attr_support() -> BackendAttrSupport {
    let mut support = BackendAttrSupport::default();
    // Generated code borrows directly out of provider-owned memory
    // (`from_raw_parts` over the provider's pointer), so memory_sharing-gated
    // shapes are ones this backend represents natively. Note the interaction with
    // the rejected primitive set: the shared corpus gates `&[f64]` constructors on
    // this flag, and floats are rejected, so pointing this backend at the shared
    // corpus fails on those methods until floats land (issue #10 on the fork).
    support.memory_sharing = true;
    support.constructors = true;
    // `named_constructor` maps to an associated function named after the declared
    // constructor name; see `method_name`.
    support.named_constructors = true;
    support.option = true;
    support.mutable_slices = true;
    support.static_slices = true;
    support.owned_byte_slice_returns = true;
    // The generated API surface is Rust, so it is UTF-8 by construction, and
    // `&DiplomatStr16` maps to `&[u16]`.
    support.utf8_strings = true;
    support.utf16_strings = true;
    support
}

pub(crate) fn run<'tcx>(
    tcx: &'tcx TypeContext,
    config: &Config,
    docs_url_gen: &DocsUrlGenerator,
) -> (FileMap, ErrorStore<'tcx, String>) {
    let files = FileMap::default();
    let errors = ErrorStore::default();
    let invalid = Cell::new(false);

    let reporter = Reporter::new(&errors, &invalid);
    validate(tcx, &reporter);
    if reporter.is_invalid() {
        return (files, errors);
    }

    let lib_name = config
        .shared_config
        .lib_name
        .as_deref()
        .unwrap_or("diplomat_native");
    let crate_name = config
        .rust_config
        .crate_name
        .clone()
        .unwrap_or_else(|| format!("{}-bindings", sanitize_package_component(lib_name)));
    let dylib_name = config
        .rust_config
        .dylib_name
        .clone()
        .unwrap_or_else(|| lib_name.to_string());

    if !valid_package_name(&crate_name) {
        if let Some((_, def)) = tcx.all_types().next() {
            let _guard = errors.set_context_ty(def.name_with_span().into());
            errors.push_error(format!(
                "[Rust backend] `{crate_name}` is not a valid generated Cargo package name"
            ));
        }
        return (files, errors);
    }
    if dylib_name.is_empty()
        || !dylib_name
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || matches!(c, '_' | '-'))
    {
        if let Some((_, def)) = tcx.all_types().next() {
            let _guard = errors.set_context_ty(def.name_with_span().into());
            errors.push_error(format!(
                "[Rust backend] `{dylib_name}` is not a valid native library name"
            ));
        }
        return (files, errors);
    }

    let package = include_str!("../../templates/rust/Cargo.toml.template")
        .replace("{{crate_name}}", &crate_name);
    let build = include_str!("../../templates/rust/build.rs.template");
    files.add_file("Cargo.toml".into(), package);
    files.add_file("build.rs".into(), build.into());
    files.add_file("src/lib.rs".into(), generate_lib());
    files.add_file("src/ffi.rs".into(), generate_ffi(tcx, &dylib_name));
    files.add_file("src/private.rs".into(), generate_private(tcx));
    files.add_file("src/types.rs".into(), generate_types(tcx, docs_url_gen));
    files.add_file("src/opaques.rs".into(), generate_opaques_index(tcx));
    for opaque in tcx.opaques().iter().filter(|ty| !ty.attrs.disable) {
        files.add_file(
            format!("src/opaques/{}.rs", opaque_module_name(opaque)),
            generate_opaque_file(tcx, opaque, docs_url_gen),
        );
    }
    (files, errors)
}

/// Collects the validation failures recorded while generation is attempted.
///
/// `ErrorStore` carries each message together with the type/method context it
/// arose under; the `Cell<bool>` records only whether *anything* was rejected,
/// which `run` uses to bail out without emitting partial bindings. The two are
/// meaningless apart, so they are passed around as one value.
struct Reporter<'a, 'tcx> {
    errors: &'a ErrorStore<'tcx, String>,
    invalid: &'a Cell<bool>,
}

impl<'a, 'tcx> Reporter<'a, 'tcx> {
    fn new(errors: &'a ErrorStore<'tcx, String>, invalid: &'a Cell<bool>) -> Self {
        Self { errors, invalid }
    }

    /// Record a validation failure, marking the whole run as ungeneratable.
    fn reject(&self, message: impl Into<String>) {
        self.invalid.set(true);
        self.errors.push_error(message.into());
    }

    /// Whether any failure has been recorded so far.
    fn is_invalid(&self) -> bool {
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

fn validate<'tcx>(tcx: &'tcx TypeContext, reporter: &Reporter<'_, 'tcx>) {
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

fn validate_methods<'tcx>(
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
fn disabled_type_name<P: hir::TyPosition>(ty: &Type<P>, tcx: &TypeContext) -> Option<String> {
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

fn is_value_type<P: hir::TyPosition>(ty: &Type<P>, tcx: &TypeContext) -> bool {
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

fn is_input_type(ty: &Type<hir::InputOnly>, tcx: &TypeContext) -> bool {
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

fn is_output_type(ty: &OutType, tcx: &TypeContext) -> bool {
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

fn is_return_type(ret: &ReturnType, tcx: &TypeContext) -> bool {
    match ret {
        ReturnType::Infallible(SuccessType::Unit) => true,
        ReturnType::Infallible(SuccessType::OutType(ty)) => is_output_type(ty, tcx),
        ReturnType::Nullable(SuccessType::OutType(ty)) => {
            !matches!(ty, Type::Opaque(_)) && is_value_type(ty, tcx)
        }
        ReturnType::Fallible(..) | ReturnType::Nullable(_) | ReturnType::Infallible(_) => false,
    }
}

/// A borrowed slice (`&[T]`, `&mut [T]`, `&str`, `&DiplomatStr`, `&DiplomatStr16`).
///
/// Owned slices (`Box<[T]>`) and slices of structs/strings/opaques are not part
/// of the supported subset.
fn is_supported_slice<P: hir::TyPosition>(slice: &Slice<P>) -> bool {
    match slice {
        Slice::Primitive(MaybeOwn::Borrow(_), primitive) => primitive_name(*primitive).is_some(),
        Slice::Str(Some(_), _) => true,
        _ => false,
    }
}

/// An owned primitive slice (`Box<[T]>`) returned by the provider.
fn is_owned_slice<P: hir::TyPosition>(slice: &Slice<P>) -> bool {
    matches!(slice, Slice::Primitive(MaybeOwn::Own, primitive) if primitive_name(*primitive).is_some())
}

/// The lifetime carried by a borrowed slice, if any.
fn slice_lifetime<P: hir::TyPosition>(slice: &Slice<P>) -> Option<MaybeStatic<hir::Lifetime>> {
    match slice {
        Slice::Primitive(MaybeOwn::Borrow(borrow), _)
        | Slice::Struct(MaybeOwn::Borrow(borrow), _)
        | Slice::Opaque(MaybeOwn::Borrow(borrow), _) => Some(borrow.lifetime),
        Slice::Str(lifetime, _) => *lifetime,
        Slice::Strs(_) => None,
        _ => None,
    }
}

fn slice_is_mutable<P: hir::TyPosition>(slice: &Slice<P>) -> bool {
    matches!(
        slice,
        Slice::Primitive(MaybeOwn::Borrow(borrow), _) if borrow.mutability == Mutability::Mutable
    )
}

/// The Rust scalar type stored in a supported slice.
fn slice_element_ty<P: hir::TyPosition>(slice: &Slice<P>) -> &'static str {
    match slice {
        Slice::Primitive(_, primitive) => primitive_name(*primitive).unwrap(),
        Slice::Str(_, StringEncoding::UnvalidatedUtf16) => "u16",
        Slice::Str(_, StringEncoding::Utf8 | StringEncoding::UnvalidatedUtf8) => "u8",
        _ => unreachable!("validated slice shape"),
    }
}

/// The safe public Rust type of a slice, e.g. `&'a [f64]`, `&'a mut str`, `Box<[u8]>`.
fn safe_slice_type<P: hir::TyPosition>(slice: &Slice<P>, lifetime: &str) -> String {
    if is_owned_slice(slice) {
        return format!("Box<[{}]>", slice_element_ty(slice));
    }
    if slice_is_mutable(slice) {
        return format!("&{lifetime}mut [{}]", slice_element_ty(slice));
    }
    match slice {
        Slice::Str(_, StringEncoding::Utf8) => format!("&{lifetime}str"),
        _ => format!("&{lifetime}[{}]", slice_element_ty(slice)),
    }
}

/// The native ABI slice container for a borrowed slice: `DiplomatSlice` for a
/// shared borrow, `DiplomatSliceMut` for an exclusive one.
fn ffi_slice_container<P: hir::TyPosition>(slice: &Slice<P>) -> &'static str {
    if slice_is_mutable(slice) {
        "DiplomatSliceMut"
    } else {
        "DiplomatSlice"
    }
}

/// Convert a safe Rust slice expression into the native ABI borrow of it, e.g.
/// `ffi::DiplomatSlice::from(samples)`.
fn ffi_borrowed_slice_expr<P: hir::TyPosition>(slice: &Slice<P>, expr: &str) -> String {
    let container = ffi_slice_container(slice);
    if matches!(slice, Slice::Str(_, StringEncoding::Utf8)) {
        // `diplomat_runtime::DiplomatUtf8StrSlice`'s field is private, so a `&str`
        // goes through the `&[u8]` conversion the runtime does expose.
        return format!("ffi::{container}::from({expr}.as_bytes())");
    }
    format!("ffi::{container}::from({expr})")
}

/// The native ABI type of a slice, with an explicit lifetime argument where the
/// position needs one: `DiplomatSlice<'a, u8>`, `DiplomatSliceMut<u8>`,
/// `DiplomatOwnedSlice<u8>`.
fn ffi_slice_type<P: hir::TyPosition>(slice: &Slice<P>, lifetime: &str) -> String {
    if is_owned_slice(slice) {
        return format!("DiplomatOwnedSlice<{}>", slice_element_ty(slice));
    }
    let container = ffi_slice_container(slice);
    let element = slice_element_ty(slice);
    if lifetime.is_empty() {
        format!("{container}<{element}>")
    } else {
        format!("{container}<{lifetime}, {element}>")
    }
}

fn generate_ffi(tcx: &TypeContext, dylib_name: &str) -> String {
    let mut out = String::from(
        "// @generated by diplomat-tool's experimental Safe Rust backend.\n\
         #![allow(dead_code)]\n\n\
         // The ABI types come from `diplomat-runtime` instead of being transcribed
         // here: a hand-maintained mirror with no layout check is a silent-UB hazard.
         // Re-exported so the sibling modules can name them as `ffi::DiplomatSlice`.
         pub(super) use diplomat_runtime::{
             DiplomatOption, DiplomatOwnedSlice, DiplomatSlice, DiplomatSliceMut,\n\
         };\n\n",
    );

    for opaque in tcx.opaques().iter().filter(|ty| !ty.attrs.disable) {
        let name = type_def_name(TypeDef::Opaque(opaque));
        writeln!(
            out,
            "#[repr(C)]\npub struct {name} {{ _private: [u8; 0] }}\n"
        )
        .unwrap();
    }
    for strct in tcx.structs().iter().filter(|ty| !ty.attrs.disable) {
        if !is_lifetime_struct(strct) {
            continue;
        }
        let name = type_def_name(TypeDef::Struct(strct));
        let (params, _) = struct_generics(strct);
        writeln!(out, "#[repr(C)]\npub struct {name}{params} {{").unwrap();
        for field in &strct.fields {
            writeln!(
                out,
                "    pub(super) {}: {},",
                field_name(field),
                ffi_struct_field_type(&field.ty, strct, tcx)
            )
            .unwrap();
        }
        out.push_str("}\n\n");
    }
    writeln!(out, "#[link(name = {:?})]\nextern \"C\" {{", dylib_name).unwrap();
    for opaque in tcx.opaques().iter().filter(|ty| !ty.attrs.disable) {
        let name = type_def_name(TypeDef::Opaque(opaque));
        writeln!(
            out,
            "    pub(super) fn {}(this: *mut {name});",
            opaque.dtor_abi_name
        )
        .unwrap();
        for method in opaque.methods.iter().filter(|m| !m.attrs.disable) {
            let ret_generics = ffi_return_generics(&method.output, tcx);
            write!(out, "    pub(super) fn {}{ret_generics}(", method.abi_name).unwrap();
            let mut params = Vec::new();
            if let Some(param_self) = &method.param_self {
                params.push(format!("this: {}", ffi_self_type(&param_self.ty, tcx)));
            }
            params.extend(
                method
                    .params
                    .iter()
                    .map(|param| format!("{}: {}", param.name, ffi_input_type(&param.ty, tcx))),
            );
            write!(out, "{}", params.join(", ")).unwrap();
            let ret = ffi_return_type(&method.output, tcx);
            if ret == "()" {
                writeln!(out, ");").unwrap();
            } else {
                writeln!(out, ") -> {ret};").unwrap();
            }
        }
    }
    out.push_str("}\n");
    out
}

fn generate_lib() -> String {
    String::from(
        "//! Safe Rust bindings generated over a Diplomat native ABI.\n\
         //!\n\
         //! The unsafe ABI layer lives in the private `ffi` module; the public API is\n\
         //! re-exported from this crate root.\n\n\
         #![allow(clippy::needless_lifetimes)]\n\
         #![allow(clippy::new_without_default)]\n\n\
         mod ffi;\n\
         mod opaques;\n\
         mod private;\n\
         mod types;\n\n\
         pub use opaques::*;\n\
         pub use types::*;\n",
    )
}

/// Sealed capability traits plus the unsafe ABI reconstruction helpers.
fn generate_private(tcx: &TypeContext) -> String {
    let mut out = String::from(
        "//! Sealed capability traits and unsafe ABI reconstruction helpers.\n\
         //!\n\
         //! This module is private, so downstream crates cannot name the sealed traits and\n\
         //! therefore cannot implement the public capability traits themselves. The helpers\n\
         //! may be unused for a given provider, hence the `dead_code` allow.\n\
         #![allow(dead_code)]\n\n",
    );
    for opaque in tcx.opaques().iter().filter(|ty| !ty.attrs.disable) {
        let name = type_def_name(TypeDef::Opaque(opaque));
        writeln!(out, "pub trait {name}SharedSealed {{").unwrap();
        writeln!(
            out,
            "    fn __as_const_ptr(&self) -> *const crate::ffi::{name};"
        )
        .unwrap();
        out.push_str("}\n");
        writeln!(out, "pub trait {name}MutSealed: {name}SharedSealed {{").unwrap();
        writeln!(
            out,
            "    fn __as_mut_ptr(&mut self) -> *mut crate::ffi::{name};"
        )
        .unwrap();
        out.push_str("}\n");
    }
    out.push('\n');
    out.push_str(
        r#"    /// Reconstruct a validated `&str` from a provider-returned UTF-8 slice.
    ///
    /// The runtime can turn a `DiplomatSlice` back into a `&[T]`, but
    /// `DiplomatUtf8StrSlice`'s field is private, so this is the one conversion the
    /// runtime cannot express and the generated crate keeps locally.
    ///
    /// # Safety
    ///
    /// The caller must uphold the provider's validity, alignment, aliasing, UTF-8,
    /// and lifetime contract for the slice for the returned lifetime. The ABI type
    /// cannot express UTF-8 validity, so the provider must send valid UTF-8.
    pub(crate) unsafe fn utf8_str_from_slice<'a>(
        slice: diplomat_runtime::DiplomatSlice<'a, u8>,
    ) -> &'a str {
        core::str::from_utf8_unchecked(<&[u8]>::from(slice))
    }

"#,
    );
    out
}

/// Generated enums and value structs.
fn generate_types(tcx: &TypeContext, docs_url_gen: &DocsUrlGenerator) -> String {
    let mut out = String::from("//! Generated enums and value structs.\n\n");
    if tcx
        .structs()
        .iter()
        .any(|strct| !strct.attrs.disable && is_lifetime_struct(strct))
    {
        out.push_str("use core::marker::PhantomData;\n\n");
    }
    for enm in tcx.enums().iter().filter(|ty| !ty.attrs.disable) {
        emit_docs(&mut out, &enm.docs, docs_url_gen, "");
        let name = type_def_name(TypeDef::Enum(enm));
        writeln!(
            out,
            "#[repr(C)]\n#[derive(Clone, Copy, Debug, PartialEq, Eq)]\npub enum {name} {{"
        )
        .unwrap();
        for variant in &enm.variants {
            emit_docs(&mut out, &variant.docs, docs_url_gen, "    ");
            writeln!(
                out,
                "    {} = {},",
                enum_variant_name(variant),
                variant.discriminant
            )
            .unwrap();
        }
        out.push_str("}\n\n");
    }
    for strct in tcx.structs().iter().filter(|ty| !ty.attrs.disable) {
        emit_docs(&mut out, &strct.docs, docs_url_gen, "");
        let name = type_def_name(TypeDef::Struct(strct));
        if is_lifetime_struct(strct) {
            let (params, _) = struct_generics(strct);
            writeln!(
                out,
                "#[derive(Clone, Copy, Debug, PartialEq, Eq)]\npub struct {name}{params} {{"
            )
            .unwrap();
            for field in &strct.fields {
                emit_docs(&mut out, &field.docs, docs_url_gen, "    ");
                writeln!(
                    out,
                    "    pub {}: {},",
                    field_name(field),
                    safe_struct_field_type(&field.ty, strct, tcx)
                )
                .unwrap();
            }
            out.push_str(&struct_lifetime_phantom(strct));
            out.push_str("}\n\n");
        } else {
            writeln!(
                out,
                "#[repr(C)]\n#[derive(Clone, Copy, Debug, PartialEq, Eq)]\npub struct {name} {{"
            )
            .unwrap();
            for field in &strct.fields {
                emit_docs(&mut out, &field.docs, docs_url_gen, "    ");
                writeln!(
                    out,
                    "    pub {}: {},",
                    field_name(field),
                    safe_value_type(&field.ty, tcx)
                )
                .unwrap();
            }
            out.push_str("}\n\n");
        }
    }
    out
}

/// One module per opaque type, re-exported flat from the `opaques` module.
fn generate_opaques_index(tcx: &TypeContext) -> String {
    let mut out = String::from("//! Generated opaque wrappers, one module per type.\n\n");
    let opaques: Vec<_> = tcx
        .opaques()
        .iter()
        .filter(|ty| !ty.attrs.disable)
        .collect();
    for opaque in &opaques {
        writeln!(out, "mod {};", opaque_module_name(opaque)).unwrap();
    }
    if !opaques.is_empty() {
        out.push('\n');
    }
    for opaque in &opaques {
        writeln!(out, "pub use {}::*;", opaque_module_name(opaque)).unwrap();
    }
    out
}

/// The contents of a single `src/opaques/<module>.rs` file.
fn generate_opaque_file(
    tcx: &TypeContext,
    opaque: &hir::OpaqueDef,
    docs_url_gen: &DocsUrlGenerator,
) -> String {
    let mut out = String::from(
        "use core::marker::PhantomData;\n\
         use core::ptr::NonNull;\n\
         use std::rc::Rc;\n\n\
         use crate::ffi;\n",
    );
    if opaque_uses_value_types(opaque) {
        out.push_str("use crate::types::*;\n");
    }
    out.push('\n');
    emit_opaque(&mut out, opaque, tcx, docs_url_gen);
    out
}

/// The module name used for an opaque type's generated file.
fn opaque_module_name(opaque: &hir::OpaqueDef) -> String {
    type_def_name(TypeDef::Opaque(opaque)).to_snake_case()
}

/// Whether a type mentions a generated enum or struct (both live in `types`).
fn references_value_type<P: hir::TyPosition>(ty: &Type<P>) -> bool {
    match ty {
        Type::Enum(_) | Type::Struct(_) => true,
        Type::DiplomatOption(inner) => references_value_type(inner.as_ref()),
        _ => false,
    }
}

fn opaque_uses_value_types(opaque: &hir::OpaqueDef) -> bool {
    opaque
        .methods
        .iter()
        .filter(|method| !method.attrs.disable)
        .any(|method| {
            let mut found = method
                .params
                .iter()
                .any(|param| references_value_type(&param.ty));
            if !found {
                method.output.with_contained_types(|ty| {
                    if references_value_type(ty) {
                        found = true;
                    }
                });
            }
            found
        })
}

/// The declared names of an opaque's type-level lifetimes, e.g. `["'a", "'b"]`.
fn opaque_lifetime_names(opaque: &hir::OpaqueDef) -> Vec<String> {
    lifetime_names(&opaque.lifetimes)
}

/// A value struct that carries lifetime parameters (and therefore borrowed
/// slice fields) rather than being a plain `repr(C)` value.
fn is_lifetime_struct(strct: &hir::StructDef) -> bool {
    strct.lifetimes.num_lifetimes() != 0
}

/// A field type that this backend can place in a generated struct.
fn supported_struct_field<P: hir::TyPosition>(ty: &Type<P>, tcx: &TypeContext) -> bool {
    match ty {
        Type::Primitive(primitive) => primitive_name(*primitive).is_some(),
        Type::Enum(path) => !path.resolve(tcx).attrs.disable,
        Type::Slice(slice) => is_supported_slice(slice),
        _ => false,
    }
}

/// The native ABI type of a struct field.
///
/// A struct field declaration cannot elide a lifetime parameter (unlike a
/// function signature), so a borrowed slice names the field's HIR lifetime.
fn ffi_struct_field_type(
    ty: &Type<hir::Everywhere>,
    strct: &hir::StructDef,
    tcx: &TypeContext,
) -> String {
    match ty {
        Type::Primitive(primitive) => primitive_name(*primitive).unwrap().into(),
        Type::Enum(path) => format!("super::{}", enum_name(path.tcx_id, tcx)),
        Type::Slice(slice) => ffi_struct_slice_type(slice, strct),
        _ => unreachable!("validated struct field"),
    }
}

/// The native ABI type of a borrowed slice field, with its lifetime spelled out.
fn ffi_struct_slice_type(slice: &Slice<hir::Everywhere>, strct: &hir::StructDef) -> String {
    let container = ffi_slice_container(slice);
    let element = slice_element_ty(slice);
    // Validation admits only borrowed slice fields carrying a lifetime; `'static`
    // is the same spelling the safe field type uses for a `MaybeStatic` lifetime.
    let lifetime = slice_lifetime(slice)
        .map(|lifetime| match lifetime {
            MaybeStatic::Static => "'static".to_string(),
            MaybeStatic::NonStatic(lifetime) => {
                format!("'{}", strct.lifetimes.fmt_lifetime(lifetime))
            }
        })
        .unwrap_or_else(|| "'static".to_string());
    format!("{container}<{lifetime}, {element}>")
}

/// The safe public Rust type of a struct field, e.g. `&'a [u16]`.
fn safe_struct_field_type(
    ty: &Type<hir::Everywhere>,
    strct: &hir::StructDef,
    tcx: &TypeContext,
) -> String {
    match ty {
        Type::Primitive(primitive) => primitive_name(*primitive).unwrap().into(),
        Type::Enum(path) => enum_name(path.tcx_id, tcx),
        Type::Slice(slice) => {
            let lifetime = slice_lifetime(slice)
                .map(|lifetime| match lifetime {
                    MaybeStatic::Static => "'static ".to_string(),
                    MaybeStatic::NonStatic(lifetime) => {
                        format!("'{} ", strct.lifetimes.fmt_lifetime(lifetime))
                    }
                })
                .unwrap_or_default();
            safe_slice_type(slice, &lifetime)
        }
        _ => unreachable!("validated struct field"),
    }
}

/// Convert a safe struct value into its native ABI mirror.
fn struct_input_expr(name: &str, strct: &hir::StructDef) -> String {
    let fields: Vec<String> = strct
        .fields
        .iter()
        .map(|field| {
            let access = format!("{name}.{f}", f = field_name(field));
            format!(
                "{f}: {value}",
                f = field_name(field),
                value = safe_field_to_ffi(&field.ty, &access)
            )
        })
        .collect();
    format!(
        "ffi::{name} {{ {fields} }}",
        name = type_def_name(TypeDef::Struct(strct)),
        fields = fields.join(", ")
    )
}

fn safe_field_to_ffi(ty: &Type<hir::Everywhere>, expr: &str) -> String {
    match ty {
        Type::Slice(slice) => ffi_borrowed_slice_expr(slice, expr),
        _ => expr.to_string(),
    }
}

/// Convert a native ABI struct mirror into the safe struct value.
fn struct_output_expr(raw: &str, strct: &hir::StructDef) -> String {
    let mut fields: Vec<String> = strct
        .fields
        .iter()
        .map(|field| {
            let access = format!("{raw}.{f}", f = field_name(field));
            format!(
                "{f}: {value}",
                f = field_name(field),
                value = ffi_field_to_safe(&field.ty, &access)
            )
        })
        .collect();
    if is_lifetime_struct(strct) {
        fields.push("_lifetimes: PhantomData".to_string());
    }
    format!(
        "{name} {{ {fields} }}",
        name = type_def_name(TypeDef::Struct(strct)),
        fields = fields.join(", ")
    )
}

fn ffi_field_to_safe(ty: &Type<hir::Everywhere>, expr: &str) -> String {
    match ty {
        Type::Slice(slice) => {
            if matches!(slice, Slice::Str(_, StringEncoding::Utf8)) {
                format!("unsafe {{ crate::private::utf8_str_from_slice({expr}) }}")
            } else {
                // The safe struct field declaration fixes the target type, so the
                // runtime's `From` conversion resolves by inference.
                format!("{expr}.into()")
            }
        }
        _ => expr.to_string(),
    }
}

/// The declared names of a set of type-level lifetimes, e.g. `["'a", "'b"]`.
fn lifetime_names(lifetimes: &hir::LifetimeEnv) -> Vec<String> {
    lifetimes
        .all_lifetimes()
        .map(|lifetime| format!("'{}", lifetimes.fmt_lifetime(lifetime)))
        .collect()
}

/// One lifetime parameter with its bounds: `'a` when it outlives nothing else,
/// otherwise `'a: 'b + 'c`, naming the shorter lifetimes it transitively outlives.
fn bounded_lifetime_name(lifetimes: &hir::LifetimeEnv, longer: hir::Lifetime) -> String {
    let name = format!("'{}", lifetimes.fmt_lifetime(longer));
    let shorter: Vec<String> = lifetimes
        .all_shorter_lifetimes(longer)
        .filter(|shorter| *shorter != longer)
        .map(|shorter| format!("'{}", lifetimes.fmt_lifetime(shorter)))
        .collect();
    if shorter.is_empty() {
        name
    } else {
        format!("{name}: {}", shorter.join(" + "))
    }
}

/// Render a generic parameter or argument list, e.g. `<a, b>`, or nothing when
/// there is nothing to declare.
fn render_generics(items: &[String]) -> String {
    if items.is_empty() {
        String::new()
    } else {
        format!("<{}>", items.join(", "))
    }
}

/// The generic parameter list (with bounds) and the use-site argument list for a
/// set of type-level lifetimes.
fn lifetime_generics(lifetimes: &hir::LifetimeEnv) -> (Vec<String>, Vec<String>) {
    let params: Vec<String> = lifetimes
        .all_lifetimes()
        .map(|longer| bounded_lifetime_name(lifetimes, longer))
        .collect();
    let args = lifetime_names(lifetimes);
    (params, args)
}

fn struct_generics(strct: &hir::StructDef) -> (String, String) {
    let (params, args) = lifetime_generics(&strct.lifetimes);
    (render_generics(&params), render_generics(&args))
}

/// The invariant `PhantomData` field of a lifetime-carrying struct.
fn struct_lifetime_phantom(strct: &hir::StructDef) -> String {
    lifetime_phantom(&lifetime_names(&strct.lifetimes))
}

/// An invariant `PhantomData` field tying a wrapper to the listed lifetimes.
fn lifetime_phantom(names: &[String]) -> String {
    if names.is_empty() {
        return String::new();
    }
    let refs: Vec<String> = names.iter().map(|name| format!("&{name} ()")).collect();
    let joined = refs.join(", ");
    let output = if refs.len() == 1 {
        refs[0].clone()
    } else {
        format!("({joined})")
    };
    format!("    pub(crate) _lifetimes: PhantomData<fn({joined}) -> {output}>,\n")
}

/// Generic parameter list (with bounds) and use-site argument list for an opaque.
/// A borrowing wrapper ([`Wrapper::Ref`] or [`Wrapper::RefMut`]) additionally
/// carries a leading `'view` borrow lifetime.
fn opaque_generics(opaque: &hir::OpaqueDef, wrapper: Wrapper) -> (String, String) {
    let (mut params, mut args) = lifetime_generics(&opaque.lifetimes);
    if wrapper.borrows() {
        params.insert(0, "'view".to_string());
        args.insert(0, "'view".to_string());
    }
    (render_generics(&params), render_generics(&args))
}

/// An invariant `PhantomData` field tying a wrapper to its type-level lifetimes.
fn opaque_lifetime_phantom(opaque: &hir::OpaqueDef) -> String {
    lifetime_phantom(&opaque_lifetime_names(opaque))
}

/// The safe public Rust type of an opaque output, including type-level lifetimes,
/// e.g. `Bar<'b, 'a>` or `FooRef<'x, 'a>`.
fn opaque_safe_type(
    path: &hir::OpaquePath<hir::Optional, MaybeOwn>,
    method: &hir::Method,
    tcx: &TypeContext,
) -> String {
    let name = opaque_name(path.tcx_id, tcx);
    let mut args = Vec::new();
    let mut head = format!("super::{name}");
    if let MaybeOwn::Borrow(borrow) = path.owner {
        args.push(lifetime_name(borrow.lifetime, method));
        head = if borrow.mutability == Mutability::Mutable {
            format!("super::{name}RefMut")
        } else {
            format!("super::{name}Ref")
        };
    }
    for lifetime in path.lifetimes.lifetimes() {
        args.push(lifetime_name(lifetime, method));
    }
    if args.is_empty() {
        head
    } else {
        format!("{head}<{}>", args.join(", "))
    }
}

fn emit_opaque(
    out: &mut String,
    opaque: &hir::OpaqueDef,
    tcx: &TypeContext,
    docs_url_gen: &DocsUrlGenerator,
) {
    let name = type_def_name(TypeDef::Opaque(opaque));
    let (owned_params, owned_args) = opaque_generics(opaque, Wrapper::Owned);
    let (ref_params, ref_args) = opaque_generics(opaque, Wrapper::Ref);
    let phantom = opaque_lifetime_phantom(opaque);
    emit_docs(out, &opaque.docs, docs_url_gen, "");
    writeln!(
        out,
        "pub struct {name}{owned_params} {{\n    pub(crate) inner: NonNull<ffi::{name}>,\n{phantom}    pub(crate) _not_send_sync: PhantomData<Rc<()>>,\n}}\n"
    )
    .unwrap();
    writeln!(
        out,
        "pub struct {name}Ref{ref_params} {{\n    pub(crate) inner: NonNull<ffi::{name}>,\n    pub(crate) _borrow: PhantomData<&'view ()>,\n{phantom}    pub(crate) _not_send_sync: PhantomData<Rc<()>>,\n}}\n"
    )
    .unwrap();
    writeln!(
        out,
        "pub struct {name}RefMut{ref_params} {{\n    pub(crate) inner: NonNull<ffi::{name}>,\n    pub(crate) _borrow: PhantomData<&'view mut ()>,\n{phantom}    pub(crate) _not_send_sync: PhantomData<Rc<()>>,\n}}\n"
    )
    .unwrap();

    writeln!(
        out,
        "#[doc(hidden)]\npub trait {name}SharedArg: crate::private::{name}SharedSealed {{}}\n"
    )
    .unwrap();
    writeln!(
        out,
        "#[doc(hidden)]\npub trait {name}MutArg: crate::private::{name}MutSealed {{}}\n"
    )
    .unwrap();
    writeln!(out, "impl{owned_params} crate::private::{name}SharedSealed for {name}{owned_args} {{ fn __as_const_ptr(&self) -> *const ffi::{name} {{ self.inner.as_ptr() }} }}").unwrap();
    writeln!(out, "impl{owned_params} crate::private::{name}MutSealed for {name}{owned_args} {{ fn __as_mut_ptr(&mut self) -> *mut ffi::{name} {{ self.inner.as_ptr() }} }}").unwrap();
    writeln!(
        out,
        "impl{owned_params} {name}SharedArg for {name}{owned_args} {{}}\nimpl{owned_params} {name}MutArg for {name}{owned_args} {{}}"
    )
    .unwrap();
    writeln!(out, "impl{ref_params} crate::private::{name}SharedSealed for {name}Ref{ref_args} {{ fn __as_const_ptr(&self) -> *const ffi::{name} {{ self.inner.as_ptr() }} }}").unwrap();
    writeln!(
        out,
        "impl{ref_params} {name}SharedArg for {name}Ref{ref_args} {{}}"
    )
    .unwrap();
    writeln!(out, "impl{ref_params} crate::private::{name}SharedSealed for {name}RefMut{ref_args} {{ fn __as_const_ptr(&self) -> *const ffi::{name} {{ self.inner.as_ptr() }} }}").unwrap();
    writeln!(out, "impl{ref_params} crate::private::{name}MutSealed for {name}RefMut{ref_args} {{ fn __as_mut_ptr(&mut self) -> *mut ffi::{name} {{ self.inner.as_ptr() }} }}").unwrap();
    writeln!(out, "impl{ref_params} {name}SharedArg for {name}RefMut{ref_args} {{}}\nimpl{ref_params} {name}MutArg for {name}RefMut{ref_args} {{}}\n").unwrap();

    writeln!(out, "impl{owned_params} Drop for {name}{owned_args} {{\n    fn drop(&mut self) {{\n        // SAFETY: this wrapper uniquely owns the non-null handle and calls the provider destructor once.\n        unsafe {{ ffi::{}(self.inner.as_ptr()) }};\n    }}\n}}\n", opaque.dtor_abi_name).unwrap();

    emit_impl(out, opaque, tcx, docs_url_gen, Wrapper::Owned);
    emit_impl(out, opaque, tcx, docs_url_gen, Wrapper::Ref);
    emit_impl(out, opaque, tcx, docs_url_gen, Wrapper::RefMut);
}

/// Which generated wrapper an impl block belongs to, named after the suffix each
/// wrapper carries: `Owned` is `T`, `Ref` is `TRef`, `RefMut` is `TRefMut`.
#[derive(Clone, Copy)]
enum Wrapper {
    Owned,
    /// `TRef`, holding a shared `'view` borrow of the opaque.
    Ref,
    /// `TRefMut`, holding an exclusive `'view` borrow of the opaque.
    RefMut,
}

impl Wrapper {
    /// Whether this wrapper holds a `'view` borrow of the opaque, as opposed to
    /// owning the handle outright.
    fn borrows(self) -> bool {
        !matches!(self, Self::Owned)
    }
}

fn emit_impl(
    out: &mut String,
    opaque: &hir::OpaqueDef,
    tcx: &TypeContext,
    docs_url_gen: &DocsUrlGenerator,
    wrapper: Wrapper,
) {
    let name = type_def_name(TypeDef::Opaque(opaque));
    let (params, args) = opaque_generics(opaque, wrapper);
    match wrapper {
        Wrapper::Owned => writeln!(out, "impl{params} {name}{args} {{").unwrap(),
        Wrapper::Ref => writeln!(out, "impl{params} {name}Ref{args} {{").unwrap(),
        Wrapper::RefMut => writeln!(out, "impl{params} {name}RefMut{args} {{").unwrap(),
    }
    for method in opaque.methods.iter().filter(|method| !method.attrs.disable) {
        let include = match (&method.param_self, wrapper) {
            (None, Wrapper::Owned) => true,
            (None, _) => false,
            (Some(param), Wrapper::Owned) => matches!(param.ty, SelfType::Opaque(_)),
            (Some(param), Wrapper::Ref) => param.ty.is_immutably_borrowed(),
            (Some(param), Wrapper::RefMut) => matches!(param.ty, SelfType::Opaque(_)),
        };
        let include = include
            && !matches!(
                (&method.param_self, wrapper),
                (Some(param), Wrapper::Ref) if param.ty.is_mutably_borrowed()
            );
        if include {
            emit_method(out, opaque, method, tcx, docs_url_gen);
        }
    }
    out.push_str("}\n\n");
}

fn emit_method(
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

fn input_expr(param: &hir::Param, tcx: &TypeContext) -> String {
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

fn return_expr(ret: &ReturnType, method: &hir::Method, tcx: &TypeContext) -> String {
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

fn opaque_return_expr(
    path: &hir::OpaquePath<hir::Optional, MaybeOwn>,
    _method: &hir::Method,
    tcx: &TypeContext,
) -> String {
    let name = opaque_name(path.tcx_id, tcx);
    let phantom = if opaque_lifetime_names(tcx.resolve_opaque(path.tcx_id)).is_empty() {
        ""
    } else {
        ", _lifetimes: PhantomData"
    };
    let construct = if path.owner.is_owned() {
        format!("super::{name} {{ inner{phantom}, _not_send_sync: PhantomData }}")
    } else if path.owner.mutability() == Mutability::Mutable {
        format!(
            "super::{name}RefMut {{ inner, _borrow: PhantomData{phantom}, _not_send_sync: PhantomData }}"
        )
    } else {
        format!("super::{name}Ref {{ inner, _borrow: PhantomData{phantom}, _not_send_sync: PhantomData }}")
    };
    if path.is_optional() {
        format!("NonNull::new(result as *mut _).map(|inner| {construct})")
    } else {
        format!(
            "{{ let inner = NonNull::new(result as *mut _).expect(\"Diplomat ABI returned null for non-null {name}\"); {construct} }}"
        )
    }
}

fn ffi_self_type(ty: &SelfType, tcx: &TypeContext) -> String {
    match ty {
        SelfType::Opaque(path) => {
            let name = opaque_name(path.tcx_id, tcx);
            match path.owner.mutability {
                Mutability::Immutable => format!("*const {name}"),
                Mutability::Mutable => format!("*mut {name}"),
            }
        }
        _ => unreachable!("validated method receiver"),
    }
}

fn ffi_input_type(ty: &Type<hir::InputOnly>, tcx: &TypeContext) -> String {
    match ty {
        Type::Opaque(path) => {
            let name = opaque_name(path.tcx_id, tcx);
            match path.owner.mutability {
                Mutability::Immutable => format!("*const {name}"),
                Mutability::Mutable => format!("*mut {name}"),
            }
        }
        Type::DiplomatOption(inner) => {
            format!("DiplomatOption<{}>", ffi_value_type(inner, tcx))
        }
        Type::Slice(slice) => ffi_slice_type(slice, ""),
        _ => ffi_value_type(ty, tcx),
    }
}

fn ffi_return_type(ret: &ReturnType, tcx: &TypeContext) -> String {
    match ret {
        ReturnType::Infallible(SuccessType::Unit) => "()".into(),
        ReturnType::Infallible(SuccessType::OutType(Type::Opaque(path))) => {
            let name = opaque_name(path.tcx_id, tcx);
            if path.owner.is_owned() || path.owner.mutability() == Mutability::Mutable {
                format!("*mut {name}")
            } else {
                format!("*const {name}")
            }
        }
        ReturnType::Infallible(SuccessType::OutType(Type::DiplomatOption(inner))) => {
            format!("DiplomatOption<{}>", ffi_value_type(inner.as_ref(), tcx))
        }
        ReturnType::Nullable(SuccessType::OutType(inner)) => {
            format!("DiplomatOption<{}>", ffi_value_type(inner, tcx))
        }
        ReturnType::Infallible(SuccessType::OutType(Type::Struct(
            ReturnableStructPath::Struct(path),
        ))) => {
            let strct = path.resolve(tcx);
            if is_lifetime_struct(strct) {
                let (_, args) = struct_generics(strct);
                format!("{}{args}", type_def_name(TypeDef::Struct(strct)))
            } else {
                ffi_value_name(TypeDef::Struct(strct))
            }
        }
        ReturnType::Infallible(SuccessType::OutType(Type::Slice(slice))) => {
            ffi_slice_type(slice, RETURN_LIFETIME)
        }
        ReturnType::Infallible(SuccessType::OutType(ty)) => ffi_value_type(ty, tcx),
        _ => unreachable!("validated return shape"),
    }
}

/// The lifetime name an extern declaration gives to a borrowed return value.
const RETURN_LIFETIME: &str = "'a";

/// The lifetime parameters an extern declaration must name for its return type.
///
/// A path lifetime can only be elided in return position when exactly one input
/// lifetime exists, and a raw-pointer receiver contributes none. So a borrowed
/// slice return gets one fresh parameter, and a lifetime-bearing struct return
/// names its own HIR lifetimes (the same ones its `repr(C)` mirror declares).
fn ffi_return_generics(ret: &ReturnType, tcx: &TypeContext) -> String {
    match ret {
        ReturnType::Infallible(SuccessType::OutType(Type::Slice(slice)))
            if !is_owned_slice(slice) =>
        {
            format!("<{RETURN_LIFETIME}>")
        }
        ReturnType::Infallible(SuccessType::OutType(Type::Struct(
            ReturnableStructPath::Struct(path),
        ))) => {
            let strct = path.resolve(tcx);
            if is_lifetime_struct(strct) {
                struct_generics(strct).0
            } else {
                String::new()
            }
        }
        _ => String::new(),
    }
}

fn safe_input_type(ty: &Type<hir::InputOnly>, method: &hir::Method, tcx: &TypeContext) -> String {
    match ty {
        Type::Opaque(path) => {
            let name = opaque_name(path.tcx_id, tcx);
            let lifetime = lifetime_prefix(path.owner.lifetime, method);
            match path.owner.mutability {
                Mutability::Immutable => format!("&{lifetime}impl super::{name}SharedArg"),
                Mutability::Mutable => format!("&{lifetime}mut impl super::{name}MutArg"),
            }
        }
        Type::DiplomatOption(inner) => {
            format!("Option<{}>", safe_value_type(inner, tcx))
        }
        Type::Slice(slice) => {
            let lifetime = slice_lifetime(slice)
                .map(|lifetime| lifetime_prefix(lifetime, method))
                .unwrap_or_default();
            safe_slice_type(slice, &lifetime)
        }
        Type::Struct(path) => {
            let name = type_def_name(tcx.resolve_type(path.id()));
            let args: Vec<String> = path
                .lifetimes()
                .lifetimes()
                .map(|lifetime| lifetime_name(lifetime, method))
                .collect();
            if args.is_empty() {
                name
            } else {
                format!("{name}<{}>", args.join(", "))
            }
        }
        _ => safe_value_type(ty, tcx),
    }
}

fn safe_return_type(ret: &ReturnType, method: &hir::Method, tcx: &TypeContext) -> String {
    match ret {
        ReturnType::Infallible(SuccessType::Unit) => "()".into(),
        ReturnType::Infallible(SuccessType::OutType(Type::Opaque(path))) => {
            let base = opaque_safe_type(path, method, tcx);
            if path.is_optional() {
                format!("Option<{base}>")
            } else {
                base
            }
        }
        ReturnType::Infallible(SuccessType::OutType(Type::Slice(slice))) => {
            let lifetime = slice_lifetime(slice)
                .map(|lifetime| format!("{} ", lifetime_name(lifetime, method)))
                .unwrap_or_default();
            safe_slice_type(slice, &lifetime)
        }
        ReturnType::Infallible(SuccessType::OutType(Type::Struct(
            ReturnableStructPath::Struct(path),
        ))) => {
            let name = type_def_name(TypeDef::Struct(path.resolve(tcx)));
            let args: Vec<String> = path
                .lifetimes()
                .lifetimes()
                .map(|lifetime| lifetime_name(lifetime, method))
                .collect();
            if args.is_empty() {
                name
            } else {
                format!("{name}<{}>", args.join(", "))
            }
        }
        ReturnType::Infallible(SuccessType::OutType(Type::DiplomatOption(inner))) => {
            format!("Option<{}>", safe_value_type(inner.as_ref(), tcx))
        }
        ReturnType::Nullable(SuccessType::OutType(inner)) => {
            format!("Option<{}>", safe_value_type(inner, tcx))
        }
        ReturnType::Infallible(SuccessType::OutType(ty)) => safe_value_type(ty, tcx),
        _ => unreachable!("validated return shape"),
    }
}

/// How the native ABI layer names a value type definition: enums and plain value
/// structs live in the parent module, while a lifetime-carrying struct is emitted
/// at the crate root.
fn ffi_value_name(def: TypeDef<'_>) -> String {
    let name = type_def_name(def);
    match def {
        TypeDef::Struct(strct) if is_lifetime_struct(strct) => name,
        _ => format!("super::{name}"),
    }
}

/// The native ABI Rust type of a value type: a primitive, enum, or struct.
///
/// Input and output positions share it, since validation rejects output-only
/// structs before generation runs.
fn ffi_value_type<P: hir::TyPosition>(ty: &Type<P>, tcx: &TypeContext) -> String {
    match ty {
        Type::Primitive(p) => primitive_name(*p).unwrap().into(),
        Type::Enum(path) => ffi_value_name(TypeDef::Enum(path.resolve(tcx))),
        Type::Struct(path) => ffi_value_name(tcx.resolve_type(path.id())),
        _ => unreachable!("validated value type"),
    }
}

/// The safe public Rust type of a value type: a primitive, enum, or struct.
///
/// Input and output positions share it, since validation rejects output-only
/// structs before generation runs.
fn safe_value_type<P: hir::TyPosition>(ty: &Type<P>, tcx: &TypeContext) -> String {
    match ty {
        Type::Primitive(p) => primitive_name(*p).unwrap().into(),
        Type::Enum(path) => type_def_name(TypeDef::Enum(path.resolve(tcx))),
        Type::Struct(path) => type_def_name(tcx.resolve_type(path.id())),
        _ => unreachable!("validated safe value type"),
    }
}

fn primitive_name(primitive: PrimitiveType) -> Option<&'static str> {
    match primitive {
        PrimitiveType::Bool => Some("bool"),
        PrimitiveType::Byte => Some("u8"),
        PrimitiveType::Int(value) => Some(value.as_str()),
        PrimitiveType::IntSize(value) => Some(value.as_str()),
        // Floats are FFI-safe scalars and this backend is Rust-to-Rust, so `f32`/`f64`
        // are the ABI type and the safe type at once; no conversion in either
        // direction.
        PrimitiveType::Float(value) => Some(value.as_str()),
        // `char` arrives as a `DiplomatChar` (a `u32` scalar), so accepting it needs a
        // decision about validating the code point on the way in. 128-bit integers are
        // not FFI-safe on every target, and `Ordering` has no agreed ABI shape.
        PrimitiveType::Char | PrimitiveType::Ordering | PrimitiveType::Int128(_) => None,
    }
}

/// Whether a method must declare its method-level lifetimes: it does when any
/// lifetime appears in the output, or when a struct parameter names one.
fn method_declares_lifetimes(method: &hir::Method, tcx: &TypeContext) -> bool {
    if !method.output.used_method_lifetimes().is_empty() {
        return true;
    }
    method.params.iter().any(|param| match &param.ty {
        Type::Struct(path) => matches!(
            tcx.resolve_type(path.id()),
            TypeDef::Struct(strct) if is_lifetime_struct(strct)
        ),
        _ => false,
    })
}

fn method_generics(method: &hir::Method, skip: usize, tcx: &TypeContext) -> String {
    if !method_declares_lifetimes(method, tcx) {
        return String::new();
    }
    let params: Vec<String> = method
        .lifetime_env
        .all_lifetimes()
        .skip(skip)
        .map(|longer| bounded_lifetime_name(&method.lifetime_env, longer))
        .collect();
    render_generics(&params)
}

fn lifetime_name(lifetime: MaybeStatic<hir::Lifetime>, method: &hir::Method) -> String {
    match lifetime {
        MaybeStatic::Static => "'static".into(),
        MaybeStatic::NonStatic(lt) => format!("'{}", method.lifetime_env.fmt_lifetime(lt)),
    }
}

fn lifetime_prefix(lifetime: MaybeStatic<hir::Lifetime>, method: &hir::Method) -> String {
    match lifetime {
        MaybeStatic::Static => "'static ".into(),
        MaybeStatic::NonStatic(lt) if !method.output.used_method_lifetimes().is_empty() => {
            format!("'{} ", method.lifetime_env.fmt_lifetime(lt))
        }
        MaybeStatic::NonStatic(_) => String::new(),
    }
}

fn type_def_name(def: TypeDef<'_>) -> String {
    def.attrs()
        .rename
        .apply(Cow::Borrowed(def.name().as_str()))
        .into_owned()
}

fn opaque_name(id: hir::OpaqueId, tcx: &TypeContext) -> String {
    type_def_name(TypeDef::Opaque(tcx.resolve_opaque(id)))
}

fn enum_name(id: hir::EnumId, tcx: &TypeContext) -> String {
    type_def_name(TypeDef::Enum(tcx.resolve_enum(id)))
}

/// The name a method is emitted under.
///
/// An explicit `#[diplomat::attr(named_constructor = "name")]` replaces the Rust
/// method name; a bare `named_constructor` leaves it alone. `rename` is applied on
/// top of whichever name was declared, matching the other backends.
fn method_name(method: &hir::Method) -> String {
    let declared = match &method.attrs.special_method {
        Some(hir::SpecialMethod::NamedConstructor(Some(name))) => name.as_str(),
        _ => method.name.as_str(),
    };
    method
        .attrs
        .rename
        .apply(Cow::Borrowed(declared))
        .into_owned()
}

fn field_name<P: hir::TyPosition>(field: &hir::StructField<P>) -> String {
    field
        .attrs
        .rename
        .apply(Cow::Borrowed(field.name.as_str()))
        .into_owned()
}

fn enum_variant_name(variant: &hir::EnumVariant) -> String {
    variant
        .attrs
        .rename
        .apply(Cow::Borrowed(variant.name.as_str()))
        .into_owned()
}

fn emit_docs(out: &mut String, docs: &hir::Docs, docs_url_gen: &DocsUrlGenerator, indent: &str) {
    for line in docs
        .to_markdown(hir::DocsTypeReferenceSyntax::SquareBrackets, docs_url_gen)
        .lines()
    {
        writeln!(out, "{indent}/// {line}").unwrap();
    }
}

fn valid_rust_ident(name: &str) -> bool {
    syn::parse_str::<syn::Ident>(name).is_ok()
}

fn sanitize_package_component(name: &str) -> String {
    let sanitized: String = name
        .chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() || matches!(c, '-' | '_') {
                c.to_ascii_lowercase()
            } else {
                '-'
            }
        })
        .collect();
    sanitized.trim_matches('-').to_string()
}

fn valid_package_name(name: &str) -> bool {
    !name.is_empty()
        && name
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || matches!(c, '-' | '_'))
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use diplomat_core::hir::{BasicAttributeValidator, DocsUrlGenerator, TypeContext};
    use quote::quote;

    use crate::Config;

    /// Concatenates every generated Rust source, for assertions about *what* is
    /// generated rather than *which file* it lands in.
    fn all_rust_sources(files: &HashMap<String, String>) -> String {
        let mut paths: Vec<&String> = files.keys().filter(|path| path.ends_with(".rs")).collect();
        paths.sort();
        paths
            .into_iter()
            .map(|path| files[path].as_str())
            .collect::<Vec<_>>()
            .join("\n")
    }

    fn generate(tokens: proc_macro2::TokenStream) -> (HashMap<String, String>, Vec<String>) {
        let file = syn::parse2::<syn::File>(tokens).unwrap();
        let mut validator = BasicAttributeValidator::new("rust");
        validator.support = super::attr_support();
        let tcx = TypeContext::from_syn(
            &file,
            Default::default(),
            validator,
            None,
            &diplomat_core::ast::SpanLocation::None,
        )
        .unwrap_or_else(|errors| panic!("HIR errors: {errors:#?}"));
        let mut config = Config::default();
        config.shared_config.lib_name = Some("safe_rust_test".into());
        let docs = DocsUrlGenerator::with_base_urls(None, HashMap::new());
        let (files, errors) = super::run(&tcx, &config, &docs);
        (
            files.take_files(),
            errors
                .take_all()
                .into_iter()
                .map(|(context, error)| format!("{context}: {error}"))
                .collect(),
        )
    }

    #[test]
    fn opaque_api_is_safe_and_raw_layer_is_private() {
        let (files, errors) = generate(quote! {
            #[diplomat::bridge]
            mod ffi {
                #[diplomat::opaque_mut]
                pub struct Counter(u32);
                impl Counter {
                    pub fn new(value: u32) -> Box<Self> { unimplemented!() }
                    pub fn increment(&mut self) { unimplemented!() }
                    pub fn get(&self) -> u32 { unimplemented!() }
                    pub fn same(&self, other: &Counter) -> bool { unimplemented!() }
                }
            }
        });
        assert!(errors.is_empty(), "{errors:#?}");
        let safe = &all_rust_sources(&files);
        let raw = &files["src/ffi.rs"];
        assert!(safe.contains("mod ffi;"));
        assert!(safe.contains("pub struct Counter"));
        assert!(safe.contains("impl Drop for Counter"));
        assert!(safe.contains("pub fn increment(&mut self)"));
        assert!(safe.contains("other: &impl super::CounterSharedArg"));
        assert!(!safe.contains("pub mod ffi"));
        assert!(raw.contains("extern \"C\""));
        assert!(raw.contains("Counter_destroy"));
    }

    #[test]
    fn borrowed_return_preserves_hir_lifetime() {
        let (files, errors) = generate(quote! {
            #[diplomat::bridge]
            mod ffi {
                #[diplomat::opaque]
                pub struct Child(u32);
                #[diplomat::opaque]
                pub struct Parent(Child);
                impl Parent {
                    pub fn child<'a>(&'a self) -> &'a Child { unimplemented!() }
                }
            }
        });
        assert!(errors.is_empty(), "{errors:#?}");
        assert!(
            all_rust_sources(&files).contains("pub fn child<'a>(&'a self) -> super::ChildRef<'a>")
        );
    }

    #[test]
    fn borrowed_return_preserves_transitive_input_bound() {
        let (files, errors) = generate(quote! {
            #[diplomat::bridge]
            mod ffi {
                #[diplomat::opaque]
                pub struct Child(u32);
                #[diplomat::opaque]
                pub struct Parent(Child);
                impl Parent {
                    pub fn child<'short, 'long: 'short>(&'long self) -> &'short Child {
                        unimplemented!()
                    }
                }
            }
        });
        assert!(errors.is_empty(), "{errors:#?}");
        let safe = &all_rust_sources(&files);
        assert!(safe.contains("'long: 'short"), "{safe}");
        assert!(
            safe.contains("&'long self) -> super::ChildRef<'short>"),
            "{safe}"
        );
        assert!(!safe.contains("'long, 'long:"), "{safe}");
    }

    #[test]
    fn value_types_and_options_are_lowered() {
        let (files, errors) = generate(quote! {
            #[diplomat::bridge]
            mod ffi {
                pub enum Mode { First = 0, Second = 2 }
                pub struct Snapshot { pub value: u32, pub enabled: bool, pub mode: Mode }
                #[diplomat::opaque]
                pub struct Counter(u32);
                impl Counter {
                    pub fn snapshot(&self) -> Snapshot { unimplemented!() }
                    pub fn maybe_value(&self, value: Option<u32>) -> Option<u32> { unimplemented!() }
                }
            }
        });
        assert!(errors.is_empty(), "{errors:#?}");
        let safe = &all_rust_sources(&files);
        assert!(safe.contains("pub enum Mode"));
        assert!(safe.contains("pub struct Snapshot"));
        assert!(safe.contains("value: Option<u32>"));
        assert!(files["src/ffi.rs"].contains("DiplomatOption<u32>"));
    }

    #[test]
    fn unsupported_slice_produces_no_files() {
        let (files, errors) = generate(quote! {
            #[diplomat::bridge]
            mod ffi {
                #[diplomat::opaque]
                pub struct Counter(u32);
                impl Counter {
                    // `i128` stays outside the supported primitive subset, so a slice of
                    // it is still rejected. (This case used `f64` until floats were
                    // added to the subset.)
                    pub fn consume(&self, samples: &[i128]) { unimplemented!() }
                }
            }
        });
        assert!(files.is_empty());
        assert!(errors
            .iter()
            .any(|error| error.contains("unsupported parameter type")));
    }

    #[test]
    fn primitive_subset_is_explicit() {
        use diplomat_core::hir::{FloatType, Int128Type, IntSizeType, IntType, PrimitiveType};

        for primitive in [
            PrimitiveType::Bool,
            PrimitiveType::Byte,
            PrimitiveType::Int(IntType::I8),
            PrimitiveType::Int(IntType::I16),
            PrimitiveType::Int(IntType::I32),
            PrimitiveType::Int(IntType::I64),
            PrimitiveType::Int(IntType::U8),
            PrimitiveType::Int(IntType::U16),
            PrimitiveType::Int(IntType::U32),
            PrimitiveType::Int(IntType::U64),
            PrimitiveType::IntSize(IntSizeType::Isize),
            PrimitiveType::IntSize(IntSizeType::Usize),
            PrimitiveType::Float(FloatType::F32),
            PrimitiveType::Float(FloatType::F64),
        ] {
            assert!(super::primitive_name(primitive).is_some());
        }
        // Deliberately still out of the subset: `char` awaits a code-point validity
        // decision, `Ordering` has no agreed ABI shape, and 128-bit integers are not
        // FFI-safe on every target. A `Some` here would be a promise we cannot keep.
        for primitive in [
            PrimitiveType::Char,
            PrimitiveType::Ordering,
            PrimitiveType::Int128(Int128Type::I128),
            PrimitiveType::Int128(Int128Type::U128),
        ] {
            assert!(super::primitive_name(primitive).is_none());
        }
    }

    #[test]
    fn rust_config_accepts_package_and_dylib_names() {
        let mut config = super::RustConfig::default();
        config.set("crate_name", toml::Value::String("safe-bindings".into()));
        config.set("dylib_name", toml::Value::String("native_owner".into()));
        assert_eq!(config.crate_name.as_deref(), Some("safe-bindings"));
        assert_eq!(config.dylib_name.as_deref(), Some("native_owner"));
    }

    #[test]
    fn optional_owned_and_borrowed_opaques_are_distinct() {
        let (files, errors) = generate(quote! {
            #[diplomat::bridge]
            mod ffi {
                #[diplomat::opaque]
                pub struct Child(u32);
                #[diplomat::opaque]
                pub struct Parent(Child);
                impl Parent {
                    pub fn maybe_new(present: bool) -> Option<Box<Self>> { unimplemented!() }
                    pub fn maybe_child<'a>(&'a self, present: bool) -> Option<&'a Child> { unimplemented!() }
                }
            }
        });
        assert!(errors.is_empty(), "{errors:#?}");
        let safe = &all_rust_sources(&files);
        assert!(safe.contains("-> Option<super::Parent>"));
        assert!(safe.contains("-> Option<super::ChildRef<'a>>"));
        assert!(safe.contains("NonNull::new(result as *mut _).map"));
    }

    #[test]
    fn fallible_results_are_rejected_without_partial_output() {
        let (files, errors) = generate(quote! {
            #[diplomat::bridge]
            mod ffi {
                #[diplomat::opaque]
                pub struct Counter(u32);
                impl Counter {
                    pub fn get(&self) -> Result<u32, u32> { unimplemented!() }
                }
            }
        });
        assert!(files.is_empty());
        assert!(errors
            .iter()
            .any(|error| error.contains("unsupported return type")));
    }

    #[test]
    fn str_parameters_are_lowered_to_byte_slices() {
        let (files, errors) = generate(quote! {
            #[diplomat::bridge]
            mod ffi {
                #[diplomat::opaque]
                pub struct Echo(String);
                impl Echo {
                    pub fn new(v: &str) -> Box<Self> { unimplemented!() }
                }
            }
        });
        assert!(errors.is_empty(), "{errors:#?}");
        let safe = &all_rust_sources(&files);
        assert!(safe.contains("pub fn new(v: &str)"), "{safe}");
        // A `&str` cannot become a `DiplomatSlice<u8>` by conversion alone — the
        // runtime's `DiplomatUtf8StrSlice` field is private — so it goes via bytes.
        assert!(
            safe.contains("ffi::DiplomatSlice::from(v.as_bytes())"),
            "{safe}"
        );
    }

    #[test]
    fn borrowed_slices_and_strings_are_lowered() {
        let (files, errors) = generate(quote! {
            #[diplomat::bridge]
            mod ffi {
                use diplomat_runtime::{DiplomatStr, DiplomatStrSlice};

                #[diplomat::opaque_mut]
                pub struct U32Vec(Vec<u32>);
                impl U32Vec {
                    pub fn new(v: &[u32]) -> Box<Self> { unimplemented!() }
                    pub fn borrow<'a>(&'a self) -> &'a [u32] { unimplemented!() }
                    pub fn fill_slice(&self, v: &mut [u32]) { unimplemented!() }
                }

                #[diplomat::opaque_mut]
                pub struct MyString(String);
                impl MyString {
                    pub fn new(v: &DiplomatStr) -> Box<Self> { unimplemented!() }
                    pub fn get<'a>(&'a self) -> DiplomatStrSlice<'a> { unimplemented!() }
                    pub fn as_str<'a>(&'a self) -> &'a str { unimplemented!() }
                }
            }
        });
        assert!(errors.is_empty(), "{errors:#?}");
        let safe = &all_rust_sources(&files);
        assert!(
            safe.contains("pub fn borrow<'a>(&'a self) -> &'a [u32]"),
            "{safe}"
        );
        assert!(
            safe.contains("pub fn fill_slice(&self, v: &mut [u32])"),
            "{safe}"
        );
        assert!(
            safe.contains("pub fn get<'a>(&'a self) -> &'a [u8]"),
            "{safe}"
        );
        assert!(
            safe.contains("pub fn as_str<'a>(&'a self) -> &'a str"),
            "{safe}"
        );
        let ffi = &files["src/ffi.rs"];
        assert!(ffi.contains("DiplomatSlice<u32>"), "{ffi}");
        assert!(ffi.contains("DiplomatSliceMut<u32>"), "{ffi}");
        assert!(
            ffi.contains("pub(super) use diplomat_runtime::{"),
            "the FFI layer must re-export the runtime ABI types: {ffi}"
        );
        assert!(
            !ffi.contains("struct DiplomatSlice"),
            "the FFI layer must not re-declare ABI types: {ffi}"
        );
        assert!(
            safe.contains("crate::private::utf8_str_from_slice(result)"),
            "{safe}"
        );
        assert!(!safe.contains("slice_from_raw_parts"), "{safe}");
    }

    #[test]
    fn slice_output_lifetime_is_tied_to_receiver() {
        let (files, errors) = generate(quote! {
            #[diplomat::bridge]
            mod ffi {
                #[diplomat::opaque_mut]
                pub struct Buf(Vec<u8>);
                impl Buf {
                    pub fn bytes<'a>(&'a self) -> &'a [u8] { unimplemented!() }
                    pub fn bytes_mut<'a>(&'a mut self) -> &'a mut [u8] { unimplemented!() }
                }
            }
        });
        assert!(errors.is_empty(), "{errors:#?}");
        let safe = &all_rust_sources(&files);
        assert!(
            safe.contains("pub fn bytes<'a>(&'a self) -> &'a [u8]"),
            "{safe}"
        );
        assert!(
            safe.contains("pub fn bytes_mut<'a>(&'a mut self) -> &'a mut [u8]"),
            "{safe}"
        );
    }
    #[test]
    fn owned_slice_returns_are_boxed() {
        let (files, errors) = generate(quote! {
            #[diplomat::bridge]
            mod ffi {
                #[diplomat::opaque]
                pub struct OwnedSliceReturn;
                impl OwnedSliceReturn {
                    pub fn make(len: u32) -> Box<[u8]> { unimplemented!() }
                }
            }
        });
        assert!(errors.is_empty(), "{errors:#?}");
        let safe = &all_rust_sources(&files);
        let ffi = &files["src/ffi.rs"];
        assert!(
            safe.contains("pub fn make(len: u32) -> Box<[u8]>"),
            "{safe}"
        );
        assert!(safe.contains("Box::from(result)"), "{safe}");
        assert!(ffi.contains("DiplomatOwnedSlice<u8>"), "{ffi}");
    }

    #[test]
    fn lifetime_structs_are_converted() {
        let (files, errors) = generate(quote! {
            #[diplomat::bridge]
            mod ffi {
                use diplomat_runtime::{DiplomatStr, DiplomatStrSlice};

                pub struct BorrowedFields<'a> {
                    a: DiplomatStrSlice<'a>,
                    b: u32,
                }

                #[diplomat::opaque]
                pub struct Foo<'a>(&'a DiplomatStr);

                impl<'a> Foo<'a> {
                    pub fn as_fields(&self) -> BorrowedFields<'a> { unimplemented!() }
                    pub fn from_fields(fields: BorrowedFields<'a>) -> Box<Self> { unimplemented!() }
                }
            }
        });
        assert!(errors.is_empty(), "{errors:#?}");
        let safe = &all_rust_sources(&files);
        let ffi = &files["src/ffi.rs"];
        assert!(safe.contains("pub struct BorrowedFields<'a>"), "{safe}");
        assert!(safe.contains("pub a: &'a [u8]"), "{safe}");
        assert!(safe.contains("pub b: u32"), "{safe}");
        assert!(
            safe.contains("pub fn as_fields<'anon_0>(&'anon_0 self) -> BorrowedFields<'a>")
                || safe.contains("-> BorrowedFields<'a>"),
            "{safe}"
        );
        assert!(ffi.contains("pub struct BorrowedFields<'a> {"), "{ffi}");
        assert!(ffi.contains("a: DiplomatSlice<'a, u8>"), "{ffi}");
    }

    #[test]
    fn opaque_type_lifetimes_are_threaded() {
        let (files, errors) = generate(quote! {
            #[diplomat::bridge]
            mod ffi {
                use diplomat_runtime::DiplomatStr;
                #[diplomat::opaque]
                pub struct Foo<'a>(&'a DiplomatStr);
                #[diplomat::opaque]
                pub struct Bar<'b, 'a: 'b>(&'b Foo<'a>);
                impl<'a> Foo<'a> {
                    pub fn new(x: &'a DiplomatStr) -> Box<Self> { unimplemented!() }
                    pub fn get_bar<'b>(&'b self) -> Box<Bar<'b, 'a>> { unimplemented!() }
                }
            }
        });
        assert!(errors.is_empty(), "{errors:#?}");
        let safe = &all_rust_sources(&files);
        assert!(safe.contains("pub struct Foo<'a>"), "{safe}");
        assert!(safe.contains("impl<'a> Drop for Foo<'a>"), "{safe}");
        assert!(safe.contains("pub struct Bar<'b, 'a: 'b>"), "{safe}");
        assert!(safe.contains("impl<'a> Foo<'a> {"), "{safe}");
        assert!(
            safe.contains("pub fn new(x: &'a [u8]) -> super::Foo<'a>"),
            "{safe}"
        );
        assert!(
            safe.contains("pub fn get_bar<'b>(&'b self) -> super::Bar<'b, 'a>"),
            "{safe}"
        );
    }

    /// `#[diplomat::attr(rust, disable)]` types are never generated, so a signature
    /// that mentions one has to be rejected. It used to be accepted — enums were
    /// checked but structs and opaques were not — and the emitted package then
    /// referenced types that do not exist and could not compile.
    #[test]
    fn disabled_types_in_signatures_are_rejected() {
        let (files, errors) = generate(quote! {
            #[diplomat::bridge]
            mod ffi {
                #[diplomat::opaque]
                #[diplomat::attr(rust, disable)]
                pub struct HiddenOpaque(u32);

                #[diplomat::attr(rust, disable)]
                pub struct HiddenStruct {
                    pub value: u32,
                }

                #[diplomat::attr(rust, disable)]
                pub enum HiddenEnum {
                    Only,
                }

                pub struct Holder {
                    pub hidden: HiddenEnum,
                }

                #[diplomat::opaque]
                pub struct Visible(u32);

                impl Visible {
                    pub fn takes_disabled_opaque(&self, hidden: &HiddenOpaque) { unimplemented!() }
                    pub fn takes_disabled_struct(&self, hidden: HiddenStruct) { unimplemented!() }
                    pub fn takes_optional_disabled(&self, hidden_opt: Option<HiddenStruct>) { unimplemented!() }
                    pub fn returns_disabled_opaque(&self) -> Box<HiddenOpaque> { unimplemented!() }
                }
            }
        });
        for expected in [
            "disabled type `HiddenOpaque` as parameter `hidden`",
            "disabled type `HiddenStruct` as parameter `hidden_opt`",
            "disabled type `HiddenOpaque` in the return type",
            "structs may contain only supported primitives, enums, and borrowed slices",
        ] {
            assert!(
                errors.iter().any(|error| error.contains(expected)),
                "missing {expected:?} in {errors:#?}"
            );
        }
        assert!(
            files.is_empty(),
            "nothing may be emitted for a rejected signature: {:#?}",
            files.keys()
        );
    }

    /// An explicit `named_constructor` name is the emitted function name; a bare
    /// `named_constructor` keeps the Rust method name. The name used to be discarded
    /// silently, so the declaration and the generated API disagreed.
    #[test]
    fn named_constructor_names_are_honoured() {
        let (files, errors) = generate(quote! {
            #[diplomat::bridge]
            mod ffi {
                #[diplomat::opaque]
                pub struct Thing(u32);
                impl Thing {
                    #[diplomat::attr(auto, named_constructor = "with_value")]
                    pub fn new_named(value: u32) -> Box<Self> { unimplemented!() }
                    #[diplomat::attr(auto, named_constructor)]
                    pub fn new_plain(value: u32) -> Box<Self> { unimplemented!() }
                }
            }
        });
        assert!(errors.is_empty(), "{errors:#?}");
        let safe = &all_rust_sources(&files);
        assert!(safe.contains("pub fn with_value(value: u32)"), "{safe}");
        assert!(safe.contains("pub fn new_plain(value: u32)"), "{safe}");
        assert!(!safe.contains("pub fn new_named("), "{safe}");
    }

    /// A `#[diplomat::cfg(supports = ...)]` method is generated only when the backend
    /// declares that flag. This ties each declaration in `attr_support` to an
    /// observable effect, so a flag cannot be dropped or added without changing what
    /// comes out.
    #[test]
    fn capability_flags_gate_generated_apis() {
        let (files, errors) = generate(quote! {
            #[diplomat::bridge]
            mod ffi {
                #[diplomat::opaque]
                pub struct Thing(Vec<u16>);
                impl Thing {
                    #[diplomat::cfg(supports = utf16_strings)]
                    pub fn utf16_len(&self) -> u32 { unimplemented!() }
                    #[diplomat::cfg(supports = callbacks)]
                    pub fn needs_callbacks(&self) { unimplemented!() }
                }
            }
        });
        assert!(errors.is_empty(), "{errors:#?}");
        let safe = &all_rust_sources(&files);
        assert!(safe.contains("pub fn utf16_len("), "{safe}");
        assert!(!safe.contains("needs_callbacks"), "{safe}");
    }

    /// Generated module names follow the repo-wide `heck` snake_case convention, the
    /// one every sibling backend's formatter uses. This is deliberately different from
    /// a naive per-underscore split for acronyms: `HTTPServer` becomes `http_server`,
    /// not `h_t_t_p_server`.
    #[test]
    fn opaque_module_names_use_snake_case() {
        let (files, errors) = generate(quote! {
            #[diplomat::bridge]
            mod ffi {
                #[diplomat::opaque]
                pub struct HTTPServer(u32);
                impl HTTPServer {
                    pub fn new() -> Box<Self> { unimplemented!() }
                }
            }
        });
        assert!(errors.is_empty(), "{errors:#?}");
        assert!(
            files.contains_key("src/opaques/http_server.rs"),
            "{:#?}",
            files.keys()
        );
    }

    /// Floats are FFI-safe scalars and the backend is Rust-to-Rust, so they need no
    /// conversion: the same `f32`/`f64` is the ABI type and the safe type.
    #[test]
    fn float_scalars_and_slices_are_lowered() {
        let (files, errors) = generate(quote! {
            #[diplomat::bridge]
            mod ffi {
                #[diplomat::opaque_mut]
                pub struct Floats(Vec<f64>);
                impl Floats {
                    pub fn new(values: &[f64]) -> Box<Self> { unimplemented!() }
                    pub fn scale(&mut self, factor: f64) -> f64 { unimplemented!() }
                    pub fn narrow(&self) -> f32 { unimplemented!() }
                    pub fn values<'a>(&'a self) -> &'a [f64] { unimplemented!() }
                }
            }
        });
        assert!(errors.is_empty(), "{errors:#?}");
        let safe = &all_rust_sources(&files);
        assert!(safe.contains("pub fn new(values: &[f64])"), "{safe}");
        assert!(
            safe.contains("pub fn scale(&mut self, factor: f64) -> f64"),
            "{safe}"
        );
        assert!(safe.contains("pub fn narrow(&self) -> f32"), "{safe}");
        assert!(
            safe.contains("pub fn values<'a>(&'a self) -> &'a [f64]"),
            "{safe}"
        );
    }

    /// The primitives left out of the subset must still stop generation outright,
    /// rather than emitting bindings that quietly drop the method.
    #[test]
    fn unsupported_primitives_still_reject_without_output() {
        for tokens in [
            quote! {
                #[diplomat::bridge]
                mod ffi {
                    #[diplomat::opaque]
                    pub struct Wide(u128);
                    impl Wide {
                        pub fn get(&self) -> u128 { unimplemented!() }
                    }
                }
            },
            quote! {
                #[diplomat::bridge]
                mod ffi {
                    use diplomat_runtime::DiplomatChar;

                    #[diplomat::opaque]
                    pub struct Letter(u32);
                    impl Letter {
                        pub fn get(&self) -> DiplomatChar { unimplemented!() }
                    }
                }
            },
        ] {
            let (files, errors) = generate(tokens);
            assert!(!errors.is_empty(), "expected a diagnostic");
            assert!(files.is_empty(), "{:#?}", files.keys());
        }
    }

    /// A plain `#[diplomat::attr(auto, constructor)]` is accepted and emitted. For Rust
    /// that is an ordinary associated function, so its generated name is just the
    /// method's own name — the flag is a promise that such methods are lowered at all.
    #[test]
    fn plain_constructors_are_accepted() {
        let (files, errors) = generate(quote! {
            #[diplomat::bridge]
            mod ffi {
                #[diplomat::opaque]
                pub struct Thing(u32);
                impl Thing {
                    #[diplomat::attr(auto, constructor)]
                    pub fn create(value: u32) -> Box<Self> { unimplemented!() }
                }
            }
        });
        assert!(errors.is_empty(), "{errors:#?}");
        let safe = &all_rust_sources(&files);
        assert!(safe.contains("pub fn create(value: u32)"), "{safe}");
    }

    /// How each capability flag claimed by `attr_support` is covered.
    ///
    /// `attr_support` is a promise to HIR lowering, not documentation: a provider
    /// guarded by `#[diplomat::cfg(supports = <flag>)]` is accepted and lowered, so an
    /// untested promise can yield bindings that compile and misbehave. A `gated:` row
    /// means the flag guards a fixture API, so dropping the flag removes the API and the
    /// consumer tests stop compiling; the other rows name the generator test plus the
    /// fixture method that exercises the shape at runtime.
    const FLAG_COVERAGE: &[(&str, &str)] = &[
        (
            "constructors",
            "plain_constructors_are_accepted; Counter::new at runtime",
        ),
        (
            "memory_sharing",
            "gated: Numbers::from_slice, Float64Vec::new",
        ),
        (
            "mutable_slices",
            "borrowed_slices_and_strings_are_lowered; Numbers::values_mut",
        ),
        (
            "named_constructors",
            "named_constructor_names_are_honoured; gated: Counter::with_value",
        ),
        (
            "option",
            "value_types_and_options_are_lowered; Counter::maybe_new",
        ),
        (
            "owned_byte_slice_returns",
            "owned_slice_returns_are_boxed; Bytes::make",
        ),
        ("static_slices", "gated: Numbers::from_static"),
        (
            "utf16_strings",
            "gated: WideMessage; capability_flags_gate_generated_apis",
        ),
        ("utf8_strings", "gated: Message::utf8_len"),
    ];

    /// Every `support.<flag> = true` in `attr_support` needs a [`FLAG_COVERAGE`] row, so
    /// a capability cannot be claimed without covering it — the mechanical half of the
    /// rule whose human half is the table itself.
    #[test]
    fn every_claimed_flag_is_covered() {
        let source = include_str!("mod.rs");
        let body = source
            .split_once("pub(crate) fn attr_support")
            .expect("attr_support is defined")
            .1
            .split_once("\n}\n")
            .expect("attr_support has a body")
            .0;
        let mut claimed: Vec<&str> = body
            .lines()
            .filter_map(|line| line.trim().strip_prefix("support."))
            .filter_map(|rest| rest.strip_suffix(" = true;"))
            .collect();
        claimed.sort_unstable();
        let mut covered: Vec<&str> = FLAG_COVERAGE.iter().map(|(flag, _)| *flag).collect();
        covered.sort_unstable();
        assert_eq!(
            claimed, covered,
            "every claimed flag needs a FLAG_COVERAGE row, and every row needs a claim"
        );
    }
}
