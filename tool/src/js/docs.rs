use diplomat_core::Env;
use std::fmt::Write;
use std::{collections::HashMap, fmt};

use diplomat_core::ast;
use indenter::indented;

use crate::docs_util::markdown_to_rst;

/// Generate RST-formatted Sphinx docs for all FFI types. Currently assumes a JS target.
pub fn gen_docs(
    env: &Env,
    outs: &mut HashMap<String, String>,
    docs_url_gen: &ast::DocsUrlGenerator,
) -> fmt::Result {
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

            let mut sorted_symbols: Vec<&String> = module.names().collect();
            sorted_symbols.sort();

            for item in module.items() {
                if let ast::ModSymbol::CustomType(ref typ) = item {
                    writeln!(out)?;
                    gen_custom_type_docs(out, typ, in_path, env, docs_url_gen)?;
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
    docs_url_gen: &ast::DocsUrlGenerator,
) -> fmt::Result {
    writeln!(out, ".. js:class:: {}", typ.name())?;

    let mut class_indented = indented(out).with_str("    ");
    let d = typ.doc_lines(docs_url_gen);
    if !d.is_empty() {
        markdown_to_rst(&mut class_indented, &d, &|shortcut_path, to| {
            let resolved = ast::TypeName::Named(shortcut_path.clone()).resolve(in_path, env);
            write!(to, ":js:class:`{}`", resolved.name())?;
            Ok(())
        })?;
        writeln!(class_indented)?;
    }

    if let ast::CustomType::Struct(strct) = typ {
        for field in strct.fields.iter() {
            writeln!(&mut class_indented)?;
            gen_field_docs(&mut class_indented, field, in_path, env)?;
        }
    }

    for method in typ.methods() {
        writeln!(&mut class_indented)?;
        gen_method_docs(&mut class_indented, method, in_path, env)?;
    }
    Ok(())
}

pub fn gen_method_docs<W: fmt::Write>(
    out: &mut W,
    method: &ast::Method,
    in_path: &ast::Path,
    env: &Env,
) -> fmt::Result {
    let mut param_names: Vec<String> = method.params.iter().map(|p| p.name.clone()).collect();
    if method.is_writeable_out() {
        param_names.remove(param_names.len() - 1);
    }

    if method.self_param.is_some() {
        writeln!(
            out,
            ".. js:function:: {}({})",
            method.name,
            param_names.join(", ")
        )?;
    } else {
        writeln!(
            out,
            ".. js:staticfunction:: {}({})",
            method.name,
            param_names.join(", ")
        )?;
    }

    if !method.doc_lines.is_empty() {
        let mut method_indented = indented(out).with_str("    ");
        markdown_to_rst(
            &mut method_indented,
            &method.doc_lines,
            &|shortcut_path, to| {
                let resolved = ast::TypeName::Named(shortcut_path.clone()).resolve(in_path, env);
                write!(to, ":js:class:`{}`", resolved.name())?;
                Ok(())
            },
        )?;
        writeln!(method_indented)?;
    }

    for p in method
        .params
        .iter()
        .filter(|p| matches!(p.ty, ast::TypeName::PrimitiveSlice(..)))
    {
        writeln!(out)?;
        writeln!(
            out,
            "    - Note: ``{}`` should be an ArrayBuffer or TypedArray corresponding to the slice type expected by Rust.",
            p.name
        )?;
    }

    Ok(())
}

pub fn gen_field_docs<W: fmt::Write>(
    out: &mut W,
    field: &(String, ast::TypeName, String),
    in_path: &ast::Path,
    env: &Env,
) -> fmt::Result {
    writeln!(out, ".. js:attribute:: {}", field.0)?;

    if !field.2.is_empty() {
        let mut field_indented = indented(out).with_str("    ");
        markdown_to_rst(&mut field_indented, &field.2, &|shortcut_path, to| {
            let resolved = ast::TypeName::Named(shortcut_path.clone()).resolve(in_path, env);
            write!(to, ":js:class:`{}`", resolved.name())?;
            Ok(())
        })?;
        writeln!(field_indented)?;
    }

    Ok(())
}
