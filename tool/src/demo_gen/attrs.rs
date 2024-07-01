use diplomat_core::ast::attrs::DemoBackendAttr;
use quote::ToTokens;
use syn::{parse_macro_input, Attribute, DeriveInput, Expr, LitStr, MetaList, MetaNameValue};


#[derive(Default, Debug)]
pub(super) struct InputCfg {
	label: String,
}

/// Master attribute for handling configuration of the markup that demo-gen produces.
/// Created from [`diplomat_core::ast::attrs::DemoBackendAttr`]
#[non_exhaustive]
#[derive(Debug)]
pub(super) enum MarkupOutCFGAttr {
	/// #[diplomat::demo(enable)]. If automatic generation is disabled by default (TODO: Right now there is no such option), then the below render terminus will be allowed to generate. 
	Enable,

	/// #[diplomat::demo(disable)]. If automatic generations is enabled by default, the below render terminus will not be allowed to generate.
	Disable,
	
	/// #[diplomat::demo(external)] represents an item that we will not evaluate, and should be passed to the rendering engine to provide.
	External,
	
	///	#[diplomat::demo(default_constructor)]
	/// We search for any methods specially tagged with `Constructor`, but if there's are no default Constructors and there's NamedConstructor that you want to be default instead, use this.
	/// TODO: Should probably ignore other `Constructors` if a default has been set.
	DefaultConstructor,

	/// #[diplomat::demo(input = {...})]
	Input(InputCfg),
}

impl PartialEq for MarkupOutCFGAttr {
	fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (MarkupOutCFGAttr::Enable, MarkupOutCFGAttr::Enable) => true,
            (MarkupOutCFGAttr::Disable, MarkupOutCFGAttr::Disable) => true,
            (MarkupOutCFGAttr::External, MarkupOutCFGAttr::External) => true,
            (MarkupOutCFGAttr::DefaultConstructor, MarkupOutCFGAttr::DefaultConstructor) => true,
            (MarkupOutCFGAttr::Input(..), MarkupOutCFGAttr::Input(..)) => true,
            _ => false,
        }
    }
}

impl MarkupOutCFGAttr {
	/// Generate MarkupOutCFGAttr from DemoBackendAttr.
	/// 
	/// FIXME: The advantage of this file is that it's specific to the diplomat demo backend, so we avoid unnecessary bloat in other places.
	/// The issue now is that you will not see errors with macros until diplomat_tool is run.
	pub fn from_demo_attr(attr : DemoBackendAttr) -> Self {
		let path = attr.meta.path();
		if let Some(path_ident) = path.get_ident() {
			if path_ident == "external" {
				return MarkupOutCFGAttr::External;
			} else if path_ident == "default_constructor" {
				return MarkupOutCFGAttr::DefaultConstructor;
			} else if path_ident == "enable" {
				return MarkupOutCFGAttr::Enable;
			} else if path_ident == "disable" {
				return MarkupOutCFGAttr::Disable;
			} else if path_ident == "input" {
				let mut input_cfg = InputCfg::default();
				
				let meta_list = attr.meta.require_list().expect("Could not get MetaList, expected #[diplomat::demo(input(...))]");

				meta_list.parse_nested_meta(|meta| {
					if meta.path.is_ident("label") {
						let value = meta.value()?;
						let s : LitStr = value.parse()?;
						input_cfg.label = s.value();
						Ok(())
					} else {
						Err(meta.error(format!("Unsupported ident {:?}", meta.path.get_ident())))
					}
				}).expect("Could not read input(...)");
				return MarkupOutCFGAttr::Input(input_cfg);
			} else { 
				panic!("Unknown demo_attr: {path_ident:?}");
			}
		} else {
			panic!("Unknown demo_attr: {path:?}");
		}
	}
}