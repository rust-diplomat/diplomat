use crate::ast::{Ident, SpanLocation};

pub(crate) fn create_report(id: Ident, title: String, label: String) -> ! {
    let span = id.span();
    let src = if let Some(sp) = &span {
        match &sp.span_location {
            SpanLocation::FilePath(f) => {
                let st = std::fs::read_to_string(f);
                if let Ok(s) = st {
                    s
                } else {
                    panic!("Could not read source file {f}: {:?}", st.unwrap_err());
                }
            }
            SpanLocation::LocalSource(src) => src.clone(),
            SpanLocation::None => "<unknown location>".into(),
        }
    } else {
        "<No associated span>".into()
    };

    let bytes_range = span.as_ref().map(|sp| {
        // Bytes range has not been stabilized in Rust macro.
        // We can't tell if we're in a proc macro context,
        // so we just check if the range doesn't make sense:
        if sp.range.len() == 0 && (sp.start.line != sp.end.line || sp.end.col - sp.start.col > 0) {
            match sp.span_location {
                SpanLocation::None => 0..0,
                _ => {
                    // Need an accurate byte count that accounts for both:
                    // CRLF and LF endings, so we just make sure to split on the end at `\n` (LF):
                    let split = src.split_inclusive('\n');
                    let mut start_byte = 0usize;
                    let mut end_byte = src.len();
                    let mut running_byte_total = 0usize;
                    for (idx, st) in split.enumerate() {
                        if (idx + 1) == sp.start.line {
                            start_byte = running_byte_total + sp.start.col;
                        }
                        if (idx + 1) == sp.end.line {
                            end_byte = running_byte_total + sp.end.col;
                            break;
                        }
                        running_byte_total += st.len();
                    }
                    start_byte..end_byte
                }
            }
        } else {
            sp.range.clone()
        }
    });
    #[cfg(feature = "pretty-print")]
    {
        use annotate_snippets::{renderer::DecorStyle, Level, Renderer, Snippet};
        let report = if let Some(sp) = span {
            use annotate_snippets::{Annotation, AnnotationKind};

            &[Level::ERROR.primary_title(&title).element(
                Snippet::<Annotation>::source(src)
                    .path(match sp.span_location {
                        SpanLocation::FilePath(f) => Some(f),
                        _ => None,
                    })
                    .annotation(
                        AnnotationKind::Context
                            .span(bytes_range.unwrap())
                            .label(label),
                    ),
            )]
        } else {
            &[Level::ERROR
                .primary_title(&title)
                .element(Level::ERROR.message(label))]
        };
        let renderer = Renderer::styled().decor_style(DecorStyle::Unicode);
        eprintln!("{}", renderer.render(report));
    }
    #[cfg(not(feature = "pretty-print"))]
    {
        let (location, excerpt_pre, excerpt, excerpt_post) = if let Some(sp) = span {
            let range = bytes_range.unwrap();
            let start = sp.start.line;
            // Columns are 0-indexed, but most editors use 1-indexing:
            let col = sp.start.col + 1;
            let span_location = match &sp.span_location {
                SpanLocation::FilePath(f) => f.clone(),
                SpanLocation::LocalSource(..) => "<Inline source>".into(),
                SpanLocation::None => "<Unknown location>".into(),
            };
            let range_pre = range.start.saturating_sub(5);
            let range_post = std::cmp::min(range.end.saturating_add(5), src.len());
            match sp.span_location {
                SpanLocation::None => (span_location, "", "<Excerpt not available>", ""),
                _ => (
                    format!("{span_location}:{start}:{col}"),
                    &src[range_pre..range.start],
                    &src[range.start..range.end],
                    &src[range.end..range_post],
                ),
            }
        } else {
            ("<No associated span>".into(), "", "<Excerpt not available>", "")
        };
        // Ansi escape codes to provide emphasis.
        // Color red, bold:
        eprint!("\x1b[1;31m");
        eprint!("Diplomat error: ");
        // Reset:
        eprint!("\x1b[0m");
        eprintln!("{title}");
        eprintln!("In {location}:");

        let excerpt_pre_trimmed = excerpt_pre.trim_start();

        if excerpt_pre.len() > 0 {
            eprint!("...{}", excerpt_pre_trimmed);
        }

        // Color red, bold, underline:
        eprint!("\x1b[1;4;31m");
        eprint!("{excerpt}");
        // Reset:
        eprint!("\x1b[0m");

        if excerpt_post.len() > 0 {
            eprintln!("{}...", excerpt_post.trim_end());
        }

        // Clarify that above is the source, and below is the label attached to the source:
        // Color blue, bold:
        eprint!("\x1b[1;34m");
        if excerpt.len() > 0 {
            // This works well for most one-line excerpts.
            // The pretty-printer tends to handle whitespacing better, however.
            eprintln!("{}{}", " ".repeat(3 + excerpt_pre_trimmed.len()), "^".repeat(excerpt.len()));
        }
        
        // Blue on the new line, just in case newlines reset in some terminals:
        eprint!("\x1b[1;34m");
        eprint!("{label}");
        // Reset:
        eprintln!("\x1b[0m");
    }
    // Rust-analyzer will not show error messages unless we panic,
    // This just tells rust-analyzer users to check stderr:
    panic!("{} (check stderr for more)", title);
}
