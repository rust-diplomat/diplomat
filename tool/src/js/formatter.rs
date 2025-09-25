//! Formatter functions for Javascript for converting Rust types into Typescript types.
//!
//! Used in [`super::type_generation`] and [`crate::demo_gen`].
use std::{borrow::Cow, fmt::Write};

use diplomat_core::hir::{
    self, Attrs, Docs, DocsTypeReferenceSyntax, DocsUrlGenerator, EnumVariant, SpecialMethod,
    TypeDef,
};
use heck::{ToLowerCamelCase, ToUpperCamelCase};

use super::FileType;

/// Javascript words that a Diplomat user shouldn't create classes or functions from.
const RESERVED: &[&str] = &[
    "break",
    "case",
    "catch",
    "class",
    "const",
    "continue",
    "debugger",
    "default",
    "delete",
    "do",
    "else",
    "export",
    "extends",
    "false",
    "finally",
    "for",
    "function",
    "if",
    "import",
    "in",
    "instanceof",
    "new",
    "null",
    "return",
    "super",
    "switch",
    "this",
    "throw",
    "true",
    "try",
    "typeof",
    "undefined",
    "var",
    "void",
    "while",
    "with",
];

/// From https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects.
///
/// If you create a class from these, JS will error. So we throw an error if that happens.
const RESERVED_TYPES: &[&str] = &["Infinity", "NaN"];

/// Helper class for us to format JS identifiers from the HIR.
pub(crate) struct JSFormatter<'tcx> {
    /// Per [`CFormatter`]'s documentation we use it for support.
    /// For generating doc.rs links
    docs_url_gen: &'tcx DocsUrlGenerator,
}

impl<'tcx> JSFormatter<'tcx> {
    pub fn new(docs_url_gen: &'tcx DocsUrlGenerator) -> Self {
        Self { docs_url_gen }
    }

    /// Given a [`TypeId`] that we're reading, make sure to rename it appropriately, or throw an error if it's reserved.
    pub fn fmt_type_name(&self, type_def: TypeDef<'tcx>) -> Cow<'tcx, str> {
        let name = type_def
            .attrs()
            .rename
            .apply(type_def.name().as_str().into());

        if RESERVED_TYPES.contains(&&*name) || RESERVED.contains(&&*name) {
            panic!("{name} is not an allowed type in JS. Please rename.")
        }

        name
    }

    /// Generate a `.mjs` or `.d.ts` file name. Just don't give it that extension yet.
    pub fn fmt_file_name_extensionless(&self, type_name: &str) -> String {
        type_name.to_string()
    }

    /// Add an extension to [`Self::fmt_file_name_extensionless`].
    pub fn fmt_file_name(&self, type_name: &str, file_type: &FileType) -> String {
        match file_type {
            FileType::Module => format!("{}.mjs", self.fmt_file_name_extensionless(type_name)),
            FileType::Typescript => format!("{}.d.ts", self.fmt_file_name_extensionless(type_name)),
        }
    }

    /// Just creates `/** */` doc strings.
    pub fn fmt_docs(&self, docs: &Docs, attrs: &Attrs) -> String {
        let mut docs = docs
            .to_markdown(DocsTypeReferenceSyntax::AtLink, self.docs_url_gen)
            .trim()
            .to_string();
        if let Some(deprecated) = attrs.deprecated.as_ref() {
            if !docs.is_empty() {
                docs.push('\n');
                docs.push('\n');
            }
            let _ = writeln!(&mut docs, "@deprecated {deprecated}");
        }
        docs
    }

    /// Creates the body of an `import` or `export` statement.
    pub fn fmt_module_statement(
        &self,
        type_name: &str,
        typescript: bool,
        relative_path: String,
        file_name: &str,
    ) -> String {
        format!(
            r#"{{ {type_name} }} from "{relative_path}{file_name}{}"#,
            match typescript {
                true => "",
                false => ".mjs",
            }
        )
    }

    /// Uses [`Self::fmt_module_statement`] to create an import statement.
    pub fn fmt_import_statement(
        &self,
        type_name: &str,
        typescript: bool,
        relative_path: String,
        file_name: &str,
    ) -> String {
        format!(
            r#"import {}{}""#,
            match typescript {
                true => "type ",
                false => "",
            },
            self.fmt_module_statement(type_name, typescript, relative_path, file_name)
        )
    }

    /// Uses [`Self::fmt_module_statement`] to create an export statement.
    pub fn fmt_export_statement(
        &self,
        type_name: &str,
        typescript: bool,
        relative_path: String,
        file_name: &str,
    ) -> String {
        format!(
            r#"export {}""#,
            self.fmt_module_statement(type_name, typescript, relative_path, file_name)
        )
    }

    // #region HIR::Type formatting.
    // This is only visible for Typescript definition files, but we use it to check if types are supported.

    /// Generate a JS primitive type from a Rust type.
    pub fn fmt_primitive_as_ffi(&self, primitive: hir::PrimitiveType) -> &'static str {
        match primitive {
            hir::PrimitiveType::Bool => "boolean",
            hir::PrimitiveType::Char => "codepoint",
            hir::PrimitiveType::Int(hir::IntType::I64 | hir::IntType::U64)
            | hir::PrimitiveType::Int128(_) => "bigint",
            hir::PrimitiveType::Int(_)
            | hir::PrimitiveType::Ordering
            | hir::PrimitiveType::IntSize(_)
            | hir::PrimitiveType::Byte
            | hir::PrimitiveType::Float(_) => "number",
        }
    }

    /// Generate a JS primitive slice type from a Rust type.
    pub fn fmt_primitive_slice(&self, primitive_type: hir::PrimitiveType) -> &'static str {
        match primitive_type {
            hir::PrimitiveType::Bool
            | hir::PrimitiveType::Byte
            | hir::PrimitiveType::Int(hir::IntType::U8) => "Uint8Array",
            hir::PrimitiveType::Int(hir::IntType::I8) => "Int8Array",
            hir::PrimitiveType::Int(hir::IntType::I16) => "Int16Array",
            hir::PrimitiveType::Int(hir::IntType::U16) => "Uint16Array",
            hir::PrimitiveType::Int(hir::IntType::I32)
            | hir::PrimitiveType::IntSize(hir::IntSizeType::Isize) => "Int32Array",
            hir::PrimitiveType::Int(hir::IntType::U32)
            | hir::PrimitiveType::IntSize(hir::IntSizeType::Usize)
            | hir::PrimitiveType::Char => "Uint32Array",
            hir::PrimitiveType::Int(hir::IntType::I64) => "BigInt64Array",
            hir::PrimitiveType::Int(hir::IntType::U64) => "BigUint64Array",
            hir::PrimitiveType::Float(hir::FloatType::F32) => "Float32Array",
            hir::PrimitiveType::Float(hir::FloatType::F64) => "Float64Array",
            hir::PrimitiveType::Int128(..) => {
                panic!("Int128 slices are not a supported type for the JS backend.")
            }
            hir::PrimitiveType::Ordering => {
                panic!("Lists of ordering not supported")
            }
        }
    }

    /// Generates a JS primitive list type from a Rust type.
    pub fn fmt_primitive_list_type(&self, primitive: hir::PrimitiveType) -> &'static str {
        match primitive {
            hir::PrimitiveType::Bool => "Array<boolean>",
            hir::PrimitiveType::Char => "Array<codepoint>",
            hir::PrimitiveType::Byte => "Uint8Array",
            hir::PrimitiveType::Int(hir::IntType::I64 | hir::IntType::U64) => "Array<bigint>",
            hir::PrimitiveType::Int(_)
            | hir::PrimitiveType::IntSize(_)
            | hir::PrimitiveType::Float(_) => "Array<number>",
            hir::PrimitiveType::Int128(_) => {
                panic!("Int128 slices are not a supported type for the JS backend.")
            }
            hir::PrimitiveType::Ordering => {
                panic!("Lists of ordering not supported")
            }
        }
    }

    /// We generate this in JS as a string that we then have to parse in `runtime.mjs`. Used we're trying to determine how to parse a given slice. See `DiplomatBuf` for more.
    pub fn fmt_primitive_list_view(&self, primitive: hir::PrimitiveType) -> &'static str {
        match primitive {
            hir::PrimitiveType::Bool => "boolean",
            hir::PrimitiveType::Char => "u16",
            hir::PrimitiveType::Byte => "u8",
            hir::PrimitiveType::Int(hir::IntType::I8) => "i8",
            hir::PrimitiveType::Int(hir::IntType::U8) => "u8",
            hir::PrimitiveType::Int(hir::IntType::I16) => "i16",
            hir::PrimitiveType::Int(hir::IntType::U16) => "u16",
            hir::PrimitiveType::Int(hir::IntType::I32)
            | hir::PrimitiveType::IntSize(hir::IntSizeType::Isize) => "i32",
            hir::PrimitiveType::Int(hir::IntType::U32)
            | hir::PrimitiveType::IntSize(hir::IntSizeType::Usize) => "u32",
            hir::PrimitiveType::Int(hir::IntType::I64) => "i64",
            hir::PrimitiveType::Int(hir::IntType::U64) => "u64",
            hir::PrimitiveType::Float(hir::FloatType::F32) => "f32",
            hir::PrimitiveType::Float(hir::FloatType::F64) => "f64",
            hir::PrimitiveType::Int128(hir::Int128Type::I128) => "i128",
            hir::PrimitiveType::Int128(hir::Int128Type::U128) => "u128",
            hir::PrimitiveType::Ordering => {
                panic!("Lists of ordering not supported")
            }
        }
    }

    pub fn fmt_void(&self) -> &'static str {
        "void"
    }

    pub fn fmt_nullable(&self, ident: &str) -> String {
        format!("{ident} | null")
    }

    pub fn fmt_string(&self) -> &'static str {
        "string"
    }
    // #endregion

    // #region Template specific formatting
    pub fn fmt_method_name(&self, method: &hir::Method) -> String {
        let name: String = method
            .attrs
            .rename
            .apply(
                if let Some(SpecialMethod::Getter(Some(ref name))) = method.attrs.special_method {
                    name
                } else {
                    method.name.as_str()
                }
                .into(),
            )
            .to_lower_camel_case();

        if RESERVED.contains(&&*name) {
            format!("{name}_")
        } else {
            name
        }
    }

    /// For formatting a JS method that has an associated name with it. Like a named constructor or getter/setter.
    pub fn fmt_method_field_name(&self, name: &Option<String>, method: &hir::Method) -> String {
        let name: String = method
            .attrs
            .rename
            .apply(name.as_deref().unwrap_or(method.name.as_str()).into())
            .to_lower_camel_case();

        if RESERVED.contains(&&*name) {
            format!("{name}_")
        } else {
            name
        }
    }

    pub fn fmt_param_name<'a>(&self, param_name: &'a str) -> Cow<'a, str> {
        param_name.to_lower_camel_case().into()
    }

    pub fn fmt_lifetime_edge_array(
        &self,
        lifetime: hir::Lifetime,
        lifetime_env: &hir::LifetimeEnv,
    ) -> Cow<'static, str> {
        format!("{}Edges", lifetime_env.fmt_lifetime(lifetime)).into()
    }

    pub fn fmt_enum_variant(&self, variant: &'tcx EnumVariant) -> Cow<'tcx, str> {
        let name = variant.name.as_str().to_upper_camel_case().into();
        variant.attrs.rename.apply(name)
    }

    // #endregion
}
