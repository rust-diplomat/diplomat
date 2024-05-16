use std::borrow::Cow;

use diplomat_core::{ast::DocsUrlGenerator, hir::{Docs, EnumVariant, TypeContext, TypeId}};
use heck::ToUpperCamelCase;

use crate::c2::CFormatter;

use super::FileType;

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

	pub fn fmt_file_name(&self, type_name : &str, file_type : FileType) -> String {
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

	pub fn fmt_enum_variant(&self, variant : &'tcx EnumVariant) -> Cow<'tcx, str> {
		let name = variant.name.as_str().to_upper_camel_case().into();
		variant.attrs.rename.apply(name)
	}
}