//! Type mapping: for each HIR type shape, its native ABI rendering, its safe
//! public rendering, and the expressions that convert between them.

use diplomat_core::hir::{
    self, MaybeOwn, MaybeStatic, Mutability, OutType, PrimitiveType, ReturnType,
    ReturnableStructPath, SelfType, Slice, StringEncoding, StructPathLike, SuccessType, Type,
    TypeContext, TypeDef,
};

use super::formatter::{enum_name, field_name, opaque_name, type_def_name};
use super::lifetimes::{lifetime_name, lifetime_prefix, struct_generics};

pub(super) fn is_supported_slice<P: hir::TyPosition>(slice: &Slice<P>) -> bool {
    match slice {
        Slice::Primitive(MaybeOwn::Borrow(_), primitive) => primitive_name(*primitive).is_some(),
        Slice::Str(Some(_), _) => true,
        _ => false,
    }
}

/// An owned primitive slice (`Box<[T]>`) returned by the provider.
pub(super) fn is_owned_slice<P: hir::TyPosition>(slice: &Slice<P>) -> bool {
    matches!(slice, Slice::Primitive(MaybeOwn::Own, primitive) if primitive_name(*primitive).is_some())
}

/// The lifetime carried by a borrowed slice, if any.
pub(super) fn slice_lifetime<P: hir::TyPosition>(
    slice: &Slice<P>,
) -> Option<MaybeStatic<hir::Lifetime>> {
    match slice {
        Slice::Primitive(MaybeOwn::Borrow(borrow), _)
        | Slice::Struct(MaybeOwn::Borrow(borrow), _)
        | Slice::Opaque(MaybeOwn::Borrow(borrow), _) => Some(borrow.lifetime),
        Slice::Str(lifetime, _) => *lifetime,
        Slice::Strs(_) => None,
        _ => None,
    }
}

pub(super) fn slice_is_mutable<P: hir::TyPosition>(slice: &Slice<P>) -> bool {
    matches!(
        slice,
        Slice::Primitive(MaybeOwn::Borrow(borrow), _) if borrow.mutability == Mutability::Mutable
    )
}

/// The Rust scalar type stored in a supported slice.
pub(super) fn slice_element_ty<P: hir::TyPosition>(slice: &Slice<P>) -> &'static str {
    match slice {
        Slice::Primitive(_, primitive) => primitive_name(*primitive).unwrap(),
        Slice::Str(_, StringEncoding::UnvalidatedUtf16) => "u16",
        Slice::Str(_, StringEncoding::Utf8 | StringEncoding::UnvalidatedUtf8) => "u8",
        _ => unreachable!("validated slice shape"),
    }
}

/// The safe public Rust type of a slice, e.g. `&'a [f64]`, `&'a mut str`, `Box<[u8]>`.
pub(super) fn safe_slice_type<P: hir::TyPosition>(slice: &Slice<P>, lifetime: &str) -> String {
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
pub(super) fn ffi_slice_container<P: hir::TyPosition>(slice: &Slice<P>) -> &'static str {
    if slice_is_mutable(slice) {
        "DiplomatSliceMut"
    } else {
        "DiplomatSlice"
    }
}

/// Convert a safe Rust slice expression into the native ABI borrow of it, e.g.
/// `ffi::DiplomatSlice::from(samples)`.
pub(super) fn ffi_borrowed_slice_expr<P: hir::TyPosition>(slice: &Slice<P>, expr: &str) -> String {
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
pub(super) fn ffi_slice_type<P: hir::TyPosition>(slice: &Slice<P>, lifetime: &str) -> String {
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

/// Whether a type mentions a generated enum or struct (both live in `types`).
pub(super) fn references_value_type<P: hir::TyPosition>(ty: &Type<P>) -> bool {
    match ty {
        Type::Enum(_) | Type::Struct(_) => true,
        Type::DiplomatOption(inner) => references_value_type(inner.as_ref()),
        _ => false,
    }
}

pub(super) fn opaque_uses_value_types(opaque: &hir::OpaqueDef) -> bool {
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

/// A value struct that carries lifetime parameters (and therefore borrowed
/// slice fields) rather than being a plain `repr(C)` value.
pub(super) fn is_lifetime_struct(strct: &hir::StructDef) -> bool {
    strct.lifetimes.num_lifetimes() != 0
}

/// The native ABI type of a struct field.
///
/// A struct field declaration cannot elide a lifetime parameter (unlike a
/// function signature), so a borrowed slice names the field's HIR lifetime.
pub(super) fn ffi_struct_field_type(
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
pub(super) fn ffi_struct_slice_type(
    slice: &Slice<hir::Everywhere>,
    strct: &hir::StructDef,
) -> String {
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
pub(super) fn safe_struct_field_type(
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
pub(super) fn struct_input_expr(name: &str, strct: &hir::StructDef) -> String {
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

pub(super) fn safe_field_to_ffi(ty: &Type<hir::Everywhere>, expr: &str) -> String {
    match ty {
        Type::Slice(slice) => ffi_borrowed_slice_expr(slice, expr),
        _ => expr.to_string(),
    }
}

/// Convert a native ABI struct mirror into the safe struct value.
pub(super) fn struct_output_expr(raw: &str, strct: &hir::StructDef) -> String {
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

pub(super) fn ffi_field_to_safe(ty: &Type<hir::Everywhere>, expr: &str) -> String {
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

pub(super) fn ffi_self_type(ty: &SelfType, tcx: &TypeContext) -> String {
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

pub(super) fn ffi_input_type(ty: &Type<hir::InputOnly>, tcx: &TypeContext) -> String {
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

pub(super) fn ffi_return_type(ret: &ReturnType, tcx: &TypeContext) -> String {
    match ret {
        ReturnType::Infallible(success) => ffi_success_type(success, tcx),
        ReturnType::Nullable(SuccessType::OutType(inner)) => {
            format!("DiplomatOption<{}>", ffi_value_type(inner, tcx))
        }
        // The ABI carries both payloads in the runtime's tagged union — the same container
        // `DiplomatOption<T>` aliases with a `()` error.
        ReturnType::Fallible(success, err) => format!(
            "DiplomatResult<{}, {}>",
            ffi_success_type(success, tcx),
            ffi_error_type(err, tcx)
        ),
        ReturnType::Nullable(_) => unreachable!("validated return shape"),
    }
}

/// The native ABI type of a return's success payload.
pub(super) fn ffi_success_type(success: &SuccessType, tcx: &TypeContext) -> String {
    match success {
        SuccessType::Unit => "()".into(),
        SuccessType::OutType(Type::Opaque(path)) => {
            let name = opaque_name(path.tcx_id, tcx);
            if path.owner.is_owned() || path.owner.mutability() == Mutability::Mutable {
                format!("*mut {name}")
            } else {
                format!("*const {name}")
            }
        }
        SuccessType::OutType(Type::DiplomatOption(inner)) => {
            format!("DiplomatOption<{}>", ffi_value_type(inner.as_ref(), tcx))
        }
        SuccessType::OutType(Type::Struct(ReturnableStructPath::Struct(path))) => {
            let strct = path.resolve(tcx);
            if is_lifetime_struct(strct) {
                let (_, args) = struct_generics(strct);
                format!("{}{args}", type_def_name(TypeDef::Struct(strct)))
            } else {
                ffi_value_name(TypeDef::Struct(strct))
            }
        }
        SuccessType::OutType(Type::Slice(slice)) => ffi_slice_type(slice, RETURN_LIFETIME),
        SuccessType::OutType(ty) => ffi_value_type(ty, tcx),
        // `Write` is rejected by validation, and `SuccessType` is `#[non_exhaustive]`, so
        // anything reaching here is a shape codegen has never been taught.
        _ => unreachable!("validated success shape"),
    }
}

/// The native ABI type of a `Result`'s error payload; `None` is `Result<T, ()>`.
pub(super) fn ffi_error_type(err: &Option<OutType>, tcx: &TypeContext) -> String {
    match err {
        None => "()".into(),
        // Validation admits only owned opaque errors, so this is always a pointer the
        // generated wrapper is responsible for destructing.
        Some(Type::Opaque(path)) => format!("*mut {}", opaque_name(path.tcx_id, tcx)),
        Some(ty) => ffi_value_type(ty, tcx),
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
pub(super) fn ffi_return_generics(ret: &ReturnType, tcx: &TypeContext) -> String {
    // Only the success payload can carry a lifetime: an error payload is restricted to
    // value types and owned opaques, neither of which borrows.
    let success = match ret {
        ReturnType::Infallible(success) | ReturnType::Fallible(success, _) => success,
        ReturnType::Nullable(_) => return String::new(),
    };
    match success {
        SuccessType::OutType(Type::Slice(slice)) if !is_owned_slice(slice) => {
            format!("<{RETURN_LIFETIME}>")
        }
        SuccessType::OutType(Type::Struct(ReturnableStructPath::Struct(path))) => {
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

pub(super) fn safe_input_type(
    ty: &Type<hir::InputOnly>,
    method: &hir::Method,
    tcx: &TypeContext,
) -> String {
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

pub(super) fn safe_return_type(
    ret: &ReturnType,
    method: &hir::Method,
    tcx: &TypeContext,
) -> String {
    match ret {
        ReturnType::Infallible(success) => safe_success_type(success, method, tcx),
        ReturnType::Nullable(SuccessType::OutType(inner)) => {
            format!("Option<{}>", safe_value_type(inner, tcx))
        }
        // Both sides are converted independently: an owned opaque payload is a raw
        // pointer in the ABI and the owning wrapper in the public API.
        ReturnType::Fallible(success, err) => format!(
            "Result<{}, {}>",
            safe_success_type(success, method, tcx),
            safe_error_type(err, method, tcx)
        ),
        ReturnType::Nullable(_) => unreachable!("validated return shape"),
    }
}

/// The safe public Rust type of a return's success payload.
pub(super) fn safe_success_type(
    success: &SuccessType,
    method: &hir::Method,
    tcx: &TypeContext,
) -> String {
    match success {
        SuccessType::Unit => "()".into(),
        SuccessType::OutType(Type::Opaque(path)) => {
            let base = opaque_safe_type(path, method, tcx);
            if path.is_optional() {
                format!("Option<{base}>")
            } else {
                base
            }
        }
        SuccessType::OutType(Type::Slice(slice)) => {
            let lifetime = slice_lifetime(slice)
                .map(|lifetime| format!("{} ", lifetime_name(lifetime, method)))
                .unwrap_or_default();
            safe_slice_type(slice, &lifetime)
        }
        SuccessType::OutType(Type::Struct(ReturnableStructPath::Struct(path))) => {
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
        SuccessType::OutType(Type::DiplomatOption(inner)) => {
            format!("Option<{}>", safe_value_type(inner.as_ref(), tcx))
        }
        SuccessType::OutType(ty) => safe_value_type(ty, tcx),
        // `Write` is rejected by validation, and `SuccessType` is `#[non_exhaustive]`, so
        // anything reaching here is a shape codegen has never been taught.
        _ => unreachable!("validated success shape"),
    }
}

/// The safe public Rust type of a `Result`'s error payload.
pub(super) fn safe_error_type(
    err: &Option<OutType>,
    method: &hir::Method,
    tcx: &TypeContext,
) -> String {
    match err {
        None => "()".into(),
        // An owned opaque error becomes the owning wrapper, so its `Drop` is what frees
        // the provider's allocation when a caller discards the error.
        Some(Type::Opaque(path)) => opaque_safe_type(path, method, tcx),
        Some(ty) => safe_value_type(ty, tcx),
    }
}

/// How the native ABI layer names a value type definition: enums and plain value
/// structs live in the parent module, while a lifetime-carrying struct is emitted
/// at the crate root.
/// The safe public Rust type of an opaque output, including type-level lifetimes,
/// e.g. `Bar<'b, 'a>` or `FooRef<'x, 'a>`.
pub(super) fn opaque_safe_type(
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

pub(super) fn ffi_value_name(def: TypeDef<'_>) -> String {
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
pub(super) fn ffi_value_type<P: hir::TyPosition>(ty: &Type<P>, tcx: &TypeContext) -> String {
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
pub(super) fn safe_value_type<P: hir::TyPosition>(ty: &Type<P>, tcx: &TypeContext) -> String {
    match ty {
        Type::Primitive(p) => primitive_name(*p).unwrap().into(),
        Type::Enum(path) => type_def_name(TypeDef::Enum(path.resolve(tcx))),
        Type::Struct(path) => type_def_name(tcx.resolve_type(path.id())),
        _ => unreachable!("validated safe value type"),
    }
}

pub(super) fn primitive_name(primitive: PrimitiveType) -> Option<&'static str> {
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
