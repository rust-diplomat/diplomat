//! #[diplomat::attr] and other attributes

use crate::ast;
use crate::ast::attrs::{AttrInheritContext, DiplomatBackendAttrCfg, StandardAttribute};
use crate::hir::{EnumVariant, LoweringError, Method, TypeDef, TypeId};

use syn::Meta;

pub use crate::ast::attrs::RenameAttr;

#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct Attrs {
    /// This attribute is always inherited except to variants
    pub disable: bool,
    /// An optional namespace.
    /// This attribute is inherited to types (and is not allowed elsewhere)
    pub namespace: Option<String>,
    /// This attribute is inherited except through methods and variants (and is not allowed on variants)
    pub rename: RenameAttr,
    /// This attribute is inherited except through variants
    pub abi_rename: RenameAttr,
    // more to be added: namespace, etc
}

/// Attributes that mark methods as "special"
#[non_exhaustive]
#[derive(Clone, Debug)]
pub enum SpecialMethod {
    Constructor,
    NamedConstructor(String),
    Getter(String),
    Setter(String),
    Stringifier,
    Comparison,
}

/// Where the attribute was found. Some attributes are only allowed in some contexts
/// (e.g. namespaces cannot be specified on methods)
#[non_exhaustive] // might add module attrs in the future
#[derive(Copy, Clone, Debug)]
pub enum AttributeContext<'a> {
    Type(TypeDef<'a>),
    EnumVariant(&'a EnumVariant),
    Method(&'a Method, TypeId),
    Module,
}

impl Attrs {
    pub fn from_ast(
        ast: &ast::Attrs,
        validator: &(impl AttributeValidator + ?Sized),
        parent_attrs: &Attrs,
        errors: &mut Vec<LoweringError>,
    ) -> Self {
        let mut this = parent_attrs.clone();
        // Backends must support this since it applies to the macro/C code.
        // No special inheritance, was already appropriately inherited in AST
        this.abi_rename = ast.abi_rename.clone();

        let support = validator.attrs_supported();
        let backend = validator.primary_name();
        for attr in &ast.attrs {
            if validator.satisfies_cfg(&attr.cfg) {
                let path = attr.meta.path();

                if path.is_ident("disable") {
                    if let Meta::Path(_) = attr.meta {
                        if this.disable {
                            errors
                                .push(LoweringError::Other("Duplicate `disable` attribute".into()));
                        } else if !support.disabling {
                            errors.push(LoweringError::Other(format!(
                                "`disable` not supported in backend {backend}"
                            )))
                        } else {
                            this.disable = true;
                        }
                    } else {
                        errors.push(LoweringError::Other(
                            "`disable` must be a simple path".into(),
                        ))
                    }
                } else if path.is_ident("rename") {
                    match RenameAttr::from_meta(&attr.meta) {
                        Ok(rename) => {
                            // We use the override extend mode: a single ast::Attrs
                            // will have had these attributes inherited into the list by appending
                            // to the end; so a later attribute in the list is more pertinent.
                            this.rename.extend(&rename);
                        }
                        Err(e) => errors.push(LoweringError::Other(format!(
                            "`rename` attr failed to parse: {e:?}"
                        ))),
                    }
                } else if path.is_ident("namespace") {
                    if !support.namespacing {
                        errors.push(LoweringError::Other(format!(
                            "`namespace` not supported in backend {backend}"
                        )));
                        continue;
                    }
                    match StandardAttribute::from_meta(&attr.meta) {
                        Ok(StandardAttribute::String(s)) if s.is_empty() => this.namespace = None,
                        Ok(StandardAttribute::String(s)) => this.namespace = Some(s),
                        Ok(_) | Err(_) => {
                            errors.push(LoweringError::Other(
                                "`namespace` must have a single string parameter".to_string(),
                            ));
                            continue;
                        }
                    }
                } else {
                    errors.push(LoweringError::Other(format!(
                        "Unknown diplomat attribute {path:?}: expected one of: `disable, rename`"
                    )));
                }
            }
        }

        this
    }

    pub(crate) fn validate(
        &self,
        validator: &(impl AttributeValidator + ?Sized),
        context: AttributeContext,
        errors: &mut Vec<LoweringError>,
    ) {
        // use an exhaustive destructure so new attributes are handled
        let Attrs {
            disable,
            namespace,
            rename: _,
            abi_rename: _,
        } = &self;

        if *disable && matches!(context, AttributeContext::EnumVariant(..)) {
            errors.push(LoweringError::Other(
                "`disable` cannot be used on enum variants".into(),
            ))
        }

        if namespace.is_some()
            && matches!(
                context,
                AttributeContext::Method(..) | AttributeContext::EnumVariant(..)
            )
        {
            errors.push(LoweringError::Other(
                "`namespace` can only be used on types".to_string(),
            ));
        }
    }

    pub(crate) fn for_inheritance(&self, context: AttrInheritContext) -> Attrs {
        let rename = self.rename.attrs_for_inheritance(context, false);

        // Disabling shouldn't inherit to variants
        let disable = if context == AttrInheritContext::Variant {
            false
        } else {
            self.disable
        };
        let namespace = if matches!(
            context,
            AttrInheritContext::Module | AttrInheritContext::Type
        ) {
            self.namespace.clone()
        } else {
            None
        };

        Attrs {
            disable,
            rename,
            namespace,
            // Was already inherited on the AST side
            abi_rename: Default::default(),
        }
    }
}

#[non_exhaustive]
#[derive(Copy, Clone, Debug, Default)]
pub struct BackendAttrSupport {
    pub disabling: bool,
    pub renaming: bool,
    pub namespacing: bool,
    pub constructors: bool,
    pub named_constructors: bool,
    pub accessors: bool,
    pub stringifiers: bool,
    pub comparison_overload: bool,
    // more to be added: namespace, etc
}

/// Defined by backends when validating attributes
pub trait AttributeValidator {
    /// The primary name of the backend, for use in diagnostics
    fn primary_name(&self) -> &str;
    /// Does this backend satisfy `cfg(backend_name)`?
    /// (Backends are allowed to satisfy multiple backend names, useful when there
    /// are multiple backends for a language)
    fn is_backend(&self, backend_name: &str) -> bool;
    /// does this backend satisfy cfg(name = value)?
    fn is_name_value(&self, name: &str, value: &str) -> bool;
    /// What backedn attrs does this support?
    fn attrs_supported(&self) -> BackendAttrSupport;

    /// Provided, checks if type satisfies a `DiplomatBackendAttrCfg`
    fn satisfies_cfg(&self, cfg: &DiplomatBackendAttrCfg) -> bool {
        match *cfg {
            DiplomatBackendAttrCfg::Not(ref c) => !self.satisfies_cfg(c),
            DiplomatBackendAttrCfg::Any(ref cs) => cs.iter().any(|c| self.satisfies_cfg(c)),
            DiplomatBackendAttrCfg::All(ref cs) => cs.iter().all(|c| self.satisfies_cfg(c)),
            DiplomatBackendAttrCfg::Star => true,
            DiplomatBackendAttrCfg::BackendName(ref n) => self.is_backend(n),
            DiplomatBackendAttrCfg::NameValue(ref n, ref v) => self.is_name_value(n, v),
        }
    }

    // Provided, constructs an attribute
    fn attr_from_ast(
        &self,
        ast: &ast::Attrs,
        parent_attrs: &Attrs,
        errors: &mut Vec<LoweringError>,
    ) -> Attrs {
        Attrs::from_ast(ast, self, parent_attrs, errors)
    }

    // Provided: validates an attribute in the context in which it was constructed
    fn validate(&self, attrs: &Attrs, context: AttributeContext, errors: &mut Vec<LoweringError>) {
        attrs.validate(self, context, errors)
    }
}

/// A basic attribute validator
#[non_exhaustive]
#[derive(Default)]
pub struct BasicAttributeValidator {
    /// The primary name of this backend (should be unique, ideally)
    pub backend_name: String,
    /// The attributes supported
    pub support: BackendAttrSupport,
    /// Additional names for this backend
    pub other_backend_names: Vec<String>,
    /// override is_name_value()
    #[allow(clippy::type_complexity)] // dyn fn is not that complex
    pub is_name_value: Option<Box<dyn Fn(&str, &str) -> bool>>,
}

impl BasicAttributeValidator {
    pub fn new(backend_name: &str) -> Self {
        BasicAttributeValidator {
            backend_name: backend_name.into(),
            ..Self::default()
        }
    }
}

impl AttributeValidator for BasicAttributeValidator {
    fn primary_name(&self) -> &str {
        &self.backend_name
    }
    fn is_backend(&self, backend_name: &str) -> bool {
        self.backend_name == backend_name
            || self.other_backend_names.iter().any(|n| n == backend_name)
    }
    fn is_name_value(&self, name: &str, value: &str) -> bool {
        // TODO: is_name_value should automatically proxy checks for `supports = constructors` etc from
        // BackendAttrSupport
        if let Some(ref nv) = self.is_name_value {
            nv(name, value)
        } else {
            false
        }
    }
    fn attrs_supported(&self) -> BackendAttrSupport {
        self.support
    }
}
