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
    /// The identifier used for the header file (without the .h)
    ///
    /// Example: the header for struct Foo is probably Foo, with the file named Foo.h
    pub identifier: String,
    /// A list of includes
    ///
    /// Example:
    /// ```c
    /// #include "Foo.h"
    /// #include "Bar.h"
    /// #include "diplomat_runtime.h"
    /// ```
    pub includes: BTreeSet<String>,
    /// The struct forward decls necessary
    ///
    /// Example:
    /// ```c
    /// typedef struct Foo Foo;
    /// typedef struct Bar Bar;
    /// ```
    pub forwards: BTreeSet<String>,
    /// The actual meat of the header: usually will contain a type definition and methods
    ///
    /// Example:
    /// ```c
    /// typeded struct Foo {
    ///   uint8_t field1;
    ///   bool field2;
    /// }
    ///
    /// Foo make_foo(uint8_t field1, bool field2);
    /// ```
    pub body: String,
}

impl Header {
    pub fn new(identifier: String) -> Self {
        Header {
            identifier,
            includes: BTreeSet::new(),
            forwards: BTreeSet::new(),
            body: String::new(),
        }
    }
}

impl fmt::Display for Header {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut forwards = String::new();
        let mut includes = String::from(BASE_INCLUDES);
        for i in &self.includes {
            includes += &format!("#include \"{}.h\"\n", i);
        }
        for f in &self.forwards {
            forwards += &format!("typedef struct {f} {f};\n");
        }
        let identifier = &self.identifier;
        let body = &self.body;

        write!(
            f,
            r#"#ifndef {identifier}_H
#define {identifier}_H

{includes}

#ifdef __cplusplus
namespace capi {{
extern "C" {{
#endif // __cplusplus

{forwards}

{body}

#ifdef __cplusplus
}} // namespace capi
}} // extern "C"
#endif // __cplusplus

#endif // {identifier}_H
"#
        )
    }
}
