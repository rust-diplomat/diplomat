use std::path::Path;

use quote::ToTokens;
use syn::Item;

use diplomat_core::extract_from_mod;

fn main() {
    let lib_file = syn_inline_mod::parse_and_inline_modules(&Path::new("./src/main.rs"));

    lib_file.items.iter().for_each(|i| {
        if let Item::Mod(item_mod) = i {
            if item_mod
                .attrs
                .iter()
                .any(|a| a.path.to_token_stream().to_string() == "diplomat :: bridge")
            {
                let all_structs = extract_from_mod(&item_mod);
                dbg!(all_structs);
            }
        }
    });
}
