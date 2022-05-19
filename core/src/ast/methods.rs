use serde::{Deserialize, Serialize};
use syn::*;

use super::docs::Docs;
use super::{Lifetime, Path, PathType, TypeName, ValidityError};
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

    /// The lifetimes introduced in this method, e.g. `'a` in `fn make_foo<'a>(&self, x: &'a u8) -> Foo<'a>`
    pub introduced_lifetimes: Vec<Lifetime>,
}

impl Method {
    /// Extracts a [`Method`] from an AST node inside an `impl`.
    pub fn from_syn(m: &ImplItemMethod, self_path: &Path) -> Method {
        let self_ident = self_path.elements.last().unwrap();
        let method_ident = &m.sig.ident;
        let extern_ident = Ident::new(
            format!("{}_{}", self_ident, method_ident).as_str(),
            m.sig.ident.span(),
        );

        let introduced_lifetimes = m
            .sig
            .generics
            .lifetimes()
            .map(|lt| {
                if !lt.bounds.is_empty() {
                    panic!("Bounds on lifetimes currently unsupported");
                }

                Lifetime::from(&lt.lifetime)
            })
            .collect();

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
                ty: if let Some(ref reference) = rec.reference {
                    TypeName::Reference(
                        Box::new(TypeName::Named(PathType::new(self_path.clone()))),
                        rec.mutability.is_some(),
                        Lifetime::from(&reference.1),
                    )
                } else {
                    TypeName::Named(PathType::new(self_path.clone()))
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
            TypeName::Reference(ref w, true, ref _lt) => **w == TypeName::Writeable,
            _ => false,
        }
    }
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

    use syn;

    use super::{Method, Path};

    #[test]
    fn static_methods() {
        insta::assert_yaml_snapshot!(Method::from_syn(
            &syn::parse_quote! {
                /// Some docs.
                #[diplomat::rust_link(foo::Bar::batz, FnInStruct)]
                fn foo(x: u64, y: MyCustomStruct) {

                }
            },
            &Path::empty().sub_path("MyStructContainingMethod".to_string())
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
            &Path::empty().sub_path("MyStructContainingMethod".to_string())
        ));
    }

    #[test]
    fn nonstatic_methods() {
        insta::assert_yaml_snapshot!(Method::from_syn(
            &syn::parse_quote! {
                fn foo(&self, x: u64, y: MyCustomStruct) {

                }
            },
            &Path::empty().sub_path("MyStructContainingMethod".to_string())
        ));

        insta::assert_yaml_snapshot!(Method::from_syn(
            &syn::parse_quote! {
                #[diplomat::rust_link(foo::Bar::batz, FnInStruct)]
                fn foo(&mut self, x: u64, y: MyCustomStruct) -> u64 {
                    x
                }
            },
            &Path::empty().sub_path("MyStructContainingMethod".to_string())
        ));
    }
}
