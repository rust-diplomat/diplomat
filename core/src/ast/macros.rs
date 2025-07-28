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
        let m = input.mac.parse_body::<TokenStream>();
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
        let m: syn::Result<TokenStream> = input.mac.parse_body();
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

macro_rules! define_macro_fragments {
    ($($i:ident($p:path)),*) => {
        #[derive(Debug)]
        pub enum MacroFrag {
            $(
                $i($p),
            )*
        }

        impl ToTokens for MacroFrag {
            fn to_token_stream(&self) -> TokenStream {
                let mut tokens = TokenStream::new();
                self.to_tokens(&mut tokens);
                tokens
            }

            fn to_tokens(&self, tokens : &mut TokenStream) {
                match self {
                    $(
                        MacroFrag::$i(item) => item.to_tokens(tokens),
                    )*
                }
            }

            fn into_token_stream(self) -> TokenStream
            where
                Self: Sized,
            {
                match self {
                    $(
                        MacroFrag::$i(item) => item.to_token_stream(),
                    )*
                }
            }
        }
    }
}

define_macro_fragments! {
    Block(syn::Block),
    Expr(syn::Expr),
    Ident(syn::Ident),
    Item(syn::Item),
    Lifetime(syn::Lifetime),
    Literal(syn::Lit),
    Meta(syn::Meta),
    Pat(syn::Pat),
    // TODO:
    // PatParam()
    Path(syn::Path),
    Stmt(syn::Stmt),
    TokenTree(proc_macro2::TokenTree),
    Ty(syn::Type),
    Vis(syn::Visibility)
}

#[derive(Debug, Clone)]
/// Represents $Identifier:MacroFragSpec (see https://doc.rust-lang.org/reference/macros-by-example.html#railroad-MacroMatch)
pub struct MacroIdent {
    /// represents Identifier.
    pub ident: Ident,
    /// Represents MacroFragSpec.
    /// Currently unused, since [`MacroUse::args`] will always expect to read an [`Expr`] when a macro is used.
    /// This is a very hands-off approach to parsing arguments in the usage of macros, and will work in most cases.
    /// However, this makes some valid macro frag specs unusable in the current parser (I believe :block doesn't work, for instance).
    /// The hope is that this will eventually be used in a more advanced parser.
    pub ty: Ident,
}

impl Parse for MacroIdent {
    fn parse(input: parse::ParseStream) -> syn::Result<Self> {
        input.parse::<Token![$]>()?;
        let ident: Ident = input.parse()?;
        input.parse::<Token![:]>()?;
        let ty: Ident = input.parse()?;
        Ok(Self { ident, ty })
    }
}

#[derive(Debug)]
#[non_exhaustive]
pub struct MacroUse {
    args: Vec<MacroFrag>,
}

impl MacroUse {
    fn parse(def: &MacroDef, stream: TokenStream) -> Self {
        let mut args = Vec::new();

        Self { args }
    }
}

#[derive(Debug)]
pub enum MacroMatch {
    Tokens(TokenStream),
    MacroMatcher(MacroMatcher),
    Ident(MacroIdent),
    // TODO: $(MacroMatch+) MacroRepSep? MacroRepOp
}

macro_rules! accepted_tokens {
    ($lookahead:ident, $tokens:ident, $input:ident, [$($i:path),+], [$($p:ident),+]) => {
        $(
            if $lookahead.peek($i) {
                $input.parse::<$i>()?.to_tokens(&mut $tokens);
            }
        )*
        $(
            if $lookahead.peek(syn::token::$p) {

                $input.parse::<syn::token::$p>()?.to_tokens(&mut $tokens);
            }
        )*
    };
}

impl Parse for MacroMatch {
    fn parse(input: parse::ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();
        let mut tokens = TokenStream::new();

        if lookahead.peek(Token![$]) {
            return Ok(MacroMatch::Ident(input.parse()?));
        } else if lookahead.peek(syn::token::Brace) {
            return Ok(MacroMatch::MacroMatcher(input.parse()?));
        } else if lookahead.peek(syn::token::Bracket) {
            return Ok(MacroMatch::MacroMatcher(input.parse()?));
        } else if lookahead.peek(syn::token::Paren) {
            return Ok(MacroMatch::MacroMatcher(input.parse()?));
        }

        accepted_tokens!(
            lookahead,
            tokens,
            input,
            [syn::Ident, syn::Lit, syn::Lifetime],
            [
                Eq, Lt, Le, EqEq, Ne, Ge, Gt, AndAnd, OrOr, Not, Tilde, Plus, Minus, Star, Slash,
                Percent, Caret, And, Or, Shl, Shr, PlusEq, MinusEq, StarEq, SlashEq, PercentEq,
                CaretEq, AndEq, OrEq, ShrEq, ShlEq, At, Dot, DotDot, DotDotDot, DotDotEq, Comma,
                Semi, Colon, PathSep, RArrow, LArrow, FatArrow, Pound, Question, Underscore
            ]
        );

        if !tokens.is_empty() {
            Ok(MacroMatch::Tokens(tokens))
        } else {
            Err(Error::new(
                input.span(),
                format!("Did not recognize token. {:?}", input),
            ))
        }
    }
}

#[derive(Debug)]
pub struct MacroMatcher {
    pub delim: proc_macro2::Delimiter,
    pub matches: Vec<MacroMatch>,
}

impl Parse for MacroMatcher {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();

        let mut matches = Vec::new();

        let delim;
        let arm;

        if lookahead.peek(token::Paren) {
            delim = proc_macro2::Delimiter::Parenthesis;
            parenthesized!(arm in input);
        } else if lookahead.peek(token::Brace) {
            delim = proc_macro2::Delimiter::Brace;
            braced!(arm in input);
        } else if lookahead.peek(token::Bracket) {
            delim = proc_macro2::Delimiter::Bracket;
            bracketed!(arm in input);
        } else {
            return Err(Error::new(input.span(), "Expected {}, (), or []"));
        }

        while !arm.is_empty() {
            matches.push(arm.parse::<MacroMatch>()?);
        }

        Ok(Self { delim, matches })
    }
}

#[derive(Debug)]
#[non_exhaustive]
/// Struct for defining a macro (i.e., `macro_rules! example`)
pub struct MacroDef {
    pub matcher: MacroMatcher,
    pub body: TokenStream,
}

impl Parse for MacroDef {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        // Read the matcher:
        let matcher = input.parse::<MacroMatcher>()?;

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

        Ok(Self { matcher, body })
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

    // fn parse_group(&self, matched: &MacroUse, inner: Cursor) -> TokenStream {
    //     let mut stream = TokenStream::new();

    //     let mut c = inner;
    //     while let Some((tt, next)) = c.token_tree() {
    //         match &tt {
    //             TokenTree::Punct(p) if p.as_char() == '$' => {
    //                 if let Some((tt, next)) = next.token_tree() {
    //                     if let TokenTree::Ident(i) = tt {
    //                         let arg = *self
    //                             .match_tokens
    //                             .get(&i)
    //                             .unwrap_or_else(|| panic!("Could not find arg ${i:?}"));
    //                         matched.args[arg].to_tokens(&mut stream);
    //                         c = next;
    //                     } else {
    //                         panic!("Expected ident next to $, got {tt:?}");
    //                     }
    //                 } else {
    //                     panic!("Expected token tree.");
    //                 }
    //             }
    //             TokenTree::Group(g) => {
    //                 let (inner, _, next) = c.group(g.delimiter()).unwrap();
    //                 let group =
    //                     proc_macro2::Group::new(g.delimiter(), self.parse_group(matched, inner));
    //                 // Once we detect a group, we push it to the array for syn to evaluate.
    //                 stream.append(group);
    //                 c = next;
    //             }
    //             _ => {
    //                 stream.append(tt);
    //                 c = next
    //             }
    //         }
    //     }

    //     stream
    // }

    // fn evaluate_buf(&self, matched: MacroUse) -> TokenStream {
    //     let mut stream = TokenStream::new();

    //     let buf = TokenBuffer::new2(self.body.clone());
    //     let mut c = buf.begin();
    //     // Search until we find a token to replace:
    //     while let Some((tt, next)) = c.token_tree() {
    //         match &tt {
    //             TokenTree::Punct(punct) if punct.as_char() == '$' => {
    //                 if let Some((tt, next)) = next.token_tree() {
    //                     if let TokenTree::Ident(i) = tt {
    //                         let arg = *self
    //                             .match_tokens
    //                             .get(&i)
    //                             .unwrap_or_else(|| panic!("Could not find arg ${i:?}"));
    //                         matched.args[arg].to_tokens(&mut stream);
    //                         c = next;
    //                     } else {
    //                         panic!("Expected ident next to $, got {tt:?}");
    //                     }
    //                 } else {
    //                     panic!("Expected token tree.");
    //                 }
    //             }
    //             TokenTree::Group(g) => {
    //                 let (inner, _, next) = c.group(g.delimiter()).unwrap();
    //                 // We need to read inside of any groups to find and replace `$` idents.
    //                 let group =
    //                     proc_macro2::Group::new(g.delimiter(), self.parse_group(&matched, inner));
    //                 stream.append(group);
    //                 c = next;
    //             }
    //             _ => {
    //                 stream.append(tt);
    //                 c = next
    //             }
    //         }
    //     }

    //     stream
    // }

    fn evaluate<T: Parse + Debug>(&self, matched: TokenStream) -> Vec<T> {
        let macro_use = MacroUse::parse(self, matched);
        // let stream = self.evaluate_buf(macro_use);
        let stream = TokenStream::new();

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
