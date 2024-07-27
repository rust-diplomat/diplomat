mod formatter;
mod header;
mod ty;

pub(crate) use self::formatter::CFormatter;
pub(crate) use self::formatter::CAPI_NAMESPACE;
pub(crate) use self::header::Header;
pub(crate) use self::ty::TyGenContext;

use crate::{ErrorStore, FileMap};
use diplomat_core::hir;
use diplomat_core::hir::BackendAttrSupport;

pub(crate) fn attr_support() -> BackendAttrSupport {
    let mut a = BackendAttrSupport::default();

    a.namespacing = false;
    a.memory_sharing = true;
    a.non_exhaustive_structs = false;
    a.method_overloading = false;
    a.utf8_strings = true;
    a.utf16_strings = true;

    a.constructors = false;
    a.named_constructors = false;
    a.fallible_constructors = false;
    a.accessors = false;
    a.comparators = false;
    a.stringifiers = false;
    a.iterators = false;
    a.iterables = false;
    a.indexing = false;

    a
}

pub(crate) fn run(tcx: &hir::TypeContext) -> (FileMap, ErrorStore<String>) {
    let files = FileMap::default();
    let formatter = CFormatter::new(tcx, false);
    let errors = ErrorStore::default();

    #[derive(askama::Template)]
    #[template(path = "c/runtime.h.jinja", escape = "none")]
    struct Runtime;

    files.add_file("diplomat_runtime.h".into(), Runtime.to_string());

    for (id, ty) in tcx.all_types() {
        if ty.attrs().disable {
            // Skip type if disabled
            continue;
        }

        let decl_header_path = formatter.fmt_decl_header_path(id);
        let impl_header_path = formatter.fmt_impl_header_path(id);

        let _guard = errors.set_context_ty(ty.name().as_str().into());
        let context = TyGenContext {
            tcx,
            formatter: &formatter,
            errors: &errors,
            is_for_cpp: false,
            id,
            decl_header_path: &decl_header_path,
            impl_header_path: &impl_header_path,
        };

        let decl_header = match ty {
            hir::TypeDef::Enum(e) => context.gen_enum_def(e),
            hir::TypeDef::Opaque(o) => context.gen_opaque_def(o),
            hir::TypeDef::Struct(s) => context.gen_struct_def(s),
            hir::TypeDef::OutStruct(s) => context.gen_struct_def(s),
            _ => unreachable!("unknown AST/HIR variant"),
        };

        let impl_header = context.gen_impl(ty);

        files.add_file(decl_header_path, decl_header.to_string());
        files.add_file(impl_header_path, impl_header.to_string());
    }

<<<<<<< HEAD
    (files, errors)
=======
    for (ref in_path, typ) in &all_results {
        gen_result_header(typ, in_path, outs, env)?;
    }

    Ok(())
}

fn gen_struct_header(
    typ: &ast::CustomType,
    in_path: &ast::Path,
    seen_results: &mut HashSet<ast::TypeName>,
    all_results: &mut Vec<(ast::Path, ast::TypeName)>,
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
        if method.attrs.skip_if_ast {
            continue;
        }
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
        if method.attrs.skip_if_ast {
            continue;
        }
        gen_method(method, in_path, env, out)?;
    }

    if typ.methods().is_empty() {
        writeln!(out)?;
    }

    write!(out, "void {}(", typ.dtor_name())?;
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
            if !matches!(
                underlying.as_ref(),
                ast::TypeName::Box(..) | ast::TypeName::Reference(..)
            ) {
                let include = format!(
                    "#include \"{}.h\"",
                    name_for_type(&ast::TypeName::Result(
                        underlying.clone(),
                        Box::new(ast::TypeName::Unit),
                        true
                    ))
                );
                if !seen_includes.contains(&include) {
                    writeln!(out, "{include}")?;
                    seen_includes.insert(include);
                }
            }
            gen_includes(underlying, in_path, env, seen_includes, out)?;
        }
        ast::TypeName::Result(_, _, _) => {
            let include = format!("#include \"{}.h\"", name_for_type(typ));
            if !seen_includes.contains(&include) {
                writeln!(out, "{include}")?;
                seen_includes.insert(include);
            }
        }
        ast::TypeName::Write => {}
        ast::TypeName::StrReference(..) => {}
        ast::TypeName::PrimitiveSlice(..) => {}
        ast::TypeName::Unit => {}
        ast::TypeName::Ordering => {}
        ast::TypeName::StrSlice(..) => {}
        ast::TypeName::Function(..) => {}
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
>>>>>>> eb5ea70 (some changes to get C working, and a manually working example for calling C)
}
