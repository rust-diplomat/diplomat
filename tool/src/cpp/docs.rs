use std::fmt::Write;
use std::{collections::HashMap, fmt};

use diplomat_core::ast::{self, Param};
use indenter::indented;

use crate::{
    cpp::{types::gen_type, util::transform_keyword_ident},
    docs_util::markdown_to_rst,
};

/// Generate RST-formatted Sphinx docs for all FFI types.
pub fn gen_docs(
    env: &HashMap<ast::Path, HashMap<String, ast::ModSymbol>>,
    outs: &mut HashMap<String, String>,
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
    let mut sorted_keys: Vec<String> = env
        .iter()
        .filter(|(_, s)| {
            s.values()
                .any(|k| matches!(k, ast::ModSymbol::CustomType(_)))
        })
        .map(|(p, _)| p.elements.join("_"))
        .collect();
    sorted_keys.sort();
    for in_path in sorted_keys {
        writeln!(&mut toctree_indent, "{}", in_path)?;
    }
    writeln!(index_out)?;
    writeln!(index_out, "Indices and tables")?;
    writeln!(index_out, "==================")?;
    writeln!(index_out)?;
    writeln!(index_out, "* :ref:`genindex`")?;
    writeln!(index_out, "* :ref:`search`")?;

    for (in_path, mod_symbols) in env.iter() {
        if mod_symbols
            .values()
            .any(|k| matches!(k, ast::ModSymbol::CustomType(_)))
        {
            let out = outs
                .entry(format!("{}.rst", in_path.elements.join("_")))
                .or_insert_with(String::new);

            let title = format!("``{}``", in_path.elements.join("::"));
            writeln!(out, "{}", title)?;
            writeln!(out, "{}", "=".repeat(title.len()))?;
            writeln!(out)?;

            let mut sorted_symbols: Vec<&String> = mod_symbols.keys().collect();
            sorted_symbols.sort();

            for symbol_name in sorted_symbols {
                let custom_type = &mod_symbols[symbol_name];
                if let ast::ModSymbol::CustomType(typ) = custom_type {
                    writeln!(out)?;
                    gen_custom_type_docs(out, typ, in_path, env)?;
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
    env: &HashMap<ast::Path, HashMap<String, ast::ModSymbol>>,
) -> fmt::Result {
    match typ {
        ast::CustomType::Struct(_) => writeln!(out, ".. cpp:struct:: {}", typ.name())?,
        ast::CustomType::Enum(_) => writeln!(out, ".. cpp:enum-struct:: {}", typ.name())?,
        ast::CustomType::Opaque(_) => writeln!(out, ".. cpp:class:: {}", typ.name())?,
    }

    writeln!(out)?;
    let mut class_indented = indented(out).with_str("    ");
    markdown_to_rst(
        &mut class_indented,
        typ.doc_lines(),
        &|shortcut_path, to| {
            let resolved = ast::TypeName::Named(shortcut_path.clone()).resolve(in_path, env);
            match resolved {
                ast::CustomType::Struct(_) => write!(to, ":cpp:struct:`{}`", resolved.name())?,
                ast::CustomType::Enum(_) => write!(to, ":cpp:enum-struct:`{}`", resolved.name())?,
                ast::CustomType::Opaque(_) => write!(to, ":cpp:class:`{}`", resolved.name())?,
            }
            Ok(())
        },
    )?;
    writeln!(class_indented)?;

    if let ast::CustomType::Struct(strct) = typ {
        for field in strct.fields.iter() {
            writeln!(&mut class_indented)?;
            gen_field_docs(&mut class_indented, field, in_path, env)?;
        }
    } else if let ast::CustomType::Enum(enm) = typ {
        for variant in &enm.variants {
            writeln!(&mut class_indented)?;
            gen_enum_variant_docs(&mut class_indented, variant, in_path, env)?;
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
    env: &HashMap<ast::Path, HashMap<String, ast::ModSymbol>>,
) -> fmt::Result {
    let mut params: Vec<Param> = method.params.clone();
    if method.is_writeable_out() {
        params.remove(params.len() - 1);
    }

    if method.self_param.is_some() {
        write!(out, ".. cpp:function:: ")?;
    } else {
        write!(out, ".. cpp:function:: static ")?;
    }

    match &method.return_type {
        None | Some(ast::TypeName::Unit) => {
            write!(out, "void")?;
        }

        Some(typ) => {
            gen_type(typ, in_path, None, env, out)?;
        }
    }

    write!(out, " {}(", transform_keyword_ident(&method.name))?;

    for (i, param) in params.iter().enumerate() {
        if i > 0 {
            write!(out, ", ")?;
        }

        gen_type(&param.ty, in_path, None, env, out)?;
        write!(out, " {}", param.name)?;
    }

    writeln!(out, ")")?;

    let mut method_indented = indented(out).with_str("    ");
    markdown_to_rst(
        &mut method_indented,
        &method.doc_lines,
        &|shortcut_path, to| {
            let resolved = ast::TypeName::Named(shortcut_path.clone()).resolve(in_path, env);
            match resolved {
                ast::CustomType::Struct(_) => write!(to, ":cpp:struct:`{}`", resolved.name())?,
                ast::CustomType::Enum(_) => write!(to, ":cpp:enum-struct:`{}`", resolved.name())?,
                ast::CustomType::Opaque(_) => write!(to, ":cpp:class:`{}`", resolved.name())?,
            }
            Ok(())
        },
    )?;
    writeln!(method_indented)?;

    Ok(())
}

pub fn gen_field_docs<W: fmt::Write>(
    out: &mut W,
    field: &(String, ast::TypeName, String),
    in_path: &ast::Path,
    env: &HashMap<ast::Path, HashMap<String, ast::ModSymbol>>,
) -> fmt::Result {
    write!(out, ".. cpp:member:: ")?;
    gen_type(&field.1, in_path, None, env, out)?;
    writeln!(out, " {}", field.0)?;

    writeln!(out)?;
    let mut field_indented = indented(out).with_str("    ");
    markdown_to_rst(&mut field_indented, &field.2, &|shortcut_path, to| {
        let resolved = ast::TypeName::Named(shortcut_path.clone()).resolve(in_path, env);
        match resolved {
            ast::CustomType::Struct(_) => write!(to, ":cpp:struct:`{}`", resolved.name())?,
            ast::CustomType::Enum(_) => write!(to, ":cpp:enum-struct:`{}`", resolved.name())?,
            ast::CustomType::Opaque(_) => write!(to, ":cpp:class:`{}`", resolved.name())?,
        }
        Ok(())
    })?;
    writeln!(field_indented)?;

    Ok(())
}

pub fn gen_enum_variant_docs<W: fmt::Write>(
    out: &mut W,
    variant: &(String, isize, String),
    in_path: &ast::Path,
    env: &HashMap<ast::Path, HashMap<String, ast::ModSymbol>>,
) -> fmt::Result {
    write!(out, ".. cpp:enumerator:: {}", variant.0)?;

    writeln!(out)?;
    let mut enum_indented = indented(out).with_str("    ");
    markdown_to_rst(&mut enum_indented, &variant.2, &|shortcut_path, to| {
        let resolved = ast::TypeName::Named(shortcut_path.clone()).resolve(in_path, env);
        match resolved {
            ast::CustomType::Struct(_) => write!(to, ":cpp:struct:`{}`", resolved.name())?,
            ast::CustomType::Enum(_) => write!(to, ":cpp:enum-struct:`{}`", resolved.name())?,
            ast::CustomType::Opaque(_) => write!(to, ":cpp:class:`{}`", resolved.name())?,
        }
        Ok(())
    })?;
    writeln!(enum_indented)?;

    Ok(())
}
