use serde::{Deserialize, Serialize};

use super::docs::Docs;
use super::{Ident, LifetimeEnv, Method, NamedLifetime, PathType, TypeName};

/// A struct declaration in an FFI module that is not opaque.
#[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
pub struct Struct {
    pub name: Ident,
    pub docs: Docs,
    pub lifetimes: LifetimeEnv,
    pub fields: Vec<(Ident, TypeName, Docs)>,
    pub methods: Vec<Method>,
}

impl Struct {
    // WARNING: this could probably be made more optimized, but here's what I'm thinking:
    // Take a predicate which accepts a `usize` denoting the position the lifetime
    // occurred at in the struct definition, and return true if that lifetime
    // should be kept. By working with indices instead of lifetime names, we're
    // robust against renaming lifetimes in different blocks.
    pub fn borrowed_fields<'a, F>(&'a self, mut pred: F) -> Vec<&(Ident, TypeName, Docs)>
    where
        F: FnMut(usize) -> bool,
    {
        // A mapping from def indices to env indices
        // The index of each element corresponds to the index in the `syn::LifetimeDef`
        // that it came from
        // The value at each index corresponds to where that particular lifetime
        // lives in the `LifetimeEnv`.
        // This would have to be stored in `Self` before hand.
        // let def_idx_to_env_idx: Vec<usize> = vec![];

        // Since lifetime names can be totally different, we care about the indices,
        // NOT the names.
        // let _lts_we_care_about: Vec<&NamedLifetime> = def_idx_to_env_idx
        //     .iter()
        //     .enumerate()
        //     .filter_map(|(idx, ptr)| {
        //         if pred(idx) {
        //             // pretend this works and is safe
        //             Some(&self.lifetimes.nodes[ptr].lifetime)
        //         } else {
        //             None
        //         }
        //     })
        //     .collect::<Vec<_>>();

        // One strategy we could do to assign indices to lifetimes: Use the
        // index of the lifetime in the `LifetimeEnv`, NOT the index at which
        // it appears in the struct definition.
        // This requires remembering the order that lifetimes are read in
        todo!()
    }
}

impl From<&syn::ItemStruct> for Struct {
    /// Extract a [`Struct`] metadata value from an AST node.
    fn from(strct: &syn::ItemStruct) -> Struct {
        let self_path_type = PathType::extract_self_type(strct);

        Struct {
            name: (&strct.ident).into(),
            docs: Docs::from_attrs(&strct.attrs),
            lifetimes: LifetimeEnv::from(&strct.generics),
            fields: strct
                .fields
                .iter()
                .map(|field| {
                    // Non-opaque tuple structs will never be allowed
                    let name = field
                        .ident
                        .as_ref()
                        .map(Into::into)
                        .expect("non-opaque tuples structs are disallowed");
                    let type_name = TypeName::from_syn(&field.ty, Some(self_path_type.clone()));
                    let docs = Docs::from_attrs(&field.attrs);

                    (name, type_name, docs)
                })
                .collect(),
            methods: vec![],
        }
    }
}

/// A struct annotated with [`diplomat::opaque`] whose fields are not visible.
/// Opaque structs cannot be passed by-value across the FFI boundary, so they
/// must be boxed or passed as references.
#[derive(Clone, Serialize, Deserialize, Debug, Hash, PartialEq, Eq)]
pub struct OpaqueStruct {
    pub name: Ident,
    pub docs: Docs,
    pub lifetimes: LifetimeEnv,
    pub methods: Vec<Method>,
}

impl From<&syn::ItemStruct> for OpaqueStruct {
    /// Extract a [`OpaqueStruct`] metadata value from an AST node.
    fn from(strct: &syn::ItemStruct) -> OpaqueStruct {
        OpaqueStruct {
            name: (&strct.ident).into(),
            docs: Docs::from_attrs(&strct.attrs),
            lifetimes: LifetimeEnv::from(&strct.generics),
            methods: vec![],
        }
    }
}

#[cfg(test)]
mod tests {
    use insta::{self, Settings};

    use syn;

    use super::Struct;

    #[test]
    fn simple_struct() {
        let mut settings = Settings::new();
        settings.set_sort_maps(true);

        settings.bind(|| {
            insta::assert_yaml_snapshot!(Struct::from(&syn::parse_quote! {
                /// Some docs.
                #[diplomat::rust_link(foo::Bar, Struct)]
                struct MyLocalStruct {
                    a: i32,
                    b: Box<MyLocalStruct>
                }
            }));
        });
    }
}
