use proc_macro2::Span;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Hash, Eq, PartialEq, Deserialize, Serialize, Clone, Debug, Ord, PartialOrd)]
pub struct Path {
    pub elements: Vec<String>,
}

impl Path {
    pub fn get_super(&self) -> Path {
        let mut new_elements = self.elements.clone();
        new_elements.remove(new_elements.len() - 1);
        Path {
            elements: new_elements,
        }
    }

    pub fn sub_path(&self, ident: String) -> Path {
        let mut new_elements = self.elements.clone();
        new_elements.push(ident);
        Path {
            elements: new_elements,
        }
    }

    pub fn to_syn(&self) -> syn::Path {
        syn::Path {
            leading_colon: None,
            segments: self
                .elements
                .iter()
                .map(|s| syn::PathSegment {
                    ident: syn::Ident::new(s, Span::call_site()),
                    arguments: syn::PathArguments::None,
                })
                .collect(),
        }
    }

    pub fn from_syn(path: &syn::Path) -> Path {
        let mut out = vec![];
        for elem in path.segments.iter() {
            out.push(elem.ident.to_string())
        }

        Path { elements: out }
    }

    pub fn empty() -> Path {
        Path { elements: vec![] }
    }
}

impl fmt::Display for Path {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.elements.join("::"))
    }
}
