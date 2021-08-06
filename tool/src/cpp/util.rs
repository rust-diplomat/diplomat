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
