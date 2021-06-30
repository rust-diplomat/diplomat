use std::collections::HashMap;

use quote::ToTokens;
use syn::*;

use crate::meta::types::CustomType;

pub mod meta;

pub fn extract_from_mod(input: &ItemMod) -> HashMap<String, meta::types::CustomType> {
    let mut custom_types_by_name = HashMap::new();
    input
        .content
        .as_ref()
        .unwrap()
        .1
        .iter()
        .for_each(|a| match a {
            Item::Struct(strct) => {
                if strct
                    .attrs
                    .iter()
                    .any(|a| a.path.to_token_stream().to_string() == "diplomat :: opaque")
                {
                    custom_types_by_name.insert(
                        strct.ident.to_string(),
                        meta::types::CustomType::Opaque(strct.ident.to_string(), vec![]),
                    );
                } else {
                    custom_types_by_name.insert(
                        strct.ident.to_string(),
                        meta::types::CustomType::Struct(meta::structs::Struct::from(strct)),
                    );
                }
            }
            Item::Impl(ipl) => {
                assert!(ipl.trait_.is_none());

                let self_typ = match ipl.self_ty.as_ref() {
                    syn::Type::Path(s) => s,
                    _ => panic!("Self type not found"),
                };

                let mut new_methods = ipl
                    .items
                    .iter()
                    .filter_map(|i| match i {
                        ImplItem::Method(m) => Some(meta::methods::Method::from_syn(m, self_typ)),
                        _ => None,
                    })
                    .collect();

                match custom_types_by_name
                    .get_mut(&self_typ.path.get_ident().unwrap().to_string())
                    .unwrap()
                {
                    CustomType::Struct(strct) => {
                        strct.methods.append(&mut new_methods);
                    }
                    CustomType::Opaque(_, methods) => methods.append(&mut new_methods),
                }
            }
            _ => {}
        });

    custom_types_by_name
}

pub fn extract_from_file(file: File) -> HashMap<String, meta::types::CustomType> {
    let mut out = HashMap::new();
    file.items.iter().for_each(|i| {
        if let Item::Mod(item_mod) = i {
            if item_mod
                .attrs
                .iter()
                .any(|a| a.path.to_token_stream().to_string() == "diplomat :: bridge")
            {
                out.extend(extract_from_mod(item_mod));
            }
        }
    });

    out
}
