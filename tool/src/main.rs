use std::{collections::HashMap, path::Path};

use diplomat_core::{extract_from_file, meta};

fn gen_js(strcts: HashMap<String, meta::types::CustomType>) {
    let mut out = vec![];
    for custom_type in strcts.values() {
        out.push(format!("export class {} {{", custom_type.name()));
        for method in custom_type.methods().iter() {
            if method.self_param.is_some() {
                let all_params = method
                    .params
                    .iter()
                    .map(|p| p.name.clone())
                    .collect::<Vec<String>>()
                    .join(", ");
                out.push(format!("{}({}) {{", method.name, &all_params));
                if method.return_type.is_some() {
                    out.push(format!(
                        "return wasm.{}(this.underlying, {});",
                        method.full_path_name, all_params
                    ));
                } else {
                    out.push(format!(
                        "wasm.{}(this.underlying, {});",
                        method.full_path_name, all_params
                    ));
                }
                out.push("}".to_string());
            } else {
            }
        }
        out.push("}".to_string());
    }

    println!("{}", out.join("\n"));
}

fn main() {
    let lib_file = syn_inline_mod::parse_and_inline_modules(Path::new("./src/main.rs"));
    let custom_types = extract_from_file(lib_file);
    dbg!(&custom_types);
    gen_js(custom_types);
}
