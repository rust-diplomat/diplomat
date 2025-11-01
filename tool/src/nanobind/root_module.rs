use askama::Template;
use std::borrow::Cow;
use std::collections::{BTreeMap, BTreeSet};
use std::string::String;

/// This abstraction allows us to build up the binding piece by piece without needing
/// to precalculate things like the list of dependent headers or classes
#[derive(Default, Template)]
#[template(path = "nanobind/root_module.cpp.jinja", escape = "none")]
pub(super) struct RootModule<'a> {
    /// The module name for this binding
    pub module_name: Cow<'a, str>,

    // Forward declaration of functions, grouped by namespace
    pub fwd_decls: BTreeMap<String, Vec<String>>,

    // A per-module list of add*_binding functions to call
    // Module names are given as vectors of nested namespaces
    pub module_fns: BTreeMap<Vec<String>, Vec<String>>,

    // a list of modules we need to define
    pub sub_modules: BTreeSet<Vec<String>>,
}

impl RootModule<'_> {
    pub fn new() -> Self {
        RootModule {
            ..Default::default()
        }
    }
}
