use proc_macro2::Span;
use quote::{quote, ToTokens};
use syn::*;

use diplomat_core::ast;

fn gen_params_at_boundary(param: &ast::Param, expanded_params: &mut Vec<FnArg>) {
    match &param.ty {
        ast::TypeName::StrReference(_, mutability)
        | ast::TypeName::PrimitiveSlice(_, mutability, _) => {
            let data_type = if let ast::TypeName::PrimitiveSlice(_, _, prim) = &param.ty {
                ast::TypeName::Primitive(*prim).to_syn().to_token_stream()
            } else {
                quote! { u8 }
            };
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
                    parse2(if mutability.is_mutable() {
                        quote! { *mut #data_type }
                    } else {
                        quote! { *const #data_type }
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
        ast::TypeName::StrReference(_, mutability)
        | ast::TypeName::PrimitiveSlice(_, mutability, _) => {
            let data_ident = Ident::new(
                (param.name.clone() + "_diplomat_data").as_str(),
                Span::call_site(),
            );
            let len_ident = Ident::new(
                (param.name.clone() + "_diplomat_len").as_str(),
                Span::call_site(),
            );

            let tokens = if let ast::TypeName::PrimitiveSlice(..) = &param.ty {
                match mutability {
                    ast::Mutability::Mutable => quote! {
                        unsafe { core::slice::from_raw_parts_mut(#data_ident, #len_ident) }
                    },
                    ast::Mutability::Immutable => quote! {
                        unsafe { core::slice::from_raw_parts(#data_ident, #len_ident) }
                    },
                }
            } else {
                // TODO(#57): don't just unwrap? or should we assume that the other side gives us a good value?
                match mutability {
                    ast::Mutability::Mutable => quote! {
                        unsafe {
                            core::str::from_utf8_mut(core::slice::from_raw_parts_mut(#data_ident, #len_ident)).unwrap()
                        }
                    },
                    ast::Mutability::Immutable => quote! {
                        unsafe {
                            core::str::from_utf8(core::slice::from_raw_parts(#data_ident, #len_ident)).unwrap()
                        }
                    },
                }
            };
            expanded_params.push(parse2(tokens).unwrap());
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

    let lifetimes = {
        let lifetimes = &m.introduced_lifetimes;
        if lifetimes.is_empty() {
            quote! {}
        } else {
            quote! { <#(#lifetimes),*> }
        }
    };

    let method_invocation = if m.self_param.is_some() {
        quote! { #this_ident.#method_ident }
    } else {
        quote! { #self_ident::#method_ident }
    };

    let return_tokens = if let Some(return_type) = &m.return_type {
        let return_type_syn = return_type.to_syn();
        quote! { -> #return_type_syn }
    } else {
        quote! {}
    };

    Item::Fn(syn::parse_quote! {
        #[no_mangle]
        extern "C" fn #extern_ident#lifetimes(#(#all_params),*) #return_tokens {
            #method_invocation(#(#all_params_invocation),*)
        }
    })
}

struct AttributeInfo {
    repr: bool,
    opaque: bool,
}

impl AttributeInfo {
    fn extract(attrs: &mut Vec<Attribute>) -> Self {
        let mut repr = false;
        let mut opaque = false;
        attrs.retain(|attr| {
            let ident = &attr.path.segments.iter().next().unwrap().ident;
            if ident == "repr" {
                repr = true;
                // don't actually extract repr attrs, just detect them
                return true;
            } else if ident == "diplomat" {
                if attr.path.segments.len() == 2 {
                    let seg = &attr.path.segments.iter().nth(1).unwrap().ident;
                    if seg == "opaque" {
                        opaque = true;
                        return false;
                    } else if seg == "rust_link" {
                        return false;
                    } else {
                        panic!("Only #[diplomat::opaque] and #[diplomat::rust_link] are supported")
                    }
                } else {
                    panic!("#[diplomat::foo] attrs have a single-segment path name")
                }
            }
            true
        });

        Self { repr, opaque }
    }
}

fn gen_bridge(input: ItemMod) -> ItemMod {
    let module = ast::Module::from_syn(&input, true);
    let (brace, mut new_contents) = input.content.unwrap();

    new_contents.iter_mut().for_each(|c| match c {
        Item::Struct(s) => {
            let info = AttributeInfo::extract(&mut s.attrs);
            if info.opaque || !info.repr {
                let repr = if info.opaque {
                    quote!(#[repr(transparent)])
                } else {
                    quote!(#[repr(C)])
                };
                *s = syn::parse_quote! {
                    #repr
                    #s
                }
            }
        }

        Item::Enum(e) => {
            let info = AttributeInfo::extract(&mut e.attrs);
            if info.opaque {
                panic!("#[diplomat::opaque] not allowed on enums")
            }
            *e = syn::parse_quote! {
                #[repr(C)]
                #e
            };
        }

        Item::Impl(i) => {
            for item in &mut i.items {
                if let syn::ImplItem::Method(ref mut m) = *item {
                    let info = AttributeInfo::extract(&mut m.attrs);
                    if info.opaque {
                        panic!("#[diplomat::opaque] not allowed on methods")
                    }
                }
            }
        }
        _ => (),
    });

    for custom_type in module.declared_types.values() {
        custom_type
            .methods()
            .iter()
            .for_each(|m| new_contents.push(gen_custom_type_method(custom_type, m)));

        let destroy_ident = Ident::new(
            format!("{}_destroy", custom_type.name()).as_str(),
            Span::call_site(),
        );

        let type_ident = custom_type.name().to_syn();

        let (lifetime_defs, lifetimes) = if let Some(lifetime_defs) = custom_type.lifetimes() {
            let lifetimes = lifetime_defs.iter().map(|lt| &lt.lifetime);
            (
                quote! { <#(#lifetime_defs),*> }, // with bounds
                quote! { <#(#lifetimes),*> },     // without bounds
            )
        } else {
            (quote! {}, quote! {})
        };

        // for now, body is empty since all we need to do is drop the box
        // TODO(#13): change to take a `*mut` and handle DST boxes appropriately
        new_contents.push(Item::Fn(syn::parse_quote! {
            #[no_mangle]
            extern "C" fn #destroy_ident#lifetime_defs(this: Box<#type_ident#lifetimes>) {}
        }));
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
                        pub fn from_str(s: &str) {
                            unimplemented!()
                        }
                    }
                }
            })
            .to_token_stream()
            .to_string()
        ));
    }

    #[test]
    fn method_taking_mutable_str() {
        insta::assert_display_snapshot!(rustfmt_code(
            &gen_bridge(parse_quote! {
                mod ffi {
                    struct Foo {}

                    impl Foo {
                        pub fn make_uppercase(s: &mut str) {
                            unimplemented!()
                        }
                    }
                }
            })
            .to_token_stream()
            .to_string()
        ));
    }

    #[test]
    fn method_taking_slice() {
        insta::assert_display_snapshot!(rustfmt_code(
            &gen_bridge(parse_quote! {
                mod ffi {
                    struct Foo {}

                    impl Foo {
                        pub fn from_slice(s: &[f64]) {
                            unimplemented!()
                        }
                    }
                }
            })
            .to_token_stream()
            .to_string()
        ));
    }

    #[test]
    fn method_taking_mutable_slice() {
        insta::assert_display_snapshot!(rustfmt_code(
            &gen_bridge(parse_quote! {
                mod ffi {
                    struct Foo {}

                    impl Foo {
                        pub fn fill_slice(s: &mut [f64]) {
                            unimplemented!()
                        }
                    }
                }
            })
            .to_token_stream()
            .to_string()
        ));
    }

    #[test]
    fn mod_with_enum() {
        insta::assert_display_snapshot!(rustfmt_code(
            &gen_bridge(parse_quote! {
                mod ffi {
                    enum Abc {
                        A,
                        B = 123,
                    }

                    impl Abc {
                        pub fn do_something(&self) {
                            unimplemented!()
                        }
                    }
                }
            })
            .to_token_stream()
            .to_string()
        ));
    }

    #[test]
    fn mod_with_writeable_result() {
        insta::assert_display_snapshot!(rustfmt_code(
            &gen_bridge(parse_quote! {
                mod ffi {
                    struct Foo {}

                    impl Foo {
                        pub fn to_string(&self, to: &mut DiplomatWriteable) -> DiplomatResult<(), ()> {
                            unimplemented!()
                        }
                    }
                }
            })
            .to_token_stream()
            .to_string()
        ));
    }

    #[test]
    fn multilevel_borrows() {
        insta::assert_display_snapshot!(rustfmt_code(
            &gen_bridge(parse_quote! {
                mod ffi {
                    #[diplomat::opaque]
                    struct Foo<'a>(&'a str);

                    #[diplomat::opaque]
                    struct Bar<'b, 'a: 'b>(&'b Foo<'a>);

                    impl<'a> Foo<'a> {
                        pub fn new(x: &'a str) -> Box<Foo<'a>> {
                            unimplemented!()
                        }

                        pub fn get_bar<'b>(&'b self) -> Box<Bar<'b, 'a>> {
                            unimplemented!()
                        }
                    }
                }
            })
            .to_token_stream()
            .to_string()
        ));
    }

    #[test]
    fn self_params() {
        insta::assert_display_snapshot!(rustfmt_code(
            &gen_bridge(parse_quote! {
                mod ffi {
                    #[diplomat::opaque]
                    struct RefList<'a> {
                        data: &'a i32,
                        next: Option<Box<Self>>,
                    }

                    impl<'b> RefList<'b> {
                        pub fn extend(&mut self, other: &Self) -> Self {
                            unimplemented!()
                        }
                    }
                }
            })
            .to_token_stream()
            .to_string()
        ));
    }
}
