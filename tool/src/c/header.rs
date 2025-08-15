use askama::Template;
use std::borrow::Cow;
use std::collections::BTreeSet;
use std::fmt;
use std::fmt::Write;

static BASE_INCLUDES: &str = r#"
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"
"#;

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
    /// For defining arrays with `typedef` (since writing `type var[size]` would require a major refactor of many backends).
    pub arr_typedefs: BTreeSet<String>,
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
    pub is_for_cpp: bool,
}

impl Header {
    pub fn new(path: String, is_for_cpp: bool) -> Self {
        Header {
            path,
            includes: BTreeSet::new(),
            decl_include: None,
            arr_typedefs: BTreeSet::new(),
            body: String::new(),
            indent_str: "  ",
            is_for_cpp,
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

#[derive(Template)]
#[template(path = "c/base.h.jinja", escape = "none")]
struct HeaderTemplate<'a> {
    header_guard: Cow<'a, str>,
    decl_include: Option<&'a String>,
    arr_typedefs: &'a BTreeSet<String>,
    includes: &'a BTreeSet<String>,
    body: Cow<'a, str>,
    is_for_cpp: bool,
}

impl fmt::Display for Header {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let header_guard = &self.path;
        let header_guard = header_guard.replace(".d.h", "_D_H");
        let header_guard = header_guard.replace(".h", "_H");
        let body: Cow<str> = if self.body.is_empty() {
            "// No Content\n\n".into()
        } else {
            self.body.replace('\t', self.indent_str).into()
        };

        HeaderTemplate {
            header_guard: header_guard.into(),
            includes: &self.includes,
            decl_include: self.decl_include.as_ref(),
            arr_typedefs: &self.arr_typedefs,
            body,
            is_for_cpp: self.is_for_cpp,
        }
        .render_into(f)
        .unwrap();
        f.write_char('\n')
    }
}
