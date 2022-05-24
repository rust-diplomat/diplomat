use serde::{Deserialize, Serialize};
use syn::*;

use super::docs::Docs;
use super::{Lifetime, Mutability, LifetimeDef, Path, PathType, TypeName, ValidityError};
use crate::Env;

/// A method declared in the `impl` associated with an FFI struct.
/// Includes both static and non-static methods, which can be distinguished
/// by inspecting [`Method::self_param`].
#[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
pub struct Method {
    /// The name of the method as initially declared.
    pub name: String,

    /// Lines of documentation for the method.
    pub docs: Docs,

    /// The name of the FFI function wrapping around the method.
    pub full_path_name: String,

    /// The `self` param of the method, if any.
    pub self_param: Option<Param>,

    /// All non-`self` params taken by the method.
    pub params: Vec<Param>,

    /// The return type of the method, if any.
    pub return_type: Option<TypeName>,

    /// The lifetimes introduced in this method and surrounding impl block.
    pub introduced_lifetimes: Vec<LifetimeDef>,
}

impl Method {
    /// Extracts a [`Method`] from an AST node inside an `impl`.
    pub fn from_syn(
        m: &ImplItemMethod,
        self_path_type: PathType,
        impl_introduced_lifetimes: Vec<LifetimeDef>,
    ) -> Method {
        let self_ident = self_path_type.path.elements.last().unwrap();
        let method_ident = &m.sig.ident;
        let extern_ident = Ident::new(
            format!("{}_{}", self_ident, method_ident).as_str(),
            m.sig.ident.span(),
        );

        // It doesn't matter if we introduce lifetimes in a method that doesn't use them,
        // so we introduce every lifetime introduced in both the impl and the method to ensure we have everything.
        // This will look idiomatic in 99% of cases, and won't even break the wack cases where
        // a lifetime is introduced in the impl but not used in the method.
        let mut introduced_lifetimes = impl_introduced_lifetimes;
        introduced_lifetimes.extend(m.sig.generics.lifetimes().map(Into::into));

        let all_params = m
            .sig
            .inputs
            .iter()
            .filter_map(|a| match a {
                FnArg::Receiver(_) => None,
                FnArg::Typed(t) => Some(Param::from_syn(t, self_path_type.clone())),
            })
            .collect::<Vec<_>>();

        let self_param = m.sig.receiver().map(|rec| match rec {
            FnArg::Receiver(rec) => Param {
                name: "self".to_string(),
                ty: if let Some(ref reference) = rec.reference {
                    TypeName::Reference(
                        Box::new(TypeName::Named(self_path_type.clone())),
                        Mutability::from_syn(&rec.mutability),
                        Lifetime::from(&reference.1),
                    )
                } else {
                    TypeName::Named(self_path_type.clone())
                },
            },
            _ => panic!("Unexpected self param type"),
        });

        let return_ty = match &m.sig.output {
            ReturnType::Type(_, return_typ) => Some(TypeName::from_syn(
                return_typ.as_ref(),
                Some(self_path_type),
            )),
            ReturnType::Default => None,
        };

        Method {
            name: method_ident.to_string(),
            docs: Docs::from_attrs(&m.attrs),
            full_path_name: extern_ident.to_string(),
            self_param,
            params: all_params,
            return_type: return_ty,
            introduced_lifetimes,
        }
    }

    /// Performs type-specific validity checks (see [TypeName::check_validity()])
    pub fn check_validity<'a>(
        &'a self,
        in_path: &Path,
        env: &Env,
        errors: &mut Vec<ValidityError>,
    ) {
        self.self_param
            .iter()
            .for_each(|m| m.ty.check_validity(in_path, env, errors));
        self.params
            .iter()
            .for_each(|m| m.ty.check_validity(in_path, env, errors));
        self.return_type
            .iter()
            .for_each(|t| t.check_validity(in_path, env, errors));
    }

    /// Checks whether the method qualifies for special writeable handling.
    /// To qualify, a method must:
    ///  - not return any value
    ///  - have the last argument be an `&mut diplomat_runtime::DiplomatWriteable`
    ///
    /// Typically, methods of this form will be transformed in the bindings to a
    /// method that doesn't take the writeable as an argument but instead creates
    /// one locally and just returns the final string.
    pub fn is_writeable_out(&self) -> bool {
        let return_compatible = self.return_type.is_none()
            || match self.return_type.as_ref().unwrap() {
                TypeName::Unit => true,
                TypeName::Result(ok, _) => matches!(ok.as_ref(), TypeName::Unit),
                _ => false,
            };

        return_compatible
            && !self.params.is_empty()
            && self.params[self.params.len() - 1].is_writeable()
    }

    /// Checks if any parameters are writeable (regardless of other compatibilities for writeable output)
    pub fn has_writeable_param(&self) -> bool {
        self.params.iter().any(|p| p.is_writeable())
    }
}

/// A parameter taken by a [`Method`], including `self`.
#[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
pub struct Param {
    /// The name of the parameter in the original method declaration.
    pub name: String,

    /// The type of the parameter, which will be a named reference to
    /// the associated struct if this is the `self` parameter.
    pub ty: TypeName,
}

impl Param {
    /// Check if this parameter is a Writeable
    pub fn is_writeable(&self) -> bool {
        match self.ty {
            TypeName::Reference(ref w, Mutability::Mutable, ref _lt) => **w == TypeName::Writeable,
            _ => false,
        }
    }

    pub fn from_syn(t: &PatType, self_path_type: PathType) -> Self {
        let ident = match t.pat.as_ref() {
            Pat::Ident(ident) => ident.clone(),
            _ => panic!("Unexpected param type"),
        };

        Param {
            name: ident.ident.to_string(),
            ty: TypeName::from_syn(&t.ty, Some(self_path_type)),
        }
    }
}

#[cfg(test)]
mod tests {
    use insta;

    use syn;

    use super::{Method, Path, PathType};

    #[test]
    fn static_methods() {
        insta::assert_yaml_snapshot!(Method::from_syn(
            &syn::parse_quote! {
                /// Some docs.
                #[diplomat::rust_link(foo::Bar::batz, FnInStruct)]
                fn foo(x: u64, y: MyCustomStruct) {

                }
            },
            PathType::new(Path::empty().sub_path("MyStructContainingMethod".to_string())),
            vec![]
        ));

        insta::assert_yaml_snapshot!(Method::from_syn(
            &syn::parse_quote! {
                /// Some docs.
                /// Some more docs.
                ///
                /// Even more docs.
                #[diplomat::rust_link(foo::Bar::batz, FnInEnum)]
                fn foo(x: u64, y: MyCustomStruct) -> u64 {
                    x
                }
            },
            PathType::new(Path::empty().sub_path("MyStructContainingMethod".to_string())),
            vec![]
        ));
    }

    #[test]
    fn nonstatic_methods() {
        insta::assert_yaml_snapshot!(Method::from_syn(
            &syn::parse_quote! {
                fn foo(&self, x: u64, y: MyCustomStruct) {

                }
            },
            PathType::new(Path::empty().sub_path("MyStructContainingMethod".to_string())),
            vec![]
        ));

        insta::assert_yaml_snapshot!(Method::from_syn(
            &syn::parse_quote! {
                #[diplomat::rust_link(foo::Bar::batz, FnInStruct)]
                fn foo(&mut self, x: u64, y: MyCustomStruct) -> u64 {
                    x
                }
            },
            PathType::new(Path::empty().sub_path("MyStructContainingMethod".to_string())),
            vec![]
        ));
    }
}
