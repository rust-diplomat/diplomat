//! This module contains functions for formatting types

use diplomat_core::hir::{self, DocsUrlGenerator, TypeContext, TypeId};
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

    pub fn fmt_import(&self, path: &str, as_show_hide: Option<&str>) -> Cow<'static, str> {
        format!(
            "import '{path}'{}{};",
            if as_show_hide.is_some() { " " } else { "" },
            as_show_hide.unwrap_or_default(),
        )
        .into()
    }

    pub fn fmt_part_of_lib(&self) -> Cow<'static, str> {
        format!("part of '{}';", self.fmt_file_name("lib")).into()
    }

    pub fn fmt_part(&self, part: &str) -> Cow<'static, str> {
        format!("part '{}';", part).into()
    }

    pub fn fmt_docs(&self, docs: &hir::Docs) -> String {
        docs.to_markdown(self.docs_url_gen)
            .trim()
            .replace('\n', "\n/// ")
            .replace(" \n", "\n")
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
        let name = variant.name.as_str().to_lower_camel_case().into();
        variant.attrs.rename.apply(name)
    }

    /// Format a field name or parameter name
    // might need splitting in the future if we decide to support renames here
    pub fn fmt_param_name<'a>(&self, ident: &'a str) -> Cow<'a, str> {
        ident.to_lower_camel_case().into()
    }

    pub fn fmt_nullable(&self, ident: &str) -> String {
        format!("{ident}?")
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

    pub fn fmt_string(&self) -> &'static str {
        "String"
    }

    pub fn fmt_utf8_primitive(&self) -> &'static str {
        "ffi.Uint8"
    }

    pub fn fmt_utf16_primitive(&self) -> &'static str {
        "ffi.Uint16"
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

    pub fn fmt_opaque(&self) -> &'static str {
        "ffi.Opaque"
    }

    pub fn fmt_usize(&self, cast: bool) -> &'static str {
        self.fmt_primitive_as_ffi(hir::PrimitiveType::IntSize(hir::IntSizeType::Usize), cast)
    }

    pub fn fmt_type_as_ident(&self, ty: Option<&str>) -> String {
        ty.unwrap_or("Void")
            .replace(&self.fmt_pointer(self.fmt_opaque()), "Opaque")
            .replace("ffi.", "")
            .replace('_', "")
    }

    pub fn fmt_enum_as_ffi(&self, cast: bool) -> &'static str {
        self.fmt_primitive_as_ffi(hir::PrimitiveType::Int(hir::IntType::I32), cast)
    }

    pub fn fmt_primitive_as_ffi(&self, prim: hir::PrimitiveType, cast: bool) -> &'static str {
        use diplomat_core::hir::{FloatType, IntSizeType, IntType, PrimitiveType};
        if cast {
            match prim {
                PrimitiveType::Bool => "bool",
                PrimitiveType::Char => "Rune",
                PrimitiveType::Int(_) | PrimitiveType::IntSize(_) | PrimitiveType::Byte => "int",
                PrimitiveType::Float(_) => "double",
                PrimitiveType::Int128(_) => panic!("i128 not supported in Dart"),
            }
        } else {
            match prim {
                PrimitiveType::Bool => "ffi.Bool",
                PrimitiveType::Char => "ffi.Uint32",
                PrimitiveType::Int(IntType::I8) => "ffi.Int8",
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

    pub fn fmt_primitive_list_type(&self, prim: hir::PrimitiveType) -> &'static str {
        use diplomat_core::hir::PrimitiveType;
        match prim {
            PrimitiveType::Bool => "core.List<bool>",
            PrimitiveType::Char => "core.List<Rune>",
            PrimitiveType::Byte => "ByteBuffer",
            PrimitiveType::Int(_) | PrimitiveType::IntSize(_) => "core.List<int>",
            PrimitiveType::Float(_) => "core.List<double>",
            PrimitiveType::Int128(_) => panic!("i128 not supported in Dart"),
        }
    }

    pub fn fmt_primitive_list_view(&self, prim: hir::PrimitiveType) -> &'static str {
        use diplomat_core::hir::{FloatType, IntSizeType, IntType, PrimitiveType};
        match prim {
            PrimitiveType::Bool => ".boolView",
            PrimitiveType::Char => ".uint32View",
            PrimitiveType::Byte => "",
            PrimitiveType::Int(IntType::I8) => ".int8View",
            PrimitiveType::Int(IntType::U8) => ".uint8View",
            PrimitiveType::Int(IntType::I16) => ".int16View",
            PrimitiveType::Int(IntType::U16) => ".uint16View",
            PrimitiveType::Int(IntType::I32) => ".int32View",
            PrimitiveType::Int(IntType::U32) => ".uint32View",
            PrimitiveType::Int(IntType::I64) => ".int64View",
            PrimitiveType::Int(IntType::U64) => ".uint64View",
            PrimitiveType::IntSize(IntSizeType::Usize) => ".usizeView",
            PrimitiveType::IntSize(IntSizeType::Isize) => ".isizeView",
            PrimitiveType::Float(FloatType::F32) => ".float32View",
            PrimitiveType::Float(FloatType::F64) => ".float64View",
            PrimitiveType::Int128(_) => panic!("i128 not supported in Dart"),
        }
    }

    pub fn fmt_slice_type(&self, prim: hir::PrimitiveType) -> &'static str {
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
        }
    }

    pub fn fmt_utf8_slice_type(&self) -> &'static str {
        "_SliceUtf8"
    }

    pub fn fmt_utf16_slice_type(&self) -> &'static str {
        "_SliceUtf16"
    }
}
