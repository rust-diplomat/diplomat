use core::panic;
use std::{collections::HashMap, env, fs::File, io::Write, path::Path};

use diplomat_core::ast;

mod c;
mod cpp;
mod docs_util;
mod js;
mod layout;
mod util;

fn main() -> std::io::Result<()> {
    let path = Path::new("src/lib.rs");
    if !path.exists() {
        let current_dir = std::env::current_dir().expect("Could not find home directory.");
        eprintln!(
            "Could not find the lib.rs file to process: {:?}",
            Path::new(&current_dir).join(path)
        );
        std::process::exit(1);
    }
    let lib_file = syn_inline_mod::parse_and_inline_modules(path);
    let custom_types = ast::File::from(&lib_file);
    let env = custom_types.all_types();

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

    let args: Vec<String> = env::args().collect();
    let target = args[1].as_str();

    let mut out_texts: HashMap<String, String> = HashMap::new();

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
        let mut docs_out_texts: HashMap<String, String> = HashMap::new();

        match target {
            "js" => js::docs::gen_docs(&env, &mut docs_out_texts).unwrap(),
            "cpp" => cpp::docs::gen_docs(&env, &mut docs_out_texts).unwrap(),
            "c" => todo!("Docs generation for C"),
            o => panic!("Unknown target: {}", o),
        }

        let docs_out_folder_path = Path::new(args[3].as_str());
        for (subpath, text) in docs_out_texts {
            let mut out_file = File::create(docs_out_folder_path.join(subpath))?;
            out_file.write_all(text.as_bytes())?;
        }
    }

    Ok(())
}
