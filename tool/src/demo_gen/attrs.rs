use diplomat_core::ast::attrs::DemoBackendAttr;


/// Master attribute for handling configuration of the markup that demo-gen produces.
/// Created from [`diplomat_core::ast::attrs::DemoBackendAttr`]
#[non_exhaustive]
#[derive(Debug, PartialEq)]
pub(super) enum MarkupOutCFGAttr {
	/// #[diplomat::demo(external)] represents an item that we will not evaluate, and should be passed to the rendering engine to provide.
	External,
	/// We search for any methods specially tagged with `Constructor`, but if there's are no default Constructors and there's NamedConstructor that you want to be default instead, use this.
	/// TODO: Should probably ignore other `Constructors` if a default has been set.
	DefaultConstructor,
}

impl MarkupOutCFGAttr {
	pub fn from_demo_attr(attr : DemoBackendAttr) -> Self {
		let path = attr.meta.path();
		if let Some(path_ident) = path.get_ident() {
			if path_ident == "external" {
				return MarkupOutCFGAttr::External;
			} else if path_ident == "default_constructor" {
				return MarkupOutCFGAttr::DefaultConstructor;
			} else { 
				panic!("Unknown demo_attr: {path_ident:?}");
			}
		} else {
			panic!("Unknown demo_attr: {path:?}");
		}
	}
}