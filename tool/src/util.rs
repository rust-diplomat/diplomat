use diplomat_core::Env;

use diplomat_core::ast;

pub fn get_all_custom_types(env: &Env) -> Vec<(&ast::Path, &ast::CustomType)> {
    let mut all_types: Vec<(&ast::Path, &ast::CustomType)> = vec![];

    for (path, _name, symbol) in env.iter_items() {
        if let ast::ModSymbol::CustomType(c) = symbol {
            all_types.push((path, c));
        }
    }

    all_types
}
