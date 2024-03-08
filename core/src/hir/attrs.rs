//! #[diplomat::attr] and other attributes

use crate::ast;
use crate::ast::attrs::{AttrInheritContext, DiplomatBackendAttrCfg, StandardAttribute};
use crate::hir::{EnumVariant, LoweringError, Method, ReturnType, SuccessType, TypeDef, TypeId};

use syn::Meta;

pub use crate::ast::attrs::RenameAttr;

/// Diplomat attribute that can be specified on items, methods, and enum variants. These
/// can be used to control the codegen in a particular backend.
///
/// Most of these are specified via `#[diplomat::attr(some cfg here, attrname)]`, where `some cfg here`
/// can be used to pick which backends something applies to.
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct Attrs {
    /// "disable" this item: do not generate code for it in the backend
    ///
    /// This attribute is always inherited except to variants
    pub disable: bool,
    /// An optional namespace. None is equivalent to the root namespace.
    ///
    /// This attribute is inherited to types (and is not allowed elsewhere)
    pub namespace: Option<String>,
    /// Rename this item/method/variant
    ///
    /// This attribute is inherited except through methods and variants (and is not allowed on variants)
    pub rename: RenameAttr,
    /// Rename this item in the C ABI. This *must* be respected by backends.
    ///
    /// This attribute is inherited except through variants
    pub abi_rename: RenameAttr,
    /// This method is "special": it should generate something other than a regular method on the other side.
    /// This can be something like a constructor, an accessor, a stringifier etc.
    ///
    /// This attribute does not participate in inheritance and must always
    /// be specified on individual methods
    pub special_method: Option<SpecialMethod>,
}

/// Attributes that mark methods as "special"
#[non_exhaustive]
#[derive(Clone, Debug)]
pub enum SpecialMethod {
    /// A constructor.
    ///
    /// Must return Self (or Result<Self> for backends with `fallible_constructors` enabled )
    Constructor,
    /// A named constructor, with optional name. If the name isn't specified, it will be derived
    /// from the method name
    ///
    /// Must return Self (or Result<Self> for backends with `fallible_constructors` enabled )
    NamedConstructor(Option<String>),

    /// A getter, with optional name. If the name isn't specified, it will be derived
    /// from the method name
    ///
    /// Must have no parameters and must return something.
    Getter(Option<String>),
    /// A setter, with optional name. If the name isn't specified, it will be derived
    /// from the method name
    ///
    /// Must have no return type (aside from potentially a `Result<(), _>`) and must have one parameter
    Setter(Option<String>),
    /// A stringifier. Must have no parameters and return a string (writeable)
    Stringifier,
    /// A comparison operator. Currently unsupported
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
            let satisfies = match validator.satisfies_cfg(&attr.cfg) {
                Ok(satisfies) => satisfies,
                Err(e) => {
                    errors.push(e);
                    continue;
                }
            };
            if satisfies {
                let path = attr.meta.path();
                if let Some(path) = path.get_ident() {
                    if path == "disable" {
                        if let Meta::Path(_) = attr.meta {
                            if this.disable {
                                errors.push(LoweringError::Other(
                                    "Duplicate `disable` attribute".into(),
                                ));
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
                    } else if path == "rename" {
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
                    } else if path == "namespace" {
                        if !support.namespacing {
                            errors.push(LoweringError::Other(format!(
                                "`namespace` not supported in backend {backend}"
                            )));
                            continue;
                        }
                        match StandardAttribute::from_meta(&attr.meta) {
                            Ok(StandardAttribute::String(s)) if s.is_empty() => {
                                this.namespace = None
                            }
                            Ok(StandardAttribute::String(s)) => this.namespace = Some(s),
                            Ok(_) | Err(_) => {
                                errors.push(LoweringError::Other(
                                    "`namespace` must have a single string parameter".to_string(),
                                ));
                                continue;
                            }
                        }
                    } else if path == "constructor" || path == "stringifier" || path == "comparison"
                    {
                        if let Some(ref existing) = this.special_method {
                            errors.push(LoweringError::Other(format!(
                            "Multiple special method markers found on the same method, found {path} and {existing:?}"
                        )));
                            continue;
                        }
                        let kind = if path == "constructor" {
                            SpecialMethod::Constructor
                        } else if path == "stringifier" {
                            SpecialMethod::Stringifier
                        } else {
                            SpecialMethod::Comparison
                        };

                        this.special_method = Some(kind);
                    } else if path == "named_constructor" || path == "getter" || path == "setter" {
                        if let Some(ref existing) = this.special_method {
                            errors.push(LoweringError::Other(format!(
                            "Multiple special method markers found on the same method, found {path} and {existing:?}"
                        )));
                            continue;
                        }
                        let kind = if path == "named_constructor" {
                            SpecialMethod::NamedConstructor
                        } else if path == "getter" {
                            SpecialMethod::Getter
                        } else {
                            SpecialMethod::Setter
                        };
                        match StandardAttribute::from_meta(&attr.meta) {
                            Ok(StandardAttribute::String(s)) => {
                                this.special_method = Some(kind(Some(s)))
                            }
                            Ok(StandardAttribute::Empty) => this.special_method = Some(kind(None)),
                            Ok(_) | Err(_) => {
                                errors.push(LoweringError::Other(format!(
                                    "`{path}` must have a single string parameter or no parameter",
                                )));
                                continue;
                            }
                        }
                    } else {
                        errors.push(LoweringError::Other(format!(
                        "Unknown diplomat attribute {path}: expected one of: `disable, rename, namespace, constructor, stringifier, comparison, named_constructor, getter, setter`"
                    )));
                    }
                } else {
                    errors.push(LoweringError::Other(format!(
                        "Unknown diplomat attribute {path:?}: expected one of: `disable, rename, namespace, constructor, stringifier, comparison, named_constructor, getter, setter`"
                    )));
                }
            }
        }

        this
    }

    /// Validate that this attribute is allowed in this context
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
            special_method,
        } = &self;

        if *disable && matches!(context, AttributeContext::EnumVariant(..)) {
            errors.push(LoweringError::Other(
                "`disable` cannot be used on enum variants".into(),
            ))
        }

        if let Some(ref special) = special_method {
            if let AttributeContext::Method(method, self_id) = context {
                match special {
                    SpecialMethod::Constructor | SpecialMethod::NamedConstructor(..) => {
                        if method.param_self.is_some() {
                            errors.push(LoweringError::Other(
                                "Constructors must not accept a self parameter".to_string(),
                            ))
                        }
                        let output = method.output.success_type();
                        match method.output {
                            ReturnType::Infallible(_) => (),
                            ReturnType::Fallible(..) => {
                                if !validator.attrs_supported().fallible_constructors {
                                    errors.push(LoweringError::Other(
                                        "This backend doesn't support fallible constructors"
                                            .to_string(),
                                    ))
                                }
                            }
                            ReturnType::Nullable(..) => {
                                errors.push(LoweringError::Other("Diplomat doesn't support turning nullable methods into constructors".to_string()));
                            }
                        }

                        if let SuccessType::OutType(t) = &output {
                            if t.id() != Some(self_id) {
                                errors.push(LoweringError::Other(
                                    "Constructors must return Self!".to_string(),
                                ));
                            }
                        } else {
                            errors.push(LoweringError::Other(
                                "Constructors must return Self!".to_string(),
                            ));
                        }
                    }
                    SpecialMethod::Getter(_) => {
                        if !method.params.is_empty() {
                            errors
                                .push(LoweringError::Other("Getter cannot have parameters".into()));
                        }

                        // Currently does not forbid nullable getters, could if desired
                    }

                    SpecialMethod::Setter(_) => {
                        if !matches!(method.output.success_type(), SuccessType::Unit) {
                            errors.push(LoweringError::Other("Setters must return unit".into()));
                        }
                        if method.params.len() != 1 {
                            errors.push(LoweringError::Other(
                                "Setter must have exactly one parameter".into(),
                            ))
                        }

                        // Currently does not forbid fallible setters, could if desired
                    }
                    SpecialMethod::Stringifier => {
                        if !method.params.is_empty() {
                            errors
                                .push(LoweringError::Other("Getter cannot have parameters".into()));
                        }
                        if !matches!(method.output.success_type(), SuccessType::Writeable) {
                            errors.push(LoweringError::Other(
                                "Stringifier must return Writeable".into(),
                            ));
                        }
                    }
                    _ => todo!("Diplomat doesn't yet support {special:?}"),
                }
            } else {
                errors.push(LoweringError::Other(format!("Special method (type {special:?}) not allowed on non-method context {context:?}")))
            }
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
            // Never inherited
            special_method: None,
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
    pub fallible_constructors: bool,
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
    fn is_name_value(&self, name: &str, value: &str) -> Result<bool, LoweringError>;
    /// What backedn attrs does this support?
    fn attrs_supported(&self) -> BackendAttrSupport;

    /// Provided, checks if type satisfies a `DiplomatBackendAttrCfg`
    fn satisfies_cfg(&self, cfg: &DiplomatBackendAttrCfg) -> Result<bool, LoweringError> {
        Ok(match *cfg {
            DiplomatBackendAttrCfg::Not(ref c) => !self.satisfies_cfg(c)?,
            DiplomatBackendAttrCfg::Any(ref cs) => {
                for c in cs {
                    if self.satisfies_cfg(c)? {
                        return Ok(true);
                    }
                }
                false
            }
            DiplomatBackendAttrCfg::All(ref cs) => {
                for c in cs {
                    if !self.satisfies_cfg(c)? {
                        return Ok(false);
                    }
                }
                true
            }
            DiplomatBackendAttrCfg::Star => true,
            DiplomatBackendAttrCfg::BackendName(ref n) => self.is_backend(n),
            DiplomatBackendAttrCfg::NameValue(ref n, ref v) => self.is_name_value(n, v)?,
        })
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
    fn is_name_value(&self, name: &str, value: &str) -> Result<bool, LoweringError> {
        Ok(if name == "supports" {
            // destructure so new fields are forced to be added
            let BackendAttrSupport {
                disabling,
                renaming,
                namespacing,
                constructors,
                named_constructors,
                fallible_constructors,
                accessors,
                stringifiers,
                comparison_overload,
            } = self.support;
            match value {
                "disabling" => disabling,
                "renaming" => renaming,
                "namespacing" => namespacing,
                "constructors" => constructors,
                "named_constructors" => named_constructors,
                "fallible_constructors" => fallible_constructors,
                "accessors" => accessors,
                "stringifiers" => stringifiers,
                "comparison_overload" => comparison_overload,
                _ => {
                    return Err(LoweringError::Other(format!(
                        "Unknown supports = value found: {value}"
                    )))
                }
            }
        } else if let Some(ref nv) = self.is_name_value {
            nv(name, value)
        } else {
            false
        })
    }
    fn attrs_supported(&self) -> BackendAttrSupport {
        self.support
    }
}
