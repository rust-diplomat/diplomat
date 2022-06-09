use proc_macro2::Span;
use quote::{quote, ToTokens};
use serde::{Deserialize, Serialize};
use std::fmt;

use super::Ident;

/// A named lifetime, e.g. `'a`.
///
/// # Invariants
///
/// Cannot be `'static` or `'_`, use [`Lifetime`] to represent those instead.
#[derive(Clone, Debug, Hash, Eq, PartialEq, Serialize, PartialOrd, Ord)]
#[serde(transparent)]
pub struct NamedLifetime(Ident);

impl<'de> Deserialize<'de> for NamedLifetime {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Special `Deserialize` impl to ensure invariants.
        let named = Ident::deserialize(deserializer)?;
        if named.as_str() == "static" {
            panic!("cannot be static");
        }
        Ok(NamedLifetime(named))
    }
}

impl From<&syn::Lifetime> for NamedLifetime {
    fn from(lt: &syn::Lifetime) -> Self {
        Lifetime::from(lt).to_named().expect("cannot be static")
    }
}

impl From<&NamedLifetime> for NamedLifetime {
    fn from(this: &NamedLifetime) -> Self {
        this.clone()
    }
}

impl PartialEq<syn::Lifetime> for NamedLifetime {
    fn eq(&self, other: &syn::Lifetime) -> bool {
        other.ident == self.0.as_str()
    }
}

impl fmt::Display for NamedLifetime {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "'{}", self.0)
    }
}

impl ToTokens for NamedLifetime {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        use proc_macro2::{Punct, Spacing};
        Punct::new('\'', Spacing::Joint).to_tokens(tokens);
        self.0.to_tokens(tokens);
    }
}

/// A lifetime dependency graph used for tracking which lifetimes outlive,
/// and are outlived by, other lifetimes.
///
/// It is similar to [`syn::LifetimeDef`], except it can also track lifetime
/// bounds defined in the `where` clause.
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct LifetimeEnv {
    nodes: Vec<LifetimeNode>,
}

impl LifetimeEnv {
    /// Construct an empty [`LifetimeEnv`].
    ///
    /// To create one outside of this module, use `LifetimeEnv::from_method_item`
    /// or `LifetimeEnv::from` on `&syn::Generics`.
    fn new() -> Self {
        Self { nodes: vec![] }
    }

    /// Collect all lifetimes that live _at least_ as long as _any_ of the
    /// provided `root_lifetimes`.
    ///
    /// This method performs a depth-first search through the lifetime graph,
    /// following edges from short lifetimes to longer lifetimes. Since lifetime
    /// bounds are transitive, this traversal ensures that every visited lifetime
    /// will outlive the root lifetime, and furthermore, DFS ensures that _all_
    /// provably longer lifetimes than the root lifetime are visited. This method
    /// also tracks which lifetimes have already been visited, making it robust
    /// to cycles.
    pub fn outlives<'i, 's, I>(&'s self, root_lifetimes: I) -> Vec<&'s NamedLifetime>
    where
        I: IntoIterator<Item = &'i NamedLifetime>,
    {
        let iter = root_lifetimes.into_iter();
        let mut outlives = Vec::with_capacity(iter.size_hint().1.unwrap_or(0));

        // Track visited lifetimes to avoid cycles.
        let mut visited = vec![false; self.nodes.len()];

        iter.filter_map(|named| {
            // Lifetimes that don't have a position aren't in the graph,
            // and thus have no known lifetimes that outlive them.
            self.iter().position(|graph_named| graph_named == named)
        })
        .for_each(|root_id| {
            dfs(root_id, &self.nodes[..], &mut outlives, &mut visited[..]);
        });

        /// Perform recursive DFS on a lifetime's sub-lifetimes.
        fn dfs<'a>(
            id: usize,
            nodes: &'a [LifetimeNode],
            outlives: &mut Vec<&'a NamedLifetime>,
            visited: &mut [bool],
        ) {
            // Note: all of these indexings SHOULD be valid because
            // `visited.len() == nodes.len()`, and the ids come from
            // calling `Iterator::position` on `nodes`, which never shrinks.
            // So we should be able to change these to `get_unchecked`...
            if !visited[id] {
                visited[id] = true;

                let node = &nodes[id];
                outlives.push(&node.lifetime);
                for &longer_id in node.longer.iter() {
                    dfs(longer_id, nodes, outlives, visited);
                }
            }
        }

        outlives
    }

    /// Constructs a new [`LifetimeEnv`] from a [`syn::ImplItemMethod`] and optional
    /// generic lifetimes from the `impl` block.
    pub fn from_method_item(
        method: &syn::ImplItemMethod,
        impl_generics: Option<&syn::Generics>,
    ) -> Self {
        let mut this = LifetimeEnv::new();
        // The impl generics _must_ be loaded into the env first, since the method
        // generics might use lifetimes defined in the impl, and `extend_generics`
        // panics if `'a: 'b` where `'b` isn't declared by the time it finishes.
        if let Some(generics) = impl_generics {
            this.extend_generics(generics);
        }
        this.extend_generics(&method.sig.generics);
        this
    }

    /// Add the lifetimes from generic parameters and where bounds.
    fn extend_generics(&mut self, generics: &syn::Generics) {
        let generic_bounds = generics.params.iter().map(|generic| match generic {
            syn::GenericParam::Type(_) => panic!("generic types are unsupported"),
            syn::GenericParam::Lifetime(def) => (&def.lifetime, &def.bounds),
            syn::GenericParam::Const(_) => panic!("const generics are unsupported"),
        });

        let generic_defs = generic_bounds.clone().map(|(lifetime, _)| lifetime);

        self.extend_lifetimes(generic_defs);
        self.extend_bounds(generic_bounds);

        if let Some(ref where_clause) = generics.where_clause {
            self.extend_bounds(where_clause.predicates.iter().map(|pred| match pred {
                syn::WherePredicate::Type(_) => panic!("trait bounds are unsupported"),
                syn::WherePredicate::Lifetime(pred) => (&pred.lifetime, &pred.bounds),
                syn::WherePredicate::Eq(_) => panic!("eq bounds are unsupported"),
            }));
        }
    }

    /// Returns the number of lifetimes in the graph.
    pub fn len(&self) -> usize {
        self.nodes.len()
    }

    /// Returns `true` if the graph contains no lifetimes.
    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }

    /// `<'a, 'b, 'c>`
    ///
    /// Write the existing lifetimes, excluding bounds, as generic parameters.
    ///
    /// To include lifetime bounds, use [`LifetimeEnv::lifetime_defs_to_tokens`].
    pub fn lifetimes_to_tokens(&self) -> proc_macro2::TokenStream {
        if self.is_empty() {
            return quote! {};
        }

        let lifetimes = self.iter();
        quote! { <#(#lifetimes),*> }
    }

    pub fn iter(&self) -> impl Iterator<Item = &NamedLifetime> {
        self.nodes.iter().map(|node| &node.lifetime)
    }

    /// Returns the index of a lifetime in the graph, or `None` if the lifetime
    /// isn't in the graph.
    fn id<L>(&mut self, lifetime: &L) -> Option<usize>
    where
        NamedLifetime: PartialEq<L>,
    {
        self.nodes
            .iter()
            .position(|node| &node.lifetime == lifetime)
    }

    /// Add isolated lifetimes to the graph.
    fn extend_lifetimes<'a, L, I>(&mut self, iter: I)
    where
        NamedLifetime: PartialEq<L> + From<&'a L>,
        L: 'a,
        I: IntoIterator<Item = &'a L>,
    {
        for lifetime in iter {
            if self.id(lifetime).is_some() {
                panic!(
                    "lifetime name `{}` declared twice in the same scope",
                    NamedLifetime::from(lifetime)
                );
            }

            self.nodes.push(LifetimeNode {
                lifetime: lifetime.into(),
                shorter: vec![],
                longer: vec![],
            });
        }
    }

    /// Add edges to the lifetime graph.
    ///
    /// This method is decoupled from [`LifetimeEnv::extend_lifetimes`] because
    /// generics can define new lifetimes, while `where` clauses cannot.
    ///
    /// # Panics
    ///
    /// This method panics if any of the lifetime bounds aren't already defined
    /// in the graph. This isn't allowed by rustc in the first place, so it should
    /// only ever occur when deserializing an invalid [`LifetimeEnv`].
    fn extend_bounds<'a, L, B, I>(&mut self, iter: I)
    where
        NamedLifetime: PartialEq<L> + From<&'a L>,
        L: 'a,
        B: IntoIterator<Item = &'a L>,
        I: IntoIterator<Item = (&'a L, B)>,
    {
        for (lifetime, bounds) in iter {
            let long = self.id(lifetime).expect("use of undeclared lifetime, this is a bug: try calling `LifetimeEnv::extend_lifetimes` first");
            for bound in bounds {
                let short = self
                    .id(bound)
                    .expect("cannot use undeclared lifetime as a bound");
                self.nodes[short].longer.push(long);
                self.nodes[long].shorter.push(short);
            }
        }
    }
}

impl fmt::Display for LifetimeEnv {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.to_token_stream().fmt(f)
    }
}

impl From<&syn::Generics> for LifetimeEnv {
    fn from(generics: &syn::Generics) -> Self {
        let mut this = LifetimeEnv::new();
        this.extend_generics(generics);
        this
    }
}

impl ToTokens for LifetimeEnv {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        for node in self.nodes.iter() {
            let lifetime = &node.lifetime;
            if node.shorter.is_empty() {
                tokens.extend(quote! { #lifetime, });
            } else {
                let bounds = node.shorter.iter().map(|&id| &self.nodes[id].lifetime);
                tokens.extend(quote! { #lifetime: #(#bounds)+*, });
            }
        }
    }
}

/// Serialize a [`LifetimeEnv`] as a map from lifetimes to their bounds.
impl Serialize for LifetimeEnv {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut seq = serializer.serialize_map(Some(self.len()))?;

        for node in self.nodes.iter() {
            /// Helper type for serializing bounds.
            struct Bounds<'a> {
                ids: &'a [usize],
                nodes: &'a [LifetimeNode],
            }

            impl<'a> Serialize for Bounds<'a> {
                fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                where
                    S: serde::Serializer,
                {
                    use serde::ser::SerializeSeq;
                    let mut seq = serializer.serialize_seq(Some(self.ids.len()))?;
                    for &id in self.ids {
                        seq.serialize_element(&self.nodes[id].lifetime)?;
                    }
                    seq.end()
                }
            }

            seq.serialize_entry(
                &node.lifetime,
                &Bounds {
                    ids: &node.shorter[..],
                    nodes: &self.nodes,
                },
            )?;
        }
        seq.end()
    }
}

impl<'de> Deserialize<'de> for LifetimeEnv {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use std::collections::BTreeMap;

        let m: BTreeMap<NamedLifetime, Vec<NamedLifetime>> =
            Deserialize::deserialize(deserializer)?;

        let mut this = LifetimeEnv::new();
        this.extend_lifetimes(m.keys());
        this.extend_bounds(m.iter());
        Ok(this)
    }
}

/// A lifetime, along with ptrs to all lifetimes that are explicitly
/// shorter/longer than it.
///
/// This type is internal to [`LifetimeGraph`]- the ptrs are stored as `usize`s,
/// meaning that they may be invalid if a `LifetimeEdges` is created in one
/// `LifetimeGraph` and then used in another.
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
struct LifetimeNode {
    /// The name of the lifetime.
    lifetime: NamedLifetime,

    /// Pointers to all lifetimes that this lives _at least_ as long as.
    ///
    /// Note: This doesn't account for transitivity.
    shorter: Vec<usize>,

    /// Pointers to all lifetimes that live _at least_ as long as this.
    ///
    /// Note: This doesn't account for transitivity.
    longer: Vec<usize>,
}

/// A lifetime, analogous to [`syn::Lifetime`].
#[derive(Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub enum Lifetime {
    /// The `'static` lifetime.
    Static,

    /// A named lifetime, like `'a`.
    Named(NamedLifetime),

    /// An elided lifetime.
    Anonymous,
}

impl Lifetime {
    pub fn to_named(self) -> Option<NamedLifetime> {
        if let Lifetime::Named(named) = self {
            return Some(named);
        }
        None
    }
}

impl fmt::Display for Lifetime {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Lifetime::Static => "'static".fmt(f),
            Lifetime::Named(ref named) => named.fmt(f),
            Lifetime::Anonymous => "'_".fmt(f),
        }
    }
}

impl ToTokens for Lifetime {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self {
            Lifetime::Static => syn::Lifetime::new("'static", Span::call_site()).to_tokens(tokens),
            Lifetime::Named(ref s) => s.to_tokens(tokens),
            Lifetime::Anonymous => syn::Lifetime::new("'_", Span::call_site()).to_tokens(tokens),
        };
    }
}

impl From<&syn::Lifetime> for Lifetime {
    fn from(lt: &syn::Lifetime) -> Self {
        if lt.ident == "static" {
            Self::Static
        } else {
            Self::Named(NamedLifetime((&lt.ident).into()))
        }
    }
}

impl From<&Option<syn::Lifetime>> for Lifetime {
    fn from(lt: &Option<syn::Lifetime>) -> Self {
        lt.as_ref().map(Into::into).unwrap_or(Self::Anonymous)
    }
}

impl Lifetime {
    /// Converts the [`Lifetime`] back into an AST node that can be spliced into a program.
    pub fn to_syn(&self) -> Option<syn::Lifetime> {
        match *self {
            Self::Static => Some(syn::Lifetime::new("'static", Span::call_site())),
            Self::Anonymous => None,
            Self::Named(ref s) => Some(syn::Lifetime::new(&s.to_string(), Span::call_site())),
        }
    }
}
