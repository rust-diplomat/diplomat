use std::fmt;

use diplomat_core::ast::{self, Ident};
use pulldown_cmark::{BrokenLink, CowStr, Event, LinkType, Options, Parser, Tag};

pub fn markdown_to_rst<W: fmt::Write>(
    out: &mut W,
    markdown: &str,
    write_reference: &dyn Fn(&ast::Path, &mut W) -> fmt::Result,
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
                    write_reference(
                        &text
                            .split("::")
                            .map(|s| Ident::from(s.to_string()))
                            .collect::<ast::Path>(),
                        out,
                    )?;
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
