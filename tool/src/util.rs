use diplomat_core::Env;
use std::collections::HashMap;

use diplomat_core::ast;

pub fn get_all_custom_types(env: &Env) -> Vec<(&ast::Path, &ast::CustomType)> {
    let mut all_types: Vec<(&ast::Path, &ast::CustomType)> = vec![];

    for (path, mod_symbols) in env {
        for symbol in mod_symbols.values() {
            if let ast::ModSymbol::CustomType(c) = symbol {
                all_types.push((path, c));
            }
        }
    }

    all_types
}
