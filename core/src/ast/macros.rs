use std::collections::BTreeMap;

use proc_macro2::TokenStream;
use syn::{braced, bracketed, buffer::{Cursor, TokenBuffer}, parenthesized, parse::{self, Parse}, parse_macro_input, token::{self, Token}, Error, Expr, ExprParen, Ident, Item, ItemMacro, PatParen, Path, Token};

pub struct Macros {
    defs : BTreeMap<Path, Macro>,
}

impl Macros {
    pub fn new() -> Macros {
        Macros {
            defs: BTreeMap::new()
        }
    }

    pub fn read_item_macro(&mut self, input : &ItemMacro) -> Option<TokenStream> {
        let mac = Macro::from_syn(input);
        if let Ok((pth, mac)) = mac {
            println!("{:?}", pth);
            // m.body
            // self.defs.insert(m.ident.clone(), m);
            None
        } else {
            // FIXME:
            // panic!("{:?}", mac.unwrap_err());
            None
        }
    }
}

#[derive(Debug)]
pub enum Macro {
    MacroRules(MacroRules),
    MacroMatch(MacroMatch)
}

impl Macro {
    pub fn from_syn(input : &ItemMacro) -> Result<(Path, Macro), syn::Error> {
        // Are we macro_rules!
        if let Some(_) = &input.ident {
            Ok((input.mac.path.clone(), Macro::MacroRules(input.mac.parse_body()?)))
        } else {
            let m = input.mac.parse_body()?;
            Ok((input.mac.path.clone(), Macro::MacroMatch(m)))
        }
    }
}

#[derive(Debug, Clone)]
pub struct MacroIdent {
    pub ident : Ident,
    pub ty : Ident
}

impl Parse for MacroIdent {
    fn parse(input: parse::ParseStream) -> syn::Result<Self> {
        input.parse::<Token![$]>()?;
        let ident : Ident = input.parse()?;
        input.parse::<Token![:]>()?;
        let ty : Ident = input.parse()?;
        Ok(Self{ident, ty})
    }
}

#[derive(Debug)]
pub struct MacroRules {
    pub match_tokens : Vec<MacroIdent>,
    pub body : TokenStream
}

impl Parse for MacroRules {
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
            return Err(Error::new(input.span(), "Expected {}, (), or []"))
        }

        let punc = arm.parse_terminated(MacroIdent::parse, Token![,])?;

        let match_tokens = punc.iter().map(|i| { i.clone() }).collect();

        let _arrow = input.parse::<Token![=>]>()?;

        // Now the expansion:
        let arm_body;
        braced!(arm_body in input);

        let body = arm_body.cursor().token_stream();

        
        // We don't support any other rules, so we ignore them.

        Ok(Self {
            match_tokens,
            body
        })
    }
}

#[derive(Debug)]
pub struct MacroMatch {
    pub args : Vec<Expr>,
}

impl Parse for MacroMatch {
    fn parse(input: parse::ParseStream) -> syn::Result<Self> {
        let mut args = Vec::new();

        // TODO: Need some custom logic based on the parent macro definition.
        let args_p = input.parse_terminated(Expr::parse, Token![,])?;
        for a in args_p {
            args.push(a);
        }

        Ok(Self {
            args
        })
    }
}