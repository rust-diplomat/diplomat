//! Type definitions for structs, output structs, opaque structs, and enums.

use super::lifetimes::LifetimeEnv;
use super::{
    Attrs, Callback, Everywhere, IdentBuf, Method, OutputOnly, SpecialMethodPresence, TyPosition,
    Type,
};
use crate::ast::Docs;

#[non_exhaustive]
pub enum ReturnableStructDef<'tcx> {
    Struct(&'tcx StructDef),
    OutStruct(&'tcx OutStructDef),
}

#[derive(Copy, Clone, Debug)]
#[non_exhaustive]
pub enum TypeDef<'tcx> {
    Struct(&'tcx StructDef),
    OutStruct(&'tcx OutStructDef),
    Opaque(&'tcx OpaqueDef),
    Enum(&'tcx EnumDef),
}

#[derive(Copy, Clone, Debug)]
#[non_exhaustive]
pub enum SymbolDef<'tcx> {
    Struct(&'tcx StructDef),
    OutStruct(&'tcx OutStructDef),
    Opaque(&'tcx OpaqueDef),
    Enum(&'tcx EnumDef),
    Trait(&'tcx TraitDef),
    Method(&'tcx Method),
}

#[derive(Debug)]
#[non_exhaustive]
pub struct TraitDef {
    // TyPosition: InputOnly
    pub docs: Docs,
    pub name: IdentBuf,
    pub methods: Vec<Callback>,
    pub attrs: Attrs,
    pub lifetimes: LifetimeEnv,
}

/// Structs that can only be returned from methods.
pub type OutStructDef = StructDef<OutputOnly>;

/// Structs that can be either inputs or outputs in methods.
#[derive(Debug)]
#[non_exhaustive]
pub struct StructDef<P: TyPosition = Everywhere> {
    pub docs: Docs,
    pub name: IdentBuf,
    pub fields: Vec<StructField<P>>,
    pub methods: Vec<Method>,
    pub attrs: Attrs,
    pub lifetimes: LifetimeEnv,
    pub special_method_presence: SpecialMethodPresence,
}

/// A struct whose contents are opaque across the FFI boundary, and can only
/// cross when behind a pointer.
///
/// All opaques can be inputs or outputs when behind a reference, but owned
/// opaques can only be returned since there isn't a general way for most languages
/// to give up ownership.
///
/// A struct marked with `#[diplomat::opaque]`.
#[derive(Debug)]
#[non_exhaustive]
pub struct OpaqueDef {
    pub docs: Docs,
    pub name: IdentBuf,
    pub methods: Vec<Method>,
    pub attrs: Attrs,
    pub lifetimes: LifetimeEnv,
    pub special_method_presence: SpecialMethodPresence,

    /// The ABI name of the generated destructor
    pub dtor_abi_name: IdentBuf,
}

/// The enum type.
#[derive(Debug)]
#[non_exhaustive]
pub struct EnumDef {
    pub docs: Docs,
    pub name: IdentBuf,
    pub variants: Vec<EnumVariant>,
    pub methods: Vec<Method>,
    pub attrs: Attrs,
    pub special_method_presence: SpecialMethodPresence,
}

/// A field on a [`OutStruct`]s.
pub type OutStructField = StructField<OutputOnly>;

/// A field on a [`Struct`]s.
#[derive(Debug)]
#[non_exhaustive]
pub struct StructField<P: TyPosition = Everywhere> {
    pub docs: Docs,
    pub name: IdentBuf,
    pub ty: Type<P>,
    pub attrs: Attrs,
}

/// A variant of an [`Enum`].
#[derive(Debug)]
#[non_exhaustive]
pub struct EnumVariant {
    pub docs: Docs,
    pub name: IdentBuf,
    pub discriminant: isize,
    pub attrs: Attrs,
}

impl TraitDef {
    pub(super) fn new(
        docs: Docs,
        name: IdentBuf,
        methods: Vec<Callback>,
        attrs: Attrs,
        lifetimes: LifetimeEnv,
    ) -> Self {
        Self {
            docs,
            name,
            methods,
            attrs,
            lifetimes,
        }
    }
}

impl<P: TyPosition> StructDef<P> {
    pub(super) fn new(
        docs: Docs,
        name: IdentBuf,
        fields: Vec<StructField<P>>,
        methods: Vec<Method>,
        attrs: Attrs,
        lifetimes: LifetimeEnv,
        special_method_presence: SpecialMethodPresence,
    ) -> Self {
        Self {
            docs,
            name,
            fields,
            methods,
            attrs,
            lifetimes,
            special_method_presence,
        }
    }
}

impl OpaqueDef {
    pub(super) fn new(
        docs: Docs,
        name: IdentBuf,
        methods: Vec<Method>,
        attrs: Attrs,
        lifetimes: LifetimeEnv,
        special_method_presence: SpecialMethodPresence,
        dtor_abi_name: IdentBuf,
    ) -> Self {
        Self {
            docs,
            name,
            methods,
            attrs,
            lifetimes,
            special_method_presence,
            dtor_abi_name,
        }
    }
}

impl EnumDef {
    pub(super) fn new(
        docs: Docs,
        name: IdentBuf,
        variants: Vec<EnumVariant>,
        methods: Vec<Method>,
        attrs: Attrs,
        special_method_presence: SpecialMethodPresence,
    ) -> Self {
        Self {
            docs,
            name,
            variants,
            methods,
            attrs,
            special_method_presence,
        }
    }
}

impl<'a, P: TyPosition> From<&'a StructDef<P>> for TypeDef<'a> {
    fn from(x: &'a StructDef<P>) -> Self {
        P::wrap_struct_def(x)
    }
}

impl<'a> From<&'a OpaqueDef> for TypeDef<'a> {
    fn from(x: &'a OpaqueDef) -> Self {
        TypeDef::Opaque(x)
    }
}

impl<'a> From<&'a EnumDef> for TypeDef<'a> {
    fn from(x: &'a EnumDef) -> Self {
        TypeDef::Enum(x)
    }
}

impl<'tcx> TypeDef<'tcx> {
    pub fn name(&self) -> &'tcx IdentBuf {
        match *self {
            Self::Struct(ty) => &ty.name,
            Self::OutStruct(ty) => &ty.name,
            Self::Opaque(ty) => &ty.name,
            Self::Enum(ty) => &ty.name,
        }
    }

    pub fn docs(&self) -> &'tcx Docs {
        match *self {
            Self::Struct(ty) => &ty.docs,
            Self::OutStruct(ty) => &ty.docs,
            Self::Opaque(ty) => &ty.docs,
            Self::Enum(ty) => &ty.docs,
        }
    }
    pub fn methods(&self) -> &'tcx [Method] {
        match *self {
            Self::Struct(ty) => &ty.methods,
            Self::OutStruct(ty) => &ty.methods,
            Self::Opaque(ty) => &ty.methods,
            Self::Enum(ty) => &ty.methods,
        }
    }

    pub fn attrs(&self) -> &'tcx Attrs {
        match *self {
            Self::Struct(ty) => &ty.attrs,
            Self::OutStruct(ty) => &ty.attrs,
            Self::Opaque(ty) => &ty.attrs,
            Self::Enum(ty) => &ty.attrs,
        }
    }

    pub fn special_method_presence(&self) -> &'tcx SpecialMethodPresence {
        match *self {
            Self::Struct(ty) => &ty.special_method_presence,
            Self::OutStruct(ty) => &ty.special_method_presence,
            Self::Opaque(ty) => &ty.special_method_presence,
            Self::Enum(ty) => &ty.special_method_presence,
        }
    }
}

impl<'a, P: TyPosition> From<&'a StructDef<P>> for SymbolDef<'a> {
    fn from(x: &'a StructDef<P>) -> Self {
        P::wrap_struct_def(x).into()
    }
}

impl<'a> From<&'a OpaqueDef> for SymbolDef<'a> {
    fn from(x: &'a OpaqueDef) -> Self {
        SymbolDef::Opaque(x)
    }
}
impl<'a> From<TypeDef<'a>> for SymbolDef<'a> {
    fn from(x: TypeDef<'a>) -> Self {
        match x {
            TypeDef::Struct(ty) => SymbolDef::Struct(ty),
            TypeDef::OutStruct(ty) => SymbolDef::OutStruct(ty),
            TypeDef::Opaque(ty) => SymbolDef::Opaque(ty),
            TypeDef::Enum(ty) => SymbolDef::Enum(ty),
        }
    }
}

impl<'a> From<&'a EnumDef> for SymbolDef<'a> {
    fn from(x: &'a EnumDef) -> Self {
        SymbolDef::Enum(x)
    }
}

impl<'a> From<&'a TraitDef> for SymbolDef<'a> {
    fn from(x: &'a TraitDef) -> Self {
        SymbolDef::Trait(x)
    }
}

impl<'a> From<&'a Method> for SymbolDef<'a> {
    fn from(x: &'a Method) -> Self {
        SymbolDef::Method(x)
    }
}

impl<'tcx> SymbolDef<'tcx> {
    pub fn name(&self) -> &'tcx IdentBuf {
        match *self {
            Self::Struct(ty) => &ty.name,
            Self::OutStruct(ty) => &ty.name,
            Self::Opaque(ty) => &ty.name,
            Self::Enum(ty) => &ty.name,
            Self::Trait(t) => &t.name,
            Self::Method(m) => &m.name,
        }
    }
    pub fn attrs(&self) -> &'tcx Attrs {
        match *self {
            Self::Struct(ty) => &ty.attrs,
            Self::OutStruct(ty) => &ty.attrs,
            Self::Opaque(ty) => &ty.attrs,
            Self::Enum(ty) => &ty.attrs,
            Self::Trait(t) => &t.attrs,
            Self::Method(m) => &m.attrs,
        }
    }
    pub fn docs(&self) -> &'tcx Docs {
        match *self {
            Self::Struct(ty) => &ty.docs,
            Self::OutStruct(ty) => &ty.docs,
            Self::Opaque(ty) => &ty.docs,
            Self::Enum(ty) => &ty.docs,
            Self::Trait(t) => &t.docs,
            Self::Method(m) => &m.docs,
        }
    }
}
