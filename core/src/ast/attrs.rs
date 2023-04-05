//! This module contains utilities for dealing with Rust attributes

use serde::ser::{Serialize, SerializeStruct, Serializer};
use syn::Attribute;

#[derive(Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct Attrs {
    pub cfg: Vec<Attribute>,
}

impl Attrs {
    fn add_attr(&mut self, attr: Attr) {
        match attr {
            Attr::Cfg(attr) => self.cfg.push(attr),
        }
    }

    pub(crate) fn merge_parent_attrs(&mut self, other: &Attrs) {
        self.cfg.extend(other.cfg.iter().cloned())
    }
    pub(crate) fn add_attrs<'a>(&mut self, attrs: &[Attribute]) {
        for attr in syn_attr_to_ast_attr(attrs) {
            self.add_attr(attr)
        }
    }
    pub(crate) fn from_attrs<'a>(attrs: &[Attribute]) -> Self {
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
    // More goes here
}

fn syn_attr_to_ast_attr(attrs: &[Attribute]) -> impl Iterator<Item = Attr> + '_ {
    let cfg_path: syn::Path = syn::parse_str("cfg").unwrap();
    attrs.iter().filter_map(move |a| {
        if a.path() == &cfg_path {
            Some(Attr::Cfg(a.clone()))
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
        // 3 is the number of fields in the struct.
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
