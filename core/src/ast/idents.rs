use quote::{ToTokens, TokenStreamExt};
use serde::{Deserialize, Serialize};
use std::borrow::{Borrow, Cow};
use std::fmt;

/// Equivalent to `proc_macro2::Span`.
#[derive(Hash, Eq, PartialEq, Serialize, Clone, Debug, Ord, PartialOrd)]
pub struct Span {
    pub start_line : usize,
    pub column_line : usize,
    pub file : String,
}

/// An identifier, analogous to `syn::Ident` and `proc_macro2::Ident`.
#[derive(Eq, Serialize, Clone, Debug)]
pub struct Ident(Cow<'static, str>, Option<Span>);

impl Ident {
    /// Validate a string
    fn validate(string: &str) -> syn::Result<()> {
        syn::parse_str::<syn::Ident>(string).map(|_| {})
    }

    /// Attempt to create a new `Ident`.
    ///
    /// This function fails if the input isn't valid according to
    /// `proc_macro2::Ident`'s invariants.
    // pub fn try_new(string: String) -> syn::Result<Self> {
    //     Self::validate(&string).map(|_| Self(Cow::from(string)))
    // }

    pub fn to_syn(&self) -> syn::Ident {
        syn::Ident::new(self.as_str(), proc_macro2::Span::call_site())
    }

    /// Get the `&str` representation.
    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn span(&self) -> Option<Span> {
        self.1.clone()
    }

    /// An [`Ident`] containing "this".
    pub const THIS: Self = Ident(Cow::Borrowed("this"), None);
}

impl std::hash::Hash for Ident {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl Ord for Ident {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}

impl PartialOrd for Ident {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(&other.0) 
    }
}

impl PartialEq for Ident {
    fn eq(&self, other: &Self) -> bool {
        self.0.eq(&other.0)
    }
}

impl From<&'static str> for Ident {
    fn from(string: &'static str) -> Self {
        Self::validate(string).unwrap();
        Self(Cow::from(string), None)
    }
}

impl From<String> for Ident {
    fn from(string: String) -> Self {
        Self::validate(&string).unwrap();
        Self(Cow::from(string), None)
    }
}

impl<'de> Deserialize<'de> for Ident {
    /// The derived `Deserialize` allows for creating `Ident`s that do not uphold
    /// the proper invariants. This custom impl ensures that this cannot happen.
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(Ident::from(String::deserialize(deserializer)?))
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
        let span = ident.span();
        let start = span.start();
        Self(Cow::from(ident.to_string()), Some(
            Span {
                start_line: start.line,
                column_line: start.column,
                file: "".into(),
            }
        ))
    }
}

impl ToTokens for Ident {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        tokens.append(self.to_syn());
    }
}
