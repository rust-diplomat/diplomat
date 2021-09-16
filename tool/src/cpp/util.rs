use std::fmt;

/// Transforms the given identifier to one that does not clash
/// with a keyword if it does.
pub fn transform_keyword_ident(ident: &str) -> String {
    // TODO(#60): handle other keywords
    if ident == "new" || ident == "default" {
        format!("{}_", ident)
    } else {
        ident.to_string()
    }
}

/// Generates a C++ comment block.
pub fn gen_comment_block<W: fmt::Write>(out: &mut W, comment: &str) -> fmt::Result {
    if !comment.is_empty() {
        writeln!(out)?;
        writeln!(out, "/**")?;
        for line in comment.lines() {
            writeln!(out, " * {}", line)?;
        }
        writeln!(out, " */")?;
    }
    Ok(())
}
