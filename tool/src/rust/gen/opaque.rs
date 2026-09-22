//! Rendering of one generated opaque module through Askama.
//!
//! Ownership, lifetime and ABI decisions are made in Rust. The template only
//! lays out the wrapper structs, capability traits, Drop/Debug impls and method
//! impl blocks.

use askama::Template;
use diplomat_core::hir::{
    self, DocsUrlGenerator, MaybeOwn, Mutability, SelfType, TypeContext, TypeDef,
};

use super::method::render_method;
use crate::r#rust::formatter::{opaque_name, render_docs, type_def_name};
use crate::r#rust::lifetimes::{
    bounded_lifetime_name, lifetime_generics, opaque_lifetime_names, opaque_lifetime_phantom,
    render_generics,
};
use crate::r#rust::type_map::opaque_uses_value_types;

#[derive(Template)]
#[template(path = "rust/opaque.rs.jinja", escape = "none")]
struct OpaqueTemplate {
    uses_value_types: bool,
    docs: String,
    name: String,
    owned_params: String,
    owned_args: String,
    ref_params: String,
    ref_args: String,
    phantom: String,
    dtor_abi_name: String,
    wrappers: Vec<WrapperView>,
}

struct WrapperView {
    type_name: String,
    debug_params: String,
    debug_args: String,
    impl_params: String,
    impl_args: String,
    methods: Vec<String>,
}

pub(super) fn opaque_generics(opaque: &hir::OpaqueDef, wrapper: Wrapper) -> (String, String) {
    let (mut params, mut args) = lifetime_generics(&opaque.lifetimes);
    if wrapper.borrows() {
        params.insert(0, "'view".to_string());
        args.insert(0, "'view".to_string());
    }
    (render_generics(&params), render_generics(&args))
}

pub(crate) fn generate_opaque_file(
    tcx: &TypeContext,
    opaque: &hir::OpaqueDef,
    docs_url_gen: &DocsUrlGenerator,
) -> String {
    let name = type_def_name(TypeDef::Opaque(opaque));
    let (owned_params, owned_args) = opaque_generics(opaque, Wrapper::Owned);
    let (ref_params, ref_args) = opaque_generics(opaque, Wrapper::Ref);
    let wrappers = [Wrapper::Owned, Wrapper::Ref, Wrapper::RefMut]
        .into_iter()
        .map(|wrapper| wrapper_view(opaque, tcx, docs_url_gen, wrapper, &name))
        .collect();

    OpaqueTemplate {
        uses_value_types: opaque_uses_value_types(opaque),
        docs: render_docs(&opaque.docs, docs_url_gen, ""),
        name,
        owned_params,
        owned_args,
        ref_params,
        ref_args,
        phantom: opaque_lifetime_phantom(opaque),
        dtor_abi_name: opaque.dtor_abi_name.to_string(),
        wrappers,
    }
    .render()
    .expect("Rust opaque template rendering cannot fail")
}

fn wrapper_view(
    opaque: &hir::OpaqueDef,
    tcx: &TypeContext,
    docs_url_gen: &DocsUrlGenerator,
    wrapper: Wrapper,
    name: &str,
) -> WrapperView {
    let (params, args) = opaque_generics(opaque, wrapper);
    let (impl_params, impl_args) = impl_generics(opaque, wrapper);
    let methods = opaque
        .methods
        .iter()
        .filter(|method| !method.attrs.disable)
        .filter(|method| includes_method(method, wrapper))
        .map(|method| render_method(opaque.lifetimes.num_lifetimes(), method, tcx, docs_url_gen))
        .collect();

    WrapperView {
        type_name: wrapper.type_name(name),
        debug_params: params,
        debug_args: args,
        impl_params,
        impl_args,
        methods,
    }
}

fn includes_method(method: &hir::Method, wrapper: Wrapper) -> bool {
    let include = match (&method.param_self, wrapper) {
        (None, Wrapper::Owned) => true,
        (None, _) => false,
        (Some(param), Wrapper::Owned) => matches!(param.ty, SelfType::Opaque(_)),
        (Some(param), Wrapper::Ref) => param.ty.is_immutably_borrowed(),
        (Some(param), Wrapper::RefMut) => matches!(param.ty, SelfType::Opaque(_)),
    };
    include
        && !matches!(
            (&method.param_self, wrapper),
            (Some(param), Wrapper::Ref) if param.ty.is_mutably_borrowed()
        )
}

/// Which generated wrapper an impl block belongs to, named after the suffix each
/// wrapper carries: `Owned` is `T`, `Ref` is `TRef`, `RefMut` is `TRefMut`.
#[derive(Clone, Copy)]
pub(super) enum Wrapper {
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

    /// The generated type name: `T`, `TRef`, or `TRefMut`.
    fn type_name(self, name: &str) -> String {
        match self {
            Self::Owned => name.to_string(),
            Self::Ref => format!("{name}Ref"),
            Self::RefMut => format!("{name}RefMut"),
        }
    }
}

/// The lifetime parameters of the *method* impl block.
fn impl_generics(opaque: &hir::OpaqueDef, wrapper: Wrapper) -> (String, String) {
    let type_lifetimes = opaque.lifetimes.num_lifetimes();
    let env = opaque
        .methods
        .iter()
        .filter(|method| !method.attrs.disable)
        .map(|method| &method.lifetime_env)
        .find(|env| env.num_lifetimes() >= type_lifetimes);

    let (mut params, mut args) = match env {
        Some(env) => (
            env.all_lifetimes()
                .take(type_lifetimes)
                .map(|lifetime| bounded_lifetime_name(env, lifetime))
                .collect::<Vec<_>>(),
            env.all_lifetimes()
                .take(type_lifetimes)
                .map(|lifetime| format!("'{}", env.fmt_lifetime(lifetime)))
                .collect::<Vec<_>>(),
        ),
        None => lifetime_generics(&opaque.lifetimes),
    };
    if wrapper.borrows() {
        params.insert(0, "'view".to_string());
        args.insert(0, "'view".to_string());
    }
    (render_generics(&params), render_generics(&args))
}

pub(super) fn opaque_return_expr(
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
        format!("crate::{name} {{ inner{phantom}, _not_send_sync: PhantomData }}")
    } else if path.owner.mutability() == Mutability::Mutable {
        format!(
            "crate::{name}RefMut {{ inner, _borrow: PhantomData{phantom}, _not_send_sync: PhantomData }}"
        )
    } else {
        format!(
            "crate::{name}Ref {{ inner, _borrow: PhantomData{phantom}, _not_send_sync: PhantomData }}"
        )
    };
    if path.is_optional() {
        format!("NonNull::new(result as *mut _).map(|inner| {construct})")
    } else {
        format!(
            "{{ let inner = NonNull::new(result as *mut _).expect(\"Diplomat ABI returned null for non-null {name}\"); {construct} }}"
        )
    }
}
