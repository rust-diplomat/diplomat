use colored::*;
use diplomat_core::Env;
use std::fmt::Write;
use std::fs;
use std::path::Path;
use std::{collections::HashMap, fmt};

use diplomat_core::ast;
use indenter::indented;

use crate::cpp::{config::LibraryConfig, structs::gen_method_interface, types::gen_type};
use crate::docs_util::{CppRst, FromMarkdown};

/// Generate RST-formatted Sphinx docs for all FFI types.
pub fn gen_docs(
    env: &Env,
    library_config_path: Option<&Path>,
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

    let index_out = outs.entry("index.rst".to_string()).or_default();
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
                .or_default();

            let title = format!("``{}``", in_path.elements.join("::"));
            writeln!(out, "{title}")?;
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
        &_ => unreachable!("unknown AST/HIR variant"),
    }

    let mut class_indented = indented(out).with_str("    ");
    if !typ.docs().is_empty() {
        CppRst::from_markdown(
            &typ.docs()
                .to_markdown(docs_url_gen, ast::MarkdownStyle::RstCompat),
            in_path,
            env,
            &mut class_indented,
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

    let mut docs =
        gen_docs_and_lifetime_notes_markdown(method, docs_url_gen, ast::MarkdownStyle::RstCompat);

    if method.params.iter().any(|p| {
        matches!(
            p,
            ast::Param {
                ty: ast::TypeName::StrReference(_, ast::StringEncoding::Utf8),
                ..
            }
        )
    }) {
        write!(
            docs,
            "\nWarning: Passing ill-formed UTF-8 is undefined behavior (and may be memory-unsafe)."
        )?;
    }

    if !docs.is_empty() {
        CppRst::from_markdown(&docs, in_path, env, &mut indented(out).with_str("    "))?;
    }
    writeln!(out)
}

pub fn gen_docs_and_lifetime_notes_markdown(
    method: &ast::Method,
    docs_url_gen: &ast::DocsUrlGenerator,
    style: ast::MarkdownStyle,
) -> String {
    let mut docs = if !method.docs.is_empty() {
        method.docs.to_markdown(docs_url_gen, style)
    } else {
        String::new()
    };

    let borrowed_params = method.borrowed_params();
    if !borrowed_params.is_empty() {
        if !docs.is_empty() {
            writeln!(docs).unwrap();
            writeln!(docs).unwrap();
        }
        write!(docs, "Lifetimes:").unwrap();
        let mut return_names = borrowed_params.return_names(&ast::Ident::THIS);
        if let Some(first) = return_names.next() {
            write!(docs, " `{first}`").unwrap();
            for param in return_names {
                write!(docs, ", `{param}`").unwrap();
            }
            writeln!(docs, " must live at least as long as the output.").unwrap();
        }

        let mut static_names = borrowed_params.static_names();
        if let Some(first) = static_names.next() {
            write!(docs, " `{first}`").unwrap();
            for param in static_names {
                write!(docs, ", `{param}`").unwrap();
            }
            writeln!(docs, " must live for the duration of the program.").unwrap();
        }
    }
    docs
}

pub fn gen_field_docs<W: fmt::Write>(
    out: &mut W,
    field: &(ast::Ident, ast::TypeName, ast::Docs),
    in_path: &ast::Path,
    env: &Env,
    library_config: &LibraryConfig,
    docs_url_gen: &ast::DocsUrlGenerator,
) -> fmt::Result {
    let ty_name = gen_type(&field.1, in_path, None, env, library_config, true)?;
    writeln!(out, ".. cpp:member:: {} {}", ty_name, field.0)?;

    let has_doc = !field.2.is_empty();
    let has_ub_warning = matches!(
        field.1,
        ast::TypeName::StrReference(_, ast::StringEncoding::Utf8)
    );

    if has_doc || has_ub_warning {
        let mut field_indented = indented(out).with_str("    ");
        if has_doc {
            CppRst::from_markdown(
                &field
                    .2
                    .to_markdown(docs_url_gen, ast::MarkdownStyle::RstCompat),
                in_path,
                env,
                &mut field_indented,
            )?;
        }

        if has_doc && has_ub_warning {
            writeln!(field_indented)?;
        }

        if has_ub_warning {
            write!(
                field_indented,
                "Warning: Setting ill-formed UTF-8 is undefined behavior (and may be memory-unsafe)."
            )?;
        }

        writeln!(field_indented)?;
    }

    Ok(())
}

pub fn gen_enum_variant_docs<W: fmt::Write>(
    out: &mut W,
    variant: &(ast::Ident, isize, ast::Docs, ast::Attrs),
    in_path: &ast::Path,
    env: &Env,
    docs_url_gen: &ast::DocsUrlGenerator,
) -> fmt::Result {
    write!(out, ".. cpp:enumerator:: {}", variant.0)?;

    writeln!(out)?;

    if !variant.2.is_empty() {
        let mut enum_indented = indented(out).with_str("    ");
        CppRst::from_markdown(
            &variant
                .2
                .to_markdown(docs_url_gen, ast::MarkdownStyle::RstCompat),
            in_path,
            env,
            &mut enum_indented,
        )?;
        writeln!(enum_indented)?;
    }

    Ok(())
}
