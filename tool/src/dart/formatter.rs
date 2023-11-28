//! This module contains functions for formatting types

use crate::c2::CFormatter;
use diplomat_core::ast::{DocsUrlGenerator, MarkdownStyle};
use diplomat_core::hir::{self, TypeContext, TypeId};
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
    c: CFormatter<'tcx>,
    docs_url_generator: &'tcx DocsUrlGenerator,
    strip_prefix: Option<String>,
}

const INVALID_METHOD_NAMES: &[&str] = &["new", "static", "default"];
const INVALID_FIELD_NAMES: &[&str] = &["new", "static", "default"];
const DISALLOWED_CORE_TYPES: &[&str] = &["Object", "String"];

impl<'tcx> DartFormatter<'tcx> {
    pub fn new(
        tcx: &'tcx TypeContext,
        docs_url_generator: &'tcx DocsUrlGenerator,
        strip_prefix: Option<String>,
    ) -> Self {
        Self {
            c: CFormatter::new(tcx),
            docs_url_generator,
            strip_prefix,
        }
    }

    pub fn fmt_file_name(&self, name: &str) -> String {
        format!("{name}.g.dart")
    }

    pub fn fmt_import(&self, path: &str, as_show_hide: Option<&str>) -> Cow<'static, str> {
        format!(
            "import '{path}'{}{};",
            if as_show_hide.is_some() { "" } else { " " },
            if let Some(s) = as_show_hide { s } else { " " },
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
        docs.to_markdown(self.docs_url_generator, MarkdownStyle::Normal)
            .trim()
            .replace('\n', "\n/// ")
            .replace(" \n", "\n")
            .replace(
                &format!("`{}", self.strip_prefix.as_deref().unwrap_or("")),
                "`",
            )
    }

    pub fn fmt_destructor_name(&self, id: TypeId) -> String {
        let ty_name = self.c.fmt_type_name(id);
        format!("{ty_name}_destroy")
    }

    /// Resolve and format a named type for use in code
    pub fn fmt_type_name(&self, id: TypeId) -> Cow<'tcx, str> {
        let resolved = self.c.tcx().resolve_type(id);
        let candidate: Cow<'tcx, str> = if let Some(rename) = resolved.attrs().rename.as_ref() {
            rename.into()
        } else if let Some(strip_prefix) = self.strip_prefix.as_ref() {
            resolved
                .name()
                .as_str()
                .strip_prefix(strip_prefix)
                .unwrap_or(resolved.name().as_str())
                .into()
        } else {
            resolved.name().as_str().into()
        };

        if DISALLOWED_CORE_TYPES.contains(&&*candidate) {
            panic!("{candidate:?} is not a valid Dart type name. Please rename.");
        }

        candidate
    }

    /// Resolve and format a named type for use in diagnostics
    /// (don't apply rename rules and such)
    pub fn fmt_type_name_diagnostics(&self, id: TypeId) -> Cow<'tcx, str> {
        self.c.fmt_type_name_diagnostics(id)
    }

    /// Format an enum variant.
    pub fn fmt_enum_variant(&self, variant: &'tcx hir::EnumVariant) -> Cow<'tcx, str> {
        if let Some(rename) = variant.attrs.rename.as_ref() {
            rename.into()
        } else {
            variant.name.as_str().to_lower_camel_case().into()
        }
    }

    /// Format a field name or parameter name
    // might need splitting in the future if we decide to support renames here
    pub fn fmt_param_name<'a>(&self, ident: &'a str) -> Cow<'a, str> {
        ident.to_lower_camel_case().into()
    }

    pub fn fmt_nullable<'a>(&self, ident: &'a str) -> Cow<'a, str> {
        format!("{ident}?").into()
    }

    /// Format a method
    pub fn fmt_method_name<'a>(&self, method: &'a hir::Method) -> Cow<'a, str> {
        // TODO(#60): handle other keywords
        if let Some(rename) = method.attrs.rename.as_ref() {
            rename.into()
        } else {
            let name = method.name.as_str().to_lower_camel_case();
            if INVALID_METHOD_NAMES.contains(&name.as_str()) {
                format!("{name}_").into()
            } else {
                name.into()
            }
        }
    }

    pub fn fmt_constructor_name(&self, method: &hir::Method) -> Option<String> {
        let mut name = self.fmt_method_name(method).into_owned();
        for prefix in ["try", "create", "new_", "new", "default_", "default", "get"] {
            name = name
                .strip_prefix(prefix)
                .map(|s| s.to_lower_camel_case())
                .unwrap_or(name);
        }

        if name.is_empty() {
            None
        } else if INVALID_METHOD_NAMES.contains(&name.as_str()) {
            Some(format!("{name}_"))
        } else {
            Some(name)
        }
    }

    pub fn fmt_setter_name(&self, method: &hir::Method) -> String {
        let name = &*self.fmt_method_name(method);
        let name = name.strip_prefix("set").unwrap().to_lower_camel_case();

        if INVALID_FIELD_NAMES.contains(&name.as_str()) {
            format!("{name}_")
        } else {
            name
        }
    }

    pub fn fmt_c_method_name<'a>(&self, ty: TypeId, method: &'a hir::Method) -> Cow<'a, str> {
        self.c.fmt_method_name(ty, method).into()
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
                PrimitiveType::Int(_) | PrimitiveType::IntSize(IntSizeType::Usize) => "int",
                PrimitiveType::IntSize(IntSizeType::Isize) => panic!("isize not supported in Dart"),
                PrimitiveType::Int128(_) => panic!("i128 not supported in Dart"),
                PrimitiveType::Float(_) => "double",
            }
        } else {
            match prim {
                PrimitiveType::Bool => "ffi.Bool",
                PrimitiveType::Char => "ffi.Uint32",
                PrimitiveType::Int(IntType::I8) => "ffi.Int8",
                PrimitiveType::Int(IntType::U8) => "ffi.Uint8",
                PrimitiveType::Int(IntType::I16) => "ffi.Int16",
                PrimitiveType::Int(IntType::U16) => "ffi.Uint16",
                PrimitiveType::Int(IntType::I32) => "ffi.Int32",
                PrimitiveType::Int(IntType::U32) => "ffi.Uint32",
                PrimitiveType::Int(IntType::I64) => "ffi.Int64",
                PrimitiveType::Int(IntType::U64) => "ffi.Uint64",
                PrimitiveType::Int128(_) => panic!("i128 not supported in Dart"),
                PrimitiveType::IntSize(IntSizeType::Isize) => panic!("isize not supported in Dart"),
                PrimitiveType::IntSize(IntSizeType::Usize) => "ffi.Size",
                PrimitiveType::Float(FloatType::F32) => "ffi.Float",
                PrimitiveType::Float(FloatType::F64) => "ffi.Double",
            }
        }
    }

    pub fn fmt_primitive_list_type(&self, prim: hir::PrimitiveType) -> &'static str {
        use diplomat_core::hir::{FloatType, IntSizeType, IntType, PrimitiveType};
        match prim {
            PrimitiveType::Bool => panic!("bool not supported in lists"),
            PrimitiveType::Char => "RuneList",
            PrimitiveType::Int(IntType::I8) => "Int8List",
            PrimitiveType::Int(IntType::U8) => "Uint8List",
            PrimitiveType::Int(IntType::I16) => "Int16List",
            PrimitiveType::Int(IntType::U16) => "Uint16List",
            PrimitiveType::Int(IntType::I32) => "Int32List",
            PrimitiveType::Int(IntType::U32) => "Uint32List",
            PrimitiveType::Int(IntType::I64) => "Int64List",
            PrimitiveType::Int(IntType::U64) => "Uint64List",
            PrimitiveType::Int128(_) => panic!("i128 not supported in Dart"),
            PrimitiveType::IntSize(IntSizeType::Isize) => panic!("isize not supported in Dart"),
            PrimitiveType::IntSize(IntSizeType::Usize) => "core.List<int>", // no typed list
            PrimitiveType::Float(FloatType::F32) => "Float32List",
            PrimitiveType::Float(FloatType::F64) => "Float64List",
        }
    }

    pub fn fmt_slice_type(&self, prim: hir::PrimitiveType) -> &'static str {
        use diplomat_core::hir::{FloatType, IntSizeType, IntType, PrimitiveType};
        match prim {
            PrimitiveType::Bool => panic!("bool not supported in lists"),
            PrimitiveType::Char => "_SliceRune",
            PrimitiveType::Int(IntType::I8) => "_SliceInt8",
            PrimitiveType::Int(IntType::U8) => "_SliceUint8",
            PrimitiveType::Int(IntType::I16) => "_SliceInt16",
            PrimitiveType::Int(IntType::U16) => "_SliceUint16",
            PrimitiveType::Int(IntType::I32) => "_SliceInt32",
            PrimitiveType::Int(IntType::U32) => "_SliceUint32",
            PrimitiveType::Int(IntType::I64) => "_SliceInt64",
            PrimitiveType::Int(IntType::U64) => "_SliceUint64",
            PrimitiveType::Int128(_) => panic!("i128 not supported in Dart"),
            PrimitiveType::IntSize(IntSizeType::Usize) => "_SliceSize",
            PrimitiveType::IntSize(_) => panic!("isize not supported in Dart"),
            PrimitiveType::Float(FloatType::F32) => "_SliceFloat",
            PrimitiveType::Float(FloatType::F64) => "_SliceDouble",
        }
    }

    pub fn fmt_utf8_slice_type(&self) -> &'static str {
        "_SliceUtf8"
    }

    pub fn fmt_utf16_slice_type(&self) -> &'static str {
        "_SliceUtf16"
    }
}
