use serde::Serialize;
use syn::ItemFn;

use crate::ast::{Attrs, Docs, Ident, LifetimeEnv, Param, PathType, TypeName};

#[derive(Clone, PartialEq, Eq, Hash, Serialize, Debug)]
pub struct Function {
    pub name: Ident,
    pub abi_name: Ident,
    // corresponds to the types in Function(Vec<Box<TypeName>>, Box<TypeName>)
    // the callback type; except here the params aren't anonymous
    pub params: Vec<Param>,
    pub output_type: Option<TypeName>,
    pub lifetimes: LifetimeEnv,
    pub attrs: Attrs,
    pub docs: Docs,
}

impl Function {
    pub(crate) fn from_syn(f: &ItemFn) -> Function {
        let ident: Ident = (&f.sig.ident).into();
        if f.sig.receiver().is_some() {
            panic!("Cannot use self parameter in free function {ident:?}")
        }

        let attrs = Attrs::from_attrs(&f.attrs);

        let extern_ident = syn::Ident::new(
            &attrs.abi_rename.apply(ident.to_string().into()),
            f.sig.ident.span(),
        );

        let path_type = PathType::from(&f.sig);

        let all_params = f
            .sig
            .inputs
            .iter()
            .filter_map(|a| match a {
                syn::FnArg::Receiver(_) => None,
                syn::FnArg::Typed(ref t) => Some(Param::from_syn(t, path_type.clone())),
            })
            .collect::<Vec<_>>();

        let output_type = match &f.sig.output {
            syn::ReturnType::Type(_, return_typ) => Some(TypeName::from_syn(
                return_typ.as_ref(),
                Some(path_type.clone()),
            )),
            syn::ReturnType::Default => None,
        };

        let lifetimes = LifetimeEnv::from_function_item(f, &all_params[..], output_type.as_ref());

        Self {
            name: ident,
            abi_name: (&extern_ident).into(),
            params: all_params,
            output_type,
            lifetimes,
            attrs,
            docs: Docs::from_attrs(&f.attrs),
        }
    }
}

#[cfg(test)]
mod tests {
    use insta;

    use syn;

    use crate::ast::Function;

    #[test]
    fn test_free_function() {
        insta::assert_yaml_snapshot!(Function::from_syn(&syn::parse_quote! {
            fn some_func(a : f32, b: f64) {

            }
        }));
    }

    #[test]
    fn test_free_function_output() {
        insta::assert_yaml_snapshot!(Function::from_syn(&syn::parse_quote! {
            fn some_func(a : SomeType) -> Option<()> {

            }
        }));
    }

    #[test]
    fn test_free_function_lifetimes() {
        insta::assert_yaml_snapshot!(Function::from_syn(&syn::parse_quote! {
            fn some_func<'a>(a : &'a SomeType) -> Option<&'a SomeType> {

            }
        }));
    }
}
