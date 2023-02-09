use colored::*;
use std::collections::{HashMap, HashSet};
use std::fmt;
use std::fmt::Write;
use std::fs;
use std::path::PathBuf;

use diplomat_core::ast;
use diplomat_core::Env;
use indenter::indented;

#[cfg(test)]
#[macro_use]
mod test_util;

mod types;

mod structs;
use structs::*;

use crate::cpp::util::gen_comment_block;

mod conversions;

pub mod docs;

mod config;

mod util;

pub(crate) static RUNTIME_HPP: &str = include_str!("runtime.hpp");

fn render_header(typ_name: &ast::Ident, headers: &[String]) -> String {
    let all_headers = headers.join("\n");

    format!(
        r##"#ifndef {typ_name}_HPP
#define {typ_name}_HPP
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <algorithm>
#include <memory>
#include <variant>
{all_headers}
#include "diplomat_runtime.hpp"

#include "{typ_name}.h"
"##
    )
}

pub fn gen_bindings(
    env: &Env,
    library_config_path: &Option<PathBuf>,
    docs_url_gen: &ast::DocsUrlGenerator,
    outs: &mut HashMap<String, String>,
) -> fmt::Result {
    // Note: Assumes the existence of C bindings!
    // This must be called alongside c::gen_bindings

    let mut library_config = config::LibraryConfig::default();
    if let Some(path) = library_config_path {
        // Should be fine, we've already verified the path
        if let Ok(contents) = fs::read_to_string(path) {
            match toml::from_str(&contents) {
                Ok(config) => library_config = config,
                Err(err) => {
                    eprintln!(
                        "{}Unable to parse library configuration file: {:?}\n{}",
                        "Error: ".red().bold(),
                        path,
                        err,
                    );
                    std::process::exit(1);
                }
            }
        }
    }

    let diplomat_runtime_out = outs
        .entry("diplomat_runtime.hpp".to_string())
        .or_insert_with(String::new);
    write!(diplomat_runtime_out, "{RUNTIME_HPP}")?;

    let all_types = crate::util::get_all_custom_types(env);

    for (in_path, typ) in &all_types {
        let out = outs
            .entry(format!("{}.hpp", typ.name()))
            .or_insert_with(String::new);

        let rendered = render_header(typ.name(), &library_config.headers);
        writeln!(out, "{rendered}").expect("Failed to write string.");

        let mut seen_includes = HashSet::new();
        seen_includes.insert(format!("#include \"{}.hpp\"", typ.name()));

        match typ {
            ast::CustomType::Opaque(_) => {}

            ast::CustomType::Enum(enm) => {
                writeln!(out)?;
                gen_comment_block(
                    out,
                    &enm.docs
                        .to_markdown(docs_url_gen, ast::MarkdownStyle::Normal),
                )?;
                writeln!(out, "enum struct {} {{", enm.name)?;
                let mut enm_indent = indented(out).with_str("  ");
                for (name, discriminant, docs) in enm.variants.iter() {
                    gen_comment_block(
                        &mut enm_indent,
                        &docs.to_markdown(docs_url_gen, ast::MarkdownStyle::Normal),
                    )?;
                    writeln!(&mut enm_indent, "{name} = {discriminant},")?;
                }
                writeln!(out, "}};")?;
            }

            ast::CustomType::Struct(strct) => {
                for (_, typ, _) in &strct.fields {
                    gen_includes(
                        typ,
                        in_path,
                        true,
                        false,
                        true,
                        env,
                        &mut seen_includes,
                        out,
                    )?;
                }
            }
        }

        for method in typ.methods() {
            for param in &method.params {
                gen_includes(
                    &param.ty,
                    in_path,
                    true,
                    false,
                    false,
                    env,
                    &mut seen_includes,
                    out,
                )?;
            }

            if let Some(return_type) = method.return_type.as_ref() {
                gen_includes(
                    return_type,
                    in_path,
                    true,
                    false,
                    false,
                    env,
                    &mut seen_includes,
                    out,
                )?;
            }
        }

        match typ {
            ast::CustomType::Opaque(_) => {
                writeln!(out)?;
                gen_struct(typ, in_path, true, env, &library_config, docs_url_gen, out)?;
            }

            ast::CustomType::Enum(_) => {}

            ast::CustomType::Struct(_) => {
                writeln!(out)?;
                gen_struct(typ, in_path, true, env, &library_config, docs_url_gen, out)?;
            }
        }

        writeln!(out)?;

        for method in typ.methods() {
            for param in &method.params {
                gen_includes(
                    &param.ty,
                    in_path,
                    false,
                    false,
                    false,
                    env,
                    &mut seen_includes,
                    out,
                )?;
            }

            if let Some(return_type) = method.return_type.as_ref() {
                gen_includes(
                    return_type,
                    in_path,
                    false,
                    false,
                    false,
                    env,
                    &mut seen_includes,
                    out,
                )?;
            }
        }

        match typ {
            ast::CustomType::Opaque(_) => {
                writeln!(out)?;
                gen_struct(typ, in_path, false, env, &library_config, docs_url_gen, out)?;
            }

            ast::CustomType::Enum(_) => {}

            ast::CustomType::Struct(_) => {
                writeln!(out)?;
                gen_struct(typ, in_path, false, env, &library_config, docs_url_gen, out)?;
            }
        }

        writeln!(out, "#endif")?
    }

    Ok(())
}

#[allow(clippy::too_many_arguments)]
fn gen_includes<W: fmt::Write>(
    typ: &ast::TypeName,
    in_path: &ast::Path,
    pre_struct: bool,
    behind_ref: bool,
    for_field: bool,
    env: &Env,
    seen_includes: &mut HashSet<String>,
    out: &mut W,
) -> fmt::Result {
    match typ {
        ast::TypeName::Named(path_type) | ast::TypeName::SelfType(path_type) => {
            let custom_typ = path_type.resolve(in_path, env);
            match custom_typ {
                ast::CustomType::Opaque(_) => {
                    if pre_struct {
                        let decl = format!("class {};", custom_typ.name());
                        if !seen_includes.contains(&decl) {
                            writeln!(out, "{decl}")?;
                            seen_includes.insert(decl);
                        }
                    } else {
                        let include = format!("#include \"{}.hpp\"", custom_typ.name());
                        if !seen_includes.contains(&include) {
                            writeln!(out, "{include}")?;
                            seen_includes.insert(include);
                        }
                    }
                }

                ast::CustomType::Struct(_) => {
                    if pre_struct && (!for_field || behind_ref) {
                        let decl = format!("struct {};", custom_typ.name());
                        if !seen_includes.contains(&decl) {
                            writeln!(out, "{decl}")?;
                            seen_includes.insert(decl);
                        }
                    } else {
                        let include = format!("#include \"{}.hpp\"", custom_typ.name());
                        if !seen_includes.contains(&include) {
                            writeln!(out, "{include}")?;
                            seen_includes.insert(include);
                        }
                    }
                }

                ast::CustomType::Enum(_) => {
                    let include = format!("#include \"{}.hpp\"", custom_typ.name());
                    if !seen_includes.contains(&include) {
                        writeln!(out, "{include}")?;
                        seen_includes.insert(include);
                    }
                }
            }
        }
        ast::TypeName::Box(underlying) => {
            gen_includes(
                underlying,
                in_path,
                pre_struct,
                true,
                for_field,
                env,
                seen_includes,
                out,
            )?;
        }
        ast::TypeName::Reference(.., underlying) => {
            gen_includes(
                underlying,
                in_path,
                pre_struct,
                true,
                for_field,
                env,
                seen_includes,
                out,
            )?;
        }
        ast::TypeName::Primitive(_) => {}
        ast::TypeName::Option(underlying) => {
            gen_includes(
                underlying,
                in_path,
                pre_struct,
                behind_ref,
                for_field,
                env,
                seen_includes,
                out,
            )?;
        }
        ast::TypeName::Result(ok, err, _) => {
            gen_includes(
                ok.as_ref(),
                in_path,
                pre_struct,
                behind_ref,
                for_field,
                env,
                seen_includes,
                out,
            )?;

            gen_includes(
                err.as_ref(),
                in_path,
                pre_struct,
                behind_ref,
                for_field,
                env,
                seen_includes,
                out,
            )?;
        }
        ast::TypeName::Writeable => {}
        ast::TypeName::StrReference(..) => {}
        ast::TypeName::PrimitiveSlice(..) => {}
        ast::TypeName::Unit => {}
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_cross_module_struct_fields() {
        test_file! {
            #[diplomat::bridge]
            mod mod1 {
                use super::mod2::Bar;

                struct Foo {
                    x: Bar,
                }
            }

            #[diplomat::bridge]
            mod mod2 {
                use super::mod1::Foo;

                struct Bar {
                    y: Box<Foo>,
                }
            }
        }
    }

    #[test]
    fn test_cross_module_struct_methods() {
        test_file! {
            #[diplomat::bridge]
            mod mod1 {
                use super::mod2::Bar;

                #[diplomat::opaque]
                struct Foo;

                impl Foo {
                    pub fn to_bar(&self) -> Bar {
                        unimplemented!()
                    }
                }
            }

            #[diplomat::bridge]
            mod mod2 {
                use super::mod1::Foo;

                struct Bar {
                    y: Box<Foo>,
                }
            }
        }
    }

    #[test]
    fn test_enum_documentation() {
        test_file! {
            #[diplomat::bridge]
            mod ffi {
                /// Documentation for MyEnum.
                enum MyEnum {
                    /// All about A.
                    A,
                    /// All about B.
                    B,
                    /// All about C.
                    C
                }
            }
        }
    }
}
