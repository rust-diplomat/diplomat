use serde::{Deserialize, Serialize};

use super::docs::Docs;
use super::{Ident, LifetimeEnv, Method, NamedLifetime, PathType, TypeName};

/// A struct declaration in an FFI module that is not opaque.
#[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
pub struct Struct {
    pub name: Ident,
    pub docs: Docs,
    pub lifetime_env: LifetimeEnv,
    pub fields: Vec<(Ident, TypeName, Docs)>,
    pub methods: Vec<Method>,
}

impl Struct {
    /// Accepts a filter determining which lifetimes to select based on indices,
    /// and returns a `Vec<&NamedLifetime>` that contains all the lifetimes in the
    /// scope of the `Struct` that must live at least as long as the selected
    /// lifetimes.
    ///
    /// This is similar to [`LifetimeEnv::outlives`], except it takes a filter
    /// of indices instead of a sequence of `&NamedLifetime`s.
    ///
    /// # Examples
    ///
    /// Say we have a struct defined as
    /// ```ignore
    /// struct Foo<'a, 'b, 'c: 'a> {
    ///     first: &'a str,
    ///     second: &'b str,
    ///     third: &'c str,
    /// }
    /// ```
    ///
    /// Let `struct_type` be this `Foo` type parsed as a `Struct`.
    /// Then we can call `Struct::borrowed_lifetimes`:
    /// ```ignore
    /// let lifetimes = struct_type.borrowed_lifetimes(|i| {
    ///     // some bool predicate that determines if we care
    ///     // about the i^th lifetime.
    ///     // for simplicity, lets say we only want the first one.
    ///     i == 0
    /// });
    /// ```
    ///
    /// Then `lifetimes` would contain `['a, 'c]`, since `'c` must live
    /// at least as long as `'a`.
    pub fn borrowed_lifetimes<F>(&self, mut pred: F) -> Vec<&NamedLifetime>
    where
        F: FnMut(usize) -> bool,
    {
        // All the lifetimes that we selected to keep alive.
        let selected_lifetimes = self
            .lifetime_env
            .iter()
            .enumerate()
            .filter_map(|(idx, named)| if pred(idx) { Some(named) } else { None });

        // All the lifetimes that live at least as long as the lifetimes that we
        // selected.
        // !!! THIS CAN BE OPTIMIZED TO WORK WITH THE INDICES INSTEAD OF THE NAMES !!!
        self.lifetime_env.outlives(selected_lifetimes)
    }
}

impl From<&syn::ItemStruct> for Struct {
    /// Extract a [`Struct`] metadata value from an AST node.
    fn from(strct: &syn::ItemStruct) -> Struct {
        let self_path_type = PathType::extract_self_type(strct);

        Struct {
            name: (&strct.ident).into(),
            docs: Docs::from_attrs(&strct.attrs),
            // Invariant: The `LifetimeEnv` only contains lifetimes
            // from `strct`'s generics, so the indices of lifetimes in the
            // graph align with the order defined in `strct`'s generics.
            lifetime_env: LifetimeEnv::from(&strct.generics),
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
