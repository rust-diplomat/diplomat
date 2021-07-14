use std::fmt::Write;
use std::{collections::HashMap, fmt};

use diplomat_core::ast;
use indenter::indented;
use pulldown_cmark::{Event, Parser, Tag};

/// Generate RST-formatted Sphinx docs for all FFI types. Currently assumes a JS target.
pub fn gen_docs<W: fmt::Write>(env: &HashMap<String, ast::CustomType>, out: &mut W) -> fmt::Result {
    let mut all_types: Vec<&ast::CustomType> = env.values().collect();
    all_types.sort_by_key(|t| t.name());
    for custom_type in all_types {
        writeln!(out)?;
        gen_custom_type_docs(out, custom_type, env)?;
    }
    Ok(())
}

pub fn gen_custom_type_docs<W: fmt::Write>(
    out: &mut W,
    typ: &ast::CustomType,
    env: &HashMap<String, ast::CustomType>,
) -> fmt::Result {
    writeln!(out, ".. js:class:: {}", typ.name())?;
    writeln!(out)?;
    let mut class_indented = indented(out).with_str("    ");
    markdown_to_rst(&mut class_indented, &typ.doc_lines())?;
    writeln!(class_indented)?;

    if let ast::CustomType::Struct(strct) = typ {
        for field in strct.fields.iter() {
            writeln!(&mut class_indented)?;
            gen_field_docs(&mut class_indented, field, env)?;
        }
    }

    for method in typ.methods() {
        writeln!(&mut class_indented)?;
        gen_method_docs(&mut class_indented, method, env)?;
    }
    Ok(())
}

pub fn gen_method_docs<W: fmt::Write>(
    out: &mut W,
    method: &ast::Method,
    _env: &HashMap<String, ast::CustomType>,
) -> fmt::Result {
    if method.self_param.is_some() {
        writeln!(out, ".. js:function:: {}", method.name)?;
    } else {
        writeln!(out, ".. js:staticfunction:: {}", method.name)?;
    }

    let mut method_indented = indented(out).with_str("    ");
    markdown_to_rst(&mut method_indented, &method.doc_lines)?;
    writeln!(method_indented)?;

    Ok(())
}

pub fn gen_field_docs<W: fmt::Write>(
    out: &mut W,
    field: &(String, ast::TypeName, String),
    _env: &HashMap<String, ast::CustomType>,
) -> fmt::Result {
    writeln!(out, ".. js:function:: {}", field.0)?;

    writeln!(out)?;
    let mut method_indented = indented(out).with_str("    ");
    markdown_to_rst(&mut method_indented, &field.2)?;
    writeln!(method_indented)?;

    Ok(())
}

fn markdown_to_rst<W: fmt::Write>(out: &mut W, markdown: &str) -> fmt::Result {
    let parser = Parser::new(markdown);
    for event in parser {
        match event {
            Event::Start(t) => match t {
                Tag::Paragraph => {
                    writeln!(out)?;
                }
                Tag::Link(_, _, _) => {
                    write!(out, "`")?;
                }
                o => todo!("{:?}", o),
            },
            Event::End(t) => match t {
                Tag::Paragraph => {}
                Tag::Link(_, url, _) => {
                    write!(out, " <{}>`__", url)?;
                }
                o => todo!("{:?}", o),
            },
            Event::Text(text) => {
                write!(out, "{}", text)?;
            }
            Event::Code(text) => {
                write!(out, "``{}``", text)?;
            }
            Event::SoftBreak => {
                write!(out, " ")?;
            }
            evt => todo!("{:?}", evt),
        }
    }

    Ok(())
}
