use colored::*;
use diplomat_core::Env;
use std::fmt::Write;
use std::fs;
use std::path::PathBuf;
use std::{collections::HashMap, fmt};

use diplomat_core::ast;
use indenter::indented;

use crate::{
    cpp::{config::LibraryConfig, structs::gen_method_interface, types::gen_type},
    docs_util::markdown_to_rst,
};

/// Generate RST-formatted Sphinx docs for all FFI types.
pub fn gen_docs(
    env: &Env,
    library_config_path: &Option<PathBuf>,
    outs: &mut HashMap<String, String>,
    docs_url_gen: &ast::DocsUrlGenerator,
) -> fmt::Result {
    let mut library_config = LibraryConfig::default();
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

    let index_out = outs
        .entry("index.rst".to_string())
        .or_insert_with(String::new);
    writeln!(index_out, "Documentation")?;
    writeln!(index_out, "=============")?;
    writeln!(index_out)?;
    writeln!(index_out, ".. toctree::")?;
    let mut toctree_indent = indented(index_out).with_str("   ");
    writeln!(&mut toctree_indent, ":maxdepth: 3")?;
    writeln!(&mut toctree_indent, ":caption: Modules:")?;
    writeln!(&mut toctree_indent)?;

    for (in_path, module) in env.iter_modules() {
        if module
            .items()
            .any(|k| matches!(k, ast::ModSymbol::CustomType(_)))
        {
            writeln!(&mut toctree_indent, "{}", in_path.elements.join("_"))?;
        }
    }
    writeln!(index_out)?;
    writeln!(index_out, "Indices and tables")?;
    writeln!(index_out, "==================")?;
    writeln!(index_out)?;
    writeln!(index_out, "* :ref:`genindex`")?;
    writeln!(index_out, "* :ref:`search`")?;

    for (in_path, module) in env.iter_modules() {
        if module
            .items()
            .any(|k| matches!(k, ast::ModSymbol::CustomType(_)))
        {
            let out = outs
                .entry(format!("{}.rst", in_path.elements.join("_")))
                .or_insert_with(String::new);

            let title = format!("``{}``", in_path.elements.join("::"));
            writeln!(out, "{}", title)?;
            writeln!(out, "{}", "=".repeat(title.len()))?;

            for item in module.items() {
                if let ast::ModSymbol::CustomType(ref typ) = item {
                    writeln!(out)?;
                    gen_custom_type_docs(out, typ, in_path, env, &library_config, docs_url_gen)?;
                }
            }
        }
    }

    Ok(())
}

pub fn gen_custom_type_docs<W: fmt::Write>(
    out: &mut W,
    typ: &ast::CustomType,
    in_path: &ast::Path,
    env: &Env,
    library_config: &LibraryConfig,
    docs_url_gen: &ast::DocsUrlGenerator,
) -> fmt::Result {
    match typ {
        ast::CustomType::Struct(_) => writeln!(out, ".. cpp:struct:: {}", typ.name())?,
        ast::CustomType::Enum(_) => writeln!(out, ".. cpp:enum-struct:: {}", typ.name())?,
        ast::CustomType::Opaque(_) => writeln!(out, ".. cpp:class:: {}", typ.name())?,
    }

    let mut class_indented = indented(out).with_str("    ");
    if !typ.docs().is_empty() {
        markdown_to_rst(
            &mut class_indented,
            &typ.docs().to_markdown(docs_url_gen),
            &|shortcut_path, to| {
                let resolved = ast::PathType::new(shortcut_path.clone()).resolve(in_path, env);
                match resolved {
                    ast::CustomType::Struct(_) => write!(to, ":cpp:struct:`{}`", resolved.name())?,
                    ast::CustomType::Enum(_) => {
                        write!(to, ":cpp:enum-struct:`{}`", resolved.name())?
                    }
                    ast::CustomType::Opaque(_) => write!(to, ":cpp:class:`{}`", resolved.name())?,
                }
                Ok(())
            },
        )?;
        writeln!(class_indented)?;
    }

    if let ast::CustomType::Struct(strct) = typ {
        for field in strct.fields.iter() {
            writeln!(&mut class_indented)?;
            gen_field_docs(
                &mut class_indented,
                field,
                in_path,
                env,
                library_config,
                docs_url_gen,
            )?;
        }
    } else if let ast::CustomType::Enum(enm) = typ {
        for variant in &enm.variants {
            writeln!(&mut class_indented)?;
            gen_enum_variant_docs(&mut class_indented, variant, in_path, env, docs_url_gen)?;
        }
    }

    for method in typ.methods() {
        writeln!(&mut class_indented)?;
        gen_method_docs(
            method,
            typ,
            in_path,
            true,
            env,
            library_config,
            docs_url_gen,
            &mut class_indented,
        )?;
    }

    Ok(())
}

#[allow(clippy::too_many_arguments)]
pub fn gen_method_docs<W: fmt::Write>(
    method: &ast::Method,
    enclosing_type: &ast::CustomType,
    in_path: &ast::Path,
    writeable_to_string: bool,
    env: &Env,
    library_config: &LibraryConfig,
    docs_url_gen: &ast::DocsUrlGenerator,
    out: &mut W,
) -> fmt::Result {
    // This method should rearrange the writeable
    let rearranged_writeable = method.is_writeable_out() && writeable_to_string;

    // This method has some writeable param that is preserved
    let has_writeable_param = method.has_writeable_param() && !writeable_to_string;

    if rearranged_writeable {
        // generate the normal method too
        gen_method_docs(
            method,
            enclosing_type,
            in_path,
            false,
            env,
            library_config,
            docs_url_gen,
            out,
        )?;
        writeln!(out)?;
    }

    write!(out, ".. cpp:function:: ")?;

    let _ = gen_method_interface(
        method,
        enclosing_type,
        in_path,
        true,
        has_writeable_param,
        rearranged_writeable,
        env,
        library_config,
        out,
        writeable_to_string,
    )?;

    writeln!(out)?;

    let mut method_indented = indented(out).with_str("    ");
    if !method.docs.is_empty() {
        markdown_to_rst(
            &mut method_indented,
            &method.docs.to_markdown(docs_url_gen),
            &|shortcut_path, to| {
                let resolved = ast::PathType::new(shortcut_path.clone()).resolve(in_path, env);
                match resolved {
                    ast::CustomType::Struct(_) => write!(to, ":cpp:struct:`{}`", resolved.name())?,
                    ast::CustomType::Enum(_) => {
                        write!(to, ":cpp:enum-struct:`{}`", resolved.name())?
                    }
                    ast::CustomType::Opaque(_) => write!(to, ":cpp:class:`{}`", resolved.name())?,
                }
                Ok(())
            },
        )?;
        writeln!(method_indented)?;
    }
    let borrowed_params = method.borrowed_params();
    let mut names = borrowed_params.names("this");

    if let Some(first) = names.next() {
        write!(method_indented, "\nLifetimes: ``{}``", first).unwrap();
        for param in names {
            write!(method_indented, ", ``{}``", param).unwrap();
        }
        writeln!(
            method_indented,
            " must live at least as long as the output."
        )
        .unwrap();
    }

    Ok(())
}

pub fn gen_field_docs<W: fmt::Write>(
    out: &mut W,
    field: &(ast::Ident, ast::TypeName, ast::Docs),
    in_path: &ast::Path,
    env: &Env,
    library_config: &LibraryConfig,
    docs_url_gen: &ast::DocsUrlGenerator,
) -> fmt::Result {
    let ty_name = gen_type(&field.1, in_path, None, env, library_config)?;
    writeln!(out, ".. cpp:member:: {} {}", ty_name, field.0)?;

    if !field.2.is_empty() {
        let mut field_indented = indented(out).with_str("    ");
        markdown_to_rst(
            &mut field_indented,
            &field.2.to_markdown(docs_url_gen),
            &|shortcut_path, to| {
                let resolved = ast::PathType::new(shortcut_path.clone()).resolve(in_path, env);
                match resolved {
                    ast::CustomType::Struct(_) => write!(to, ":cpp:struct:`{}`", resolved.name())?,
                    ast::CustomType::Enum(_) => {
                        write!(to, ":cpp:enum-struct:`{}`", resolved.name())?
                    }
                    ast::CustomType::Opaque(_) => write!(to, ":cpp:class:`{}`", resolved.name())?,
                }
                Ok(())
            },
        )?;
        writeln!(field_indented)?;
    }

    Ok(())
}

pub fn gen_enum_variant_docs<W: fmt::Write>(
    out: &mut W,
    variant: &(ast::Ident, isize, ast::Docs),
    in_path: &ast::Path,
    env: &Env,
    docs_url_gen: &ast::DocsUrlGenerator,
) -> fmt::Result {
    write!(out, ".. cpp:enumerator:: {}", variant.0)?;

    writeln!(out)?;

    if !variant.2.is_empty() {
        let mut enum_indented = indented(out).with_str("    ");
        markdown_to_rst(
            &mut enum_indented,
            &variant.2.to_markdown(docs_url_gen),
            &|shortcut_path, to| {
                let resolved = ast::PathType::new(shortcut_path.clone()).resolve(in_path, env);
                match resolved {
                    ast::CustomType::Struct(_) => write!(to, ":cpp:struct:`{}`", resolved.name())?,
                    ast::CustomType::Enum(_) => {
                        write!(to, ":cpp:enum-struct:`{}`", resolved.name())?
                    }
                    ast::CustomType::Opaque(_) => write!(to, ":cpp:class:`{}`", resolved.name())?,
                }
                Ok(())
            },
        )?;
        writeln!(enum_indented)?;
    }

    Ok(())
}
