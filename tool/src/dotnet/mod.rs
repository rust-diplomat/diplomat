use std::collections::HashMap;
use std::fmt;
use std::fmt::Write;
use std::fs;
use std::path::PathBuf;

use colored::*;
use diplomat_core::Env;

use self::config::LibraryConfig;
use crate::util::CodeWriter;
use crate::util::SetOfAstTypes;

mod config;
mod conversions;
mod idiomatic;
mod raw;
mod types;
mod util;

static RUNTIME_CS: &str = include_str!("Runtime.cs");

const INDENTATION: &str = "    ";
const SCOPE_OPENING: &str = "{";
const SCOPE_CLOSING: &str = "}";

pub fn gen_bindings(
    env: &Env,
    library_config_path: &Option<PathBuf>,
    docs_url_gen: &diplomat_core::ast::DocsUrlGenerator,
    outs: &mut HashMap<String, String>,
) -> fmt::Result {
    let mut library_config = LibraryConfig::default();
    if let Some(path) = library_config_path {
        // Should be fine, we've already verified the path
        let contents = fs::read_to_string(path).unwrap();
        match toml::from_str(&contents) {
            Ok(config) => library_config = config,
            Err(err) => {
                eprintln!(
                    "{} Unable to parse library configuration file: {path:?}\n{err}",
                    "Error:".red().bold(),
                );
                std::process::exit(1);
            }
        }
    }

    let diplomat_runtime_out = outs
        .entry("DiplomatRuntime.cs".to_owned())
        .or_insert_with(String::new);
    write!(
        diplomat_runtime_out,
        "{}",
        RUNTIME_CS.replace("{{NAMESPACE}}", &library_config.namespace)
    )?;

    let all_types = crate::util::get_all_custom_types(env);
    let mut errors = SetOfAstTypes::default();

    {
        // Raw API generation pass

        let mut results = SetOfAstTypes::default();

        for (in_path, typ) in &all_types {
            let mut out_buf = String::new();
            let mut out = CodeWriter::new(&mut out_buf, INDENTATION, SCOPE_OPENING, SCOPE_CLOSING);
            raw::gen_header(&library_config, &mut out)?;
            raw::gen(
                env,
                &library_config,
                &mut results,
                &mut errors,
                typ,
                in_path,
                docs_url_gen,
                &mut out,
            )?;
            outs.insert(format!("Raw{}.cs", typ.name()), out_buf)
                .and_then::<String, _>(|_| panic!("file created twice: Raw{}.cs", typ.name()));
        }

        for (in_path, typ) in results {
            let result_name = types::gen_type_name_to_string(typ, &in_path, env)?;
            let mut out_buf = String::new();
            let mut out = CodeWriter::new(&mut out_buf, INDENTATION, SCOPE_OPENING, SCOPE_CLOSING);
            raw::gen_header(&library_config, &mut out)?;
            raw::gen_result(typ, &in_path, env, &mut out)?;
            outs.insert(format!("Raw{}.cs", result_name), out_buf)
                .and_then::<String, _>(|_| panic!("file created twice: Raw{}.cs", result_name));
        }
    }

    {
        // Idiomatic API generation pass

        for (in_path, typ) in &all_types {
            let mut out_buf = String::new();
            let mut out = CodeWriter::new(&mut out_buf, INDENTATION, SCOPE_OPENING, SCOPE_CLOSING);
            idiomatic::gen_header(&library_config, &mut out)?;
            idiomatic::gen(typ, in_path, env, &library_config, docs_url_gen, &mut out)?;
            outs.insert(format!("{}.cs", typ.name()), out_buf)
                .and_then::<String, _>(|_| panic!("file created twice: {}.cs", typ.name()));
        }

        for (in_path, typ) in errors {
            let idiomatic::ExceptionCtx { name, .. } =
                idiomatic::error_type_to_exception_name(env, &library_config, typ, &in_path)?;
            let mut out_buf = String::new();
            let mut out = CodeWriter::new(&mut out_buf, INDENTATION, SCOPE_OPENING, SCOPE_CLOSING);
            idiomatic::gen_header(&library_config, &mut out)?;
            idiomatic::gen_exception(env, &library_config, typ, &in_path, &mut out)?;
            outs.insert(format!("{}.cs", name), out_buf)
                .and_then::<String, _>(|_| panic!("file created twice: {}.cs", name));
        }
    }

    Ok(())
}
