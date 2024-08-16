use serde::{Deserialize, Serialize};

use super::docs::Docs;
use super::{Attrs, Ident, LifetimeEnv, Method, Mutability, Param, PathType, SelfParam, TypeName};

/// A struct declaration in an FFI module that is not opaque.
#[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
#[non_exhaustive]
pub struct Trait {
    pub name: Ident,
    // pub lifetimes: LifetimeEnv,
    pub fcts: Vec<TraitFct>,
}

#[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
#[non_exhaustive]
pub struct TraitFct {
    pub name: Ident,
    pub abi_name: Ident,
    pub self_param: Option<SelfParam>,
    // corresponds to the types in Function(Vec<Box<TypeName>>, Box<TypeName>)
    // the callback type; except here the params aren't anonymous
    pub params: Vec<Param>,
    pub output_type: Option<TypeName>,
}

impl Trait {
    /// Extract an [`Enum`] metadata value from an AST node.
    pub fn new(trt: &syn::ItemTrait, parent_attrs: &Attrs) -> Self {
        let mut attrs = parent_attrs.clone();
        attrs.add_attrs(&trt.attrs);

        let mut trait_fcts = Vec::new();

        let self_ident = &trt.ident;
        // TODO check this
        let self_path_type = PathType::from(&syn::TypePath {
            qself: None,
            path: syn::PathSegment {
                ident: self_ident.clone(),
                arguments: syn::PathArguments::None,
            }
            .into(),
        });
        for trait_item in trt.items.iter() {
            if let syn::TraitItem::Fn(fct) = trait_item {
                // copied from the method parsing
                let fct_ident = &fct.sig.ident;
                let concat_fct_ident = format!("{self_ident}_{fct_ident}");
                let extern_ident = syn::Ident::new(
                    &attrs.abi_rename.apply(concat_fct_ident.into()),
                    fct.sig.ident.span(),
                );

                let all_params = fct
                    .sig
                    .inputs
                    .iter()
                    .filter_map(|a| match a {
                        syn::FnArg::Receiver(_) => None,
                        syn::FnArg::Typed(ref t) => {
                            Some(Param::from_syn(t, self_path_type.clone()))
                        }
                    })
                    .collect::<Vec<_>>();

                let self_param = fct
                    .sig
                    .receiver()
                    .map(|rec| SelfParam::from_syn(rec, self_path_type.clone()));

                let output_type = match &fct.sig.output {
                    syn::ReturnType::Type(_, return_typ) => Some(TypeName::from_syn(
                        return_typ.as_ref(),
                        Some(self_path_type.clone()),
                    )),
                    syn::ReturnType::Default => None,
                };

                trait_fcts.push(TraitFct {
                    name: fct_ident.into(),
                    abi_name: (&extern_ident).into(),
                    self_param,
                    params: all_params,
                    output_type,
                });
            }
        }

        Self {
            name: (&trt.ident).into(),
            fcts: trait_fcts,
        }
    }
}
