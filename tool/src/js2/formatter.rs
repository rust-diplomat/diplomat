use std::borrow::Cow;

use diplomat_core::{ast::DocsUrlGenerator, hir::{self, Docs, EnumVariant, TypeContext, TypeId}};
use heck::{ToLowerCamelCase, ToUpperCamelCase};

use crate::{c2::CFormatter, common::ErrorContextGuard};

use super::FileType;

const RESERVED : &[&str] = &[
	"break",
	"case", "catch", "class", "const", "continue",
	"debugger", "default", "delete", "do",
	"else", "export", "extends",
	"false", "finally", "for", "function",
	"if", "import", "in", "instanceof",
	"new", "null",
	"return",
	"super", "switch",
	"this", "throw", "true", "try", "typeof",
	"var", "void",
	"while", "with"
];

/// Helper class for us to format JS identifiers from the HIR.
pub(super) struct JSFormatter<'tcx> {
	/// Per [`CFormatter`]'s documentation we use it for support.
	c_formatter : CFormatter<'tcx>,
	
	/// For generating doc.rs links
	docs_url_gen : &'tcx DocsUrlGenerator,
	/// If there's something we need to remove during formatting. Set by user.
	strip_prefix : Option<String>,
}

impl<'tcx> JSFormatter<'tcx> {
	pub fn new(tcx : &'tcx TypeContext, docs_url_gen : &'tcx DocsUrlGenerator, strip_prefix : Option<String>) -> Self {
		Self {
			c_formatter: CFormatter::new(tcx),
			docs_url_gen,
			strip_prefix: strip_prefix
		}
	}

	pub fn fmt_type_name(&self, id : TypeId) -> Cow<'tcx, str> {
		let type_def = self.c_formatter.tcx().resolve_type(id);
		
		let candidate : Cow<str>;

		if let Some(strip_prefix) = self.strip_prefix.as_ref() {
			candidate = type_def.name().as_str().strip_prefix(strip_prefix)
			.unwrap_or(type_def.name().as_str()).into();
		} else {
			candidate = type_def.name().as_str().into();
		}
		
		type_def.attrs().rename.apply(candidate)
	}

	pub fn fmt_type_name_diagnostics(&self, type_id : TypeId) -> Cow<'tcx, str> {
		self.c_formatter.fmt_type_name_diagnostics(type_id)
	}

	pub fn fmt_file_name(&self, type_name : &str, file_type : &FileType) -> String {
		match file_type {
			FileType::Module => format!("{}.mjs", type_name),
			FileType::Typescript => format!("{}.d.ts", type_name)
		}
	}

	pub fn fmt_docs(&self, docs : &Docs) -> String {
		docs.to_markdown(self.docs_url_gen, diplomat_core::ast::MarkdownStyle::Normal)
		.trim()
		.replace('\n', "\n*")
		.replace(" \n", "\n")

	}

	// #region HIR::Type formatting.
	// This is only visible for Typescript definition files, but we use it to check if types are supported.

	/// Generate a primitive type.
	/// `cast: bool` - Basically, do we want to use `number` instead of `u8`? 
	pub fn fmt_primitive_as_ffi(&self, primitive : hir::PrimitiveType, cast : bool) -> &'static str {
		if cast {
			return match primitive {
				hir::PrimitiveType::Bool => "boolean",
				hir::PrimitiveType::Char => "char",
				hir::PrimitiveType::Int(_)| hir::PrimitiveType::IntSize(_) | hir::PrimitiveType::Byte | hir::PrimitiveType::Float(_) => "number",
				hir::PrimitiveType::Int128(_) => panic!("Javascript backend does not currently support BigInt."),
			};
		} else {
			return match primitive {
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
				hir::PrimitiveType::Int128(_) => panic!("Javascript backend does not currently support BigInt.")
			};
		}
	}

	pub fn fmt_enum_as_ffi(&self, cast : bool) -> &'static str {
		self.fmt_primitive_as_ffi(hir::PrimitiveType::Int(hir::IntType::I32), cast)
	}
	

	pub fn fmt_primitive_list_type(&self, primitive : hir::PrimitiveType) -> &'static str {
		match primitive {
			hir::PrimitiveType::Bool => "Array<bool>",
			hir::PrimitiveType::Char => "Array<char>",
			hir::PrimitiveType::Byte => "Uint8Array",
			hir::PrimitiveType::Int(_) | hir::PrimitiveType::IntSize(_) | hir::PrimitiveType::Float(_) => "Array<number>",
			hir::PrimitiveType::Int128(_) => panic!("Javascript backend does not currently support BigInt."),
		}
	}

	pub fn fmt_primitive_list_view(&self, primitive : hir::PrimitiveType) -> &'static str {
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
			hir::PrimitiveType::Int128(_) => panic!("Javascript backend does not currently support BigInt")
		}
	}

	pub fn fmt_void(&self) -> &'static str {
		"void".into()
	}

	pub fn fmt_nullable(&self, ident : &str) -> String {
		format!("{ident} | undefined")
	}
	
	pub fn fmt_string(&self) -> &'static str {
		"String"
	}
	// #endregion

	// #region Template specific formatting
	pub fn fmt_method_name(&self, method : &hir::Method) -> String {
		let name : String = method.attrs.rename
		.apply(method.name.as_str().into()).to_lower_camel_case();
		if RESERVED.contains(&&*name) {
			format!("{name}_")
		} else {
			name
		}
	}

	/// For formatting a JS method that has an associated name with it. Like a named constructor or getter/setter.
	pub fn fmt_method_field_name(&self, name: &Option<String>, method: &hir::Method) -> String {
		let name: String = method.attrs.rename
		.apply(
			name.as_deref().unwrap_or(method.name.as_str()).into()
		)
		.to_lower_camel_case();

		if RESERVED.contains(&&*name) {
			format!("{name}_")
		} else {
			name
		}
	}

	pub fn fmt_c_method_name<'a>(&self, type_id: TypeId, method: &'a hir::Method) -> Cow<'a, str> {
		self.c_formatter.fmt_method_name(type_id, method).into()
	}

	pub fn fmt_param_name<'a>(&self, param_name: &'a str) -> Cow<'a, str> {
		param_name.to_lower_camel_case().into()
	}

	pub fn fmt_lifetime_edge_array(&self, lifetime : hir::Lifetime, lifetime_env : &hir::LifetimeEnv) -> Cow<'static, str> {
		format!("{}Edges", lifetime_env.fmt_lifetime(lifetime)).into()
	}

	pub fn fmt_enum_variant(&self, variant : &'tcx EnumVariant) -> Cow<'tcx, str> {
		let name = variant.name.as_str().to_upper_camel_case().into();
		variant.attrs.rename.apply(name)
	}
	
	pub fn fmt_destructor_name(&self, type_id : TypeId) -> String {
		self.c_formatter.fmt_dtor_name(type_id)
	}
	// #endregion
}