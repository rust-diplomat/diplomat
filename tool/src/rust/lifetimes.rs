//! HIR lifetime environments rendered as Rust generic parameters, bounds and
//! invariant `PhantomData` fields.

use diplomat_core::hir::{self, MaybeStatic, StructPathLike, Type, TypeContext, TypeDef};

use super::type_map::is_lifetime_struct;

pub(super) fn opaque_lifetime_names(opaque: &hir::OpaqueDef) -> Vec<String> {
    lifetime_names(&opaque.lifetimes)
}

/// The declared names of a set of type-level lifetimes, e.g. `["'a", "'b"]`.
pub(super) fn lifetime_names(lifetimes: &hir::LifetimeEnv) -> Vec<String> {
    lifetimes
        .all_lifetimes()
        .map(|lifetime| format!("'{}", lifetimes.fmt_lifetime(lifetime)))
        .collect()
}

/// One lifetime parameter with its bounds: `'a` when it outlives nothing else,
/// otherwise `'a: 'b + 'c`, naming the shorter lifetimes it transitively outlives.
pub(super) fn bounded_lifetime_name(lifetimes: &hir::LifetimeEnv, longer: hir::Lifetime) -> String {
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
pub(super) fn render_generics(items: &[String]) -> String {
    if items.is_empty() {
        String::new()
    } else {
        format!("<{}>", items.join(", "))
    }
}

/// The generic parameter list (with bounds) and the use-site argument list for a
/// set of type-level lifetimes.
pub(super) fn lifetime_generics(lifetimes: &hir::LifetimeEnv) -> (Vec<String>, Vec<String>) {
    let params: Vec<String> = lifetimes
        .all_lifetimes()
        .map(|longer| bounded_lifetime_name(lifetimes, longer))
        .collect();
    let args = lifetime_names(lifetimes);
    (params, args)
}

pub(super) fn struct_generics(strct: &hir::StructDef) -> (String, String) {
    let (params, args) = lifetime_generics(&strct.lifetimes);
    (render_generics(&params), render_generics(&args))
}

/// The invariant `PhantomData` field of a lifetime-carrying struct.
pub(super) fn struct_lifetime_phantom(strct: &hir::StructDef) -> String {
    lifetime_phantom(&lifetime_names(&strct.lifetimes))
}

/// An invariant `PhantomData` field tying a wrapper to the listed lifetimes.
///
/// Invariance matters: a wrapper must not be usable at a longer lifetime than the
/// one it was created with, so each lifetime has to appear in both a covariant and
/// a contravariant position. `*mut T` is invariant in `T`, so `*mut &'a ()` pins
/// `'a` on its own — one occurrence per lifetime rather than the two a
/// `fn(A) -> B` signature needs, which keeps the field type simple enough not to
/// trip `clippy::type_complexity`.
pub(super) fn lifetime_phantom_type(names: &[String]) -> String {
    let refs: Vec<String> = names
        .iter()
        .map(|name| format!("*mut &{name} ()"))
        .collect();
    let joined = refs.join(", ");
    let output = if refs.len() == 1 {
        refs[0].clone()
    } else {
        format!("({joined})")
    };
    format!("PhantomData<{output}>")
}

pub(super) fn lifetime_witness_type(names: &[String]) -> String {
    format!("core::marker::{}", lifetime_phantom_type(names))
}

pub(super) fn lifetime_phantom(names: &[String]) -> String {
    if names.is_empty() {
        return String::new();
    }
    format!(
        "    pub(crate) _lifetimes: {},\n",
        lifetime_phantom_type(names)
    )
}

/// An invariant `PhantomData` field tying a wrapper to its type-level lifetimes.
pub(super) fn opaque_lifetime_phantom(opaque: &hir::OpaqueDef) -> String {
    lifetime_phantom(&opaque_lifetime_names(opaque))
}

/// Whether a method must declare its method-level lifetimes: it does when any
/// lifetime appears in the output, or when a struct parameter names one.
pub(super) fn method_declares_lifetimes(method: &hir::Method, tcx: &TypeContext) -> bool {
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

pub(super) fn method_generics(method: &hir::Method, skip: usize, tcx: &TypeContext) -> String {
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

pub(super) fn lifetime_name(lifetime: MaybeStatic<hir::Lifetime>, method: &hir::Method) -> String {
    match lifetime {
        MaybeStatic::Static => "'static".into(),
        MaybeStatic::NonStatic(lt) => format!("'{}", method.lifetime_env.fmt_lifetime(lt)),
    }
}

pub(super) fn lifetime_prefix(
    lifetime: MaybeStatic<hir::Lifetime>,
    method: &hir::Method,
) -> String {
    match lifetime {
        MaybeStatic::Static => "'static ".into(),
        MaybeStatic::NonStatic(lt) if !method.output.used_method_lifetimes().is_empty() => {
            format!("'{} ", method.lifetime_env.fmt_lifetime(lt))
        }
        MaybeStatic::NonStatic(_) => String::new(),
    }
}
