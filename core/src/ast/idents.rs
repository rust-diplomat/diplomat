use proc_macro2::Span;
use quote::{ToTokens, TokenStreamExt};
use serde::{Deserialize, Serialize};
use std::{borrow::Borrow, fmt};
 
/// An identifier, analogous to `syn::Ident` and `proc_macro2::Ident`.
#[derive(Hash, Eq, PartialEq, Deserialize, Serialize, Clone, Debug, Ord, PartialOrd)]
pub struct Ident(String);
 
impl Ident {
   pub fn to_syn(&self) -> syn::Ident {
       syn::Ident::new(&self.0, Span::call_site())
   }
}
 
impl Borrow<str> for Ident {
   fn borrow(&self) -> &str {
       &self.0
   }
}
 
impl fmt::Display for Ident {
   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       self.0.fmt(f)
   }
}
 
impl From<&syn::Ident> for Ident {
   fn from(ident: &syn::Ident) -> Self {
       Ident(ident.to_string())
   }
}
 
impl ToTokens for Ident {
   fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
       tokens.append(proc_macro2::Ident::new(&self.0, Span::call_site()));
   }
}
 
