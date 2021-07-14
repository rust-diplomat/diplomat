use quote::ToTokens;
use syn::Attribute;

pub fn get_doc_lines(attrs: &[Attribute]) -> Vec<String> {
    let mut lines: Vec<String> = vec![];
    let mut string_so_far: Option<String> = None;

    attrs.iter().for_each(|attr| {
        let maybe_ident = attr.path.get_ident();
        if maybe_ident.is_some() && *maybe_ident.unwrap() == "doc" {
            let literal_token = attr.tokens.clone().into_iter().nth(1).unwrap();
            let node: syn::LitStr = syn::parse2(literal_token.to_token_stream()).unwrap();
            let line = node.value().trim().to_string();
            if line.is_empty() {
                if let Some(acc) = &string_so_far {
                    lines.push(acc.clone());
                    string_so_far = None;
                }
            } else if let Some(acc) = &mut string_so_far {
                acc.push(' ');
                acc.push_str(&line);
            } else {
                string_so_far = Some(line);
            }
        }
    });

    if let Some(acc) = &string_so_far {
        lines.push(acc.clone());
    }

    lines
}
