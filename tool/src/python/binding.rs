use askama::Template;
use std::borrow::Cow;
use std::collections::BTreeSet;
use std::fmt::{self};
use std::string::String;

/// This abstraction allows us to build up the binding piece by piece without needing
/// to precalculate things like the list of dependent headers or classes
#[derive(Default, Template)]
#[template(path = "python/binding.cpp.jinja", escape = "none")]
pub(super) struct Binding<'a> {
    /// The module name for this binding
    pub module_name: Cow<'a, str>,
    /// A list of includes
    ///
    /// Example:
    /// ```c
    /// #include "Foo.h"
    /// #include "Bar.h"
    /// #include "diplomat_runtime.h"
    /// ```
    pub includes: BTreeSet<Cow<'a, str>>,
    /// The actual meat of the impl: usually will contain a type definition and methods
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
}

impl Binding<'_> {
    pub fn new() -> Self {
        Binding {
            includes: BTreeSet::new(),
            ..Default::default()
        }
    }
}

impl fmt::Write for Binding<'_> {
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
