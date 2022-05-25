use diplomat_core::ast;
use std::fmt;

/// Transforms the given identifier to one that does not clash
/// with a keyword if it does.
pub fn transform_keyword_ident(ident: &ast::Ident) -> ast::Ident {
    // TODO(#60): handle other keywords
    match ident.as_str() {
        "new" | "default" => ast::Ident::from(format!("{}_", ident)),
        _ => ident.clone(),
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
