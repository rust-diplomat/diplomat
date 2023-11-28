use diplomat_core::Env;
use std::fmt::Write;
use std::{collections::HashMap, fmt};

use diplomat_core::ast;
use indenter::indented;

use crate::docs_util::{FromMarkdown, JsRst};

/// Generate RST-formatted Sphinx docs for all FFI types. Currently assumes a JS target.
pub fn gen_docs(
    env: &Env,
    outs: &mut HashMap<String, String>,
    docs_url_gen: &ast::DocsUrlGenerator,
) -> fmt::Result {
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
    if !typ.docs().is_empty() {
        JsRst::from_markdown(
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
            gen_field_docs(&mut class_indented, field, in_path, docs_url_gen, env)?;
        }
    }

    for method in typ.methods() {
        writeln!(&mut class_indented)?;
        gen_method_docs(&mut class_indented, method, in_path, docs_url_gen, env)?;
    }
    Ok(())
}

pub fn gen_method_docs<W: fmt::Write>(
    out: &mut W,
    method: &ast::Method,
    in_path: &ast::Path,
    docs_url_gen: &ast::DocsUrlGenerator,
    env: &Env,
) -> fmt::Result {
    let mut param_names = method
        .params
        .iter()
        .map(|p| p.name.as_str())
        .collect::<Vec<_>>();
    if method.is_writeable_out() {
        param_names.remove(param_names.len() - 1);
    }

    if method.self_param.is_some() {
        writeln!(
            out,
            ".. js:method:: {}({})",
            method.name,
            param_names.join(", ")
        )?;
    } else {
        writeln!(
            out,
            ".. js:function:: {}({})",
            method.name,
            param_names.join(", ")
        )?;
    }

    let mut method_indented = indented(out).with_str("    ");
    if !method.docs.is_empty() {
        JsRst::from_markdown(
            &method
                .docs
                .to_markdown(docs_url_gen, ast::MarkdownStyle::RstCompat),
            in_path,
            env,
            &mut method_indented,
        )?;
        writeln!(method_indented)?;
    }

    let static_borrows = method.borrowed_params();
    let static_borrows = static_borrows.static_names().collect::<Vec<_>>();
    if !static_borrows.is_empty() {
        write!(method_indented, "- Warning: This method leaks memory.")?;
        if static_borrows.len() == 1 {
            writeln!(method_indented, " The parameter `{}` will not be freed as it is required to live for the duration of the program.", static_borrows[0])?;
        } else {
            write!(method_indented, " The parameters `{}`", static_borrows[0])?;
            for name in static_borrows.iter().skip(1) {
                write!(method_indented, ", {name}")?;
            }
            writeln!(
                method_indented,
                " will not be freed as they are required to live for the duration of the program."
            )?;
        }
        writeln!(method_indented)?;
    }

    Ok(())
}

pub fn gen_field_docs<W: fmt::Write>(
    out: &mut W,
    field: &(ast::Ident, ast::TypeName, ast::Docs),
    in_path: &ast::Path,
    docs_url_gen: &ast::DocsUrlGenerator,
    env: &Env,
) -> fmt::Result {
    writeln!(out, ".. js:attribute:: {}", field.0)?;

    if !field.2.is_empty() {
        let mut field_indented = indented(out).with_str("    ");
        JsRst::from_markdown(
            &field
                .2
                .to_markdown(docs_url_gen, ast::MarkdownStyle::RstCompat),
            in_path,
            env,
            &mut field_indented,
        )?;
        writeln!(field_indented)?;
    }

    Ok(())
}
