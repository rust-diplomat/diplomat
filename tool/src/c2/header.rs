use core::fmt;
use std::collections::BTreeSet;

static BASE_INCLUDES: &'static str = r#"
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"
"#;

pub struct Header {
    identifier: String,
    includes: BTreeSet<String>,
    forwards: BTreeSet<String>,
    body: String,
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
            includes += &format!(r#"#include "{}.h"\n"#, i);
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
