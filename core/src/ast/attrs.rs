//! This module contains utilities for dealing with Rust attributes

use serde::ser::{SerializeStruct, Serializer};
use serde::Serialize;
use syn::parse::{Error as ParseError, Parse, ParseStream};
use syn::{Attribute, Ident, LitStr, Meta, Token};

/// The list of attributes on a type
#[derive(Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct Attrs {
    pub cfg: Vec<Attribute>,
    pub attrs: Vec<DiplomatAttr>,
}

impl Attrs {
    fn add_attr(&mut self, attr: Attr) {
        match attr {
            Attr::Cfg(attr) => self.cfg.push(attr),
            Attr::DiplomatAttr(attr) => self.attrs.push(attr),
        }
    }

    /// Merge attributes that should be inherited from the parent
    pub(crate) fn merge_parent_attrs(&mut self, other: &Attrs) {
        self.cfg.extend(other.cfg.iter().cloned())
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
    DiplomatAttr(DiplomatAttr),
    // More goes here
}

fn syn_attr_to_ast_attr(attrs: &[Attribute]) -> impl Iterator<Item = Attr> + '_ {
    let cfg_path: syn::Path = syn::parse_str("cfg").unwrap();
    let dattr_path: syn::Path = syn::parse_str("diplomat::attr").unwrap();
    attrs.iter().filter_map(move |a| {
        if a.path() == &cfg_path {
            Some(Attr::Cfg(a.clone()))
        } else if a.path() == &dattr_path {
            Some(Attr::DiplomatAttr(
                a.parse_args()
                    .expect("Failed to parse malformed diplomat::attr"),
            ))
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
        let cfg: Vec<_> = self
            .cfg
            .iter()
            .map(|a| quote::quote!(#a).to_string())
            .collect();
        state.serialize_field("cfg", &cfg)?;
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
pub struct DiplomatAttr {
    pub cfg: DiplomatAttrCfg,
    #[serde(serialize_with = "serialize_meta")]
    pub attr: Meta,
}

fn serialize_meta<S>(m: &Meta, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    quote::quote!(#m).to_string().serialize(s)
}

#[derive(Clone, PartialEq, Eq, Hash, Debug, Serialize)]
pub enum DiplomatAttrCfg {
    Not(Box<DiplomatAttrCfg>),
    Any(Vec<DiplomatAttrCfg>),
    All(Vec<DiplomatAttrCfg>),
    Star,
    BackendName(String),
    NameValue(String, String),
}

impl Parse for DiplomatAttrCfg {
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(Ident) {
            let name: Ident = input.parse()?;
            if name == "not" {
                let content;
                let _paren = syn::parenthesized!(content in input);
                Ok(DiplomatAttrCfg::Not(Box::new(content.parse()?)))
            } else if name == "any" || name == "all" {
                let content;
                let _paren = syn::parenthesized!(content in input);
                let list = content.parse_terminated(Self::parse, Token![,])?;
                let vec = list.into_iter().collect();
                if name == "any" {
                    Ok(DiplomatAttrCfg::Any(vec))
                } else {
                    Ok(DiplomatAttrCfg::All(vec))
                }
            } else if input.peek(Token![=]) {
                let _t: Token![=] = input.parse()?;
                if input.peek(Ident) {
                    let value: Ident = input.parse()?;
                    Ok(DiplomatAttrCfg::NameValue(
                        name.to_string(),
                        value.to_string(),
                    ))
                } else {
                    let value: LitStr = input.parse()?;
                    Ok(DiplomatAttrCfg::NameValue(name.to_string(), value.value()))
                }
            } else {
                Ok(DiplomatAttrCfg::BackendName(name.to_string()))
            }
        } else if lookahead.peek(Token![*]) {
            let _t: Token![*] = input.parse()?;
            Ok(DiplomatAttrCfg::Star)
        } else {
            Err(ParseError::new(
                input.span(),
                "CFG portion of #[diplomat::attr] fails to parse",
            ))
        }
    }
}

/// Meant to be used with Attribute::parse_args()
impl Parse for DiplomatAttr {
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        let cfg = input.parse()?;
        let _comma: Token![,] = input.parse()?;
        let attr = input.parse()?;
        Ok(Self { cfg, attr })
    }
}

#[cfg(test)]
mod tests {
    use insta;

    use syn;

    use super::{DiplomatAttr, DiplomatAttrCfg};

    #[test]
    fn test_cfgs() {
        let attr_cfg: DiplomatAttrCfg = syn::parse_quote!(*);
        insta::assert_yaml_snapshot!(attr_cfg);
        let attr_cfg: DiplomatAttrCfg = syn::parse_quote!(cpp);
        insta::assert_yaml_snapshot!(attr_cfg);
        let attr_cfg: DiplomatAttrCfg = syn::parse_quote!(has = overloading);
        insta::assert_yaml_snapshot!(attr_cfg);
        let attr_cfg: DiplomatAttrCfg = syn::parse_quote!(has = "overloading");
        insta::assert_yaml_snapshot!(attr_cfg);
        let attr_cfg: DiplomatAttrCfg =
            syn::parse_quote!(any(all(*, cpp, has="overloading"), not(js)));
        insta::assert_yaml_snapshot!(attr_cfg);
    }

    #[test]
    fn test_attr() {
        let attr: syn::Attribute =
            syn::parse_quote!(#[diplomat::attr(any(cpp, has = "overloading"), namespacing)]);
        let attr: DiplomatAttr = attr.parse_args().unwrap();
        insta::assert_yaml_snapshot!(attr);
    }
}
