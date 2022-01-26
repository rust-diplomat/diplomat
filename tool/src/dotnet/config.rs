use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct LibraryConfig {
    pub namespace: String,
    pub native_lib: String,
    #[serde(default)]
    pub usings: Vec<String>,
    #[serde(default = "bool_true")]
    pub rename_exceptions: bool,
    #[serde(default)]
    pub error_message_method: Option<String>,
}

pub fn bool_true() -> bool {
    true
}

impl LibraryConfig {
    pub fn default() -> LibraryConfig {
        LibraryConfig {
            namespace: "Interop".to_owned(),
            native_lib: "rust".to_owned(),
            usings: Vec::new(),
            rename_exceptions: true,
            error_message_method: None,
        }
    }
}
