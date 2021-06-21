use std::str::FromStr;

use proc_macro2::{Span, TokenStream};
use quote::{quote, ToTokens};
use syn::*;

use diplomat_core::extract_from_mod;
use diplomat_core::meta;

fn gen_trait_method(strct: &meta::Struct, m: &meta::Method) -> Item {
    let self_ident = Ident::new(strct.name.as_str(), Span::call_site());
    let method_ident = Ident::new(m.name.as_str(), Span::call_site());
    let extern_ident = Ident::new(m.full_path_name.as_str(), Span::call_site());

    let mut all_params = m
        .params
        .iter()
        .map(|p| {
            FnArg::Typed(PatType {
                attrs: vec![],
                pat: Box::new(Pat::Ident(PatIdent {
                    attrs: vec![],
                    by_ref: None,
                    mutability: None,
                    ident: Ident::new(p.name.as_str(), Span::call_site()),
                    subpat: None,
                })),
                colon_token: syn::token::Colon(Span::call_site()),
                ty: Box::new(p.tpe.to_syn()),
            })
        })
        .collect::<Vec<FnArg>>();
    let all_param_names = m
        .params
        .iter()
        .map(|p| Ident::new(p.name.as_str(), Span::call_site()))
        .collect::<Vec<Ident>>();

    let this_ident = Pat::Ident(PatIdent {
        attrs: vec![],
        by_ref: None,
        mutability: None,
        ident: Ident::new("this", Span::call_site()),
        subpat: None,
    });

    if let Some(self_param) = &m.self_param {
        all_params.insert(
            0,
            FnArg::Typed(PatType {
                attrs: vec![],
                pat: Box::new(this_ident.clone()),
                colon_token: syn::token::Colon(Span::call_site()),
                ty: Box::new(self_param.tpe.to_syn()),
            }),
        );
    }

    let method_invocation = match &m.self_param {
        Some(_) => {
            quote! {
                #this_ident.#method_ident
            }
        }
        None => {
            quote! {
                #self_ident::#method_ident
            }
        }
    };

    match &m.return_type {
        None => Item::Fn(
            syn::parse2(quote! {
                #[no_mangle]
                pub extern "C" fn #extern_ident(#(#all_params),*) {
                    #method_invocation(#(#all_param_names),*);
                }
            })
            .unwrap(),
        ),
        Some(return_typ) => {
            let return_typ_syn = syn::Type::Path(
                syn::parse2(TokenStream::from_str(return_typ.name.as_str()).unwrap()).unwrap(),
            );

            Item::Fn(
                syn::parse2(quote! {
                    #[no_mangle]
                    pub extern "C" fn #extern_ident(#(#all_params),*) -> #return_typ_syn {
                        #method_invocation(#(#all_param_names),*)
                    }
                })
                .unwrap(),
            )
        }
    }
}

fn gen_bridge(input: ItemMod) -> ItemMod {
    let all_structs = extract_from_mod(&input);
    let (brace, mut new_contents) = input.content.unwrap();

    for strct in all_structs.iter() {
        strct
            .methods
            .iter()
            .for_each(|m| new_contents.push(gen_trait_method(&strct, m)));
    }

    ItemMod {
        attrs: input.attrs,
        vis: input.vis,
        mod_token: input.mod_token,
        ident: input.ident,
        content: Some((brace, new_contents)),
        semi: input.semi,
    }
}

#[proc_macro_attribute]
pub fn bridge(
    _attr: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let expanded = gen_bridge(parse_macro_input!(input));
    proc_macro::TokenStream::from(expanded.to_token_stream())
}
