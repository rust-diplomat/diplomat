use proc_macro2::Span;
use quote::{ToTokens, TokenStreamExt};
use serde::{Deserialize, Serialize};
use std::{borrow::Borrow, fmt};

/// An identifier, analogous to `syn::Ident` and `proc_macro2::Ident`.
#[derive(Hash, Eq, PartialEq, Deserialize, Serialize, Clone, Debug, Ord, PartialOrd)]
pub struct Ident(String);

impl Ident {
    /// Create a new `Ident`.
    ///
    /// # Panics
    ///
    /// This method panics if the provided string isn't a valid identifier.
    pub fn new(string: &str) -> Self {
        Ident(syn::parse_str::<syn::Ident>(string).unwrap().to_string())
    }

    pub fn to_syn(&self) -> syn::Ident {
        syn::Ident::new(self.as_str(), Span::call_site())
    }

    pub fn to_proc_macro2(&self) -> proc_macro2::Ident {
        proc_macro2::Ident::new(self.as_str(), Span::call_site())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Borrow<str> for Ident {
    fn borrow(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for Ident {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.as_str().fmt(f)
    }
}

impl From<&syn::Ident> for Ident {
    fn from(ident: &syn::Ident) -> Self {
        Ident(ident.to_string())
    }
}

impl ToTokens for Ident {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        tokens.append(self.to_proc_macro2());
    }
}
