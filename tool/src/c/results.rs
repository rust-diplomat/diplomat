use std::fmt::Write;
use std::{collections::HashSet, fmt};

use diplomat_core::ast;
use diplomat_core::Env;
use indenter::indented;

use super::types::{gen_type, name_for_type};

pub fn collect_results<'a>(
    typ: &'a ast::TypeName,
    in_path: &ast::Path,
    env: &Env,
    seen: &mut HashSet<(ast::Path, &'a ast::TypeName)>,
    results: &mut Vec<(ast::Path, &'a ast::TypeName)>,
) {
    match typ {
        ast::TypeName::Named(_) => {}
        ast::TypeName::Box(underlying) => {
            collect_results(underlying, in_path, env, seen, results);
        }
        ast::TypeName::Reference(underlying, _, _lt) => {
            collect_results(underlying, in_path, env, seen, results);
        }
        ast::TypeName::Primitive(_) => {}
        ast::TypeName::Option(underlying) => {
            collect_results(underlying, in_path, env, seen, results);
        }
        ast::TypeName::Result(ok, err) => {
            let seen_key = (in_path.clone(), typ);
            if !seen.contains(&seen_key) {
                seen.insert(seen_key.clone());
                collect_results(ok, in_path, env, seen, results);
                collect_results(err, in_path, env, seen, results);
                results.push(seen_key);
            }
        }
        ast::TypeName::Writeable => {}
        ast::TypeName::StrReference(..) => {}
        ast::TypeName::PrimitiveSlice(..) => {}
        ast::TypeName::Unit => {}
    }
}

pub fn gen_result<W: fmt::Write>(
    typ: &ast::TypeName,
    in_path: &ast::Path,
    env: &Env,
    out: &mut W,
) -> fmt::Result {
    if let ast::TypeName::Result(ok, err) = typ {
        let result_name = format!("{}_{}", in_path.elements.join("_"), name_for_type(typ));
        writeln!(out, "typedef struct {} {{", result_name)?;
        let mut result_indent = indented(out).with_str("    ");
        // zero-sized types in C unions work differently across C and C++
        // we avoid the problem by omitting variants or even the entire union
        // if parts are zero-sized. This matches what rustc effectively does
        // with zero-sized union variants
        if !ok.is_zst() || !err.is_zst() {
            writeln!(&mut result_indent, "union {{")?;
            let mut union_indent = indented(&mut result_indent).with_str("    ");

            if !ok.is_zst() {
                gen_type(
                    ok,
                    in_path,
                    env,
                    &mut ((&mut union_indent) as &mut dyn fmt::Write),
                )?;
                writeln!(&mut union_indent, " ok;")?;
            }

            if !err.is_zst() {
                gen_type(
                    err,
                    in_path,
                    env,
                    &mut ((&mut union_indent) as &mut dyn fmt::Write),
                )?;
                writeln!(&mut union_indent, " err;")?;
            }
            writeln!(&mut result_indent, "}};")?;
        }
        writeln!(&mut result_indent, "bool is_ok;")?;
        writeln!(out, "}} {};", result_name)?;

        Ok(())
    } else {
        panic!()
    }
}
