use std::borrow::Cow;
use std::collections::BTreeSet;
use std::fmt;

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
        let mut includes = String::from(BASE_INCLUDES);
        for i in &self.includes {
            includes += &format!("#include \"{}\"\n", i);
        }
        let decl_header_include: Cow<str> = match self.decl_include {
            Some(ref v) => format!("\n#include \"{v}\"\n").into(),
            None => "".into(),
        };
        let header_guard = &self.path;
        let header_guard = header_guard.replace(".d.h", "_D_H");
        let header_guard = header_guard.replace(".h", "_H");
        let body: Cow<str> = if self.body.is_empty() {
            "// No Content\n\n".into()
        } else {
            self.body.replace('\t', self.indent_str).into()
        };

        write!(
            f,
            r#"#ifndef {header_guard}
#define {header_guard}
{includes}{decl_header_include}
#ifdef __cplusplus
namespace capi {{
extern "C" {{
#endif // __cplusplus


{body}
#ifdef __cplusplus
}} // extern "C"
}} // namespace capi
#endif // __cplusplus

#endif // {header_guard}
"#
        )
    }
}
