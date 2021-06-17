use proc_macro::{TokenStream};
use quote::{ToTokens, __private::Span, quote};
use syn::*;

fn gen_trait_method(self_type: &TypePath, m: &ImplItemMethod, new_contents: &mut Vec<Item>) {
    let self_ident = self_type.path.get_ident().unwrap();
    let method_ident = &m.sig.ident;
    let extern_ident = Ident::new(format!(
        "{}_{}",
        self_ident.to_string(),
        method_ident.to_string()
    ).as_str(), m.sig.ident.span());

    let mut all_params = m.sig.inputs.iter().map(|a| a.clone()).collect::<Vec<_>>();
    let all_param_names = m.sig.inputs.iter().filter_map(|a| match a {
        FnArg::Typed(PatType { attrs: _, pat, colon_token: _, ty: _ }) => {
            match pat.as_ref() {
                Pat::Ident(ident) => Some(ident.ident.clone()),
                _ => panic!("Unexpected param type")
            }
        },
        _ => None
    }).collect::<Vec<_>>();

    let mut extra_checks = vec![];

    match m.sig.receiver() {
        Some(FnArg::Receiver(rec)) => {
            all_params[0] = FnArg::Typed(PatType {
                attrs: vec![],
                pat: Box::new(Pat::Ident(PatIdent {
                    attrs: vec![],
                    by_ref: None,
                    mutability: None,
                    ident: Ident::new("this", Span::call_site()),
                    subpat: None
                })),
                colon_token: syn::token::Colon(Span::call_site()),
                ty: Box::new(match &rec.reference {
                    Some((and_token, lifetime)) => {
                        Type::Reference(TypeReference {
                            and_token: and_token.clone(),
                            lifetime: lifetime.clone(),
                            mutability: rec.mutability,
                            elem: Box::new(Type::Path(self_type.clone()))
                        })
                    },
                    None => {
                        let self_type_ref = Type::Path(self_type.clone());
                        extra_checks.push(quote! {
                            diplomat_internal_is_sized::<#self_type_ref>()
                        });
                        self_type_ref
                    }
                })
            });

            match &m.sig.output {
                ReturnType::Default => {
                    new_contents.push(Item::Fn(syn::parse2(quote! {
                        #[no_mangle]
                        pub extern "C" fn #extern_ident(#(#all_params),*) {
                            #(#extra_checks)*
                            this.#method_ident(#(#all_param_names),*);
                        }
                    }).unwrap()));
                },
                ReturnType::Type(_, return_typ) => {
                    new_contents.push(Item::Fn(syn::parse2(quote! {
                        #[no_mangle]
                        pub extern "C" fn #extern_ident(#(#all_params),*) -> #return_typ {
                            #(#extra_checks)*
                            this.#method_ident(#(#all_param_names),*)
                        }
                    }).unwrap()));
                }
            }
        },
        Some(_) => panic!(),
        None => {
            match &m.sig.output {
                ReturnType::Default => {
                    new_contents.push(Item::Fn(syn::parse2(quote! {
                        #[no_mangle]
                        pub extern "C" fn #extern_ident(#(#all_params),*) {
                            #(#extra_checks)*
                            #(self_ident)::#method_ident(#(#all_param_names),*);
                        }
                    }).unwrap()));
                },
                ReturnType::Type(_, return_typ) => {
                    new_contents.push(Item::Fn(syn::parse2(quote! {
                        #[no_mangle]
                        pub extern "C" fn #extern_ident(#(#all_params),*) -> #return_typ {
                            #(#extra_checks)*
                            #self_ident::#method_ident(#(#all_param_names),*)
                        }
                    }).unwrap()));
                }
            }
        }
    }
}

#[proc_macro_attribute]
pub fn bridge(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemMod);
    let (brace, contents) = input.content.unwrap();

    let mut new_contents = vec![];
    new_contents.push(Item::Fn(syn::parse2(quote! {
        fn diplomat_internal_is_sized<T: Sized>() {}
    }).unwrap()));

    contents.iter().for_each(|a| match a {
        Item::Struct(str) => {
            new_contents.push(Item::Struct(syn::parse2(quote! {
                #[repr(C)]
                #str
            }).unwrap()));
        },
        Item::Impl(ipl) => {
            let self_typ = match ipl.self_ty.as_ref() {
                Type::Path(s) => s,
                _ => panic!("Self type not found")
            };
            new_contents.push(Item::Impl(ipl.clone()));
            ipl.items.iter().for_each(|i| match i {
                ImplItem::Method(m) => {
                    gen_trait_method(self_typ, m, &mut new_contents);
                },
                _ => {}
            });
            assert!(ipl.trait_.is_none());
        },
        o => new_contents.push(o.clone())
    });

    let expanded = ItemMod {
        attrs: input.attrs,
        vis: input.vis,
        mod_token: input.mod_token,
        ident: input.ident,
        content: Some((brace, new_contents)),
        semi: input.semi
    };

    TokenStream::from(expanded.to_token_stream())
}
