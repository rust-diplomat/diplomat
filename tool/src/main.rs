use std::{env, fs::File, io::Write, path::Path};

use diplomat_core::ast;

mod js;

fn main() -> std::io::Result<()> {
    let lib_file = syn_inline_mod::parse_and_inline_modules(Path::new("./src/main.rs"));
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

    let bindings = js::gen_bindings(&env);
    let args: Vec<String> = env::args().collect();
    let mut out_file = File::create(args[1].clone())?;
    out_file.write_all(bindings.join("\n").as_bytes())?;
    Ok(())
}
