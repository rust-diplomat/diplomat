use quote::ToTokens;
use syn::Attribute;

pub fn get_doc_lines(attrs: &[Attribute]) -> String {
    let mut lines: String = String::new();

    attrs.iter().for_each(|attr| {
        let maybe_ident = attr.path.get_ident();
        if maybe_ident.is_some() && *maybe_ident.unwrap() == "doc" {
            let literal_token = attr.tokens.clone().into_iter().nth(1).unwrap();
            let node: syn::LitStr = syn::parse2(literal_token.to_token_stream()).unwrap();
            let line = node.value().trim().to_string();

            if !lines.is_empty() {
                lines.push('\n');
            }

            lines.push_str(&line);
        }
    });

    lines
}
