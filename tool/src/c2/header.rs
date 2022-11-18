use std::collections::BTreeSet;

static BASE_INCLUDES: &'static str = r#"
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
    /// The identifier used for the header file
    pub identifier: String,
    /// A list of includes
    pub includes: BTreeSet<String>,
    /// The struct forward decls necessary
    pub forwards: BTreeSet<String>,
    /// The actual meat of the header: usually will contain a type definition and methods
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

    pub fn to_string(&self) -> String {
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

        format!(
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
