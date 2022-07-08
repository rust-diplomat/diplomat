use diplomat_core::{ast, Env};
use std::collections::HashSet;
use std::fmt::Write;
use std::{collections::HashMap, fmt};

use crate::util;

#[cfg(test)]
#[macro_use]
mod test_util;

pub mod docs;

pub mod types;

pub mod structs;
use structs::*;

pub mod conversions;

pub mod display;

static RUNTIME_MJS: &str = include_str!("runtime.mjs");

pub fn gen_bindings(env: &Env, outs: &mut HashMap<String, String>) -> fmt::Result {
    let diplomat_runtime_out = outs.entry("diplomat-runtime.mjs".to_string()).or_default();
    write!(diplomat_runtime_out, "{}", RUNTIME_MJS)?;

    let mut all_types = util::get_all_custom_types(env);
    all_types.sort_by_key(|t| t.1.name());

    for (in_path, custom_type) in all_types {
        let out = outs
            .entry(format!("{}.mjs", custom_type.name()))
            .or_default();

        writeln!(out, "import wasm from \"../wasm.mjs\"")?;
        writeln!(
            out,
            "import * as diplomatRuntime from \"./diplomat-runtime.mjs\""
        )?;
        for custom_type in used_types(custom_type, &in_path, env) {
            if let ast::CustomType::Enum(enm) = custom_type {
                writeln!(
                    out,
                    "import {{ {0}_js_to_rust, {0}_rust_to_js }} from \"./{0}.mjs\"",
                    enm.name
                )?;
            } else {
                writeln!(out, "import {0} from \"./{0}.mjs\"", custom_type.name())?;
            }
        }
        writeln!(out)?;

        gen_struct(out, custom_type, &in_path, env)?;
    }

    Ok(())
}

/// Returns all the types that `custom_type` needs in scope.
///
/// We define types as "in scope" if they appear in the arguments or return type
/// of any method of `custom_type`, or if `custom_type` is a struct and the type
/// appears in one of its fields.
///
/// Non-opaque structs construct their fields within their constructor, meaning
/// we don't have to eagerly import every type recursively. However, we do have
/// to recursively import all enum types. This is because we fully unpack structs
/// at the boundary and have to convert all enums, including those unpacked from
/// a struct, at the boundary as well.
fn used_types<'env>(
    custom_type: &'env ast::CustomType,
    in_path: &ast::Path,
    env: &'env Env,
) -> HashSet<&'env ast::CustomType> {
    let mut set = HashSet::new();

    if let ast::CustomType::Struct(strct) = custom_type {
        for (_, typ, _) in strct.fields.iter() {
            used_types_inner(typ, &mut set, in_path, env, false);
        }
    }

    for method in custom_type.methods() {
        for param in method.params.iter() {
            used_types_inner(&param.ty, &mut set, in_path, env, false);
        }

        if let Some(ref return_type) = method.return_type {
            used_types_inner(return_type, &mut set, in_path, env, false);
        }
    }

    set.remove(custom_type);
    set
}

/// Traverse a type tree, adding all non-nested `TypeNamed::Named` types,
/// as well as all nested enums.
///
/// See [`used_types`] for more details.
fn used_types_inner<'env>(
    typ: &'env ast::TypeName,
    set: &mut HashSet<&'env ast::CustomType>,
    in_path: &ast::Path,
    env: &'env Env,
    enum_only: bool,
) {
    match typ {
        ast::TypeName::Named(path_type) => {
            let custom = path_type.resolve(in_path, env);
            if let ast::CustomType::Enum(_) = custom {
                set.insert(custom);
            } else if !enum_only {
                set.insert(custom);
            }
            // Always recurse deeper, there could be more nested enums.
            if let ast::CustomType::Struct(strct) = custom {
                for (_, typ, _) in strct.fields.iter() {
                    used_types_inner(typ, set, in_path, env, true)
                }
            }
        }
        ast::TypeName::Reference(.., typ)
        | ast::TypeName::Box(typ)
        | ast::TypeName::Option(typ) => used_types_inner(typ, set, in_path, env, enum_only),
        ast::TypeName::Result(ok, err) => {
            used_types_inner(ok, set, in_path, env, enum_only);
            used_types_inner(err, set, in_path, env, enum_only);
        }
        _ => {}
    }
}
