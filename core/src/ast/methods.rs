use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use syn::*;

use super::{CustomType, TypeName};

/// A method declared in the `impl` associated with an FFI struct.
/// Includes both static and non-static methods, which can be distinguished
/// by inspecting [`Method::self_param`].
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Method {
    /// The name of the method as initially declared.
    pub name: String,

    /// The name of the FFI function wrapping around the method.
    pub full_path_name: String,

    /// The `self` param of the method, if any.
    pub self_param: Option<Param>,

    /// All non-`self` params taken by the method.
    pub params: Vec<Param>,

    /// The return type of the method, if any.
    pub return_type: Option<TypeName>,
}

impl Method {
    /// Extracts a [`Method`] from an AST node inside an `impl`.
    pub fn from_syn(m: &ImplItemMethod, self_type: &TypePath) -> Method {
        let self_ident = self_type.path.get_ident().unwrap();
        let method_ident = &m.sig.ident;
        let extern_ident = Ident::new(
            format!("{}_{}", &self_ident.to_string(), method_ident.to_string()).as_str(),
            m.sig.ident.span(),
        );

        let all_params = m
            .sig
            .inputs
            .iter()
            .filter_map(|a| match a {
                FnArg::Receiver(_) => None,
                FnArg::Typed(t) => Some(t.into()),
            })
            .collect::<Vec<_>>();

        let self_param = m.sig.receiver().map(|rec| match rec {
            FnArg::Receiver(rec) => Param {
                name: "self".to_string(),
                ty: if rec.reference.is_some() {
                    TypeName::Reference(
                        Box::new(TypeName::Named(self_ident.to_string())),
                        rec.mutability.is_some(),
                    )
                } else {
                    TypeName::Named(self_ident.to_string())
                },
            },
            _ => panic!("Unexpected self param type"),
        });

        let return_ty = match &m.sig.output {
            ReturnType::Type(_, return_typ) => Some(return_typ.as_ref().into()),
            ReturnType::Default => None,
        };

        Method {
            name: method_ident.to_string(),
            full_path_name: extern_ident.to_string(),
            self_param,
            params: all_params,
            return_type: return_ty,
        }
    }

    /// Checks that any references to opaque structs in parameters or return values
    /// are always behind a box or reference.
    ///
    /// Any references to opaque structs that are invalid are pushed into the `errors` vector.
    pub fn check_opaque<'a>(
        &'a self,
        env: &HashMap<String, CustomType>,
        errors: &mut Vec<&'a TypeName>,
    ) {
        self.self_param
            .iter()
            .for_each(|m| m.ty.check_opaque(env, errors));
        self.params
            .iter()
            .for_each(|m| m.ty.check_opaque(env, errors));
        self.return_type
            .iter()
            .for_each(|t| t.check_opaque(env, errors));
    }
}

/// A parameter taken by a [`Method`], including `self`.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Param {
    /// The name of the parameter in the original method declaration.
    pub name: String,

    /// The type of the parameter, which will be a named reference to
    /// the associated struct if this is the `self` parameter.
    pub ty: TypeName,
}

impl From<&syn::PatType> for Param {
    fn from(t: &PatType) -> Param {
        let ident = match t.pat.as_ref() {
            Pat::Ident(ident) => ident.clone(),
            _ => panic!("Unexpected param type"),
        };

        Param {
            name: ident.ident.to_string(),
            ty: t.ty.as_ref().into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use insta;

    use quote::quote;
    use syn;

    use super::Method;

    #[test]
    fn static_methods() {
        insta::assert_yaml_snapshot!(Method::from_syn(
            &syn::parse2(quote! {
                fn foo(x: u64, y: MyCustomStruct) {

                }
            })
            .unwrap(),
            &syn::parse2(quote! {
                MyStructContainingMethod
            })
            .unwrap()
        ));

        insta::assert_yaml_snapshot!(Method::from_syn(
            &syn::parse2(quote! {
                fn foo(x: u64, y: MyCustomStruct) -> u64 {
                    x
                }
            })
            .unwrap(),
            &syn::parse2(quote! {
                MyStructContainingMethod
            })
            .unwrap()
        ));
    }

    #[test]
    fn nonstatic_methods() {
        insta::assert_yaml_snapshot!(Method::from_syn(
            &syn::parse2(quote! {
                fn foo(&self, x: u64, y: MyCustomStruct) {

                }
            })
            .unwrap(),
            &syn::parse2(quote! {
                MyStructContainingMethod
            })
            .unwrap()
        ));

        insta::assert_yaml_snapshot!(Method::from_syn(
            &syn::parse2(quote! {
                fn foo(&mut self, x: u64, y: MyCustomStruct) -> u64 {
                    x
                }
            })
            .unwrap(),
            &syn::parse2(quote! {
                MyStructContainingMethod
            })
            .unwrap()
        ));
    }
}
