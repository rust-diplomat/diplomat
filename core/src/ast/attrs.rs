//! This module contains utilities for dealing with Rust attributes

use serde::ser::{SerializeStruct, Serializer};
use serde::Serialize;
use std::borrow::Cow;
use std::convert::Infallible;
use std::str::FromStr;
use syn::parse::{Error as ParseError, Parse, ParseStream};
use syn::{Attribute, Expr, Ident, Lit, LitStr, Meta, Token};

/// The list of attributes on a type
#[derive(Clone, PartialEq, Eq, Hash, Debug, Default)]
#[non_exhaustive]
pub struct Attrs {
    /// The regular #[cfg()] attributes. Inherited, though the inheritance onto methods is the
    /// only relevant one here.
    pub cfg: Vec<Attribute>,
    /// Inherited, but only during lowering. See hir::Attrs for details on which attributes are inherited
    pub attrs: Vec<DiplomatBackendAttr>,
    /// AST backends only. For using features that may panic AST backends, like returning references.
    ///
    /// This isn't a regular attribute since AST backends do not handle regular attributes. Do not use
    /// in HIR backends,
    ///
    /// Not inherited
    pub skip_if_unsupported: bool,

    /// Renames to apply to the underlying C symbol. Can be found on methods, impls, and bridge modules, and is inherited.
    ///
    /// Affects method names when inherited onto methods.
    /// Affects destructor names when inherited onto types.
    ///
    /// Inherited.
    pub abi_rename: RenameAttr,
}

impl Attrs {
    fn add_attr(&mut self, attr: Attr) {
        match attr {
            Attr::Cfg(attr) => self.cfg.push(attr),
            Attr::DiplomatBackend(attr) => self.attrs.push(attr),
            Attr::SkipIfUnsupported => self.skip_if_unsupported = true,
            Attr::CRename(rename) => self.abi_rename.extend(&rename),
        }
    }

    /// Get a copy of these attributes for use in inheritance, masking out things
    /// that should not be inherited
    pub(crate) fn attrs_for_inheritance(&self, context: AttrInheritContext) -> Self {
        // These attributes are inherited during lowering (since that's when they're parsed)
        //
        // Except for impls: lowering has no concept of impls so these get inherited early. This
        // is fine since impls have no inherent behavior and all attributes on impls are necessarily
        // only there for inheritance
        let attrs = if context == AttrInheritContext::MethodFromImpl {
            self.attrs.clone()
        } else {
            Vec::new()
        };

        let abi_rename = self.abi_rename.attrs_for_inheritance(context, true);
        Self {
            cfg: self.cfg.clone(),

            attrs,
            // HIR only, for methods only. not inherited
            skip_if_unsupported: false,
            abi_rename,
        }
    }

    pub(crate) fn add_attrs(&mut self, attrs: &[Attribute]) {
        for attr in syn_attr_to_ast_attr(attrs) {
            self.add_attr(attr)
        }
    }
    pub(crate) fn from_attrs(attrs: &[Attribute]) -> Self {
        let mut this = Self::default();
        this.add_attrs(attrs);
        this
    }
}

impl From<&[Attribute]> for Attrs {
    fn from(other: &[Attribute]) -> Self {
        Self::from_attrs(other)
    }
}

enum Attr {
    Cfg(Attribute),
    DiplomatBackend(DiplomatBackendAttr),
    SkipIfUnsupported,
    CRename(RenameAttr),
    // More goes here
}

fn syn_attr_to_ast_attr(attrs: &[Attribute]) -> impl Iterator<Item = Attr> + '_ {
    let cfg_path: syn::Path = syn::parse_str("cfg").unwrap();
    let dattr_path: syn::Path = syn::parse_str("diplomat::attr").unwrap();
    let crename_attr: syn::Path = syn::parse_str("diplomat::abi_rename").unwrap();
    let skipast: syn::Path = syn::parse_str("diplomat::skip_if_unsupported").unwrap();
    attrs.iter().filter_map(move |a| {
        if a.path() == &cfg_path {
            Some(Attr::Cfg(a.clone()))
        } else if a.path() == &dattr_path {
            Some(Attr::DiplomatBackend(
                a.parse_args()
                    .expect("Failed to parse malformed diplomat::attr"),
            ))
        } else if a.path() == &crename_attr {
            Some(Attr::CRename(RenameAttr::from_meta(&a.meta).unwrap()))
        } else if a.path() == &skipast {
            Some(Attr::SkipIfUnsupported)
        } else {
            None
        }
    })
}

impl Serialize for Attrs {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // 1 is the number of fields in the struct.
        let mut state = serializer.serialize_struct("Attrs", 1)?;
        if !self.cfg.is_empty() {
            let cfg: Vec<_> = self
                .cfg
                .iter()
                .map(|a| quote::quote!(#a).to_string())
                .collect();
            state.serialize_field("cfg", &cfg)?;
        }
        if !self.attrs.is_empty() {
            state.serialize_field("attrs", &self.attrs)?;
        }
        if self.skip_if_unsupported {
            state.serialize_field("skip_if_unsupported", &self.skip_if_unsupported)?;
        }
        if !self.abi_rename.is_empty() {
            state.serialize_field("abi_rename", &self.abi_rename)?;
        }
        state.end()
    }
}

/// A `#[diplomat::attr(...)]` attribute
///
/// Its contents must start with single element that is a CFG-expression
/// (so it may contain `foo = bar`, `foo = "bar"`, `ident`, `*` atoms,
/// and `all()`, `not()`, and `any()` combiners), and then be followed by one
/// or more backend-specific attributes, which can be any valid meta-item
#[derive(Clone, PartialEq, Eq, Hash, Debug, Serialize)]
#[non_exhaustive]
pub struct DiplomatBackendAttr {
    pub cfg: DiplomatBackendAttrCfg,
    #[serde(serialize_with = "serialize_meta")]
    pub meta: Meta,
}

fn serialize_meta<S>(m: &Meta, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    quote::quote!(#m).to_string().serialize(s)
}

#[derive(Clone, PartialEq, Eq, Hash, Debug, Serialize)]
#[non_exhaustive]
pub enum DiplomatBackendAttrCfg {
    Not(Box<DiplomatBackendAttrCfg>),
    Any(Vec<DiplomatBackendAttrCfg>),
    All(Vec<DiplomatBackendAttrCfg>),
    Star,
    BackendName(String),
    NameValue(String, String),
}

impl Parse for DiplomatBackendAttrCfg {
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(Ident) {
            let name: Ident = input.parse()?;
            if name == "not" {
                let content;
                let _paren = syn::parenthesized!(content in input);
                Ok(DiplomatBackendAttrCfg::Not(Box::new(content.parse()?)))
            } else if name == "any" || name == "all" {
                let content;
                let _paren = syn::parenthesized!(content in input);
                let list = content.parse_terminated(Self::parse, Token![,])?;
                let vec = list.into_iter().collect();
                if name == "any" {
                    Ok(DiplomatBackendAttrCfg::Any(vec))
                } else {
                    Ok(DiplomatBackendAttrCfg::All(vec))
                }
            } else if input.peek(Token![=]) {
                let _t: Token![=] = input.parse()?;
                if input.peek(Ident) {
                    let value: Ident = input.parse()?;
                    Ok(DiplomatBackendAttrCfg::NameValue(
                        name.to_string(),
                        value.to_string(),
                    ))
                } else {
                    let value: LitStr = input.parse()?;
                    Ok(DiplomatBackendAttrCfg::NameValue(
                        name.to_string(),
                        value.value(),
                    ))
                }
            } else {
                Ok(DiplomatBackendAttrCfg::BackendName(name.to_string()))
            }
        } else if lookahead.peek(Token![*]) {
            let _t: Token![*] = input.parse()?;
            Ok(DiplomatBackendAttrCfg::Star)
        } else {
            Err(ParseError::new(
                input.span(),
                "CFG portion of #[diplomat::attr] fails to parse",
            ))
        }
    }
}

/// Meant to be used with Attribute::parse_args()
impl Parse for DiplomatBackendAttr {
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        let cfg = input.parse()?;
        let _comma: Token![,] = input.parse()?;
        let meta = input.parse()?;
        Ok(Self { cfg, meta })
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub(crate) enum AttrInheritContext {
    Variant,
    Type,
    /// When a method or an impl is inheriting from a module
    MethodOrImplFromModule,
    /// When a method is inheriting from an impl
    ///
    /// This distinction is made because HIR attributes are pre-inherited from the impl to the
    /// method, so the boundary of "method inheriting from module" is different
    MethodFromImpl,
    // Currently there's no way to feed an attribute to a Module, but such inheritance will
    // likely apply during lowering for config defaults.
    #[allow(unused)]
    Module,
}

/// A pattern for use in rename attributes, like `#[diplomat::abi_rename]`
///
/// This can be parsed from a string, typically something like `icu4x_{0}`.
/// It can have up to one {0} for replacement.
///
/// In the future this may support transformations like to_camel_case, etc,
/// probably specified as a list like `#[diplomat::abi_rename("foo{0}", to_camel_case)]`
#[derive(Default, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize)]
pub struct RenameAttr {
    pattern: Option<RenamePattern>,
}

impl RenameAttr {
    /// Apply all renames to a given string
    pub fn apply<'a>(&'a self, name: Cow<'a, str>) -> Cow<'a, str> {
        if let Some(ref pattern) = self.pattern {
            let replacement = &pattern.replacement;
            if let Some(index) = pattern.insertion_index {
                format!("{}{name}{}", &replacement[..index], &replacement[index..]).into()
            } else {
                replacement.into()
            }
        } else {
            name
        }
    }

    /// Whether this rename is empty and will perform no changes
    fn is_empty(&self) -> bool {
        self.pattern.is_none()
    }

    pub(crate) fn extend(&mut self, other: &Self) {
        if other.pattern.is_some() {
            self.pattern = other.pattern.clone()
        }

        // In the future if we support things like to_lower_case they may inherit separately
        // from patterns.
    }

    /// Get a copy of these attributes for use in inheritance, masking out things
    /// that should not be inherited
    pub(crate) fn attrs_for_inheritance(
        &self,
        context: AttrInheritContext,
        is_abi_rename: bool,
    ) -> Self {
        let pattern = match context {
            // No inheritance from modules to method-likes for the rename attribute
            AttrInheritContext::MethodOrImplFromModule
                if !is_abi_rename =>
            {
                Default::default()
            }
            // No effect on variants
            AttrInheritContext::Variant  => {
                Default::default()
            }
            _ => self.pattern.clone(),
        };
        // In the future if we support things like to_lower_case they may inherit separately
        // from patterns.
        Self { pattern }
    }

    /// From a replacement pattern, like "icu4x_{0}". Can have up to one {0} in it for substitution.
    fn from_pattern(s: &str) -> Self {
        Self {
            pattern: Some(s.parse().unwrap()),
        }
    }

    pub(crate) fn from_meta(meta: &Meta) -> Result<Self, Cow<'static, str>> {
        static ABI_RENAME_ERROR: &str = "#[diplomat::abi_rename] must be given a string value";

        match meta {
            Meta::Path(..) => Err(ABI_RENAME_ERROR.into()),
            Meta::NameValue(ref nv) => {
                // Support a shortcut `abi_rename = "..."`
                let Expr::Lit(ref lit) = nv.value else {
                    return Err(ABI_RENAME_ERROR.into());
                };
                let Lit::Str(ref lit) = lit.lit else {
                    return Err(ABI_RENAME_ERROR.into());
                };
                Ok(RenameAttr::from_pattern(&lit.value()))
            }
            // The full syntax to which we'll add more things in the future, `abi_rename("")`
            Meta::List(list) => list.parse_args().map_err(|e| {
                format!("Failed to parse malformed #[diplomat::abi_rename(...)]: {e}").into()
            }),
        }
    }
}

impl FromStr for RenamePattern {
    type Err = Infallible;
    fn from_str(s: &str) -> Result<Self, Infallible> {
        if let Some(index) = s.find("{0}") {
            let replacement = format!("{}{}", &s[..index], &s[index + 3..]);
            Ok(Self {
                replacement,
                insertion_index: Some(index),
            })
        } else {
            Ok(Self {
                replacement: s.into(),
                insertion_index: None,
            })
        }
    }
}

/// Meant to be used with Attribute::parse_args()
impl Parse for RenameAttr {
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        let value: LitStr = input.parse()?;
        let attr = RenameAttr::from_pattern(&value.value());
        Ok(attr)
    }
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize)]
struct RenamePattern {
    /// The string to replace with
    replacement: String,
    /// The index in `replacement` in which to insert the original string. If None,
    /// this is a pure rename
    insertion_index: Option<usize>,
}

#[cfg(test)]
mod tests {
    use insta;

    use syn;

    use super::{DiplomatBackendAttr, DiplomatBackendAttrCfg, RenameAttr};

    #[test]
    fn test_cfgs() {
        let attr_cfg: DiplomatBackendAttrCfg = syn::parse_quote!(*);
        insta::assert_yaml_snapshot!(attr_cfg);
        let attr_cfg: DiplomatBackendAttrCfg = syn::parse_quote!(cpp);
        insta::assert_yaml_snapshot!(attr_cfg);
        let attr_cfg: DiplomatBackendAttrCfg = syn::parse_quote!(has = overloading);
        insta::assert_yaml_snapshot!(attr_cfg);
        let attr_cfg: DiplomatBackendAttrCfg = syn::parse_quote!(has = "overloading");
        insta::assert_yaml_snapshot!(attr_cfg);
        let attr_cfg: DiplomatBackendAttrCfg =
            syn::parse_quote!(any(all(*, cpp, has="overloading"), not(js)));
        insta::assert_yaml_snapshot!(attr_cfg);
    }

    #[test]
    fn test_attr() {
        let attr: syn::Attribute =
            syn::parse_quote!(#[diplomat::attr(any(cpp, has = "overloading"), namespacing)]);
        let attr: DiplomatBackendAttr = attr.parse_args().unwrap();
        insta::assert_yaml_snapshot!(attr);
    }

    #[test]
    fn test_rename() {
        let attr: syn::Attribute = syn::parse_quote!(#[diplomat::abi_rename = "foobar_{0}"]);
        let attr = RenameAttr::from_meta(&attr.meta).unwrap();
        insta::assert_yaml_snapshot!(attr);
        let attr: syn::Attribute = syn::parse_quote!(#[diplomat::abi_rename("foobar_{0}")]);
        let attr = RenameAttr::from_meta(&attr.meta).unwrap();
        insta::assert_yaml_snapshot!(attr);
    }
}
