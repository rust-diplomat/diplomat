use core::fmt;
use core::fmt::Write as _;
use core::hash::Hash;

use diplomat_core::{ast, Env};

pub fn get_all_custom_types(env: &Env) -> SetOfAstTypes<&ast::CustomType> {
    let mut all_types = SetOfAstTypes::default();

    for (path, _name, symbol) in env.iter_items() {
        if let ast::ModSymbol::CustomType(c) = symbol {
            all_types.insert((path.clone(), c));
        }
    }

    all_types
}

pub struct CodeWriter<'io> {
    writer: &'io mut dyn fmt::Write,
    ind_level: usize,
    indentation: &'static str,
    scope_opening: &'static str,
    scope_closing: &'static str,
    is_indented: bool,
}

impl<'io> CodeWriter<'io> {
    pub fn new(
        writer: &'io mut dyn fmt::Write,
        indentation: &'static str,
        scope_opening: &'static str,
        scope_closing: &'static str,
    ) -> Self {
        Self {
            writer,
            ind_level: 0,
            indentation,
            scope_opening,
            scope_closing,
            is_indented: false,
        }
    }

    pub fn indent(&mut self) {
        self.ind_level += 1;
    }

    pub fn dedent(&mut self) {
        self.ind_level -= 1;
    }

    pub fn scope<F>(&mut self, func: F) -> fmt::Result
    where
        F: FnOnce(&mut CodeWriter<'_>) -> fmt::Result,
    {
        let scope_opening = self.scope_opening;
        writeln!(self, "{}", scope_opening)?;
        self.indent();

        func(self)?;

        self.dedent();
        let scope_closing = self.scope_closing;
        writeln!(self, "{}", scope_closing)?;

        Ok(())
    }
}

impl fmt::Write for CodeWriter<'_> {
    fn write_str(&mut self, mut input: &str) -> fmt::Result {
        loop {
            let next_line_break_idx = input.find('\n');

            if !self.is_indented {
                let should_indent = match next_line_break_idx {
                    Some(idx) => !input[..idx].is_empty(),
                    None => !input.is_empty(),
                };
                if should_indent {
                    for _ in 0..self.ind_level {
                        self.writer.write_str(self.indentation)?;
                    }
                }
                self.is_indented = true;
            }

            match next_line_break_idx {
                Some(idx) => {
                    self.writer.write_str(&input[..idx + 1])?;
                    input = &input[idx + 1..];
                    self.is_indented = false;
                }
                None => {
                    self.writer.write_str(input)?;
                    input = "";
                }
            }

            if input.is_empty() {
                break;
            }
        }

        Ok(())
    }
}

type AstElement<T> = (ast::Path, T);

/// Ordered set of AST types for deterministic traversal
pub struct SetOfAstTypes<T> {
    set: std::collections::HashSet<AstElement<T>>,
    order: Vec<AstElement<T>>,
}

impl<T> Default for SetOfAstTypes<T> {
    fn default() -> Self {
        Self {
            set: std::collections::HashSet::new(),
            order: Vec::new(),
        }
    }
}

impl<T: Eq + Hash> SetOfAstTypes<T> {
    pub fn sort_by_key<K, F>(&mut self, f: F)
    where
        F: FnMut(&AstElement<T>) -> K,
        K: Ord,
    {
        self.order.sort_by_key(f)
    }

    pub fn contains(&self, elem: &AstElement<T>) -> bool {
        self.set.contains(elem)
    }

    pub fn insert(&mut self, elem: AstElement<T>)
    where
        T: Clone,
    {
        self.set.insert(elem.clone());
        self.order.push(elem);
    }
}

impl<'a, T> IntoIterator for &'a SetOfAstTypes<T> {
    type Item = &'a AstElement<T>;

    type IntoIter = std::slice::Iter<'a, AstElement<T>>;

    fn into_iter(self) -> Self::IntoIter {
        (&self.order).iter()
    }
}

impl<T> IntoIterator for SetOfAstTypes<T> {
    type Item = AstElement<T>;

    type IntoIter = std::vec::IntoIter<AstElement<T>>;

    fn into_iter(self) -> Self::IntoIter {
        self.order.into_iter()
    }
}
