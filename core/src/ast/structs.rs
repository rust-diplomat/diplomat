use serde::{Deserialize, Serialize};

use super::docs::Docs;
use super::{Ident, LifetimeEnv, Method, PathType, TypeName};

/// A struct declaration in an FFI module that is not opaque.
#[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
pub struct Struct {
    pub name: Ident,
    pub docs: Docs,
    pub lifetimes: LifetimeEnv,
    pub fields: Vec<(Ident, TypeName, Docs)>,
    pub methods: Vec<Method>,
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
