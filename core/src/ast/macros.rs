use std::collections::BTreeMap;

use proc_macro2::{Delimiter, TokenStream, TokenTree};
use quote::{ToTokens, TokenStreamExt};
use syn::{braced, bracketed, buffer::{Cursor, TokenBuffer}, parenthesized, parse::{self, Parse, ParseBuffer}, parse_macro_input, spanned::Spanned, token::{self, Token}, Error, Expr, ExprParen, Ident, Item, ItemMacro, PatParen, Path, Token};

pub struct Macros {
    defs : BTreeMap<Ident, MacroRules>,
}

impl Macros {
    pub fn new() -> Macros {
        Macros {
            defs: BTreeMap::new()
        }
    }

    pub fn read_item_macro(&mut self, input : &ItemMacro) -> Option<Vec<Item>> {
        let mac = Macro::from_syn(&input);
        if let Ok((ident, mac)) = mac {
            match mac {
                Macro::MacroRules(rules) => {
                    self.defs.insert(ident, rules);
                    // Macro rules add no new items: 
                    None
                },
                Macro::MacroMatch(matched) => {
                    if let Some(def) = self.defs.get(&ident) {
                        Some(def.evaluate(matched))
                    } else {
                        panic!("Could not find definition for {:?}", ident);
                    } 
                }
            }
        } else {
            // We handle errors automatically in `diplomat/macro`
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
    pub fn from_syn(input : &ItemMacro) -> Result<(Ident, Macro), syn::Error> {
        // Are we macro_rules!
        if let Some(ident) = &input.ident {
            let r = input.mac.parse_body()?;
            Ok((ident.clone(), Macro::MacroRules(r)))
        } else {
            let m = input.mac.parse_body()?;
            // FIXME: Extremely hacky.
            let path_ident = input.mac.path.segments.last().unwrap().ident.clone();
            Ok((path_ident, Macro::MacroMatch(m)))
        }
    }

    pub fn validate(input: ItemMacro) -> TokenStream {
        if input.ident.is_some() {
            let r = input.mac.parse_body::<MacroRules>();

            if let Ok(..) = r {
                TokenStream::default()
            } else {
                r.unwrap_err().to_compile_error()
            }
        } else {
            let m = input.mac.parse_body::<MacroMatch>();
            
            if let Ok(..) = m {
                TokenStream::default()
            } else {
                m.unwrap_err().to_compile_error()
            }
        }
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
    pub body : TokenStream,
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

        // FIXME: This is not a comma separated list in actuality.
        let punc = arm.parse_terminated(MacroIdent::parse, Token![,])?;

        let match_tokens = punc.iter().map(|i| { i.clone() }).collect();

        let _arrow = input.parse::<Token![=>]>()?;

        // Now the expansion:
        let arm_body;
        braced!(arm_body in input);

        let body = arm_body.parse::<TokenStream>()?;

        let _semicolon = input.parse::<Token![;]>()?;

        if !input.is_empty() {
            return Err(syn::Error::new(input.span(), "Diplomat does not support macros of more than one arm."));
        }
        
        // We don't support any other rules, so we ignore them.

        Ok(Self {
            match_tokens,
            body
        })
    }
}

impl MacroRules {
    fn evaluate(&self, matched : MacroMatch) -> Vec<Item> {
        let mut out = Vec::new();

        let mut stream = TokenStream::new();

        // Cheap trick to get syn to parse items for us:
        let mut streams = Vec::new();

        let buf = TokenBuffer::new2(self.body.clone());
        let mut c = buf.begin();
        // Search until we find a token to replace:
        while let Some((tt, next)) = c.token_tree() {
            match &tt {
                TokenTree::Punct(punct) if punct.as_char() == '$' => {
                    if let Some((tt, next)) = next.token_tree() {
                        if let TokenTree::Ident(i) = tt {
                            let arg = self.match_tokens.iter().position(|mi| {
                                mi.ident == i
                            });
                            matched.args[arg.expect(&format!("Could not find arg ${:?}", i))].to_tokens(&mut stream);
                            c = next;
                        } else {
                            panic!("Expected ident next to $, got {:?}", tt);
                        }
                    } else {
                        panic!("Expected token tree.");
                    }
                },
                TokenTree::Group(g) => {
                    // Once we detect a group, we push it to the array for syn to evaluate.
                    stream.append(TokenTree::Group(g.clone()));
                    streams.push(stream.clone());
                    stream = TokenStream::new();
                    c = next;
                },
                _ => {
                    stream.append(tt);
                    c = next
                },
            }
        }

        // Now we have a stream to read through. 

        for s in streams {
            let maybe_item = syn::parse_str::<syn::Item>(&s.to_string());

            if let Ok(i) = maybe_item {
                out.push(i);
            } else {
                panic!("{:?}", maybe_item.unwrap_err());
            }
        }
        

        out
    }
}
