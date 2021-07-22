use std::fmt::Write;
use std::{collections::HashMap, fmt};

use diplomat_core::ast;
use indenter::indented;
use pulldown_cmark::{BrokenLink, CowStr, Event, LinkType, Options, Parser, Tag};

use crate::util;

/// Generate RST-formatted Sphinx docs for all FFI types. Currently assumes a JS target.
pub fn gen_docs<W: fmt::Write>(
    env: &HashMap<ast::Path, HashMap<String, ast::ModSymbol>>,
    out: &mut W,
) -> fmt::Result {
    let mut all_types: Vec<(&ast::Path, &ast::CustomType)> = util::get_all_custom_types(env);
    all_types.sort_by_key(|t| t.1.name());
    for (in_path, custom_type) in all_types {
        writeln!(out)?;
        gen_custom_type_docs(out, custom_type, in_path, env)?;
    }
    Ok(())
}

pub fn gen_custom_type_docs<W: fmt::Write>(
    out: &mut W,
    typ: &ast::CustomType,
    in_path: &ast::Path,
    env: &HashMap<ast::Path, HashMap<String, ast::ModSymbol>>,
) -> fmt::Result {
    writeln!(out, ".. js:class:: {}", typ.name())?;
    writeln!(out)?;
    let mut class_indented = indented(out).with_str("    ");
    markdown_to_rst(&mut class_indented, typ.doc_lines(), in_path, env)?;
    writeln!(class_indented)?;

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
    env: &HashMap<ast::Path, HashMap<String, ast::ModSymbol>>,
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

    let mut method_indented = indented(out).with_str("    ");
    markdown_to_rst(&mut method_indented, &method.doc_lines, in_path, env)?;
    writeln!(method_indented)?;

    Ok(())
}

pub fn gen_field_docs<W: fmt::Write>(
    out: &mut W,
    field: &(String, ast::TypeName, String),
    in_path: &ast::Path,
    env: &HashMap<ast::Path, HashMap<String, ast::ModSymbol>>,
) -> fmt::Result {
    writeln!(out, ".. js:attribute:: {}", field.0)?;

    writeln!(out)?;
    let mut method_indented = indented(out).with_str("    ");
    markdown_to_rst(&mut method_indented, &field.2, in_path, env)?;
    writeln!(method_indented)?;

    Ok(())
}

fn markdown_to_rst<W: fmt::Write>(
    out: &mut W,
    markdown: &str,
    in_path: &ast::Path,
    env: &HashMap<ast::Path, HashMap<String, ast::ModSymbol>>,
) -> fmt::Result {
    let mut broken_link_callback = |broken: BrokenLink| {
        Some((
            CowStr::from(broken.reference.to_string()),
            CowStr::from(broken.reference.to_string()),
        ))
    };

    let parser = Parser::new_with_broken_link_callback(
        markdown,
        Options::empty(),
        Some(&mut broken_link_callback),
    );
    let mut in_shortcut = false;
    for event in parser {
        match event {
            Event::Start(t) => match t {
                Tag::Paragraph => {
                    writeln!(out)?;
                }
                Tag::Link(typ, _, _) => {
                    if typ == LinkType::ShortcutUnknown {
                        if in_shortcut {
                            panic!("Nested shortcuts are not allowed");
                        } else {
                            in_shortcut = true;
                        }
                    } else {
                        write!(out, "`")?;
                    }
                }
                o => todo!("{:?}", o),
            },
            Event::End(t) => match t {
                Tag::Paragraph => {}
                Tag::Link(typ, url, _) => {
                    if typ == LinkType::ShortcutUnknown {
                        in_shortcut = false;
                    } else {
                        write!(out, " <{}>`__", url)?;
                    }
                }
                o => todo!("{:?}", o),
            },
            Event::Text(text) => {
                write!(out, "{}", text)?;
            }
            Event::Code(text) => {
                if in_shortcut {
                    let shortcut_path = ast::Path {
                        elements: text.split("::").map(|s| s.to_string()).collect(),
                    };
                    let resolved = ast::TypeName::Named(shortcut_path).resolve(in_path, env);
                    write!(out, ":js:class:`{}`", resolved.name())?;
                } else {
                    write!(out, "``{}``", text)?;
                }
            }
            Event::SoftBreak => {
                write!(out, " ")?;
            }
            evt => todo!("{:?}", evt),
        }
    }

    Ok(())
}
