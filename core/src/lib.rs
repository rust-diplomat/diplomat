use quote::ToTokens;
use syn::*;

pub mod meta;

pub fn extract_from_mod(input: &ItemMod) -> Vec<meta::Struct> {
    input
        .content
        .as_ref()
        .unwrap()
        .1
        .iter()
        .filter_map(|a| match a {
            Item::Impl(ipl) => {
                assert!(ipl.trait_.is_none());

                let self_typ = match ipl.self_ty.as_ref() {
                    syn::Type::Path(s) => s,
                    _ => panic!("Self type not found"),
                };

                Some(meta::Struct {
                    name: self_typ.path.get_ident().unwrap().to_string(),
                    methods: ipl
                        .items
                        .iter()
                        .filter_map(|i| match i {
                            ImplItem::Method(m) => Some(meta::Method::from_syn(m, self_typ)),
                            _ => None,
                        })
                        .collect(),
                })
            }
            _ => None,
        })
        .collect()
}

pub fn extract_from_file(file: File) -> Vec<meta::Struct> {
    let mut out = vec![];
    file.items.iter().for_each(|i| {
        if let Item::Mod(item_mod) = i {
            if item_mod
                .attrs
                .iter()
                .any(|a| a.path.to_token_stream().to_string() == "diplomat :: bridge")
            {
                out.append(&mut extract_from_mod(item_mod));
            }
        }
    });

    out
}
