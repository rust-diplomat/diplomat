use std::borrow::Cow;

use diplomat_core::{
    ast::DocsUrlGenerator,
    hir::{self, Docs, EnumVariant, TypeContext, TypeId},
};
use heck::{ToLowerCamelCase, ToUpperCamelCase};

use crate::c2::CFormatter;

use super::FileType;

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
    "var",
    "void",
    "while",
    "with",
];

/// Helper class for us to format JS identifiers from the HIR.
pub(super) struct JSFormatter<'tcx> {
    /// Per [`CFormatter`]'s documentation we use it for support.
    c_formatter: CFormatter<'tcx>,

    /// For generating doc.rs links
    docs_url_gen: &'tcx DocsUrlGenerator,
}

impl<'tcx> JSFormatter<'tcx> {
    pub fn new(tcx: &'tcx TypeContext, docs_url_gen: &'tcx DocsUrlGenerator) -> Self {
        Self {
            c_formatter: CFormatter::new(tcx, false),
            docs_url_gen,
        }
    }

    pub fn fmt_type_name(&self, id: TypeId) -> Cow<'tcx, str> {
        let type_def = self.c_formatter.tcx().resolve_type(id);

        type_def
            .attrs()
            .rename
            .apply(type_def.name().as_str().into())
    }

    pub fn fmt_file_name_extensionless(&self, type_name: &str) -> String {
        type_name.to_string()
    }

    pub fn fmt_file_name(&self, type_name: &str, file_type: &FileType) -> String {
        match file_type {
            FileType::Module => format!("{}.mjs", self.fmt_file_name_extensionless(type_name)),
            FileType::Typescript => format!("{}.d.ts", self.fmt_file_name_extensionless(type_name)),
        }
    }

    pub fn fmt_docs(&self, docs: &Docs) -> String {
        docs.to_markdown(self.docs_url_gen, diplomat_core::ast::MarkdownStyle::Normal)
            .trim()
            .replace('\n', "\n*")
            .replace(" \n", "\n")
    }

    pub fn fmt_module_statement(
        &self,
        type_name: &str,
        typescript: bool,
        relative_path: String,
    ) -> String {
        let file_name = self.fmt_file_name_extensionless(type_name);
        format!(
            r#"{{ {type_name} }} from "{relative_path}{file_name}{}"#,
            match typescript {
                true => "",
                false => ".mjs",
            }
        )
    }

    pub fn fmt_import_statement(
        &self,
        type_name: &str,
        typescript: bool,
        relative_path: String,
    ) -> String {
        format!(
            r#"import {}{}""#,
            match typescript {
                true => "type ",
                false => "",
            },
            self.fmt_module_statement(type_name, typescript, relative_path)
        )
    }

    pub fn fmt_export_statement(
        &self,
        type_name: &str,
        typescript: bool,
        relative_path: String,
    ) -> String {
        format!(
            r#"export {}""#,
            self.fmt_module_statement(type_name, typescript, relative_path)
        )
    }

    // #region HIR::Type formatting.
    // This is only visible for Typescript definition files, but we use it to check if types are supported.

    /// Generate a primitive type.
    /// `cast: bool` - Basically, do we want to use `number` instead of `u8`?
    pub fn fmt_primitive_as_ffi(&self, primitive: hir::PrimitiveType, cast: bool) -> &'static str {
        if cast {
            match primitive {
                hir::PrimitiveType::Bool => "boolean",
                hir::PrimitiveType::Char => "char",
                hir::PrimitiveType::Int(_)
                | hir::PrimitiveType::IntSize(_)
                | hir::PrimitiveType::Byte
                | hir::PrimitiveType::Float(_) => "number",
                hir::PrimitiveType::Int128(_) => {
                    panic!("Javascript backend does not currently support BigInt.")
                }
            }
        } else {
            match primitive {
                hir::PrimitiveType::Bool => "boolean",
                hir::PrimitiveType::Char => "char",
                hir::PrimitiveType::Int(hir::IntType::I8) => "i8",
                hir::PrimitiveType::Int(hir::IntType::U8) | hir::PrimitiveType::Byte => "u8",
                hir::PrimitiveType::Int(hir::IntType::I16) => "i16",
                hir::PrimitiveType::Int(hir::IntType::U16) => "u16",
                hir::PrimitiveType::Int(hir::IntType::I32) => "i32",
                hir::PrimitiveType::Int(hir::IntType::U32) => "u32",
                hir::PrimitiveType::Int(hir::IntType::I64) => "i64",
                hir::PrimitiveType::Int(hir::IntType::U64) => "u64",
                hir::PrimitiveType::IntSize(hir::IntSizeType::Isize) => "isize",
                hir::PrimitiveType::IntSize(hir::IntSizeType::Usize) => "usize",
                hir::PrimitiveType::Float(hir::FloatType::F32) => "f32",
                hir::PrimitiveType::Float(hir::FloatType::F64) => "f64",
                hir::PrimitiveType::Int128(_) => {
                    panic!("Javascript backend does not currently support BigInt.")
                }
            }
        }
    }

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
                panic!("Int128 is not a supported type for the JS backend.")
            }
        }
    }

    pub fn fmt_primitive_list_type(&self, primitive: hir::PrimitiveType) -> &'static str {
        match primitive {
            hir::PrimitiveType::Bool => "Array<bool>",
            hir::PrimitiveType::Char => "Array<char>",
            hir::PrimitiveType::Byte => "Uint8Array",
            hir::PrimitiveType::Int(_)
            | hir::PrimitiveType::IntSize(_)
            | hir::PrimitiveType::Float(_) => "Array<number>",
            hir::PrimitiveType::Int128(_) => {
                panic!("Javascript backend does not currently support BigInt.")
            }
        }
    }

    pub fn fmt_primitive_list_view(&self, primitive: hir::PrimitiveType) -> &'static str {
        match primitive {
            hir::PrimitiveType::Bool => "bool",
            hir::PrimitiveType::Char => "u16",
            hir::PrimitiveType::Byte => "u8",
            hir::PrimitiveType::Int(hir::IntType::I8) => "i8",
            hir::PrimitiveType::Int(hir::IntType::U8) => "u8",
            hir::PrimitiveType::Int(hir::IntType::I16) => "i16",
            hir::PrimitiveType::Int(hir::IntType::U16) => "u16",
            hir::PrimitiveType::Int(hir::IntType::I32) => "i32",
            hir::PrimitiveType::Int(hir::IntType::U32) => "u32",
            hir::PrimitiveType::Int(hir::IntType::I64) => "i64",
            hir::PrimitiveType::Int(hir::IntType::U64) => "u64",
            hir::PrimitiveType::IntSize(hir::IntSizeType::Isize) => "isize",
            hir::PrimitiveType::IntSize(hir::IntSizeType::Usize) => "usize",
            hir::PrimitiveType::Float(hir::FloatType::F32) => "f32",
            hir::PrimitiveType::Float(hir::FloatType::F64) => "f64",
            hir::PrimitiveType::Int128(_) => {
                panic!("Javascript backend does not currently support BigInt")
            }
        }
    }

    pub fn fmt_void(&self) -> &'static str {
        "void"
    }

    pub fn fmt_nullable(&self, ident: &str) -> String {
        format!("{ident} | undefined")
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
            .apply(method.name.as_str().into())
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
