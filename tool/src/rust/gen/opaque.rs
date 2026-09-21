//! One generated module per opaque type: the wrappers, sealed capability traits,
//! `Drop`, borrowed-view constructors, and the impl block for each wrapper.

use std::fmt::Write as _;

use diplomat_core::hir::{
    self, DocsUrlGenerator, MaybeOwn, Mutability, SelfType, TypeContext, TypeDef,
};

use super::method::emit_method;
use crate::r#rust::formatter::{emit_docs, opaque_name, type_def_name};
use crate::r#rust::lifetimes::{
    bounded_lifetime_name, lifetime_generics, opaque_lifetime_names, opaque_lifetime_phantom,
    render_generics,
};

pub(super) fn opaque_generics(opaque: &hir::OpaqueDef, wrapper: Wrapper) -> (String, String) {
    let (mut params, mut args) = lifetime_generics(&opaque.lifetimes);
    if wrapper.borrows() {
        params.insert(0, "'view".to_string());
        args.insert(0, "'view".to_string());
    }
    (render_generics(&params), render_generics(&args))
}

pub(super) fn emit_opaque(
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

    emit_debug(out, &name, Wrapper::Owned, &owned_params, &owned_args);
    emit_debug(out, &name, Wrapper::Ref, &ref_params, &ref_args);
    emit_debug(out, &name, Wrapper::RefMut, &ref_params, &ref_args);

    emit_impl(out, opaque, tcx, docs_url_gen, Wrapper::Owned);
    emit_impl(out, opaque, tcx, docs_url_gen, Wrapper::Ref);
    emit_impl(out, opaque, tcx, docs_url_gen, Wrapper::RefMut);
}

/// Hand-written so `Debug` does not name `pub(crate)` fields. A derive would
/// print `inner` and `_not_send_sync` and lock those names into the public format.
fn emit_debug(out: &mut String, name: &str, wrapper: Wrapper, params: &str, args: &str) {
    let type_name = wrapper.type_name(name);
    writeln!(
        out,
        "impl{params} fmt::Debug for {type_name}{args} {{\n    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {{\n        f.debug_tuple(\"{type_name}\").field(&self.inner.as_ptr()).finish()\n    }}\n}}\n"
    )
    .unwrap();
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
///
/// This is deliberately not [`opaque_generics`]. A provider spells a type's
/// lifetime in the struct declaration and again in each `impl` block, and HIR
/// keeps both: `feature_tests/src/selftype.rs` declares `struct RefList<'a>` and
/// then `impl<'b> RefList<'b>`, and `lifetimes.rs` declares `struct One<'a>` and
/// `impl<'o> One<'o>` with methods that themselves declare `<'a, ...>`.
///
/// Method signatures are rendered from `method.lifetime_env`, so the impl has to
/// be too — using the declaration's spelling here is what produced `impl<'a>
/// One<'a>` containing a method that redeclares `'a` (E0496), and `impl<'a>
/// RefList<'a>` containing a method that names an undeclared `'b` (E0261).
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

pub(super) fn emit_impl(
    out: &mut String,
    opaque: &hir::OpaqueDef,
    tcx: &TypeContext,
    docs_url_gen: &DocsUrlGenerator,
    wrapper: Wrapper,
) {
    let name = type_def_name(TypeDef::Opaque(opaque));
    let (params, args) = impl_generics(opaque, wrapper);
    let type_name = wrapper.type_name(&name);
    writeln!(out, "impl{params} {type_name}{args} {{").unwrap();
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
            emit_method(
                out,
                opaque.lifetimes.num_lifetimes(),
                method,
                tcx,
                docs_url_gen,
            );
        }
    }
    out.push_str("}\n\n");
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
