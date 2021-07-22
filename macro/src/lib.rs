use proc_macro2::Span;
use quote::{quote, ToTokens};
use syn::*;

use diplomat_core::ast;

fn gen_params_at_boundary(param: &ast::Param, expanded_params: &mut Vec<FnArg>) {
    match &param.ty {
        ast::TypeName::StrReference => {
            expanded_params.push(FnArg::Typed(PatType {
                attrs: vec![],
                pat: Box::new(Pat::Ident(PatIdent {
                    attrs: vec![],
                    by_ref: None,
                    mutability: None,
                    ident: Ident::new(
                        (param.name.clone() + "_diplomat_data").as_str(),
                        Span::call_site(),
                    ),
                    subpat: None,
                })),
                colon_token: syn::token::Colon(Span::call_site()),
                ty: Box::new(
                    parse2(quote! {
                        *const u8
                    })
                    .unwrap(),
                ),
            }));

            expanded_params.push(FnArg::Typed(PatType {
                attrs: vec![],
                pat: Box::new(Pat::Ident(PatIdent {
                    attrs: vec![],
                    by_ref: None,
                    mutability: None,
                    ident: Ident::new(
                        (param.name.clone() + "_diplomat_len").as_str(),
                        Span::call_site(),
                    ),
                    subpat: None,
                })),
                colon_token: syn::token::Colon(Span::call_site()),
                ty: Box::new(
                    parse2(quote! {
                        usize
                    })
                    .unwrap(),
                ),
            }));
        }
        o => {
            expanded_params.push(FnArg::Typed(PatType {
                attrs: vec![],
                pat: Box::new(Pat::Ident(PatIdent {
                    attrs: vec![],
                    by_ref: None,
                    mutability: None,
                    ident: Ident::new(param.name.as_str(), Span::call_site()),
                    subpat: None,
                })),
                colon_token: syn::token::Colon(Span::call_site()),
                ty: Box::new(o.to_syn()),
            }));
        }
    }
}

fn gen_params_invocation(param: &ast::Param, expanded_params: &mut Vec<Expr>) {
    match &param.ty {
        ast::TypeName::StrReference => {
            let data_ident = Ident::new(
                (param.name.clone() + "_diplomat_data").as_str(),
                Span::call_site(),
            );
            let len_ident = Ident::new(
                (param.name.clone() + "_diplomat_len").as_str(),
                Span::call_site(),
            );
            // TODO(shadaj): don't just unwrap? or should we assume that the other side gives us a good value?
            expanded_params.push(parse2(quote! {
                unsafe {
                    std::str::from_utf8(std::slice::from_raw_parts(#data_ident, #len_ident)).unwrap()
                }
            }).unwrap());
        }
        _ => {
            expanded_params.push(Expr::Path(ExprPath {
                attrs: vec![],
                qself: None,
                path: Ident::new(param.name.as_str(), Span::call_site()).into(),
            }));
        }
    }
}

fn gen_custom_type_method(strct: &ast::CustomType, m: &ast::Method) -> Item {
    let self_ident = Ident::new(strct.name().as_str(), Span::call_site());
    let method_ident = Ident::new(m.name.as_str(), Span::call_site());
    let extern_ident = Ident::new(m.full_path_name.as_str(), Span::call_site());

    let mut all_params = vec![];
    m.params.iter().for_each(|p| {
        gen_params_at_boundary(p, &mut all_params);
    });

    let mut all_params_invocation = vec![];
    m.params.iter().for_each(|p| {
        gen_params_invocation(p, &mut all_params_invocation);
    });

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
                ty: Box::new(self_param.ty.to_syn()),
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
                extern "C" fn #extern_ident(#(#all_params),*) {
                    #method_invocation(#(#all_params_invocation),*);
                }
            })
            .unwrap(),
        ),
        Some(return_typ) => {
            let return_typ_syn = return_typ.to_syn();

            Item::Fn(
                syn::parse2(quote! {
                    #[no_mangle]
                    extern "C" fn #extern_ident(#(#all_params),*) -> #return_typ_syn {
                        #method_invocation(#(#all_params_invocation),*)
                    }
                })
                .unwrap(),
            )
        }
    }
}

fn gen_bridge(input: ItemMod) -> ItemMod {
    let module = ast::Module::from_syn(&input, true);
    let (brace, mut new_contents) = input.content.unwrap();

    new_contents.iter_mut().for_each(|c| {
        if let Item::Struct(s) = c {
            if !s.attrs.iter().any(|a| {
                let string_path = a.path.to_token_stream().to_string();
                string_path == "repr" || string_path == "diplomat :: opaque"
            }) {
                *s = syn::parse2(quote! {
                    #[repr(C)]
                    #s
                })
                .unwrap();
            }
        }
    });

    for custom_type in module.declared_types.values() {
        custom_type
            .methods()
            .iter()
            .for_each(|m| new_contents.push(gen_custom_type_method(custom_type, m)));

        let destroy_ident = Ident::new(
            (custom_type.name().to_string() + "_destroy").as_str(),
            Span::call_site(),
        );
        let type_ident = Ident::new(custom_type.name(), Span::call_site());

        // for now, body is empty since all we need to do is drop the box
        // TODO(shadaj): change to take a pointer and handle DST boxes appropriately
        new_contents.push(Item::Fn(
            syn::parse2(quote! {
                #[no_mangle]
                extern "C" fn #destroy_ident(this: Box<#type_ident>) {}
            })
            .unwrap(),
        ));
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

/// Mark a module to be exposed through Diplomat-generated FFI.
#[proc_macro_attribute]
pub fn bridge(
    _attr: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let expanded = gen_bridge(parse_macro_input!(input));
    proc_macro::TokenStream::from(expanded.to_token_stream())
}

/// Mark a struct as opaque, which means that its field will not be
/// visible across the FFI boundary and all instances of the struct
/// must be passed as references.
#[proc_macro_attribute]
pub fn opaque(
    _attr: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let strct: ItemStruct = parse_macro_input!(input);
    proc_macro::TokenStream::from(quote! {
        #[repr(transparent)]
        #strct
    })
}

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::{Read, Write};
    use std::process::Command;

    use quote::ToTokens;
    use syn::parse_quote;
    use tempfile::tempdir;

    use super::gen_bridge;

    fn rustfmt_code(code: &str) -> String {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("temp.rs");
        let mut file = File::create(file_path.clone()).unwrap();

        writeln!(file, "{}", code).unwrap();
        drop(file);

        Command::new("rustfmt")
            .arg(file_path.to_str().unwrap())
            .spawn()
            .unwrap()
            .wait()
            .unwrap();

        let mut file = File::open(file_path).unwrap();
        let mut data = String::new();
        file.read_to_string(&mut data).unwrap();
        drop(file);
        dir.close().unwrap();
        data
    }

    #[test]
    fn method_taking_str() {
        insta::assert_display_snapshot!(rustfmt_code(
            &gen_bridge(parse_quote! {
                mod ffi {
                    struct Foo {}

                    impl Foo {
                        fn from_str(s: &str) {
                            todo!()
                        }
                    }
                }
            })
            .to_token_stream()
            .to_string()
        ));
    }
}
