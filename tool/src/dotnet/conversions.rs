use std::fmt::{self, Write as _};

use diplomat_core::{ast, Env};

use super::types::{gen_type_name_to_string, type_name_for_prim};
use crate::util::CodeWriter;

pub struct SliceParam {
    pub array_var_name: String,
    pub ptr_var_name: String,
    pub length_var_name: String,
    pub underlying_type: ast::PrimitiveType,
}

impl SliceParam {
    pub fn new(name: String, underlying_type: ast::PrimitiveType) -> Self {
        let ptr_var_name = format!("{name}Ptr");
        let length_var_name = format!("{name}Length");
        Self {
            array_var_name: name,
            ptr_var_name,
            length_var_name,
            underlying_type,
        }
    }

    pub fn open_fixed_block(&self, out: &mut CodeWriter) -> fmt::Result {
        writeln!(
            out,
            "fixed ({}* {} = {})",
            type_name_for_prim(&self.underlying_type),
            self.ptr_var_name,
            self.array_var_name,
        )?;
        writeln!(out, "{{")?;
        out.indent();
        Ok(())
    }

    pub fn close_fixed_block(self, out: &mut CodeWriter) -> fmt::Result {
        out.dedent();
        writeln!(out, "}}")
    }
}

/// Writes the raw → idiomatic conversion code
///
/// Primitive type: nothing to do
/// Struct/opaque custom type: wrap pointer into a managed class
/// Enum: cast from raw enum
pub fn to_idiomatic_object<W: fmt::Write>(
    env: &Env,
    typ: &ast::TypeName,
    in_path: &ast::Path,
    input_var_name: &str,
    out: &mut W,
) -> fmt::Result {
    match typ {
        ast::TypeName::Primitive(_) => out.write_str(input_var_name),
        ast::TypeName::Box(boxed) => {
            to_idiomatic_object(env, boxed.as_ref(), in_path, input_var_name, out)
        }
        ast::TypeName::Reference(.., reference) => {
            to_idiomatic_object(env, reference.as_ref(), in_path, input_var_name, out)
        }
        ast::TypeName::Option(opt) => {
            to_idiomatic_object(env, opt.as_ref(), in_path, input_var_name, out)
        }
        _ => {
            let name = gen_type_name_to_string(typ, in_path, env)?;
            match typ {
                ast::TypeName::Named(path_type) | ast::TypeName::SelfType(path_type) => {
                    match path_type.resolve(in_path, env) {
                        ast::CustomType::Struct(_) | ast::CustomType::Opaque(_) => {
                            write!(out, "new {name}({input_var_name})")
                        }
                        ast::CustomType::Enum(_) => {
                            write!(out, "({name}){input_var_name}")
                        }
                        &_ => unreachable!("unknown AST/HIR variant"),
                    }
                }
                other => panic!("expected named type name, found `{}`", other),
            }
        }
    }
}

/// Writes the idiomatic → raw conversion code
///
/// Primitive type: nothing to do
/// Struct/opaque custom type: extract raw representation using `.AsFFI()`
/// Enum: cast from idiomatic enum
pub fn to_raw_object<W: fmt::Write>(
    env: &Env,
    typ: &ast::TypeName,
    in_path: &ast::Path,
    input_var_name: &str,
    out: &mut W,
) -> fmt::Result {
    match typ {
        ast::TypeName::Primitive(_) => out.write_str(input_var_name),
        ast::TypeName::Box(boxed) => {
            to_raw_object(env, boxed.as_ref(), in_path, input_var_name, out)
        }
        ast::TypeName::Reference(.., reference) => {
            to_raw_object(env, reference.as_ref(), in_path, input_var_name, out)
        }
        ast::TypeName::Option(opt) => {
            to_raw_object(env, opt.as_ref(), in_path, input_var_name, out)
        }
        _ => {
            let name = gen_type_name_to_string(typ, in_path, env)?;
            match typ {
                ast::TypeName::Named(path_type) | ast::TypeName::SelfType(path_type) => {
                    match path_type.resolve(in_path, env) {
                        ast::CustomType::Struct(_) | ast::CustomType::Opaque(_) => {
                            write!(out, "{input_var_name}.AsFFI()")
                        }
                        ast::CustomType::Enum(_) => {
                            write!(out, "(Raw.{name}){input_var_name}")
                        }
                        &_ => unreachable!("unknown AST/HIR variant"),
                    }
                }
                other => panic!("expected named type name, found `{}`", other),
            }
        }
    }
}
