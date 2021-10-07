use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct LibraryType {
    pub name: String,
    pub expr: String,
}

#[derive(Debug, Deserialize)]
pub struct LibraryConfig {
    pub headers: Vec<String>,
    pub span: LibraryType,
    pub string_view: LibraryType,
    pub unique_ptr: LibraryType,
}

impl LibraryConfig {
    pub fn default() -> LibraryConfig {
        LibraryConfig {
            headers: vec![
                "#include <stdint.h>".into(),
                "#include <stddef.h>".into(),
                "#include <stdbool.h>".into(),
                "#include <algorithm>".into(),
                "#include <memory>".into(),
                "#include <optional>".into(),
                "#include <span>".into(),
                "#include <variant>".into(),
            ],
            span: LibraryType {
                name: "std::span".into(),
                expr: "std::span".into(),
            },
            string_view: LibraryType {
                name: "std::string_view".into(),
                expr: "std::string_view".into(),
            },
            unique_ptr: LibraryType {
                name: "std::unique_ptr".into(),
                expr: "std::unique_ptr".into(),
            },
        }
    }
}
