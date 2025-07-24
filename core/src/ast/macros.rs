use std::{
    collections::{BTreeMap, HashMap},
    fmt::Debug,
};

use proc_macro2::{TokenStream, TokenTree};
use quote::{ToTokens, TokenStreamExt};
use syn::{
    braced, bracketed,
    buffer::{Cursor, TokenBuffer},
    parenthesized,
    parse::{self, Parse},
    token, Error, Expr, Ident, ImplItem, ImplItemMacro, Item, ItemMacro, Token,
};

#[derive(Default)]
pub struct Macros {
    defs: BTreeMap<Ident, MacroDef>,
}

impl Macros {
    pub fn new() -> Macros {
        Macros {
            defs: BTreeMap::new(),
        }
    }

    pub fn add_item_macro(&mut self, input: &ItemMacro) {
        assert!(
            input.ident.is_some(),
            "Expected macro_rules! def. Got {input:?}"
        );
        let m = input.mac.parse_body::<MacroDef>();
        if let Ok(mac) = m {
            let ident = input.ident.clone().unwrap();
            self.defs.insert(ident, mac);
        }
    }

    pub fn evaluate_item_macro(&self, input: &ItemMacro) -> Vec<Item> {
        assert!(input.ident.is_none(), "Expected macro usage. Got {input:?}");
        let m = input.mac.parse_body::<MacroUse>();
        if let Ok(mac) = m {
            // FIXME: Extremely hacky. In the future for importing macros, we'll want to do something else.
            let ident = input.mac.path.segments.last().unwrap().ident.clone();

            if let Some(def) = self.defs.get(&ident) {
                def.evaluate(mac)
            } else {
                panic!("Could not find definition for {ident}. Have you tried creating a #[diplomat::macro_rules] macro_rules! {ident} definition?");
            }
        } else {
            // We handle errors automatically in `diplomat/macro`
            Vec::new()
        }
    }

    pub fn evaluate_impl_item_macro(&self, input: &ImplItemMacro) -> Vec<ImplItem> {
        let m: syn::Result<MacroUse> = input.mac.parse_body();
        // FIXME: Extremely hacky. In the future for importing macros, we'll want to do something else.
        let path_ident = input.mac.path.segments.last().unwrap().ident.clone();

        if let Ok(matched) = m {
            if let Some(def) = self.defs.get(&path_ident) {
                def.evaluate(matched)
            } else {
                panic!("Could not find definition for {path_ident}. Have you tried creating a #[diplomat::macro_rules] macro_rules! {path_ident} definition?");
            }
        } else {
            // We handle errors automatically in `diplomat/macro`
            Vec::new()
        }
    }
}

#[derive(Debug)]
#[non_exhaustive]
pub struct MacroUse {
    pub args: Vec<Expr>,
}

impl Parse for MacroUse {
    fn parse(input: parse::ParseStream) -> syn::Result<Self> {
        let mut args = Vec::new();

        // TODO: Need some custom logic based on the parent macro definition.
        let args_p = input.parse_terminated(Expr::parse, Token![,])?;
        for a in args_p {
            args.push(a);
        }

        Ok(Self { args })
    }
}

#[derive(Debug, Clone)]
pub struct MacroIdent {
    pub ident: Ident,
    // To be used in future arg parsing:
    pub _ty: Ident,
}

impl Parse for MacroIdent {
    fn parse(input: parse::ParseStream) -> syn::Result<Self> {
        input.parse::<Token![$]>()?;
        let ident: Ident = input.parse()?;
        input.parse::<Token![:]>()?;
        let ty: Ident = input.parse()?;
        Ok(Self { ident, _ty: ty })
    }
}

#[derive(Debug)]
#[non_exhaustive]
pub struct MacroDef {
    pub match_tokens: HashMap<Ident, usize>,
    pub body: TokenStream,
}

impl Parse for MacroDef {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();

        // Read the matcher:
        let arm;
        if lookahead.peek(token::Paren) {
            parenthesized!(arm in input);
        } else if lookahead.peek(token::Brace) {
            braced!(arm in input);
        } else if lookahead.peek(token::Bracket) {
            bracketed!(arm in input);
        } else {
            return Err(Error::new(input.span(), "Expected {}, (), or []"));
        }

        // TODO: This is not a comma separated list in actuality, at some point we'll want to add more complicated parsing.
        let punc = arm.parse_terminated(MacroIdent::parse, Token![,])?;

        let match_tokens = punc
            .iter()
            .cloned()
            .enumerate()
            .map(|(c, i)| (i.ident, c))
            .collect();

        let _arrow = input.parse::<Token![=>]>()?;

        // Now the expansion:
        let arm_body;
        braced!(arm_body in input);

        let body = arm_body.parse::<TokenStream>()?;

        let _semicolon = input.parse::<Token![;]>()?;

        if !input.is_empty() {
            return Err(syn::Error::new(
                input.span(),
                "Diplomat does not support macros of more than one arm.",
            ));
        }

        // We don't support any other rules, so we ignore them.

        Ok(Self { match_tokens, body })
    }
}

impl MacroDef {
    pub fn validate(input: ItemMacro) -> TokenStream {
        let r = input.mac.parse_body::<Self>();

        if let Err(e) = r {
            e.to_compile_error()
        } else {
            TokenStream::default()
        }
    }

    fn parse_group(&self, matched: &MacroUse, inner: Cursor) -> TokenStream {
        let mut stream = TokenStream::new();

        let mut c = inner;
        while let Some((tt, next)) = c.token_tree() {
            match &tt {
                TokenTree::Punct(p) if p.as_char() == '$' => {
                    if let Some((tt, next)) = next.token_tree() {
                        if let TokenTree::Ident(i) = tt {
                            let arg = *self
                                .match_tokens
                                .get(&i)
                                .unwrap_or_else(|| panic!("Could not find arg ${i:?}"));
                            matched.args[arg].to_tokens(&mut stream);
                            c = next;
                        } else {
                            panic!("Expected ident next to $, got {tt:?}");
                        }
                    } else {
                        panic!("Expected token tree.");
                    }
                }
                TokenTree::Group(g) => {
                    let (inner, _, next) = c.group(g.delimiter()).unwrap();
                    let group =
                        proc_macro2::Group::new(g.delimiter(), self.parse_group(matched, inner));
                    // Once we detect a group, we push it to the array for syn to evaluate.
                    stream.append(group);
                    c = next;
                }
                _ => {
                    stream.append(tt);
                    c = next
                }
            }
        }

        stream
    }

    fn evaluate_buf(&self, matched: MacroUse) -> TokenStream {
        let mut stream = TokenStream::new();

        let buf = TokenBuffer::new2(self.body.clone());
        let mut c = buf.begin();
        // Search until we find a token to replace:
        while let Some((tt, next)) = c.token_tree() {
            match &tt {
                TokenTree::Punct(punct) if punct.as_char() == '$' => {
                    if let Some((tt, next)) = next.token_tree() {
                        if let TokenTree::Ident(i) = tt {
                            let arg = *self
                                .match_tokens
                                .get(&i)
                                .unwrap_or_else(|| panic!("Could not find arg ${i:?}"));
                            matched.args[arg].to_tokens(&mut stream);
                            c = next;
                        } else {
                            panic!("Expected ident next to $, got {tt:?}");
                        }
                    } else {
                        panic!("Expected token tree.");
                    }
                }
                TokenTree::Group(g) => {
                    let (inner, _, next) = c.group(g.delimiter()).unwrap();
                    // We need to read inside of any groups to find and replace `$` idents.
                    let group =
                        proc_macro2::Group::new(g.delimiter(), self.parse_group(&matched, inner));
                    stream.append(group);
                    c = next;
                }
                _ => {
                    stream.append(tt);
                    c = next
                }
            }
        }

        stream
    }

    fn evaluate<T: Parse + Debug>(&self, matched: MacroUse) -> Vec<T> {
        let stream = self.evaluate_buf(matched);

        // Now we have a stream to read through. We read through the whole thing and assume each thing we read is a top level item.

        let maybe_list = syn::parse_str::<ItemList<T>>(&stream.to_string());
        if let Ok(i) = maybe_list {
            i.items
        } else {
            panic!("{:?}", maybe_list.unwrap_err());
        }
    }
}

#[derive(Debug)]
struct ItemList<T: Parse> {
    items: Vec<T>,
}

impl<T: Parse> Parse for ItemList<T> {
    fn parse(input: parse::ParseStream) -> syn::Result<Self> {
        let mut items = Vec::new();
        while !input.is_empty() {
            items.push(input.parse::<T>()?);
        }

        Ok(Self { items })
    }
}
