use askama::Template;
use diplomat_core::hir::TypeDef;
use std::borrow::Cow;
use std::collections::{BTreeMap, BTreeSet};
use std::fmt::{self, Write};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub(crate) enum Forward {
    Class(String),
    #[allow(dead_code)]
    Struct(String),
    #[allow(dead_code)]
    EnumStruct(String),
}

#[derive(Template)]
#[template(path = "cpp/base.h.jinja", escape = "none")]
struct HeaderTemplate<'a> {
    header_guard: Cow<'a, str>,
    decl_include: Option<Cow<'a, str>>,
    includes: Vec<Cow<'a, str>>,
    forwards: &'a BTreeMap<Option<String>, BTreeSet<Forward>>,
    body: Cow<'a, str>,
}

/// This abstraction allows us to build up headers piece by piece without needing
/// to precalculate things like the list of dependent headers or forward declarations
#[derive(Default)]
pub(crate) struct Header {
    /// The path name used for the header file (for example Foo.h)
    pub path: String,
    /// A list of includes
    ///
    /// Example:
    /// ```c
    /// #include "Foo.h"
    /// #include "Bar.h"
    /// #include "diplomat_runtime.h"
    /// ```
    pub includes: BTreeSet<String>,
    /// The decl file corresponding to this impl file. Empty if this is not an impl file.
    pub decl_include: Option<String>,
    /// The struct forward decls necessary.
    ///
    /// The keys on this map are the namespaces (None = root namespace)
    ///
    /// Example:
    /// ```c
    /// typedef struct Foo Foo;
    /// typedef struct Bar Bar;
    /// ```
    pub forwards: BTreeMap<Option<String>, BTreeSet<Forward>>,
    /// The actual meat of the header: usually will contain a type definition and methods
    ///
    /// Example:
    /// ```c
    /// typedef struct Foo {
    ///   uint8_t field1;
    ///   bool field2;
    /// } Foo;
    ///
    /// Foo make_foo(uint8_t field1, bool field2);
    /// ```
    pub body: String,
    /// What string to use for indentation.
    pub indent_str: &'static str,
}

impl Header {
    pub fn new(path: String) -> Self {
        Header {
            path,
            includes: BTreeSet::from_iter(["diplomat_runtime.hpp".into()]),
            decl_include: None,
            forwards: BTreeMap::new(),
            body: String::new(),
            indent_str: "  ",
        }
    }

    fn forward_for(def: TypeDef, ty_name_unnamespaced: &str) -> Forward {
        match def {
            TypeDef::Enum(..) => Forward::EnumStruct(ty_name_unnamespaced.into()),
            TypeDef::Opaque(..) => Forward::Class(ty_name_unnamespaced.into()),
            TypeDef::Struct(..) | TypeDef::OutStruct(..) => {
                Forward::Struct(ty_name_unnamespaced.into())
            }
            _ => unimplemented!("no other TypeDef variants!"),
        }
    }

    pub fn append_forward(&mut self, def: TypeDef, ty_name_unnamespaced: &str) {
        let forward = Self::forward_for(def, ty_name_unnamespaced);

        let ns = def.attrs().namespace.clone();
        self.forwards.entry(ns).or_default().insert(forward);
    }
    pub fn rm_forward(&mut self, def: TypeDef, ty_name_unnamespaced: &str) {
        let ns = &def.attrs().namespace;
        let forward = Self::forward_for(def, ty_name_unnamespaced);
        if let Some(ns_table) = self.forwards.get_mut(ns) {
            ns_table.remove(&forward);
            if ns_table.is_empty() {
                self.forwards.remove(ns);
            }
        }
    }
}

impl fmt::Write for Header {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.body.write_str(s)
    }
    fn write_char(&mut self, c: char) -> fmt::Result {
        self.body.write_char(c)
    }
    fn write_fmt(&mut self, args: fmt::Arguments<'_>) -> fmt::Result {
        self.body.write_fmt(args)
    }
}

impl fmt::Display for Header {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let header_guard = &self
            .path
            .replace(".d.hpp", "_D_HPP")
            .replace(".hpp", "_HPP")
            .replace("\\", "_")
            .replace("/", "_");
        let body: Cow<str> = if self.body.is_empty() {
            "// No Content\n\n".into()
        } else {
            self.body.replace('\t', self.indent_str).into()
        };

        HeaderTemplate {
            header_guard: header_guard.into(),
            decl_include: self
                .decl_include
                .as_ref()
                // The decl is always in the same namespace/directory
                .map(|s| Cow::Borrowed(s.as_str().split('/').next_back().unwrap())),
            includes: self
                .includes
                .iter()
                .map(|s| path_diff(&self.path, s))
                .collect(),
            forwards: &self.forwards,
            body,
        }
        .render_into(f)
        .unwrap();
        f.write_char('\n')
    }
}

// As rsplit_once, except the first of the tuple will include the delimiter pattern
fn rsplit_once_inclusive(str: &str, delim: char) -> Option<(&str, &str)> {
    str.rfind(delim).map(|i| str.split_at(i + 1))
}

/// Returns the path to 'path', relative to 'base'
fn path_diff<'a>(base: &'a str, path: &'a str) -> Cow<'a, str> {
    let (mut base_ns, _) = rsplit_once_inclusive(base, '/').unwrap_or(("", base));
    let (mut path_ns, file) = rsplit_once_inclusive(path, '/').unwrap_or(("", path));

    let mut matching_chars = 0;
    // Consume and count the length of the matching section
    loop {
        let b = base_ns.split_once('/');
        let p = path_ns.split_once('/');
        if let (Some(b), Some(p)) = (b, p) {
            if b.0 == p.0 {
                base_ns = b.1;
                path_ns = p.1;
                matching_chars += b.0.len() + 1; // 1 for the consumed delimiter
                continue;
            }
        }
        break;
    }

    // Base has run out without a mismatch, the relative path is a strict subset of path & can be borrowed
    if base_ns.is_empty() {
        path.split_at(matching_chars).1.into()
    } else {
        let up_dirs = base_ns.matches('/').count();
        ("../".repeat(up_dirs) + path_ns + file).into()
    }
}

#[test]
fn test_path_diff() {
    let a = "a/same.hpp";
    let b = "a/same2.hpp";
    assert_eq!(path_diff(a, b), "same2.hpp");

    let a = "root.hpp";
    let b = "a/nested.hpp";
    assert_eq!(path_diff(a, b), "a/nested.hpp");

    let a = "a/nested.hpp";
    let b = "root.hpp";
    assert_eq!(path_diff(a, b), "../root.hpp");

    let a = "a/b/c/d.hpp";
    let b = "a/b/z/c/d.hpp";

    assert_eq!(path_diff(a, b), "../z/c/d.hpp");
}
