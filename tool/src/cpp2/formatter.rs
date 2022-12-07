//! This module contains functions for formatting types

use diplomat_core::hir::{self, OpaqueOwner, Type, TypeContext, TypeId};
use std::borrow::Cow;

/// This type mediates all formatting
///
/// All identifiers from the HIR should go through here before being formatted
/// into the output: This makes it easy to handle reserved words or add rename support
///
/// If you find yourself needing an identifier formatted in a context not yet available here, please add a new method
///
/// This type may be used by other backends attempting to figure out the names
/// of C types and methods.
pub struct Cpp2Formatter<'tcx> {
    tcx: &'tcx TypeContext,
}

impl<'tcx> Cpp2Formatter<'tcx> {
    pub fn new(tcx: &'tcx TypeContext) -> Self {
        Self { tcx }
    }
    /// Resolve and format a named type for use in code
    pub fn fmt_type_name(&self, id: TypeId) -> Cow<'tcx, str> {
        // Currently don't do anything fancy
        // Eventually apply rename rules and such
        self.tcx.resolve_type(id).name().as_str().into()
    }
    /// Resolve and format the name of a type for use in header names
    pub fn fmt_header_name(&self, id: TypeId) -> Cow<'tcx, str> {
        self.fmt_type_name(id)
    }
    /// Format an enum variant.
    pub fn fmt_enum_variant(&self, variant: &'tcx hir::EnumVariant) -> Cow<'tcx, str> {
        variant.name.as_str().into()
    }
    /// Format a field name or parameter name
    // might need splitting in the future if we decide to support renames here
    pub fn fmt_param_name<'a>(&self, ident: &'a str) -> Cow<'a, str> {
        ident.into()
    }

    pub fn fmt_optional<'a>(&self, ident: &'a str) -> Cow<'a, str> {
        format!("std::optional<{}>", ident).into()
    }

    pub fn fmt_borrowed<'a>(&self, ident: &'a str) -> Cow<'a, str> {
        format!("{}&", ident).into()
    }

    pub fn fmt_optional_borrowed<'a>(&self, ident: &'a str) -> Cow<'a, str> {
        format!("std::optional<{}&>", ident).into()
    }

    pub fn fmt_owned<'a>(&self, ident: &'a str) -> Cow<'a, str> {
        format!("std::unique_ptr<{}>", ident).into()
    }

    pub fn fmt_borrowed_slice<'a>(&self, ident: &'a str) -> Cow<'a, str> {
        format!("std::span<{}>", ident).into()
    }

    pub fn fmt_borrowed_str(&self) -> Cow<'static, str> {
        "std::string_view".into()
    }

    /// Format a method
    pub fn fmt_method_name(&self, ty: TypeId, method: &hir::Method) -> String {
        let ty_name = self.fmt_type_name(ty);
        let method_name = method.name.as_str();
        format!("{ty_name}_{method_name}")
    }

    /// Given a mutability, format a `const ` prefix for pointers if necessary,
    /// including a space for prepending
    pub fn fmt_constness<'a>(&self, ident: &'a str, mutability: hir::Mutability) -> Cow<'a, str> {
        mutability.if_mut_else(ident.into(), format!("const {}", ident).into())
    }

    /// Generates an identifier that uniquely identifies the given *C* type.
    /// Rust types that map to the same C type will get the same C identifier
    /// (e.g. &mut Foo and Option<&mut Foo> are all the same)
    ///
    /// This is primarily used for generating structs for result types,
    /// which require one struct for each distinct instance.
    pub fn fmt_type_name_uniquely<P: hir::TyPosition>(&self, ty: &'tcx Type<P>) -> Cow<'tcx, str> {
        match ty {
            Type::Primitive(p) => self.fmt_primitive_as_c(*p),
            Type::Opaque(o) => {
                let o_name = self.fmt_type_name(o.tcx_id.into());
                // Todo (breaking): box should be unified with the mutable branch
                let ownership = match o.owner.mutability() {
                    None => "box_",
                    Some(hir::Mutability::Mutable) => "",
                    Some(hir::Mutability::Immutable) => "const_",
                };
                format!("{ownership}{o_name}").into()
            }
            Type::Struct(s) => self.fmt_type_name(P::id_for_path(s)),
            Type::Enum(e) => self.fmt_type_name(e.tcx_id.into()),
            Type::Slice(hir::Slice::Str(_)) => "str_ref".into(),
            Type::Slice(hir::Slice::Primitive(borrow, p)) => {
                let constness = borrow.mutability.if_mut_else("", "const_");
                let prim = self.fmt_primitive_as_c(*p);
                format!("ref_{constness}prim_slice_{prim}").into()
            }
        }
    }

    /// Get the primitive type as a C type
    pub fn fmt_primitive_as_c(&self, prim: hir::PrimitiveType) -> Cow<'static, str> {
        use diplomat_core::hir::{FloatType, IntSizeType, IntType, PrimitiveType};
        let s = match prim {
            PrimitiveType::Bool => "bool",
            PrimitiveType::Char => "char32_t",
            PrimitiveType::Int(IntType::I8) => "int8_t",
            PrimitiveType::Int(IntType::U8) => "uint8_t",
            PrimitiveType::Int(IntType::I16) => "int16_t",
            PrimitiveType::Int(IntType::U16) => "uint16_t",
            PrimitiveType::Int(IntType::I32) => "int32_t",
            PrimitiveType::Int(IntType::U32) => "uint32_t",
            PrimitiveType::Int(IntType::I64) => "int64_t",
            PrimitiveType::Int(IntType::U64) => "uint64_t",
            PrimitiveType::Int128(_) => panic!("i128 not supported in C"),
            PrimitiveType::IntSize(IntSizeType::Isize) => "ssize_t",
            PrimitiveType::IntSize(IntSizeType::Usize) => "size_t",
            PrimitiveType::Float(FloatType::F32) => "float",
            PrimitiveType::Float(FloatType::F64) => "double",
        };
        s.into()
    }
}
