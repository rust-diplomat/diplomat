//! #[diplomat::attr] and other attributes

use crate::ast;
use crate::ast::attrs::DiplomatAttrCfg;
use crate::hir::LoweringError;

use syn::Meta;

#[non_exhaustive]
#[derive(Clone, Default)]
pub struct Attrs {
    pub disable: bool,
    // more to be added: rename, namespace, etc
}

impl Attrs {
    pub fn from_ast(
        ast: &ast::Attrs,
        validator: &impl AttributeValidator,
        errors: &mut Vec<LoweringError>,
    ) -> Self {
        let mut this = Attrs::default();
        let support = validator.attrs_supported();
        for attr in &ast.attrs {
            if validator.satisfies_cfg(&attr.cfg) {
                match &attr.attr {
                    Meta::Path(p) => {
                        if p.is_ident("disable") {
                            if this.disable {
                                errors.push(LoweringError::Other(
                                    "Duplicate `disable` attribute".into(),
                                ));
                            } else if !support.disabling {
                                errors.push(LoweringError::Other(format!(
                                    "`disable` not supported in backend {}",
                                    validator.primary_name()
                                )))
                            } else {
                                this.disable = true;
                            }
                        } else {
                            errors.push(LoweringError::Other(format!(
                                "Unknown diplomat attribute {p:?}: expected one of: `disable`"
                            )));
                        }
                    }
                    other => {
                        errors.push(LoweringError::Other(format!(
                            "Unknown diplomat attribute {other:?}: expected one of: `disable`"
                        )));
                    }
                }
            }
        }

        this
    }
}

#[non_exhaustive]
#[derive(Copy, Clone, Debug)]
pub struct BackendAttrSupport {
    disabling: bool,
    // more to be added: rename, namespace, etc
}

/// Defined by backends when validating attributes
pub trait AttributeValidator {
    /// The primary name of the backend, for use in diagnostics
    fn primary_name(&self) -> &'static str;
    /// Does this backend satisfy `cfg(backend_name)`?
    /// (Backends are allowed to satisfy multiple backend names, useful when there
    /// are multiple backends for a language)
    fn is_backend(&self, backend_name: &str) -> bool;
    /// does this backend satisfy cfg(name = value)?
    fn is_name_value(&self, name: &str, value: &str) -> bool;
    /// What backedn attrs does this support?
    fn attrs_supported(&self) -> BackendAttrSupport;

    /// Provided, checks if type satisfies a `DiplomatAttrCfg`
    fn satisfies_cfg(&self, cfg: &DiplomatAttrCfg) -> bool {
        match *cfg {
            DiplomatAttrCfg::Not(ref c) => !self.satisfies_cfg(c),
            DiplomatAttrCfg::Any(ref cs) => cs.iter().any(|c| self.satisfies_cfg(c)),
            DiplomatAttrCfg::All(ref cs) => cs.iter().all(|c| self.satisfies_cfg(c)),
            DiplomatAttrCfg::Star => true,
            DiplomatAttrCfg::BackendName(ref n) => self.is_backend(n),
            DiplomatAttrCfg::NameValue(ref n, ref v) => self.is_name_value(n, v),
        }
    }
}
