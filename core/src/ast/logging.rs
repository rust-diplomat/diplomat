use std::sync::RwLock;

use crate::ast::{Ident, SpanLocation};

/// For overwriting by tests.
static WRITER: RwLock<&(dyn Fn() -> Box<dyn std::io::Write> + Send + Sync)> =
    RwLock::new(&(|| Box::new(std::io::stderr())));

pub(crate) fn create_report(id: Ident, title: String, label: String) -> ! {
    use std::io::Write;
    let mut out = WRITER.read().unwrap()();

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
        if sp.range.is_empty() && (sp.start.line != sp.end.line || sp.end.col - sp.start.col > 0) {
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

    if let Some(b) = &bytes_range {
        // If we go past the length, then we've somehow got the wrong SpanLocation.
        if matches!(
            span.as_ref().map(|s| &s.span_location),
            Some(SpanLocation::FilePath(..)) | Some(SpanLocation::LocalSource(..))
        ) && b.end > src.len()
        {
            panic!("Span source improperly calculated. Got range {0} > {1}. Original error: {title}: {label}", b.end, src.len());
        }
    }
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
        writeln!(out, "{}", renderer.render(report)).expect("Could not write report.");
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
            (
                "<No associated span>".into(),
                "",
                "<Excerpt not available>",
                "",
            )
        };
        // Ansi escape codes to provide emphasis.
        // Color red, bold:
        write!(out, "\x1b[1;31m").expect("Could not write to report.");
        write!(out, "Diplomat error: ").expect("Could not write to report.");
        // Reset:
        write!(out, "\x1b[0m").expect("Could not write to report.");
        writeln!(out, "{title}").expect("Could not write to report.");
        writeln!(out, "In {location}:").expect("Could not write to report.");

        let excerpt_pre_trimmed = excerpt_pre.trim_start();

        if !excerpt_pre.is_empty() {
            write!(out, "...{}", excerpt_pre_trimmed).expect("Could not write to report.");
        }

        // Color red, bold, underline:
        write!(out, "\x1b[1;4;31m").expect("Could not write to report.");
        write!(out, "{excerpt}").expect("Could not write to report.");
        // Reset:
        write!(out, "\x1b[0m").expect("Could not write to report.");

        if !excerpt_post.is_empty() {
            writeln!(out, "{}...", excerpt_post.trim_end()).expect("Could not write to report.");
        }

        // Clarify that above is the source, and below is the label attached to the source:
        // Color blue, bold:
        write!(out, "\x1b[1;34m").expect("Could not write to report.");
        if !excerpt.is_empty() {
            // This works well for most one-line excerpts.
            // The pretty-printer tends to handle whitespacing better, however.
            writeln!(
                out,
                "{}{}",
                " ".repeat(3 + excerpt_pre_trimmed.len()),
                "^".repeat(excerpt.len())
            )
            .expect("Could not write to report.");
        }

        // Blue on the new line, just in case newlines reset in some terminals:
        write!(out, "\x1b[1;34m").expect("Could not write to report.");
        write!(out, "{label}").expect("Could not write to report.");
        // Reset:
        writeln!(out, "\x1b[0m").expect("Could not write to report.");
    }
    out.flush().expect("Could not write to output.");
    // Rust-analyzer will not show error messages unless we panic,
    // This just tells rust-analyzer users to check stderr:
    panic!("{} (check stderr for more)", title);
}

#[cfg(all(test, not(feature = "pretty-print")))]
mod tests {
    use std::fmt::Write;

    #[derive(Clone, Debug)]
    struct StderrWrapper {
        buf: String,
    }
    impl std::io::Write for StderrWrapper {
        fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
            let st = str::from_utf8(buf).expect("Could not read utf8");
            self.buf.write_str(st).expect("Could not write str");
            Ok(buf.len())
        }

        fn flush(&mut self) -> std::io::Result<()> {
            insta::assert_snapshot!(self.buf);
            Ok(())
        }
    }

    fn reader_fn() -> Box<dyn std::io::Write> {
        Box::new(StderrWrapper { buf: String::new() })
    }

    fn parse_file_hook_errors(file_loc: &str, suffix: &str) {
        let crate_dir = env!("CARGO_MANIFEST_DIR");
        let local_path = format!("src/ast/snapshots/span_testing/{file_loc}");
        let file_path = std::path::Path::new(crate_dir).join(&local_path);

        let mut settings = insta::Settings::clone_current();
        settings.set_snapshot_suffix(format!("{file_loc}_{suffix}"));
        settings.set_snapshot_path("snapshots/span_testing");
        let _drop = settings.bind_to_scope();

        {
            let mut inner = super::WRITER.try_write().unwrap();
            *inner = &reader_fn;
        }

        let st = std::fs::read_to_string(&file_path).expect("Could not read file.");
        let p = syn::parse_str::<syn::ItemMod>(&st).expect("Could not parse syn mod");
        crate::ast::Module::from_syn(
            &p,
            true,
            None,
            &crate::ast::SpanLocation::FilePath(local_path),
        );
    }

    const FILES_TO_TEST: &[&str] = &["duplicate_attrs.rs", "enum_field_variant.rs"];

    fn test_file_list(suffix: &'static str) {
        let mut threads = vec![];
        for f in FILES_TO_TEST {
            let t = std::thread::spawn(|| {
                parse_file_hook_errors(f, suffix);
            });
            threads.push(t);
        }
        for t in threads {
            // The panic will still be printed, but we don't care about panicking or not:
            let res = t.join();
            match res {
                Ok(_) => {}
                Err(p) => {
                    if let Some(st) = p.downcast_ref::<String>() {
                        if st.contains("snapshot assertion") {
                            panic!("{st}");
                        }
                    } else {
                        panic!("Could not convert error to string.");
                    }
                }
            }
        }
    }

    #[cfg(not(feature = "pretty-print"))]
    #[test]
    fn test_errors_ugly() {
        test_file_list("ugly");
    }
}
