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

/// Whether a method must declare its method-level lifetimes: the output uses
/// one, a struct parameter names one, or an input lifetime carries a bound
/// that is not already in scope as a type-level lifetime.
pub(super) fn method_declares_lifetimes(
    method: &hir::Method,
    type_lifetimes: usize,
    tcx: &TypeContext,
) -> bool {
    if !method.output.used_method_lifetimes().is_empty() {
        return true;
    }
    if method.params.iter().any(|param| match &param.ty {
        Type::Struct(path) => matches!(
            tcx.resolve_type(path.id()),
            TypeDef::Struct(strct) if is_lifetime_struct(strct)
        ),
        _ => false,
    }) {
        return true;
    }
    input_lifetimes(method).any(|lifetime| {
        !is_type_lifetime(lifetime, method, type_lifetimes)
            && lifetime_carries_a_bound(lifetime, method)
    })
}

fn is_type_lifetime(lifetime: hir::Lifetime, method: &hir::Method, type_lifetimes: usize) -> bool {
    method
        .lifetime_env
        .all_lifetimes()
        .take(type_lifetimes)
        .any(|candidate| candidate == lifetime)
}

/// Lifetimes written on the receiver and on parameters.
fn input_lifetimes(method: &hir::Method) -> impl Iterator<Item = hir::Lifetime> + '_ {
    let receiver = method
        .param_self
        .as_ref()
        .and_then(|param| match &param.ty {
            hir::SelfType::Opaque(path) => nonstatic(path.borrowed().lifetime),
            hir::SelfType::Struct(path) => match path.owner {
                hir::MaybeOwn::Borrow(borrow) => nonstatic(borrow.lifetime),
                hir::MaybeOwn::Own => None,
            },
            _ => None,
        });
    let params = method.params.iter().flat_map(|param| {
        let mut found = Vec::new();
        collect_input_lifetimes(&param.ty, &mut found);
        found
    });
    receiver.into_iter().chain(params)
}

fn collect_input_lifetimes(ty: &Type<hir::InputOnly>, found: &mut Vec<hir::Lifetime>) {
    match ty {
        Type::Slice(slice) => {
            if let Some(lifetime) = super::type_map::slice_lifetime(slice).and_then(nonstatic) {
                found.push(lifetime);
            }
        }
        Type::Opaque(path) => {
            if let Some(lifetime) = nonstatic(path.owner.lifetime) {
                found.push(lifetime);
            }
            found.extend(path.lifetimes.lifetimes().filter_map(nonstatic));
        }
        Type::DiplomatOption(inner) => collect_input_lifetimes(inner.as_ref(), found),
        _ => {}
    }
}

fn nonstatic(lifetime: MaybeStatic<hir::Lifetime>) -> Option<hir::Lifetime> {
    match lifetime {
        MaybeStatic::NonStatic(lifetime) => Some(lifetime),
        MaybeStatic::Static => None,
    }
}

/// `'a` on `Slot<'a>` , or any lifetime with an outlives bound. A fresh
/// call-only borrow has neither, and eliding it is the same constraint.
fn lifetime_carries_a_bound(lifetime: hir::Lifetime, method: &hir::Method) -> bool {
    let env = &method.lifetime_env;
    env.all_shorter_lifetimes(lifetime)
        .any(|other| other != lifetime)
        || env
            .all_longer_lifetimes(lifetime)
            .any(|other| other != lifetime)
}

pub(super) fn method_generics(method: &hir::Method, skip: usize, tcx: &TypeContext) -> String {
    if !method_declares_lifetimes(method, skip, tcx) {
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

/// The lifetime as it appears before a reference: `"'a "` or `"'static "`.
///
/// A lifetime is spelled when the return value uses one, when it is one of the
/// enclosing type's lifetimes, or when it has an outlives bound. Eliding
/// `Slot<'a>::store`'s `&'a [u8]` to `&[u8]` would accept a borrow that does
/// not live for `'a`. A fresh borrow that dies at the end of the call stays
/// elided.
pub(super) fn lifetime_prefix(
    lifetime: MaybeStatic<hir::Lifetime>,
    method: &hir::Method,
    type_lifetimes: usize,
) -> String {
    match lifetime {
        MaybeStatic::Static => "'static ".into(),
        MaybeStatic::NonStatic(lt)
            if !method.output.used_method_lifetimes().is_empty()
                || is_type_lifetime(lt, method, type_lifetimes)
                || lifetime_carries_a_bound(lt, method) =>
        {
            format!("'{} ", method.lifetime_env.fmt_lifetime(lt))
        }
        MaybeStatic::NonStatic(_) => String::new(),
    }
}
