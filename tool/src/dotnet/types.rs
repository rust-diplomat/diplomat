use core::fmt;
use diplomat_core::ast;
use diplomat_core::Env;
use heck::ToUpperCamelCase;

pub fn gen_type_name_to_string(
    typ: &ast::TypeName,
    in_path: &ast::Path,
    env: &Env,
) -> Result<String, fmt::Error> {
    let mut s = String::new();
    gen_type_name(typ, in_path, env, &mut s)?;
    Ok(s)
}

pub fn gen_type_name(
    typ: &ast::TypeName,
    in_path: &ast::Path,
    env: &Env,
    out: &mut dyn fmt::Write,
) -> fmt::Result {
    match typ {
        ast::TypeName::Named(path_type) | ast::TypeName::SelfType(path_type) => {
            write!(out, "{}", path_type.resolve(in_path, env).name())
        }

        ast::TypeName::Box(underlying) => gen_type_name(underlying.as_ref(), in_path, env, out),

        ast::TypeName::Reference(.., underlying) => {
            gen_type_name(underlying.as_ref(), in_path, env, out)
        }

        ast::TypeName::Option(underlying) => gen_type_name(underlying.as_ref(), in_path, env, out),

        ast::TypeName::Result(..) => {
            write!(
                out,
                "{}{}",
                in_path.elements.join("_").to_upper_camel_case(),
                name_for_type(typ)
            )
        }

        ast::TypeName::Primitive(prim) => {
            write!(out, "{}", type_name_for_prim(prim))
        }

        ast::TypeName::Writeable => {
            write!(out, "DiplomatWriteable")
        }

        ast::TypeName::StrReference(
            _,
            ast::StringEncoding::UnvalidatedUtf8 | ast::StringEncoding::Utf8,
        ) => {
            write!(out, "string")
        }

        ast::TypeName::StrReference(_, ast::StringEncoding::UnvalidatedUtf16) => {
            write!(out, "ushort[]")
        }

        ast::TypeName::PrimitiveSlice(.., prim) => {
            write!(out, "{}[]", type_name_for_prim(prim))
        }

        ast::TypeName::Unit => {
            write!(out, "void")
        }
        &_ => unreachable!("unknown AST/HIR variant"),
    }
}

pub fn type_name_for_prim(prim: &ast::PrimitiveType) -> &str {
    match prim {
        ast::PrimitiveType::i8 => "sbyte",
        ast::PrimitiveType::u8 => "byte",
        ast::PrimitiveType::i16 => "short",
        ast::PrimitiveType::u16 => "ushort",
        ast::PrimitiveType::i32 => "int",
        ast::PrimitiveType::u32 => "uint",
        ast::PrimitiveType::i64 => "long",
        ast::PrimitiveType::u64 => "ulong",
        ast::PrimitiveType::i128 => "Int128",
        ast::PrimitiveType::u128 => "UInt128",
        ast::PrimitiveType::isize => "nint",
        ast::PrimitiveType::usize => "nuint",
        ast::PrimitiveType::f32 => "float",
        ast::PrimitiveType::f64 => "double",
        ast::PrimitiveType::bool => "bool",
        ast::PrimitiveType::char => "uint",
    }
}

/// Generates a struct name that uniquely identifies the given type.
pub fn name_for_type(typ: &ast::TypeName) -> ast::Ident {
    match typ {
        ast::TypeName::Named(name) | ast::TypeName::SelfType(name) => {
            name.path.elements.last().unwrap().clone()
        }
        ast::TypeName::Box(underlying) => {
            ast::Ident::from(format!("Box{}", name_for_type(underlying)))
        }
        ast::TypeName::Reference(_, ast::Mutability::Mutable, underlying) => {
            ast::Ident::from(format!("RefMut{}", name_for_type(underlying)))
        }
        ast::TypeName::Reference(_, ast::Mutability::Immutable, underlying) => {
            ast::Ident::from(format!("Ref{}", name_for_type(underlying)))
        }
        ast::TypeName::Primitive(prim) => ast::Ident::from(prim.to_string().to_upper_camel_case()),
        ast::TypeName::Option(underlying) => {
            ast::Ident::from(format!("Opt{}", name_for_type(underlying)))
        }
        ast::TypeName::Result(ok, err, _) => {
            ast::Ident::from(format!("Result{}{}", name_for_type(ok), name_for_type(err)))
        }
        ast::TypeName::Writeable => ast::Ident::from("Writeable"),
        ast::TypeName::StrReference(
            _,
            ast::StringEncoding::UnvalidatedUtf8 | ast::StringEncoding::Utf8,
        ) => ast::Ident::from("StrRef8"),
        ast::TypeName::StrReference(_, ast::StringEncoding::UnvalidatedUtf16) => {
            ast::Ident::from("RefMutPrimSliceU16")
        }
        ast::TypeName::PrimitiveSlice(_, ast::Mutability::Mutable, prim) => ast::Ident::from(
            format!("RefMutPrimSlice{}", prim.to_string().to_upper_camel_case()),
        ),
        ast::TypeName::PrimitiveSlice(_, ast::Mutability::Immutable, prim) => ast::Ident::from(
            format!("RefPrimSlice{}", prim.to_string().to_upper_camel_case()),
        ),
        ast::TypeName::Unit => ast::Ident::from("Void"),
        &_ => unreachable!("unknown AST/HIR variant"),
    }
}
