use serde::Serialize;

use super::docs::Docs;
use super::{Attrs, Ident, LifetimeEnv, Method, Mutability, PathType, TypeName};

/// A struct declaration in an FFI module that is not opaque.
#[derive(Clone, PartialEq, Eq, Hash, Serialize, Debug)]
#[non_exhaustive]
pub struct Struct {
    pub name: Ident,
    pub docs: Docs,
    pub lifetimes: LifetimeEnv,
    pub fields: Vec<(Ident, TypeName, Docs)>,
    pub methods: Vec<Method>,
    pub output_only: bool,
    pub attrs: Attrs,
}

impl Struct {
    /// Extract a [`Struct`] metadata value from an AST node.
    pub fn new(strct: &syn::ItemStruct, output_only: bool) -> Self {
        let self_path_type = PathType::extract_self_type(strct);
        let fields: Vec<_> = strct
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
            .collect();

        let lifetimes = LifetimeEnv::from_struct_item(strct, &fields[..]);

        Struct {
            name: (&strct.ident).into(),
            docs: Docs::from_attrs(&strct.attrs),
            lifetimes,
            fields,
            methods: vec![],
            output_only,
            attrs: (&*strct.attrs).into(),
        }
    }
}

/// A struct annotated with [`diplomat::opaque`] whose fields are not visible.
/// Opaque structs cannot be passed by-value across the FFI boundary, so they
/// must be boxed or passed as references.
#[derive(Clone, Serialize, Debug, Hash, PartialEq, Eq)]
#[non_exhaustive]
pub struct OpaqueStruct {
    pub name: Ident,
    pub docs: Docs,
    pub lifetimes: LifetimeEnv,
    pub methods: Vec<Method>,
    pub mutability: Mutability,
    pub attrs: Attrs,
}

impl OpaqueStruct {
    /// Extract a [`OpaqueStruct`] metadata value from an AST node.
    pub fn new(strct: &syn::ItemStruct, mutability: Mutability) -> Self {
        OpaqueStruct {
            name: Ident::from(&strct.ident),
            docs: Docs::from_attrs(&strct.attrs),
            lifetimes: LifetimeEnv::from_struct_item(strct, &[]),
            methods: vec![],
            mutability,
            attrs: (&*strct.attrs).into(),
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
            insta::assert_yaml_snapshot!(Struct::new(
                &syn::parse_quote! {
                    /// Some docs.
                    #[diplomat::rust_link(foo::Bar, Struct)]
                    struct MyLocalStruct {
                        a: i32,
                        b: Box<MyLocalStruct>
                    }
                },
                true
            ));
        });
    }
}
