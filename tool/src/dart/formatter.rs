//! This module contains functions for formatting types

use diplomat_core::hir::{self, DocsTypeReferenceSyntax, DocsUrlGenerator, TypeContext, TypeId};
use heck::ToLowerCamelCase;
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
pub(super) struct DartFormatter<'tcx> {
    tcx: &'tcx TypeContext,
    docs_url_gen: &'tcx DocsUrlGenerator,
}

const INVALID_METHOD_NAMES: &[&str] = &["new", "static", "default"];
const INVALID_FIELD_NAMES: &[&str] = &["new", "static", "default"];
const DISALLOWED_CORE_TYPES: &[&str] = &["Object", "String"];

impl<'tcx> DartFormatter<'tcx> {
    pub fn new(tcx: &'tcx TypeContext, docs_url_gen: &'tcx DocsUrlGenerator) -> Self {
        Self { tcx, docs_url_gen }
    }

    pub fn fmt_lifetime_edge_array(
        &self,
        lifetime: hir::Lifetime,
        lifetime_env: &hir::LifetimeEnv,
    ) -> Cow<'static, str> {
        format!("{}Edges", lifetime_env.fmt_lifetime(lifetime)).into()
    }

    pub fn fmt_file_name(&self, name: &str) -> String {
        format!("{name}.g.dart")
    }

    pub fn fmt_import(
        &self,
        path: &str,
        as_show_hide: Option<&str>,
        ignore_warning: Option<&str>,
    ) -> Cow<'static, str> {
        let ignore = if let Some(ignore) = ignore_warning {
            format!("// ignore: {ignore}\n")
        } else {
            Default::default()
        };

        format!(
            "{ignore}import '{path}'{}{};",
            if as_show_hide.is_some() { " " } else { "" },
            as_show_hide.unwrap_or_default(),
        )
        .into()
    }

    pub fn fmt_part_of_lib(&self) -> Cow<'static, str> {
        format!("part of '{}';", self.fmt_file_name("lib")).into()
    }

    pub fn fmt_part(&self, part: &str) -> Cow<'static, str> {
        format!("part '{part}';").into()
    }

    pub fn fmt_docs(&self, docs: &hir::Docs) -> String {
        docs.to_markdown(DocsTypeReferenceSyntax::SquareBrackets, self.docs_url_gen)
            .trim()
            .to_string()
    }

    /// Resolve and format a named type for use in code
    pub fn fmt_type_name(&self, id: TypeId) -> Cow<'tcx, str> {
        let resolved = self.tcx.resolve_type(id);

        let candidate = resolved.name().as_str();

        if DISALLOWED_CORE_TYPES.contains(&candidate) {
            panic!("{candidate:?} is not a valid Dart type name. Please rename.");
        }

        resolved.attrs().rename.apply(candidate.into())
    }

    /// Format an enum variant.
    pub fn fmt_enum_variant(&self, variant: &'tcx hir::EnumVariant) -> Cow<'tcx, str> {
        variant
            .attrs
            .rename
            .apply(variant.name.as_str().into())
            .to_lower_camel_case()
            .into()
    }

    /// Format a field name or parameter name
    // might need splitting in the future if we decide to support renames here
    pub fn fmt_param_name<'a>(&self, ident: &'a str) -> Cow<'a, str> {
        ident.to_lower_camel_case().into()
    }

    pub fn fmt_nullable(&self, ident: &str) -> String {
        if ident.ends_with('?') {
            ident.to_string()
        } else {
            format!("{ident}?")
        }
    }

    /// Format a method
    pub fn fmt_method_name(&self, method: &hir::Method) -> String {
        // TODO(#60): handle other keywords
        let name = method
            .attrs
            .rename
            .apply(method.name.as_str().into())
            .to_lower_camel_case();
        if INVALID_METHOD_NAMES.contains(&&*name) {
            format!("{name}_")
        } else {
            name
        }
    }

    pub fn fmt_constructor_name(&self, name: &Option<String>, method: &hir::Method) -> String {
        let name = method
            .attrs
            .rename
            .apply(name.as_deref().unwrap_or(method.name.as_str()).into())
            .to_lower_camel_case();

        if INVALID_METHOD_NAMES.contains(&name.as_str()) {
            format!("{name}_")
        } else {
            name
        }
    }

    pub fn fmt_accessor_name(&self, name: &Option<String>, method: &hir::Method) -> String {
        let name = method
            .attrs
            .rename
            .apply(name.as_deref().unwrap_or(method.name.as_str()).into())
            .to_lower_camel_case();

        if INVALID_FIELD_NAMES.contains(&name.as_str()) {
            format!("{name}_")
        } else {
            name
        }
    }

    pub fn fmt_void(&self) -> &'static str {
        "void"
    }

    pub fn fmt_ffi_void(&self) -> &'static str {
        "ffi.Void"
    }

    pub fn fmt_pointer(&self, target: &str) -> String {
        format!("ffi.Pointer<{target}>")
    }

    pub fn fmt_opaque_as_ffi(&self) -> String {
        self.fmt_pointer("ffi.Opaque")
    }

    pub fn fmt_enum_as_ffi(&self, cast: bool) -> &'static str {
        self.fmt_primitive_as_ffi(hir::PrimitiveType::Int(hir::IntType::I32), cast)
    }

    pub fn fmt_type_as_ident(&self, ty: Option<&str>) -> String {
        ty.unwrap_or("Void")
            .replace(&self.fmt_opaque_as_ffi(), "Opaque")
            .replace("ffi.", "")
            .replace('_', "")
    }

    pub fn fmt_primitive_as_ffi(&self, prim: hir::PrimitiveType, cast: bool) -> &'static str {
        use diplomat_core::hir::{FloatType, IntSizeType, IntType, PrimitiveType};
        if cast {
            match prim {
                PrimitiveType::Bool => "bool",
                PrimitiveType::Char => "Rune",
                PrimitiveType::Int(_)
                | PrimitiveType::IntSize(_)
                | PrimitiveType::Byte
                | PrimitiveType::Ordering => "int",
                PrimitiveType::Float(_) => "double",
                PrimitiveType::Int128(_) => panic!("i128 not supported in Dart"),
            }
        } else {
            match prim {
                PrimitiveType::Bool => "ffi.Bool",
                PrimitiveType::Char => "ffi.Uint32",
                PrimitiveType::Int(IntType::I8) | PrimitiveType::Ordering => "ffi.Int8",
                PrimitiveType::Int(IntType::U8) | PrimitiveType::Byte => "ffi.Uint8",
                PrimitiveType::Int(IntType::I16) => "ffi.Int16",
                PrimitiveType::Int(IntType::U16) => "ffi.Uint16",
                PrimitiveType::Int(IntType::I32) => "ffi.Int32",
                PrimitiveType::Int(IntType::U32) => "ffi.Uint32",
                PrimitiveType::Int(IntType::I64) => "ffi.Int64",
                PrimitiveType::Int(IntType::U64) => "ffi.Uint64",
                PrimitiveType::IntSize(IntSizeType::Isize) => "ffi.IntPtr",
                PrimitiveType::IntSize(IntSizeType::Usize) => "ffi.Size",
                PrimitiveType::Float(FloatType::F32) => "ffi.Float",
                PrimitiveType::Float(FloatType::F64) => "ffi.Double",
                PrimitiveType::Int128(_) => panic!("i128 not supported in Dart"),
            }
        }
    }

    pub fn fmt_string_element_as_ffi(&self, encoding: hir::StringEncoding) -> &'static str {
        match encoding {
            hir::StringEncoding::Utf8 | hir::StringEncoding::UnvalidatedUtf8 => "ffi.Uint8",
            hir::StringEncoding::UnvalidatedUtf16 => "ffi.Uint16",
            _ => unreachable!("unknown AST/HIR variant"),
        }
    }

    pub fn fmt_primitive_list_type(&self, prim: hir::PrimitiveType) -> &'static str {
        use diplomat_core::hir::PrimitiveType;
        match prim {
            PrimitiveType::Bool => "core.List<bool>",
            PrimitiveType::Char => "core.List<Rune>",
            PrimitiveType::Byte => "ByteBuffer",
            PrimitiveType::Int(_) | PrimitiveType::IntSize(_) => "core.List<int>",
            PrimitiveType::Float(_) => "core.List<double>",
            PrimitiveType::Int128(_) => panic!("i128 not supported in Dart"),
            PrimitiveType::Ordering => panic!("List of ordering not supported"),
        }
    }

    pub fn fmt_string_type(&self, _encoding: hir::StringEncoding) -> &'static str {
        "String"
    }

    pub fn fmt_string_list_type(&self, _encoding: hir::StringEncoding) -> &'static str {
        "core.List<core.String>"
    }

    pub fn fmt_primitive_alloc_in(&self, prim: hir::PrimitiveType) -> &'static str {
        use diplomat_core::hir::{FloatType, IntSizeType, IntType, PrimitiveType};
        match prim {
            PrimitiveType::Bool => "_boolAllocIn",
            PrimitiveType::Byte => unreachable!("custom handling"),
            PrimitiveType::Int(IntType::I8) => "_int8AllocIn",
            PrimitiveType::Int(IntType::U8) => "_uint8AllocIn",
            PrimitiveType::Int(IntType::I16) => "_int16AllocIn",
            PrimitiveType::Int(IntType::U16) => "_uint16AllocIn",
            PrimitiveType::Int(IntType::I32) => "_int32AllocIn",
            PrimitiveType::Int(IntType::U32) | PrimitiveType::Char => "_uint32AllocIn",
            PrimitiveType::Int(IntType::I64) => "_int64AllocIn",
            PrimitiveType::Int(IntType::U64) => "_uint64AllocIn",
            PrimitiveType::IntSize(IntSizeType::Usize) => "_usizeAllocIn",
            PrimitiveType::IntSize(IntSizeType::Isize) => "_isizeAllocIn",
            PrimitiveType::Float(FloatType::F32) => "_float32AllocIn",
            PrimitiveType::Float(FloatType::F64) => "_float64AllocIn",
            PrimitiveType::Int128(_) => panic!("i128 not supported in Dart"),
            PrimitiveType::Ordering => panic!("List of ordering not supported"),
        }
    }

    pub fn fmt_str_alloc_in(&self, encoding: hir::StringEncoding) -> &'static str {
        match encoding {
            hir::StringEncoding::Utf8 | hir::StringEncoding::UnvalidatedUtf8 => "_utf8AllocIn",
            hir::StringEncoding::UnvalidatedUtf16 => "_utf16AllocIn",
            _ => unreachable!("unknown AST/HIR variant"),
        }
    }

    pub fn fmt_str_slice_alloc_in(&self, encoding: hir::StringEncoding) -> &'static str {
        match encoding {
            hir::StringEncoding::Utf8 | hir::StringEncoding::UnvalidatedUtf8 => "_utf8SliceAllocIn",
            hir::StringEncoding::UnvalidatedUtf16 => "_utf16SliceAllocIn",
            _ => unreachable!("unknown AST/HIR variant"),
        }
    }

    /// Get the FFI slice type corresponding to a slice
    ///
    /// Note: you probably want to call gen_slice() to ensure helpers get made
    pub fn fmt_slice_type<P: hir::TyPosition>(&self, slice: &hir::Slice<P>) -> &'static str {
        match slice {
            hir::Slice::Primitive(_, p) => self.fmt_prim_slice_type(*p),
            hir::Slice::Str(_, encoding) => self.fmt_str_slice_type(*encoding),
            hir::Slice::Strs(encoding) => self.fmt_str_slice_slice_type(*encoding),
            _ => unreachable!("unknown AST/HIR variant"),
        }
    }

    fn fmt_prim_slice_type(&self, prim: hir::PrimitiveType) -> &'static str {
        use diplomat_core::hir::{FloatType, IntSizeType, IntType, PrimitiveType};
        match prim {
            PrimitiveType::Bool => "_SliceBool",
            PrimitiveType::Char => "_SliceRune",
            PrimitiveType::Int(IntType::I8) => "_SliceInt8",
            PrimitiveType::Int(IntType::U8) | PrimitiveType::Byte => "_SliceUint8",
            PrimitiveType::Int(IntType::I16) => "_SliceInt16",
            PrimitiveType::Int(IntType::U16) => "_SliceUint16",
            PrimitiveType::Int(IntType::I32) => "_SliceInt32",
            PrimitiveType::Int(IntType::U32) => "_SliceUint32",
            PrimitiveType::Int(IntType::I64) => "_SliceInt64",
            PrimitiveType::Int(IntType::U64) => "_SliceUint64",
            PrimitiveType::IntSize(IntSizeType::Usize) => "_SliceUsize",
            PrimitiveType::IntSize(IntSizeType::Isize) => "_SliceIsize",
            PrimitiveType::Float(FloatType::F32) => "_SliceFloat",
            PrimitiveType::Float(FloatType::F64) => "_SliceDouble",
            PrimitiveType::Int128(_) => panic!("i128 not supported in Dart"),
            PrimitiveType::Ordering => panic!("List of ordering not supported"),
        }
    }

    fn fmt_str_slice_type(&self, encoding: hir::StringEncoding) -> &'static str {
        match encoding {
            hir::StringEncoding::Utf8 | hir::StringEncoding::UnvalidatedUtf8 => "_SliceUtf8",
            hir::StringEncoding::UnvalidatedUtf16 => "_SliceUtf16",
            _ => unreachable!("unknown AST/HIR variant"),
        }
    }

    fn fmt_str_slice_slice_type(&self, encoding: hir::StringEncoding) -> &'static str {
        match encoding {
            hir::StringEncoding::Utf8 | hir::StringEncoding::UnvalidatedUtf8 => "_SliceSliceUtf8",
            hir::StringEncoding::UnvalidatedUtf16 => "_SliceSliceUtf16",
            _ => unreachable!("unknown AST/HIR variant"),
        }
    }
}
