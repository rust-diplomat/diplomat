use std::path::Path;

fn main() {
    let lib_file = syn_inline_mod::parse_and_inline_modules(&Path::new("./src/main.rs"));

    dbg!(lib_file);
}
