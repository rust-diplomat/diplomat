use core::panic;
use std::{collections::HashMap, env, fs::File, io::Write, path::Path};

use diplomat_core::ast;

mod c;
mod cpp;
mod js;
mod layout;
mod util;

fn main() -> std::io::Result<()> {
    let lib_file = syn_inline_mod::parse_and_inline_modules(Path::new("./src/lib.rs"));
    let custom_types = ast::File::from(&lib_file);
    let env = custom_types.all_types();

    dbg!(&env);

    let mut opaque_errors = vec![];
    custom_types.check_opaque(&env, &mut opaque_errors);
    if !opaque_errors.is_empty() {
        opaque_errors.iter().for_each(|e| {
            println!(
                "An opaque type crossed the FFI boundary as a value: {:?}",
                e
            )
        });
        panic!();
    }

    dbg!(&env);

    let args: Vec<String> = env::args().collect();
    let target = args[1].as_str();

    let mut out_texts: HashMap<&str, String> = HashMap::new();

    match target {
        "js" => js::gen_bindings(&env, &mut out_texts).unwrap(),
        "c" => c::gen_bindings(&env, &mut out_texts).unwrap(),
        "cpp" => cpp::gen_bindings(&env, &mut out_texts).unwrap(),
        o => panic!("Unknown target: {}", o),
    }

    let out_folder_path = Path::new(args[2].as_str());
    for (subpath, text) in out_texts {
        let mut out_file = File::create(out_folder_path.join(subpath))?;
        out_file.write_all(text.as_bytes())?;
    }

    if args.len() > 3 {
        let mut docs_text = String::new();

        match target {
            "js" => js::docs::gen_docs(&env, &mut docs_text).unwrap(),
            "c" => todo!("Docs generation for C"),
            o => panic!("Unknown target: {}", o),
        }

        let mut out_docs = File::create(args[3].clone())?;
        out_docs.write_all(docs_text.as_bytes())?;
    }

    Ok(())
}
