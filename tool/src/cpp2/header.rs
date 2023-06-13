use askama::Template;
use std::borrow::{Borrow, Cow};
use std::collections::BTreeSet;
use std::fmt::{self, Write};

static BASE_INCLUDES: &str = r#"
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
"#;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum Forward {
    Class(String),
    #[allow(dead_code)]
    Struct(String),
    #[allow(dead_code)]
    EnumStruct(String),
}

// NOTE: This isn't exactly a valid impl of Borrow, but it makes things more convenient
impl Borrow<str> for Forward {
    fn borrow(&self) -> &str {
        match self {
            Forward::Class(s) => s,
            Forward::Struct(s) => s,
            Forward::EnumStruct(s) => s,
        }
    }
}

#[derive(Template)]
#[template(path = "cpp2/base.txt")]
struct HeaderTemplate<'a> {
    header_guard: Cow<'a, str>,
    decl_include: Option<Cow<'a, str>>,
    includes: Vec<Cow<'a, str>>,
    forwards: Vec<Forward>,
    body: Cow<'a, str>,
}

/// This abstraction allows us to build up headers piece by piece without needing
/// to precalculate things like the list of dependent headers or forward declarations
#[derive(Default)]
pub struct Header {
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
    /// The struct forward decls necessary
    ///
    /// Example:
    /// ```c
    /// typedef struct Foo Foo;
    /// typedef struct Bar Bar;
    /// ```
    pub forwards: BTreeSet<Forward>,
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
            includes: BTreeSet::new(),
            decl_include: None,
            forwards: BTreeSet::new(),
            body: String::new(),
            indent_str: "  ",
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
        let header_guard = &self.path;
        let header_guard = header_guard.replace(".d.hpp", "_D_HPP");
        let header_guard = header_guard.replace(".hpp", "_HPP");
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
                .map(|s| Cow::Borrowed(s.as_str())),
            includes: self
                .includes
                .iter()
                .map(|s| Cow::Borrowed(s.as_str()))
                .collect(),
            forwards: self.forwards.iter().cloned().collect(),
            body,
        }
        .render_into(f)
        .unwrap();
        f.write_char('\n')
    }
}
