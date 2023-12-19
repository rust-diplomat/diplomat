use core::panic;
use std::collections::{HashMap, HashSet};
use std::fmt;
use std::fmt::Write;

use diplomat_core::ast;
use diplomat_core::Env;
use indenter::indented;

use crate::util;

#[cfg(test)]
#[macro_use]
mod test_util;

pub mod types;
use types::*;

mod structs;
use structs::*;

mod results;
use results::*;

pub static RUNTIME_H: &str = include_str!("runtime.h");

pub fn gen_bindings(env: &Env, outs: &mut HashMap<String, String>) -> fmt::Result {
    let diplomat_runtime_out = outs.entry("diplomat_runtime.h".to_string()).or_default();
    write!(diplomat_runtime_out, "{RUNTIME_H}")?;

    let all_types = util::get_all_custom_types(env);
    let mut seen_results = HashSet::new();
    let mut all_results = Vec::new();

    for (in_path, typ) in all_types {
        gen_struct_header(
            typ,
            &in_path,
            &mut seen_results,
            &mut all_results,
            outs,
            env,
        )?;
    }

    for (ref in_path, typ) in &all_results {
        gen_result_header(typ, in_path, outs, env)?;
    }

    Ok(())
}

fn gen_struct_header<'a>(
    typ: &'a ast::CustomType,
    in_path: &ast::Path,
    seen_results: &mut HashSet<&'a ast::TypeName>,
    all_results: &mut Vec<(ast::Path, &'a ast::TypeName)>,
    outs: &mut HashMap<String, String>,
    env: &Env,
) -> Result<(), fmt::Error> {
    let out = outs.entry(format!("{}.h", typ.name())).or_default();

    writeln!(out, "#ifndef {}_H", typ.name())?;
    writeln!(out, "#define {}_H", typ.name())?;
    writeln!(out, "#include <stdio.h>")?;
    writeln!(out, "#include <stdint.h>")?;
    writeln!(out, "#include <stddef.h>")?;
    writeln!(out, "#include <stdbool.h>")?;
    writeln!(out, "#include \"diplomat_runtime.h\"")?;
    writeln!(out)?;

    let mut seen_includes = HashSet::new();
    seen_includes.insert(format!("#include \"{}.h\"", typ.name()));
    seen_includes.insert(format!("typedef struct {} {};", typ.name(), typ.name()));

    if let ast::CustomType::Struct(strct) = typ {
        for (_, typ, _) in &strct.fields {
            gen_includes(typ, in_path, env, &mut seen_includes, out)?;
            collect_results(typ, in_path, env, seen_results, all_results);
        }
    }

    writeln!(out, "#ifdef __cplusplus")?;
    writeln!(out, "namespace capi {{")?;
    writeln!(out, "#endif")?;

    match typ {
        ast::CustomType::Opaque(_) | ast::CustomType::Struct(_) => {
            writeln!(out)?;
            gen_struct(typ, in_path, env, out)?;
        }

        ast::CustomType::Enum(enm) => {
            writeln!(out)?;
            writeln!(out, "typedef enum {} {{", enm.name)?;
            let mut enum_body_out = indented(out).with_str("  ");
            for (name, discriminant, _, _attrs) in enm.variants.iter() {
                writeln!(
                    &mut enum_body_out,
                    "{}_{} = {},",
                    enm.name, name, discriminant
                )?;
            }
            writeln!(out, "}} {};", enm.name)?;
        }
        &_ => unreachable!("unknown AST/HIR variant"),
    }

    writeln!(out, "#ifdef __cplusplus")?;
    writeln!(out, "}} // namespace capi")?;
    writeln!(out, "#endif")?;

    let mut seen_includes = HashSet::new();
    seen_includes.insert(format!("#include \"{}.h\"", typ.name()));

    if let ast::CustomType::Struct(strct) = typ {
        for (_, typ, _) in &strct.fields {
            gen_includes(typ, in_path, env, &mut seen_includes, out)?;
            collect_results(typ, in_path, env, seen_results, all_results);
        }
    }
    for method in typ.methods() {
        for param in &method.params {
            gen_includes(&param.ty, in_path, env, &mut seen_includes, out)?;
            collect_results(&param.ty, in_path, env, seen_results, all_results);
        }

        if let Some(return_type) = method.return_type.as_ref() {
            gen_includes(return_type, in_path, env, &mut seen_includes, out)?;
            collect_results(return_type, in_path, env, seen_results, all_results);
        }
    }

    writeln!(out, "#ifdef __cplusplus")?;
    writeln!(out, "namespace capi {{")?;
    writeln!(out, "extern \"C\" {{")?;
    writeln!(out, "#endif")?;

    for method in typ.methods() {
        writeln!(out)?;
        gen_method(method, in_path, env, out)?;
    }

    if typ.methods().is_empty() {
        writeln!(out)?;
    }

    write!(out, "void {}_destroy(", typ.name())?;
    gen_type(
        &ast::TypeName::Box(Box::new(ast::TypeName::Named(ast::PathType::new(
            ast::Path::empty().sub_path(typ.name().clone()),
        )))),
        in_path,
        env,
        out,
    )?;
    writeln!(out, " self);")?;

    writeln!(out)?;

    writeln!(out, "#ifdef __cplusplus")?;
    writeln!(out, "}} // extern \"C\"")?;
    writeln!(out, "}} // namespace capi")?;
    writeln!(out, "#endif")?;
    writeln!(out, "#endif")?;
    Ok(())
}

fn gen_result_header(
    typ: &ast::TypeName,
    in_path: &ast::Path,
    outs: &mut HashMap<String, String>,
    env: &Env,
) -> fmt::Result {
    if let ast::TypeName::Result(ok, err, _) = typ {
        let out = outs.entry(format!("{}.h", name_for_type(typ))).or_default();

        writeln!(out, "#ifndef {}_H", name_for_type(typ))?;
        writeln!(out, "#define {}_H", name_for_type(typ))?;
        writeln!(out, "#include <stdio.h>")?;
        writeln!(out, "#include <stdint.h>")?;
        writeln!(out, "#include <stddef.h>")?;
        writeln!(out, "#include <stdbool.h>")?;
        writeln!(out, "#include \"diplomat_runtime.h\"")?;
        writeln!(out)?;
        let mut seen_includes = HashSet::new();
        gen_includes(ok.as_ref(), in_path, env, &mut seen_includes, out)?;
        gen_includes(err.as_ref(), in_path, env, &mut seen_includes, out)?;
        writeln!(out, "#ifdef __cplusplus")?;
        writeln!(out, "namespace capi {{")?;
        writeln!(out, "extern \"C\" {{")?;
        writeln!(out, "#endif")?;

        gen_result(typ, in_path, env, out)?;

        writeln!(out, "#ifdef __cplusplus")?;
        writeln!(out, "}} // extern \"C\"")?;
        writeln!(out, "}} // namespace capi")?;
        writeln!(out, "#endif")?;
        writeln!(out, "#endif")?;
    } else {
        panic!()
    }

    Ok(())
}

pub fn gen_includes<W: fmt::Write>(
    typ: &ast::TypeName,
    in_path: &ast::Path,
    env: &Env,
    seen_includes: &mut HashSet<String>,
    out: &mut W,
) -> fmt::Result {
    match typ {
        ast::TypeName::Named(path_type) | ast::TypeName::SelfType(path_type) => {
            let custom_typ = path_type.resolve(in_path, env);
            match custom_typ {
                ast::CustomType::Opaque(_)
                | ast::CustomType::Struct(_)
                | ast::CustomType::Enum(_) => {
                    let include = format!("#include \"{}.h\"", custom_typ.name());
                    if !seen_includes.contains(&include) {
                        writeln!(out, "{include}")?;
                        seen_includes.insert(include);
                    }
                }

                _ => unreachable!("unknown AST/HIR variant"),
            }
        }
        ast::TypeName::Box(underlying) => {
            gen_includes(underlying, in_path, env, seen_includes, out)?;
        }
        ast::TypeName::Reference(.., underlying) => {
            gen_includes(underlying, in_path, env, seen_includes, out)?;
        }
        ast::TypeName::Primitive(_) => {}
        ast::TypeName::Option(underlying) => {
            gen_includes(underlying, in_path, env, seen_includes, out)?;
        }
        ast::TypeName::Result(_, _, _) => {
            let include = format!("#include \"{}.h\"", name_for_type(typ));
            if !seen_includes.contains(&include) {
                writeln!(out, "{include}")?;
                seen_includes.insert(include);
            }
        }
        ast::TypeName::Writeable => {}
        ast::TypeName::StrReference(..) => {}
        ast::TypeName::PrimitiveSlice(..) => {}
        ast::TypeName::Unit => {}
        &_ => unreachable!("unknown AST/HIR variant"),
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_cross_module_struct_fields() {
        test_file! {
            #[diplomat::bridge]
            mod mod1 {
                use super::mod2::Bar;

                struct Foo {
                    x: Bar,
                }
            }

            #[diplomat::bridge]
            mod mod2 {
                use super::mod1::Foo;

                struct Bar {
                    y: Box<Foo>,
                }
            }
        }
    }
}
