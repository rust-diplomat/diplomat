//! This module contains utilities for dealing with Rust attributes

use syn::Attribute;

pub(crate) fn extract_cfg_attrs(attrs: &[Attribute]) -> impl Iterator<Item = String> + '_ {
    attrs
        .iter()
        .filter(|&a| a.path == syn::parse_str("cfg").unwrap())
        .map(|a| quote::quote!(#a).to_string())
}
