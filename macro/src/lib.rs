use proc_macro2::Span;
use quote::{quote, ToTokens};
use syn::*;

use diplomat_core::ast::{self, StdlibOrDiplomat};

mod enum_convert;
mod transparent_convert;

fn cfgs_to_stream(attrs: &[Attribute]) -> proc_macro2::TokenStream {
    attrs
        .iter()
        .fold(quote!(), |prev, attr| quote!(#prev #attr))
}

fn param_ty(param_ty: &ast::TypeName) -> syn::Type {
    match &param_ty {
        ast::TypeName::StrReference(lt @ Some(_lt), encoding, _) => {
            // At the param boundary we MUST use FFI-safe diplomat slice types,
            // not Rust stdlib types (which are not FFI-safe and must be converted)
            encoding.get_diplomat_slice_type(lt)
        }
        ast::TypeName::StrReference(None, encoding, _) => encoding.get_diplomat_slice_type(&None),
        ast::TypeName::StrSlice(encoding, _) => {
            // At the param boundary we MUST use FFI-safe diplomat slice types,
            // not Rust stdlib types (which are not FFI-safe and must be converted)
            let inner = encoding.get_diplomat_slice_type(&Some(ast::Lifetime::Anonymous));
            syn::parse_quote_spanned!(Span::call_site() => diplomat_runtime::DiplomatSlice<#inner>)
        }
        ast::TypeName::PrimitiveSlice(ltmt, prim, _) => {
            // At the param boundary we MUST use FFI-safe diplomat slice types,
            // not Rust stdlib types (which are not FFI-safe and must be converted)
            prim.get_diplomat_slice_type(ltmt)
        }
        ast::TypeName::Option(..) if !param_ty.is_ffi_safe() => {
            param_ty.ffi_safe_version().to_syn()
        }
        _ => param_ty.to_syn(),
    }
}

fn param_conversion(
    name: &ast::Ident,
    param_type: &ast::TypeName,
    cast_to: Option<&syn::Type>,
) -> Option<proc_macro2::TokenStream> {
    match &param_type {
        // conversion only needed for slices that are specified as Rust types rather than diplomat_runtime types
        ast::TypeName::StrReference(.., StdlibOrDiplomat::Stdlib)
        | ast::TypeName::StrSlice(.., StdlibOrDiplomat::Stdlib)
        | ast::TypeName::PrimitiveSlice(.., StdlibOrDiplomat::Stdlib)
        | ast::TypeName::Result(..) => Some(if let Some(cast_to) = cast_to {
            quote!(let #name: #cast_to = #name.into();)
        } else {
            quote!(let #name = #name.into();)
        }),
        // Convert Option<struct/enum/primitive> and DiplomatOption<opaque>
        // simplify the check by just checking is_ffi_safe()
        ast::TypeName::Option(..) if !param_type.is_ffi_safe() => {
            Some(quote!(let #name = #name.into();))
        }
        ast::TypeName::Function(in_types, out_type) => {
            let cb_wrap_ident = &name;
            let mut cb_param_list = vec![];
            let mut cb_params_and_types_list = vec![];
            let mut cb_arg_type_list = vec![];
            let mut all_params_conversion = vec![];
            for (index, in_ty) in in_types.iter().enumerate() {
                let param_ident_str = format!("arg{}", index);
                let orig_type = in_ty.to_syn();
                let param_converted_type = param_ty(in_ty);
                if let Some(conversion) = param_conversion(
                    &ast::Ident::from(param_ident_str.clone()),
                    in_ty,
                    Some(&param_converted_type),
                ) {
                    all_params_conversion.push(conversion);
                }
                let param_ident = Ident::new(&param_ident_str, Span::call_site());
                cb_arg_type_list.push(param_converted_type);
                cb_params_and_types_list.push(quote!(#param_ident: #orig_type));
                cb_param_list.push(param_ident);
            }
            let cb_ret_type = out_type.to_syn();

            let tokens = quote! {
                let #cb_wrap_ident = move | #(#cb_params_and_types_list,)* | unsafe {
                    #(#all_params_conversion)*
                    std::mem::transmute::<unsafe extern "C" fn (*const c_void, ...) -> #cb_ret_type, unsafe extern "C" fn (*const c_void, #(#cb_arg_type_list,)*) -> #cb_ret_type>
                        (#cb_wrap_ident.run_callback)(#cb_wrap_ident.data, #(#cb_param_list,)*)
                };
            };
            Some(parse2(tokens).unwrap())
        }
        _ => None,
    }
}

fn gen_custom_vtable(custom_trait: &ast::Trait, custom_trait_vtable_type: &Ident) -> Item {
    let mut method_sigs: Vec<proc_macro2::TokenStream> = vec![];
    method_sigs.push(quote!(
        pub destructor: Option<unsafe extern "C" fn(*const c_void)>,
        pub size: usize,
        pub alignment: usize,
    ));
    for m in &custom_trait.methods {
        // TODO check that this is the right conversion, it might be the wrong direction
        let mut param_types: Vec<syn::Type> = m.params.iter().map(|p| param_ty(&p.ty)).collect();
        let method_name = Ident::new(&format!("run_{}_callback", m.name), Span::call_site());
        let return_tokens = match &m.output_type {
            Some(ret_ty) => {
                let conv_ret_ty = ret_ty.to_syn();
                quote!( -> #conv_ret_ty)
            }
            None => {
                quote! {}
            }
        };
        param_types.insert(0, syn::parse_quote!(*const c_void));
        method_sigs.push(quote!(
            pub #method_name: unsafe extern "C" fn (#(#param_types),*) #return_tokens,

        ));
    }
    syn::parse_quote!(
        #[repr(C)]
        pub struct #custom_trait_vtable_type {
            #(#method_sigs)*
        }
    )
}

fn gen_custom_trait_impl(custom_trait: &ast::Trait, custom_trait_struct_name: &Ident) -> Item {
    let mut methods: Vec<Item> = vec![];
    for m in &custom_trait.methods {
        let param_names: Vec<proc_macro2::TokenStream> = m
            .params
            .iter()
            .map(|p| {
                let p_name = &p.name;
                quote! {, #p_name}
            })
            .collect();
        let mut all_params_conversion = vec![];
        let mut param_names_and_types: Vec<proc_macro2::TokenStream> = m
            .params
            .iter()
            .map(|p| {
                let orig_type = p.ty.to_syn();
                let p_ty = param_ty(&p.ty);
                if let Some(conversion) = param_conversion(&p.name.clone(), &p.ty, Some(&p_ty)) {
                    all_params_conversion.push(conversion);
                }
                let p_name = &p.name;
                quote!(#p_name : #orig_type)
            })
            .collect();
        let method_name = &m.name;
        let (return_tokens, end_token) = match &m.output_type {
            Some(ret_ty) => {
                let conv_ret_ty = ret_ty.to_syn();
                (quote!( -> #conv_ret_ty), quote! {})
            }
            None => (quote! {}, quote! {;}),
        };
        if let Some(self_param) = &m.self_param {
            let mut self_modifier = quote! {};
            if let Some((lifetime, mutability)) = &self_param.reference {
                let lifetime_mod = if *lifetime == ast::Lifetime::Anonymous {
                    quote! { & }
                } else {
                    let prime = "'".to_string();
                    let lifetime = lifetime.to_syn();
                    quote! { & #prime #lifetime }
                };
                let mutability_mod = if *mutability == ast::Mutability::Mutable {
                    quote! {mut}
                } else {
                    quote! {}
                };
                self_modifier = quote! { #lifetime_mod #mutability_mod }
            }
            param_names_and_types.insert(0, quote!(#self_modifier self));
        }

        let lifetimes = {
            let lifetime_env = &m.lifetimes;
            if lifetime_env.is_empty() {
                quote! {}
            } else {
                quote! { <#lifetime_env> }
            }
        };
        let runner_method_name =
            Ident::new(&format!("run_{}_callback", method_name), Span::call_site());
        methods.push(syn::Item::Fn(syn::parse_quote!(
            fn #method_name #lifetimes (#(#param_names_and_types),*) #return_tokens {
                unsafe {
                    #(#all_params_conversion)*
                    ((self.vtable).#runner_method_name)(self.data #(#param_names)*)#end_token
                }
            }

        )));
    }
    let trait_name = &custom_trait.name;
    syn::parse_quote!(
        impl #trait_name for #custom_trait_struct_name {
            #(#methods)*
        }
    )
}

fn gen_custom_type_method(strct: &ast::CustomType, m: &ast::Method) -> Item {
    let self_ident = Ident::new(strct.name().as_str(), Span::call_site());
    let method_ident = Ident::new(m.name.as_str(), Span::call_site());
    let extern_ident = Ident::new(m.abi_name.as_str(), Span::call_site());

    let mut all_params = vec![];

    let mut all_params_conversion = vec![];
    let mut all_params_names = vec![];
    m.params.iter().for_each(|p| {
        let ty = param_ty(&p.ty);
        let name = &p.name;
        all_params_names.push(name);
        all_params.push(syn::parse_quote!(#name: #ty));
        if let Some(conversion) = param_conversion(&p.name, &p.ty, None) {
            all_params_conversion.push(conversion);
        }
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
                ty: Box::new(self_param.to_typename().to_syn()),
            }),
        );
    }

    let lifetimes = {
        let lifetime_env = &m.lifetime_env;
        if lifetime_env.is_empty() {
            quote! {}
        } else {
            quote! { <#lifetime_env> }
        }
    };

    let method_invocation = if m.self_param.is_some() {
        quote! { #this_ident.#method_ident }
    } else {
        quote! { #self_ident::#method_ident }
    };

    let (return_tokens, maybe_into) = if let Some(return_type) = &m.return_type {
        if let ast::TypeName::Result(ok, err, StdlibOrDiplomat::Stdlib) = return_type {
            let ok = ok.to_syn();
            let err = err.to_syn();
            (
                quote! { -> diplomat_runtime::DiplomatResult<#ok, #err> },
                quote! { .into() },
            )
        } else if let ast::TypeName::StrReference(_, _, StdlibOrDiplomat::Stdlib)
        | ast::TypeName::StrSlice(.., StdlibOrDiplomat::Stdlib)
        | ast::TypeName::PrimitiveSlice(_, _, StdlibOrDiplomat::Stdlib) = return_type
        {
            let return_type_syn = return_type.ffi_safe_version().to_syn();
            (quote! { -> #return_type_syn }, quote! { .into() })
        } else if let ast::TypeName::Ordering = return_type {
            let return_type_syn = return_type.to_syn();
            (quote! { -> #return_type_syn }, quote! { as i8 })
        } else if let ast::TypeName::Option(ty, is_std_option) = return_type {
            match ty.as_ref() {
                // pass by reference, Option becomes null
                ast::TypeName::Box(..) | ast::TypeName::Reference(..) => {
                    let return_type_syn = return_type.to_syn();
                    let conversion = if *is_std_option == StdlibOrDiplomat::Stdlib {
                        quote! {}
                    } else {
                        quote! {.into()}
                    };
                    (quote! { -> #return_type_syn }, conversion)
                }
                // anything else goes through DiplomatResult
                _ => {
                    let ty = ty.to_syn();
                    let conversion = if *is_std_option == StdlibOrDiplomat::Stdlib {
                        quote! { .ok_or(()).into() }
                    } else {
                        quote! {}
                    };
                    (
                        quote! { -> diplomat_runtime::DiplomatResult<#ty, ()> },
                        conversion,
                    )
                }
            }
        } else {
            let return_type_syn = return_type.to_syn();
            (quote! { -> #return_type_syn }, quote! {})
        }
    } else {
        (quote! {}, quote! {})
    };

    let write_flushes = m
        .params
        .iter()
        .filter(|p| p.is_write())
        .map(|p| {
            let p = &p.name;
            quote! { #p.flush(); }
        })
        .collect::<Vec<_>>();

    let cfg = cfgs_to_stream(&m.attrs.cfg);
    if write_flushes.is_empty() {
        Item::Fn(syn::parse_quote! {
            #[no_mangle]
            #cfg
            extern "C" fn #extern_ident #lifetimes(#(#all_params),*) #return_tokens {
                #(#all_params_conversion)*
                #method_invocation(#(#all_params_names),*) #maybe_into
            }
        })
    } else {
        Item::Fn(syn::parse_quote! {
            #[no_mangle]
            #cfg
            extern "C" fn #extern_ident #lifetimes(#(#all_params),*) #return_tokens {
                #(#all_params_conversion)*
                let ret = #method_invocation(#(#all_params_names),*);
                #(#write_flushes)*
                ret #maybe_into
            }
        })
    }
}

struct AttributeInfo {
    repr: bool,
    opaque: bool,
    #[allow(unused)]
    is_out: bool,
}

impl AttributeInfo {
    fn extract(attrs: &mut Vec<Attribute>) -> Self {
        let mut repr = false;
        let mut opaque = false;
        let mut is_out = false;
        attrs.retain(|attr| {
            let ident = &attr.path().segments.iter().next().unwrap().ident;
            if ident == "repr" {
                repr = true;
                // don't actually extract repr attrs, just detect them
                return true;
            } else if ident == "diplomat" {
                if attr.path().segments.len() == 2 {
                    let seg = &attr.path().segments.iter().nth(1).unwrap().ident;
                    if seg == "opaque" {
                        opaque = true;
                        return false;
                    } else if seg == "out" {
                        is_out = true;
                        return false;
                    } else if seg == "rust_link"
                        || seg == "out"
                        || seg == "attr"
                        || seg == "abi_rename"
                        || seg == "demo"
                    {
                        // diplomat-tool reads these, not diplomat::bridge.
                        // throw them away so rustc doesn't complain about unknown attributes
                        return false;
                    } else if seg == "enum_convert" || seg == "transparent_convert" {
                        // diplomat::bridge doesn't read this, but it's handled separately
                        // as an attribute
                        return true;
                    } else {
                        panic!("Only #[diplomat::opaque] and #[diplomat::rust_link] are supported: {:?}", seg)
                    }
                } else {
                    panic!("#[diplomat::foo] attrs have a single-segment path name")
                }
            }
            true
        });

        Self {
            repr,
            opaque,
            is_out,
        }
    }
}

fn gen_bridge(mut input: ItemMod) -> ItemMod {
    let module = ast::Module::from_syn(&input, true);
    // Clean out any diplomat attributes so Rust doesn't get mad
    let _attrs = AttributeInfo::extract(&mut input.attrs);
    let (brace, mut new_contents) = input.content.unwrap();

    new_contents.push(parse2(quote! { use diplomat_runtime::*; }).unwrap());
    new_contents.push(parse2(quote! { use core::ffi::c_void; }).unwrap());

    new_contents.iter_mut().for_each(|c| match c {
        Item::Struct(s) => {
            let info = AttributeInfo::extract(&mut s.attrs);

            if !info.opaque {
                // This is validated by HIR, but it's also nice to validate it in the macro so that there
                // are early error messages
                for field in s.fields.iter_mut() {
                    let _attrs = AttributeInfo::extract(&mut field.attrs);
                    let ty = ast::TypeName::from_syn(&field.ty, None);
                    if !ty.is_ffi_safe() {
                        let ffisafe = ty.ffi_safe_version();
                        panic!(
                            "Found non-FFI safe type inside struct: {}, try {}",
                            ty, ffisafe
                        );
                    }
                }
            }

            // Normal opaque types don't need repr(transparent) because the inner type is
            // never referenced. #[diplomat::transparent_convert] handles adding repr(transparent)
            // on its own
            if !info.opaque {
                let repr = if !info.repr {
                    quote!(#[repr(C)])
                } else {
                    quote!()
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
            for v in &mut e.variants {
                let info = AttributeInfo::extract(&mut v.attrs);
                if info.opaque {
                    panic!("#[diplomat::opaque] not allowed on enum variants");
                }
            }
            *e = syn::parse_quote! {
                #[repr(C)]
                #[derive(Clone, Copy)]
                #e
            };
        }

        Item::Impl(i) => {
            for item in &mut i.items {
                if let syn::ImplItem::Fn(ref mut m) = *item {
                    let info = AttributeInfo::extract(&mut m.attrs);
                    if info.opaque {
                        panic!("#[diplomat::opaque] not allowed on methods")
                    }
                    for i in m.sig.inputs.iter_mut() {
                        let _attrs = match i {
                            syn::FnArg::Receiver(s) => AttributeInfo::extract(&mut s.attrs),
                            syn::FnArg::Typed(t) => AttributeInfo::extract(&mut t.attrs),
                        };
                    }
                }
            }
        }
        _ => (),
    });

    for custom_type in module.declared_types.values() {
        custom_type.methods().iter().for_each(|m| {
            let gen_m = gen_custom_type_method(custom_type, m);
            new_contents.push(gen_m);
        });

        if let ast::CustomType::Opaque(opaque) = custom_type {
            let destroy_ident = Ident::new(opaque.dtor_abi_name.as_str(), Span::call_site());

            let type_ident = custom_type.name().to_syn();

            let (lifetime_defs, lifetimes) = if let Some(lifetime_env) = custom_type.lifetimes() {
                (
                    quote! { <#lifetime_env> },
                    lifetime_env.lifetimes_to_tokens(),
                )
            } else {
                (quote! {}, quote! {})
            };

            let cfg = cfgs_to_stream(&custom_type.attrs().cfg);

            // for now, body is empty since all we need to do is drop the box
            // TODO(#13): change to take a `*mut` and handle DST boxes appropriately
            new_contents.push(Item::Fn(syn::parse_quote! {
                #[no_mangle]
                #cfg
                extern "C" fn #destroy_ident #lifetime_defs(this: Box<#type_ident #lifetimes>) {}
            }));
        }
    }

    for custom_trait in module.declared_traits.values() {
        let custom_trait_name = Ident::new(
            &format!("DiplomatTraitStruct_{}", custom_trait.name),
            Span::call_site(),
        );
        let custom_trait_vtable_type =
            Ident::new(&format!("{}_VTable", custom_trait.name), Span::call_site());

        // vtable
        new_contents.push(gen_custom_vtable(custom_trait, &custom_trait_vtable_type));

        // trait struct
        new_contents.push(syn::parse_quote! {
            #[repr(C)]
            pub struct #custom_trait_name {
                data: *const c_void,
                pub vtable: #custom_trait_vtable_type,
            }
        });

        // trait struct wrapper for all methods
        new_contents.push(gen_custom_trait_impl(custom_trait, &custom_trait_name));

        // destructor
        new_contents.push(syn::parse_quote! {
            impl Drop for #custom_trait_name {
                fn drop(&mut self) {
                    if let Some(destructor) = self.vtable.destructor {
                        unsafe {
                            (destructor)(self.data);
                        }
                    }
                }
            }
        })
    }

    ItemMod {
        attrs: input.attrs,
        vis: input.vis,
        mod_token: input.mod_token,
        ident: input.ident,
        content: Some((brace, new_contents)),
        semi: input.semi,
        unsafety: None,
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

/// Generate From and Into implementations for a Diplomat enum
///
/// This is invoked as `#[diplomat::enum_convert(OtherEnumName)]`
/// on a Diplomat enum. It will assume the other enum has exactly the same variants
/// and generate From and Into implementations using those. In case that enum is `#[non_exhaustive]`,
/// you may use `#[diplomat::enum_convert(OtherEnumName, needs_wildcard)]` to generate a panicky wildcard
/// branch. It is up to the library author to ensure the enums are kept in sync. You may use the `#[non_exhaustive_omitted_patterns]`
/// lint to enforce this.
#[proc_macro_attribute]
pub fn enum_convert(
    attr: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    // proc macros handle compile errors by using special error tokens.
    // In case of an error, we don't want the original code to go away too
    // (otherwise that will cause more errors) so we hold on to it and we tack it in
    // with no modifications below
    let input_cached: proc_macro2::TokenStream = input.clone().into();
    let expanded =
        enum_convert::gen_enum_convert(parse_macro_input!(attr), parse_macro_input!(input));

    let full = quote! {
        #expanded
        #input_cached
    };
    proc_macro::TokenStream::from(full.to_token_stream())
}

/// Generate conversions from inner types for opaque Diplomat types with a single field
///
/// This is invoked as `#[diplomat::transparent_convert]`
/// on an opaque Diplomat type. It will add `#[repr(transparent)]` and implement `pub(crate) fn transparent_convert()`
/// which allows constructing an `&Self` from a reference to the inner field.
#[proc_macro_attribute]
pub fn transparent_convert(
    _attr: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    // proc macros handle compile errors by using special error tokens.
    // In case of an error, we don't want the original code to go away too
    // (otherwise that will cause more errors) so we hold on to it and we tack it in
    // with no modifications below
    let input_cached: proc_macro2::TokenStream = input.clone().into();
    let expanded = transparent_convert::gen_transparent_convert(parse_macro_input!(input));

    let full = quote! {
        #expanded
        #[repr(transparent)]
        #input_cached
    };
    proc_macro::TokenStream::from(full.to_token_stream())
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

        writeln!(file, "{code}").unwrap();
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
        insta::assert_snapshot!(rustfmt_code(
            &gen_bridge(parse_quote! {
                mod ffi {
                    struct Foo {}

                    impl Foo {
                        pub fn from_str(s: &DiplomatStr) {
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
    fn slices() {
        insta::assert_snapshot!(rustfmt_code(
            &gen_bridge(parse_quote! {
                mod ffi {
                    use diplomat_runtime::{DiplomatStr, DiplomatStr16, DiplomatByte, DiplomatOwnedSlice,
                                           DiplomatOwnedStr16Slice, DiplomatOwnedStrSlice, DiplomatOwnedUTF8StrSlice,
                                           DiplomatSlice, DiplomatSliceMut, DiplomatStr16Slice, DiplomatStrSlice, DiplomatUtf8StrSlice};
                    struct Foo<'a> {
                        a: DiplomatSlice<'a, u8>,
                        b: DiplomatSlice<'a, u16>,
                        c: DiplomatUtf8StrSlice<'a>,
                        d: DiplomatStrSlice<'a>,
                        e: DiplomatStr16Slice<'a>,
                        f: DiplomatSlice<'a, DiplomatByte>,
                    }

                    impl Foo {
                        pub fn make(a: &'a [u8], b: &'a [u16], c: &'a str, d: &'a DiplomatStr, e: &'a DiplomatStr16, f: &'a [DiplomatByte]) -> Self {
                            Foo {
                                a, b, c, d, e, f,
                            }
                        }
                        pub fn make_runtime_types(a: DiplomatSlice<'a, u8>, b: DiplomatSlice<'a, u16>, c: DiplomatUtf8StrSlice<'a>, d: DiplomatStrSlice<'a>, e: DiplomatStr16Slice<'a>, f: DiplomatSlice<'a, DiplomatByte>) -> Self {
                            Foo {
                                a: a.into(),
                                b: b.into(),
                                c: c.into(),
                                d: d.into(),
                                e: e.into(),
                                f: f.into(),
                            }
                        }
                        pub fn boxes(a: Box<[u8]>, b: Box<[u16]>, c: Box<str>, d: Box<DiplomatStr>, e: Box<DiplomatStr16>, f: Box<[DiplomatByte]>) -> Self {
                            unimplemented!()
                        }
                        pub fn boxes_runtime_types(a: DiplomatOwnedSlice<u8>, b: DiplomatOwnedSlice<u16>, c: DiplomatOwnedUTF8StrSlice, d: DiplomatOwnedStrSlice, e: DiplomatOwnedStr16Slice, f: DiplomatOwnedSlice<DiplomatByte>) -> Self {
                            unimplemented!()
                        }
                        pub fn a(self) -> &[u8] {
                            self.a
                        }
                        pub fn b(self) -> &[u16] {
                            self.b
                        }
                        pub fn c(self) -> &str {
                            self.c
                        }
                        pub fn d(self) -> &DiplomatStr {
                            self.d
                        }
                        pub fn e(self) -> &DiplomatStr16 {
                            self.e
                        }
                        pub fn f(self) -> &[DiplomatByte] {
                            self.f
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
        insta::assert_snapshot!(rustfmt_code(
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
        insta::assert_snapshot!(rustfmt_code(
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
    fn method_taking_owned_slice() {
        insta::assert_snapshot!(rustfmt_code(
            &gen_bridge(parse_quote! {
                mod ffi {
                    struct Foo {}

                    impl Foo {
                        pub fn fill_slice(s: Box<[u16]>) {
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
    fn method_taking_owned_str() {
        insta::assert_snapshot!(rustfmt_code(
            &gen_bridge(parse_quote! {
                mod ffi {
                    struct Foo {}

                    impl Foo {
                        pub fn something_with_str(s: Box<str>) {
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
        insta::assert_snapshot!(rustfmt_code(
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
    fn mod_with_write_result() {
        insta::assert_snapshot!(rustfmt_code(
            &gen_bridge(parse_quote! {
                mod ffi {
                    struct Foo {}

                    impl Foo {
                        pub fn to_string(&self, to: &mut DiplomatWrite) -> Result<(), ()> {
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
    fn mod_with_rust_result() {
        insta::assert_snapshot!(rustfmt_code(
            &gen_bridge(parse_quote! {
                mod ffi {
                    struct Foo {}

                    impl Foo {
                        pub fn bar(&self) -> Result<(), ()> {
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
        insta::assert_snapshot!(rustfmt_code(
            &gen_bridge(parse_quote! {
                mod ffi {
                    #[diplomat::opaque]
                    struct Foo<'a>(&'a str);

                    #[diplomat::opaque]
                    struct Bar<'b, 'a: 'b>(&'b Foo<'a>);

                    struct Baz<'x, 'y> {
                        foo: &'y Foo<'x>,
                    }

                    impl<'a> Foo<'a> {
                        pub fn new(x: &'a str) -> Box<Foo<'a>> {
                            unimplemented!()
                        }

                        pub fn get_bar<'b>(&'b self) -> Box<Bar<'b, 'a>> {
                            unimplemented!()
                        }

                        pub fn get_baz<'b>(&'b self) -> Baz<'b, 'a> {
                            Bax { foo: self }
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
        insta::assert_snapshot!(rustfmt_code(
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

    #[test]
    fn cfged_method() {
        insta::assert_snapshot!(rustfmt_code(
            &gen_bridge(parse_quote! {
                mod ffi {
                    struct Foo {}

                    impl Foo {
                        #[cfg(feature = "foo")]
                        pub fn bar(s: u8) {
                            unimplemented!()
                        }
                    }
                }
            })
            .to_token_stream()
            .to_string()
        ));

        insta::assert_snapshot!(rustfmt_code(
            &gen_bridge(parse_quote! {
                mod ffi {
                    struct Foo {}

                    #[cfg(feature = "bar")]
                    impl Foo {
                        #[cfg(feature = "foo")]
                        pub fn bar(s: u8) {
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
    fn cfgd_struct() {
        insta::assert_snapshot!(rustfmt_code(
            &gen_bridge(parse_quote! {
                mod ffi {
                    #[diplomat::opaque]
                    #[cfg(feature = "foo")]
                    struct Foo {}
                    #[cfg(feature = "foo")]
                    impl Foo {
                        pub fn bar(s: u8) {
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
    fn callback_arguments() {
        insta::assert_snapshot!(rustfmt_code(
            &gen_bridge(parse_quote! {
                mod ffi {
                    pub struct Wrapper {
                        cant_be_empty: bool,
                    }
                    pub struct TestingStruct {
                        x: i32,
                        y: i32,
                    }
                    impl Wrapper {
                        pub fn test_multi_arg_callback(f: impl Fn(i32) -> i32, x: i32) -> i32 {
                            f(10 + x)
                        }
                        pub fn test_multiarg_void_callback(f: impl Fn(i32, &str)) {
                            f(-10, "hello it's a string\0");
                        }
                        pub fn test_mod_array(g: impl Fn(&[u8])) {
                            let bytes: Vec<u8> = vec![0x11, 0x22];
                            g(bytes.as_slice().into());
                        }
                        pub fn test_no_args(h: impl Fn()) -> i32 {
                            h();
                            -5
                        }
                        pub fn test_cb_with_struct(f: impl Fn(TestingStruct) -> i32) -> i32 {
                            let arg = TestingStruct {
                                x: 1,
                                y: 5,
                            };
                            f(arg)
                        }
                        pub fn test_multiple_cb_args(f: impl Fn() -> i32, g: impl Fn(i32) -> i32) -> i32 {
                            f() + g(5)
                        }
                    }
                }
            })
            .to_token_stream()
            .to_string()
        ));
    }

    #[test]
    fn traits() {
        insta::assert_snapshot!(rustfmt_code(
            &gen_bridge(parse_quote! {
                mod ffi {
                    pub struct TestingStruct {
                        x: i32,
                        y: i32,
                    }

                    pub trait TesterTrait {
                        fn test_trait_fn(&self, x: i32) -> i32;
                        fn test_void_trait_fn(&self);
                        fn test_struct_trait_fn(&self, s: TestingStruct) -> i32;
                        fn test_slice_trait_fn(&self, s: &[u8]) -> i32;
                    }

                    pub struct Wrapper {
                        cant_be_empty: bool,
                    }

                    impl Wrapper {
                        pub fn test_with_trait(t: impl TesterTrait, x: i32) -> i32 {
                            t.test_void_trait_fn();
                            t.test_trait_fn(x)
                        }

                        pub fn test_trait_with_struct(t: impl TesterTrait) -> i32 {
                            let arg = TestingStruct {
                                x: 1,
                                y: 5,
                            };
                            t.test_struct_trait_fn(arg)
                        }
                    }

                }
            })
            .to_token_stream()
            .to_string()
        ));
    }

    #[test]
    fn both_kinds_of_option() {
        insta::assert_snapshot!(rustfmt_code(
            &gen_bridge(parse_quote! {
                mod ffi {
                    use diplomat_runtime::DiplomatOption;
                    #[diplomat::opaque]
                    struct Foo {}
                    struct CustomStruct {
                        num: u8,
                        b: bool,
                        diplo_option: DiplomatOption<u8>,
                    }
                    impl Foo {
                        pub fn diplo_option_u8(x: DiplomatOption<u8>) -> DiplomatOption<u8> {
                            x
                        }
                        pub fn diplo_option_ref(x: DiplomatOption<&Foo>) -> DiplomatOption<&Foo> {
                            x
                        }
                        pub fn diplo_option_box() -> DiplomatOption<Box<Foo>> {
                            x
                        }
                        pub fn diplo_option_struct(x: DiplomatOption<CustomStruct>) -> DiplomatOption<CustomStruct> {
                            x
                        }
                        pub fn option_u8(x: Option<u8>) -> Option<u8> {
                            x
                        }
                        pub fn option_ref(x: Option<&Foo>) -> Option<&Foo> {
                            x
                        }
                        pub fn option_box() -> Option<Box<Foo>> {
                            x
                        }
                        pub fn option_struct(x: Option<CustomStruct>) -> Option<CustomStruct> {
                            x
                        }
                    }
                }
            })
            .to_token_stream()
            .to_string()
        ));
    }
}
