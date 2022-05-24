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
        ast::TypeName::Named(_) => write!(out, "{}", typ.resolve(in_path, env).name()),

        ast::TypeName::Box(underlying) => gen_type_name(underlying.as_ref(), in_path, env, out),

        ast::TypeName::Reference(underlying, _mutable, _lt) => {
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

        ast::TypeName::StrReference(_mut) => {
            write!(out, "string")
        }

        ast::TypeName::PrimitiveSlice(prim, _mut) => {
            write!(out, "{}[]", type_name_for_prim(prim))
        }

        ast::TypeName::Unit => {
            write!(out, "void")
        }
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
pub fn name_for_type(typ: &ast::TypeName) -> String {
    match typ {
        ast::TypeName::Named(name) => name.path.elements.last().unwrap().clone(),
        ast::TypeName::Box(underlying) => format!("Box{}", name_for_type(underlying)),
        ast::TypeName::Reference(underlying, ast::Mutability::Mutable, _lt) => {
            format!("RefMut{}", name_for_type(underlying))
        }
        ast::TypeName::Reference(underlying, ast::Mutability::Immutable, _lt) => {
            format!("Ref{}", name_for_type(underlying))
        }
        ast::TypeName::Primitive(prim) => prim.to_string().to_upper_camel_case(),
        ast::TypeName::Option(underlying) => format!("Opt{}", name_for_type(underlying)),
        ast::TypeName::Result(ok, err) => {
            format!("Result{}{}", name_for_type(ok), name_for_type(err))
        }
        ast::TypeName::Writeable => "Writeable".to_owned(),
        ast::TypeName::StrReference(ast::Mutability::Mutable) => "StrRefMut".to_owned(),
        ast::TypeName::StrReference(ast::Mutability::Immutable) => "StrRef".to_owned(),
        ast::TypeName::PrimitiveSlice(prim, ast::Mutability::Mutable) => {
            format!("RefMutPrimSlice{}", prim.to_string().to_upper_camel_case())
        }
        ast::TypeName::PrimitiveSlice(prim, ast::Mutability::Immutable) => {
            format!("RefPrimSlice{}", prim.to_string().to_upper_camel_case())
        }
        ast::TypeName::Unit => "Void".to_owned(),
    }
}
