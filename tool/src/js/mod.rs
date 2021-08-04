use std::fmt::Write;
use std::{collections::HashMap, fmt};

use diplomat_core::ast;
use indenter::indented;

use crate::util;

#[cfg(test)]
#[macro_use]
mod test_util;

pub mod docs;

pub mod types;

pub mod structs;
use structs::*;

pub mod conversions;

static RUNTIME_MJS: &str = include_str!("runtime.mjs");

pub fn gen_bindings(
    env: &HashMap<ast::Path, HashMap<String, ast::ModSymbol>>,
    outs: &mut HashMap<String, String>,
) -> fmt::Result {
    let diplomat_runtime_out = outs
        .entry("diplomat-runtime.mjs".to_string())
        .or_insert_with(String::new);
    write!(diplomat_runtime_out, "{}", RUNTIME_MJS)?;

    let out = outs
        .entry("api.mjs".to_string())
        .or_insert_with(String::new);

    writeln!(out, "import wasm from \"./wasm.mjs\"")?;
    writeln!(
        out,
        "import * as diplomatRuntime from \"./diplomat-runtime.mjs\""
    )?;

    writeln!(
        out,
        "const diplomat_alloc_destroy_registry = new FinalizationRegistry(obj => {{"
    )?;
    writeln!(
        indented(out).with_str("  "),
        "wasm.diplomat_free(obj[\"ptr\"], obj[\"size\"]);"
    )?;
    writeln!(out, "}});")?;

    let mut all_types = util::get_all_custom_types(env);
    all_types.sort_by_key(|t| t.1.name());
    for (in_path, custom_type) in all_types {
        writeln!(out)?;
        gen_struct(out, custom_type, in_path, env)?;
    }

    Ok(())
}
