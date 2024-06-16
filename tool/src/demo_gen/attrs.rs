use diplomat_core::ast::attrs::DemoBackendAttr;


/// Master attribute for handling configuration of the markup that demo-gen produces.
pub(super) struct MarkupOutCFGAttr {

}

impl MarkupOutCFGAttr {
	pub fn from_demo_attr(attr : DemoBackendAttr) -> Self {
		let path = attr.meta.path();
		MarkupOutCFGAttr {}
	}
}