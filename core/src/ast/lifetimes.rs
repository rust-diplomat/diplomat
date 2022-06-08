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
#[derive(Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct LifetimeEnv {
    edges: Vec<LifetimeEdges>,
}

impl LifetimeEnv {
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
        let mut visited = vec![false; self.edges.len()];

        iter.filter_map(|named| {
            // Lifetimes that don't have a position aren't in the graph,
            // and thus have no known lifetimes that outlive them.
            self.edges.iter().position(|edge| edge.lifetime == *named)
        })
        .for_each(|root_id| {
            dfs(root_id, &self.edges[..], &mut outlives, &mut visited[..]);
        });

        /// Perform recursive DFS on a lifetime's sub-lifetimes.
        fn dfs<'a>(
            id: usize,
            edges: &'a [LifetimeEdges],
            outlives: &mut Vec<&'a NamedLifetime>,
            visited: &mut [bool],
        ) {
            // Note: all of these indexings SHOULD be valid because
            // `visited.len() == edges.len()`, and the ids come from
            // calling `Iterator::position` on `edges`, which never shrinks.
            // So we should be able to change these to `get_unchecked`...
            if !visited[id] {
                visited[id] = true;

                let edge = &edges[id];
                outlives.push(&edge.lifetime);
                for &longer_id in edge.longer.iter() {
                    dfs(longer_id, edges, outlives, visited);
                }
            }
        }

        outlives
    }

    /// Add the lifetimes from generic parameters and where bounds.
    pub fn extend_generics(&mut self, generics: &syn::Generics) {
        // It's the responsibility of the validity checks to ensure
        // that all the generics are lifetime bounds, not generic types.
        self.extend_from_parts(generics.lifetimes().map(|def| (&def.lifetime, &def.bounds)));
        if let Some(ref where_clause) = generics.where_clause {
            self.extend_from_parts(where_clause.predicates.iter().map(|pred| match pred {
                syn::WherePredicate::Type(_) => panic!("trait bounds are unsupported"),
                syn::WherePredicate::Lifetime(pred) => (&pred.lifetime, &pred.bounds),
                syn::WherePredicate::Eq(_) => panic!("eq bounds are unsupported"),
            }));
        }
    }

    /// Returns the number of lifetimes in the graph.
    pub fn len(&self) -> usize {
        self.edges.len()
    }

    /// Returns `true` if the graph contains no lifetimes.
    pub fn is_empty(&self) -> bool {
        self.edges.is_empty()
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

        let lifetimes = self.edges.iter().map(|edge| &edge.lifetime);
        quote! { <#(#lifetimes),*> }
    }

    /// Helper function for `LifetimeGraph::extend_from_parts`,
    /// which allows for getting the id of an `Edge` associated with a lifetime.
    ///
    /// Returns `Ok` if the lifetime was already in the graph, and `Err` if a
    /// new lifetime had to be inserted.
    fn id_or_insert<L>(&mut self, lifetime: &L) -> Result<usize, usize>
    where
        NamedLifetime: PartialEq<L> + for<'a> From<&'a L>,
    {
        // We have to index because we need to call this twice in the same scope
        // in `extend_from_parts`, otherwise I would've just returned `&mut Edge`.
        if let Some(idx) = self
            .edges
            .iter()
            .position(|edge| &edge.lifetime == lifetime)
        {
            // The edge for this lifetime already exists.
            Ok(idx)
        } else {
            // The edge doesn't exist yet, create it and return its id.
            let id = self.edges.len();
            self.edges.push(LifetimeEdges {
                lifetime: lifetime.into(),
                shorter: vec![],
                longer: vec![],
            });
            Err(id)
        }
    }

    /// Extends the `LifetimeEnv` from an iterator of parts found in lifetime bounds.
    ///
    /// # Panics
    ///
    /// This method panics if, by the time all the parts are read in, there are
    /// any lifetimes that have been used as bounds but _not_ declared. This should
    /// only be of concern during deserialization.
    // These trait bounds look intimidating, here's what they do:
    //  * L: A `NamedLifetime`-like type
    //  * B: An iterator of L's
    //  * I: An iterator of (L, B), basically just each lifetime and its bounds.
    // The reason we have this is so we can extend from `syn` parts, or from
    // `&NamedLifetime`s for when we're deserializing.
    pub fn extend_from_parts<'a, I, L, B>(&mut self, iter: I)
    where
        NamedLifetime: PartialEq<L> + for<'b> From<&'b L>,
        L: 'a,
        B: 'a + IntoIterator<Item = &'a L>,
        I: IntoIterator<Item = (&'a L, B)>,
    {
        // Lifetimes that have been used as bounds, but not actually declared
        let mut undeclared = vec![];

        for (lifetime, bounds) in iter {
            // Since all the indices come from me, we could theoretically
            // change these to `get_unchecked`. But unsafe is spooky :/
            let long_id = match self.id_or_insert(lifetime) {
                Ok(id) => {
                    if let Some(undeclared_id) = undeclared
                        .iter()
                        .position(|undeclared_id| *undeclared_id == id)
                    {
                        // This lifetime was previously used as a bound, but now
                        // we're declaring it.
                        undeclared.swap_remove(undeclared_id);
                    }
                    id
                }
                Err(id) => id,
            };

            for bound in bounds {
                let short_id = self.id_or_insert(bound).unwrap_or_else(|id| {
                    // A lifetime has been used as a bound but hasn't been declared yet.
                    undeclared.push(id);
                    id
                });
                // This doesn't catch repeats. But that doesn't break anything
                // and it won't slow down the DFS, so...
                self.edges[short_id].longer.push(long_id);
                self.edges[long_id].shorter.push(short_id);
            }
        }

        if let Some((&first, rest)) = undeclared.split_first() {
            // There's at least one lifetime that's undeclared.
            use std::fmt::Write;
            let mut msg = format!(
                "used undeclared lifetimes in bounds: [{}",
                self.edges[first].lifetime
            );
            for &id in rest {
                write!(msg, ", {}", self.edges[id].lifetime).unwrap();
            }
            msg.write_char(']').unwrap();
            panic!("{}", msg)
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
        let mut this = Self::default();
        this.extend_generics(generics);
        this
    }
}

impl ToTokens for LifetimeEnv {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        for edge in self.edges.iter() {
            let lifetime = &edge.lifetime;
            if edge.shorter.is_empty() {
                tokens.extend(quote! { #lifetime, });
            } else {
                let bounds = edge.shorter.iter().map(|&id| &self.edges[id].lifetime);
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

        for edge in self.edges.iter() {
            /// Helper type for serializing bounds.
            struct Bounds<'a> {
                ids: &'a [usize],
                edges: &'a [LifetimeEdges],
            }

            impl<'a> Serialize for Bounds<'a> {
                fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                where
                    S: serde::Serializer,
                {
                    use serde::ser::SerializeSeq;
                    let mut seq = serializer.serialize_seq(Some(self.ids.len()))?;
                    for &id in self.ids {
                        seq.serialize_element(&self.edges[id].lifetime)?;
                    }
                    seq.end()
                }
            }

            seq.serialize_entry(
                &edge.lifetime,
                &Bounds {
                    ids: &edge.shorter[..],
                    edges: &self.edges,
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

        let mut this = LifetimeEnv::default();
        this.extend_from_parts(m.iter());
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
struct LifetimeEdges {
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
