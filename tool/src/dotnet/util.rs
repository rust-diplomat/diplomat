use core::fmt;
use core::fmt::Write as _;

use diplomat_core::{ast, Env};

use crate::util::{CodeWriter, SetOfAstTypes};

pub fn gen_doc_block(out: &mut CodeWriter, comment: &str) -> fmt::Result {
    if !comment.is_empty() {
        let mut summary_is_open = true;
        writeln!(out, "/// <summary>")?;
        for line in comment.lines() {
            if line.is_empty() {
                if summary_is_open {
                    writeln!(out, "/// </summary>")?;
                    writeln!(out, "/// <remarks>")?;
                    summary_is_open = false;
                } else {
                    writeln!(out, "/// <br/>")?;
                }
            } else {
                writeln!(out, "/// {}", line)?;
            }
        }

        if summary_is_open {
            writeln!(out, "/// </summary>")?;
        } else {
            writeln!(out, "/// </remarks>")?;
        }
    }
    Ok(())
}

pub fn collect_results<'ast>(
    typ: &'ast ast::TypeName,
    in_path: &ast::Path,
    env: &Env,
    results: &mut SetOfAstTypes<&'ast ast::TypeName>,
) {
    match typ {
        ast::TypeName::Box(underlying) => {
            collect_results(underlying, in_path, env, results);
        }
        ast::TypeName::Reference(.., underlying) => {
            collect_results(underlying, in_path, env, results);
        }
        ast::TypeName::Option(underlying) => {
            collect_results(underlying, in_path, env, results);
        }
        ast::TypeName::Result(ok, err) => {
            let key = (in_path.clone(), typ);
            if !results.contains(&key) {
                results.insert(key);
                collect_results(ok, in_path, env, results);
                collect_results(err, in_path, env, results);
            }
        }
        ast::TypeName::Unit
        | ast::TypeName::Writeable
        | ast::TypeName::StrReference(..)
        | ast::TypeName::PrimitiveSlice(..)
        | ast::TypeName::Named(_)
        | ast::TypeName::Primitive(_) => {}
    }
}

pub fn collect_errors<'ast>(
    typ: &'ast ast::TypeName,
    in_path: &ast::Path,
    env: &Env,
    errors: &mut SetOfAstTypes<&'ast ast::TypeName>,
) {
    collect_errors_impl(typ, in_path, env, errors, false)
}

fn collect_errors_impl<'ast>(
    typ: &'ast ast::TypeName,
    in_path: &ast::Path,
    env: &Env,
    errors: &mut SetOfAstTypes<&'ast ast::TypeName>,
    is_err_variant: bool,
) {
    match typ {
        ast::TypeName::Result(ok, err) => {
            if is_err_variant {
                let key = (in_path.clone(), typ);
                if !errors.contains(&key) {
                    errors.insert(key);
                }
            }

            collect_errors_impl(ok, in_path, env, errors, false);
            collect_errors_impl(err, in_path, env, errors, true);
        }
        ast::TypeName::Box(underlying) => {
            collect_errors_impl(underlying, in_path, env, errors, is_err_variant);
        }
        ast::TypeName::Reference(.., underlying) => {
            collect_errors_impl(underlying, in_path, env, errors, is_err_variant);
        }
        ast::TypeName::Option(underlying) => {
            collect_errors_impl(underlying, in_path, env, errors, is_err_variant);
        }
        ast::TypeName::Named(path_type) => {
            if is_err_variant {
                let (custom_ty_path, _) = path_type.resolve_with_path(in_path, env);
                let key = (custom_ty_path, typ);
                if !errors.contains(&key) {
                    errors.insert(key);
                }
            }
        }
        ast::TypeName::Primitive(_) => {
            if is_err_variant {
                let key = (ast::Path::empty(), typ);
                if !errors.contains(&key) {
                    errors.insert(key);
                }
            }
        }
        ast::TypeName::Unit
        | ast::TypeName::Writeable
        | ast::TypeName::StrReference(..)
        | ast::TypeName::PrimitiveSlice(..) => {}
    }
}
