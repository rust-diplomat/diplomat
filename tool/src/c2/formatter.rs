//! This module contains functions for formatting types

use super::ty::ResultType;
use diplomat_core::hir::{self, OpaqueOwner, StringEncoding, Type, TypeContext, TypeId};
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
pub struct CFormatter<'tcx> {
    tcx: &'tcx TypeContext,
}

impl<'tcx> CFormatter<'tcx> {
    pub fn new(tcx: &'tcx TypeContext) -> Self {
        Self { tcx }
    }
    pub fn tcx(&self) -> &'tcx TypeContext {
        self.tcx
    }

    /// Resolve and format a named type for use in code
    pub fn fmt_type_name(&self, id: TypeId) -> Cow<'tcx, str> {
        // Currently don't do anything fancy
        // Eventually apply rename rules and such
        self.tcx.resolve_type(id).name().as_str().into()
    }

    /// Resolve and format a named type for use in diagnostics
    /// (don't apply rename rules and such)
    pub fn fmt_type_name_diagnostics(&self, id: TypeId) -> Cow<'tcx, str> {
        self.tcx.resolve_type(id).name().as_str().into()
    }
    /// Resolve and format the name of a type for use in header names: decl version
    //
    /// Enums can't be forward-declared in C, but we do want enums to have methods,
    /// which may require additional #includes leading to potential cycles.
    /// To handle this, we make a separate header file called Foo_decl.h, that contains
    /// *just* the enum. It is included from Foo.h, and external users should not be importing
    /// it directly. (We can potentially add a #define guard that makes this actually private, if needed)
    pub fn fmt_decl_header_path(&self, id: TypeId) -> String {
        let type_name = self.fmt_type_name(id);
        format!("{type_name}.d.h")
    }
    /// Resolve and format the name of a type for use in header names: impl version
    pub fn fmt_impl_header_path(&self, id: TypeId) -> String {
        let type_name = self.fmt_type_name(id);
        format!("{type_name}.h")
    }
    /// Resolve and format the name of a type for use in header names: result version
    pub fn fmt_result_header_path(&self, type_name: &str) -> String {
        format!("{type_name}.d.h")
    }
    /// Format an enum variant.
    pub fn fmt_enum_variant(
        &self,
        type_name: &str,
        variant: &'tcx hir::EnumVariant,
    ) -> Cow<'tcx, str> {
        let variant_name = variant.name.as_str();
        format!("{type_name}_{variant_name}").into()
    }
    /// Format a field name or parameter name
    // might need splitting in the future if we decide to support renames here
    pub fn fmt_param_name<'a>(&self, ident: &'a str) -> Cow<'a, str> {
        ident.into()
    }

    /// Format a method
    pub fn fmt_method_name(&self, ty: TypeId, method: &hir::Method) -> String {
        let ty_name = self.fmt_type_name(ty);
        let method_name = method.name.as_str();
        format!("{ty_name}_{method_name}")
    }

    pub fn fmt_ptr<'a>(&self, ident: &'a str, mutability: hir::Mutability) -> Cow<'a, str> {
        // TODO: Where is the right place to put `const` here?
        if mutability.is_mutable() {
            format!("{ident}*").into()
        } else {
            format!("const {ident}*").into()
        }
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
            Type::Slice(hir::Slice::Str(_, StringEncoding::UnvalidatedUtf8)) => "str_ref8".into(),
            Type::Slice(hir::Slice::Str(_, StringEncoding::Utf8)) => "str_refv8".into(),
            Type::Slice(hir::Slice::Str(_, StringEncoding::UnvalidatedUtf16)) => "str_ref16".into(),
            Type::Slice(hir::Slice::Primitive(borrow, p)) => {
                let constness = borrow.mutability.if_mut_else("", "const_");
                let prim = self.fmt_primitive_as_c(*p);
                format!("ref_{constness}prim_slice_{prim}").into()
            }
            &_ => unreachable!("unknown AST/HIR variant"),
        }
    }

    pub fn fmt_result_name(&self, ok_ty_name: &str, err_ty_name: &str) -> String {
        format!("diplomat_result_{ok_ty_name}_{err_ty_name}")
    }

    pub fn fmt_result_for_diagnostics(&self, r: ResultType) -> String {
        let ok = if let Some(ok) = r.0 {
            self.fmt_type_name_uniquely(ok)
        } else {
            "()".into()
        };
        let err = if let Some(err) = r.1 {
            self.fmt_type_name_uniquely(err)
        } else {
            "()".into()
        };

        format!("Result<{ok},{err}>")
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
            PrimitiveType::IntSize(IntSizeType::Isize) => "intptr_t",
            PrimitiveType::IntSize(IntSizeType::Usize) => "size_t",
            PrimitiveType::Float(FloatType::F32) => "float",
            PrimitiveType::Float(FloatType::F64) => "double",
        };
        s.into()
    }
}
