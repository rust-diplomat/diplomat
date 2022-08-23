use std::fmt;
use std::fmt::Write as _;

use diplomat_core::ast;
use diplomat_core::Env;
use heck::ToLowerCamelCase;
use heck::ToUpperCamelCase;

use super::config::LibraryConfig;
use super::types::gen_type_name;
use super::util::{collect_errors, collect_results, gen_doc_block};
use crate::util::{CodeWriter, SetOfAstTypes};

pub fn gen_header(library_config: &LibraryConfig, out: &mut CodeWriter) -> fmt::Result {
    writeln!(out, "// <auto-generated/> by Diplomat")?;
    writeln!(out)?;

    writeln!(out, "#pragma warning disable 0105")?;
    writeln!(out, "using System;")?;
    writeln!(out, "using System.Runtime.InteropServices;")?;
    writeln!(out)?;
    for using in &library_config.usings {
        writeln!(out, "using {};", using)?;
    }
    writeln!(out, "using {}.Diplomat;", library_config.namespace)?;
    writeln!(out, "#pragma warning restore 0105")?;
    writeln!(out)?;

    writeln!(out, "namespace {}.Raw;", library_config.namespace)?;

    writeln!(out)?;
    writeln!(out, "#nullable enable")?;

    Ok(())
}

#[allow(clippy::too_many_arguments)]
pub fn gen<'ast>(
    env: &Env,
    library_config: &LibraryConfig,
    results: &mut SetOfAstTypes<&'ast ast::TypeName>,
    errors: &mut SetOfAstTypes<&'ast ast::TypeName>,
    typ: &'ast ast::CustomType,
    in_path: &'ast ast::Path,
    docs_url_gen: &ast::DocsUrlGenerator,
    out: &mut CodeWriter,
) -> fmt::Result {
    for method in typ.methods() {
        for param in &method.params {
            collect_results(&param.ty, in_path, env, results);
            collect_errors(&param.ty, in_path, env, errors);
        }

        if let Some(return_type) = method.return_type.as_ref() {
            collect_results(return_type, in_path, env, results);
            collect_errors(return_type, in_path, env, errors);
        }
    }

    writeln!(out)?;

    match typ {
        ast::CustomType::Struct(strct) => {
            for (_, typ, _) in &strct.fields {
                collect_results(typ, in_path, env, results);
                collect_errors(typ, in_path, env, errors);
            }

            gen_doc_block(out, &strct.docs.to_markdown(docs_url_gen, false))?;
            writeln!(out, "[StructLayout(LayoutKind.Sequential)]")?;
            writeln!(out, "public partial struct {}", typ.name())?;

            out.scope(|out| {
                writeln!(
                    out,
                    "private const string NativeLib = \"{}\";",
                    library_config.native_lib
                )?;

                for (name, typ, doc) in strct.fields.iter() {
                    gen_field(name, doc, typ, in_path, env, docs_url_gen, out)?;
                }

                for method in typ.methods() {
                    gen_method(typ, method, in_path, env, docs_url_gen, out)?;
                }

                Ok(())
            })
        }

        ast::CustomType::Opaque(opaque) => {
            gen_doc_block(out, &opaque.docs.to_markdown(docs_url_gen, false))?;
            writeln!(out, "[StructLayout(LayoutKind.Sequential)]")?;
            writeln!(out, "public partial struct {}", typ.name())?;

            out.scope(|out| {
                writeln!(
                    out,
                    "private const string NativeLib = \"{}\";",
                    library_config.native_lib
                )?;

                for method in typ.methods() {
                    gen_method(typ, method, in_path, env, docs_url_gen, out)?;
                }

                writeln!(out)?;
                writeln!(
                    out,
                    r#"[DllImport(NativeLib, CallingConvention = CallingConvention.Cdecl, EntryPoint = "{}_destroy", ExactSpelling = true)]"#,
                    typ.name()
                )?;
                writeln!(out, "public static unsafe extern void Destroy({}* self);", typ.name())
            })
        }

        ast::CustomType::Enum(enm) => {
            gen_doc_block(out, &enm.docs.to_markdown(docs_url_gen, false))?;
            writeln!(out, "public enum {}", enm.name)?;
            out.scope(|out| {
                for (name, discriminant, docs) in enm.variants.iter() {
                    gen_doc_block(out, &docs.to_markdown(docs_url_gen, false))?;
                    writeln!(out, "{} = {},", name, discriminant)?;
                }

                Ok(())
            })
        }
    }
}

fn gen_field(
    name: &ast::Ident,
    docs: &ast::Docs,
    typ: &ast::TypeName,
    in_path: &ast::Path,
    env: &Env,
    docs_url_gen: &ast::DocsUrlGenerator,
    out: &mut CodeWriter,
) -> fmt::Result {
    let mut type_declaration = String::new();
    gen_type_name_decl_position(typ, in_path, env, &mut type_declaration)?;
    let is_unsafe = type_declaration.ends_with('*');

    writeln!(out)?;
    gen_doc_block(out, &docs.to_markdown(docs_url_gen, false))?;
    gen_annotations_for_field(typ, out)?;
    write!(out, "public ")?;
    if is_unsafe {
        write!(out, "unsafe ")?;
    }
    writeln!(out, "{type_declaration} {name};")
}

fn gen_method(
    typ: &ast::CustomType,
    method: &ast::Method,
    in_path: &ast::Path,
    env: &Env,
    docs_url_gen: &ast::DocsUrlGenerator,
    out: &mut CodeWriter,
) -> fmt::Result {
    writeln!(out)?;

    gen_doc_block(out, &method.docs.to_markdown(docs_url_gen, false))?;
    gen_annotations_for_method(method, out)?;
    write!(out, "public static unsafe extern ")?;
    gen_type_name_return_position(method.return_type.as_ref(), in_path, env, out)?;

    write!(
        out,
        " {}(",
        method
            .full_path_name
            .as_str()
            .replace(&format!("{}_", typ.name()), "")
            .to_upper_camel_case()
    )?;

    let mut first = true;

    if let Some(ref self_param) = method.self_param {
        gen_param("self", &self_param.to_typename(), false, in_path, env, out)?;
        first = false;
    }

    for param in method.params.iter() {
        if first {
            first = false;
        } else {
            write!(out, ", ")?;
        }

        let name = param.name.as_str().to_lower_camel_case();
        gen_param(&name, &param.ty, param.is_writeable(), in_path, env, out)?;
    }

    writeln!(out, ");")?;

    Ok(())
}

pub fn gen_result(
    typ: &ast::TypeName,
    in_path: &ast::Path,
    env: &Env,
    out: &mut CodeWriter,
) -> fmt::Result {
    let (ok, err) = if let ast::TypeName::Result(ok, err) = typ {
        (ok, err)
    } else {
        panic!("not a result: {:?}", typ);
    };

    writeln!(out)?;
    writeln!(out, "[StructLayout(LayoutKind.Sequential)]")?;
    write!(out, "public partial struct ")?;
    gen_type_name(typ, in_path, env, out)?;
    writeln!(out)?;

    out.scope(|out| {
        // Omit variants or even the entire union if parts are zero-sized.
        // This matches what rustc effectively does with zero-sized union variants
        if !ok.is_zst() || !err.is_zst() {
            writeln!(out, "[StructLayout(LayoutKind.Explicit)]")?;
            writeln!(out, "private unsafe struct InnerUnion")?;

            out.scope(|out| {
                if !ok.is_zst() {
                    writeln!(out, "[FieldOffset(0)]")?;
                    write!(out, "internal ")?;
                    gen_type_name_decl_position(ok, in_path, env, out)?;
                    writeln!(out, " ok;")?;
                }

                if !err.is_zst() {
                    writeln!(out, "[FieldOffset(0)]")?;
                    write!(out, "internal ")?;
                    gen_type_name_decl_position(err, in_path, env, out)?;
                    writeln!(out, " err;")?;
                }

                Ok(())
            })?;

            writeln!(out)?;
            writeln!(out, "private InnerUnion _inner;")?;
            writeln!(out)?;
        }

        writeln!(out, "[MarshalAs(UnmanagedType.U1)]")?;
        writeln!(out, "public bool isOk;")?;

        if !ok.is_zst() {
            writeln!(out)?;
            write!(out, "public unsafe ")?;
            gen_type_name_decl_position(ok, in_path, env, out)?;
            writeln!(out, " Ok")?;
            out.scope(|out| {
                writeln!(out, "get")?;
                out.scope(|out| writeln!(out, "return _inner.ok;"))
            })?;
        }

        if !err.is_zst() {
            writeln!(out)?;
            write!(out, "public unsafe ")?;
            gen_type_name_decl_position(err, in_path, env, out)?;
            writeln!(out, " Err")?;
            out.scope(|out| {
                writeln!(out, "get")?;
                out.scope(|out| writeln!(out, "return _inner.err;"))
            })?;
        }

        Ok(())
    })?;

    Ok(())
}

fn gen_annotations_for_method(method: &ast::Method, out: &mut dyn fmt::Write) -> fmt::Result {
    writeln!(
        out,
        r#"[DllImport(NativeLib, CallingConvention = CallingConvention.Cdecl, EntryPoint = "{}", ExactSpelling = true)]"#,
        method.full_path_name
    )?;
    match &method.return_type {
        Some(ast::TypeName::Primitive(ast::PrimitiveType::bool)) => {
            writeln!(out, "[return: MarshalAs(UnmanagedType.U1)]")
        }
        _ => Ok(()),
    }
}

fn gen_annotations_for_param(typ: &ast::TypeName, out: &mut dyn fmt::Write) -> fmt::Result {
    match typ {
        ast::TypeName::Primitive(ast::PrimitiveType::bool) => {
            write!(out, "[MarshalAs(UnmanagedType.U1)] ")
        }
        _ => Ok(()),
    }
}

fn gen_annotations_for_field(typ: &ast::TypeName, out: &mut dyn fmt::Write) -> fmt::Result {
    match typ {
        ast::TypeName::Primitive(ast::PrimitiveType::bool) => {
            writeln!(out, "[MarshalAs(UnmanagedType.U1)]")
        }
        _ => Ok(()),
    }
}

fn gen_type_name_decl_position(
    typ: &ast::TypeName,
    in_path: &ast::Path,
    env: &Env,
    out: &mut dyn fmt::Write,
) -> fmt::Result {
    match typ {
        ast::TypeName::Option(opt) => match opt.as_ref() {
            ast::TypeName::Box(ptr) | ast::TypeName::Reference(.., ptr) => {
                gen_type_name_decl_position(ptr.as_ref(), in_path, env, out)?;
                write!(out, "*")
            }
            _ => panic!("Options without a pointer type are not yet supported"),
        },
        ast::TypeName::Box(underlying) | ast::TypeName::Reference(.., underlying) => {
            gen_type_name_decl_position(underlying.as_ref(), in_path, env, out)?;
            write!(out, "*")
        }
        ast::TypeName::Unit => panic!("unexpected unit type in declaration position"),
        _ => gen_type_name(typ, in_path, env, out),
    }
}

fn gen_type_name_return_position<'ast>(
    typ: impl Into<Option<&'ast ast::TypeName>>,
    in_path: &ast::Path,
    env: &Env,
    out: &mut dyn fmt::Write,
) -> fmt::Result {
    match &typ.into() {
        None | Some(ast::TypeName::Unit) => write!(out, "void"),
        Some(other) => gen_type_name_decl_position(other, in_path, env, out),
    }
}

fn gen_param(
    name: &str,
    typ: &ast::TypeName,
    is_writeable: bool,
    in_path: &ast::Path,
    env: &Env,
    out: &mut dyn fmt::Write,
) -> fmt::Result {
    if is_writeable {
        write!(out, "DiplomatWriteable* {name}")
    } else {
        match typ {
            ast::TypeName::StrReference(..) => {
                write!(out, "byte* {name}, nuint {name}Sz")
            }
            ast::TypeName::PrimitiveSlice(.., prim) => {
                write!(
                    out,
                    "{}* {name}, nuint {name}Sz",
                    super::types::type_name_for_prim(prim),
                )
            }
            ast::TypeName::Option(opt) => gen_param(name, opt.as_ref(), false, in_path, env, out),
            _ => {
                gen_annotations_for_param(typ, out)?;
                gen_type_name_decl_position(typ, in_path, env, out)?;
                write!(out, " {name}")
            }
        }
    }
}
