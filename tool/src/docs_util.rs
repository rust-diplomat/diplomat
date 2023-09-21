use std::fmt;

use diplomat_core::{
    ast::{self, Ident},
    Env,
};
use pulldown_cmark::{BrokenLink, CowStr, Event, LinkType, Options, Parser, Tag};

pub(crate) trait FromMarkdown<W: fmt::Write> {
    fn link_start(url: &str, out: &mut W) -> fmt::Result;

    fn link_end(url: &str, out: &mut W) -> fmt::Result;

    fn code(text: &str, out: &mut W) -> fmt::Result;

    fn write_reference(custom_type: &ast::CustomType, out: &mut W) -> fmt::Result;

    fn from_markdown(markdown: &str, in_path: &ast::Path, env: &Env, out: &mut W) -> fmt::Result {
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
                    Tag::Link(typ, url, _) => {
                        if typ == LinkType::ShortcutUnknown {
                            if in_shortcut {
                                panic!("Nested shortcuts are not allowed");
                            } else {
                                in_shortcut = true;
                            }
                        } else {
                            Self::link_start(&url, out)?;
                        }
                    }
                    o => todo!("{:?}", o),
                },
                Event::End(t) => match t {
                    Tag::Paragraph => writeln!(out)?,
                    Tag::Link(typ, url, _) => {
                        if typ == LinkType::ShortcutUnknown {
                            in_shortcut = false;
                        } else {
                            Self::link_end(&url, out)?;
                        }
                    }
                    o => todo!("{:?}", o),
                },
                Event::Text(text) => {
                    write!(out, "{text}")?;
                }
                Event::Code(text) => {
                    if in_shortcut {
                        Self::write_reference(
                            ast::PathType::new(
                                text.split("::")
                                    .map(|s| Ident::from(s.to_string()))
                                    .collect::<ast::Path>(),
                            )
                            .resolve(in_path, env),
                            out,
                        )?;
                    } else {
                        Self::code(&text, out)?;
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
}

pub struct CppRst;

impl<W: fmt::Write> FromMarkdown<W> for CppRst {
    fn link_start(_url: &str, out: &mut W) -> fmt::Result {
        write!(out, "`")
    }

    fn link_end(url: &str, out: &mut W) -> fmt::Result {
        write!(out, " <{url}>`__")
    }

    fn code(text: &str, out: &mut W) -> fmt::Result {
        write!(out, "``{text}``")
    }

    fn write_reference(custom_type: &ast::CustomType, out: &mut W) -> fmt::Result {
        match custom_type {
            ast::CustomType::Struct(strct) => write!(out, ":cpp:struct:`{}`", strct.name),
            ast::CustomType::Enum(enm) => write!(out, ":cpp:enum-struct:`{}`", enm.name),
            ast::CustomType::Opaque(opaque) => write!(out, ":cpp:class:`{}`", opaque.name),
            &_ => unreachable!("unknown AST/HIR variant"),
        }
    }
}

pub struct JsRst;

impl<W: fmt::Write> FromMarkdown<W> for JsRst {
    fn link_start(_url: &str, out: &mut W) -> fmt::Result {
        write!(out, "`")
    }

    fn link_end(url: &str, out: &mut W) -> fmt::Result {
        write!(out, " <{url}>`__")
    }

    fn code(text: &str, out: &mut W) -> fmt::Result {
        write!(out, "``{text}``")
    }

    fn write_reference(custom_type: &ast::CustomType, out: &mut W) -> fmt::Result {
        write!(out, ":js:class:`{}`", custom_type.name())
    }
}

pub struct TsDoc;

impl<W: fmt::Write> FromMarkdown<W> for TsDoc {
    fn link_start(url: &str, out: &mut W) -> fmt::Result {
        write!(out, "{{@link {url} ")
    }

    fn link_end(_url: &str, out: &mut W) -> fmt::Result {
        write!(out, "}}")
    }

    fn code(text: &str, out: &mut W) -> fmt::Result {
        write!(out, "`{text}`")
    }

    fn write_reference(custom_type: &ast::CustomType, out: &mut W) -> fmt::Result {
        write!(out, "{{@link {0} `{0}`}}", custom_type.name())
    }
}
