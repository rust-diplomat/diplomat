use diplomat_core::{ast, Env};
use std::collections::HashMap;
use std::fmt::{self, Write};

use crate::util;

#[cfg(test)]
#[macro_use]
mod test_util;

pub mod docs;

pub mod types;

pub mod structs;
use structs::*;

use self::conversions::Csv;

pub mod conversions;

pub mod display;

pub fn gen_bindings(
    env: &Env,
    outs: &mut HashMap<String, String>,
    docs_url_gen: Option<&ast::DocsUrlGenerator>,
) -> fmt::Result {
    outs.entry("diplomat-runtime.mjs".to_string())
        .or_default()
        .write_str(include_str!("runtime.mjs"))?;
    outs.entry("diplomat-runtime.d.ts".to_string())
        .or_default()
        .write_str(include_str!("runtime.d.ts"))?;
    outs.entry("diplomat-wasm.mjs".to_string())
        .or_default()
        .write_str(include_str!("wasm.mjs"))?;

    let mut all_types = util::get_all_custom_types(env);
    all_types.sort_by_key(|t| t.1.name());

    let index_ts = outs.entry("index.d.ts".to_string()).or_default();
    writeln!(
        index_ts,
        "export {{ FFIError, i8, u8, i16, u16, i32, u32, i64, u64, f32, f64, char }} from './diplomat-runtime';"
    )?;
    for (_, custom_type) in &all_types {
        let name = custom_type.name();
        writeln!(index_ts, "export {{ {name} }} from './{name}';",)?;
    }

    let index_js = outs.entry("index.mjs".to_string()).or_default();
    writeln!(
        index_js,
        "export {{ FFIError }} from './diplomat-runtime.mjs';"
    )?;
    for (_, custom_type) in &all_types {
        let name = custom_type.name();
        writeln!(index_js, "export {{ {name} }} from './{name}.mjs';",)?;
    }

    for (in_path, custom_type) in &all_types {
        let imports = Imports::new(custom_type, in_path, env);

        let out = outs
            .entry(format!("{}.mjs", custom_type.name()))
            .or_default();

        writeln!(out, "import wasm from \"./diplomat-wasm.mjs\"")?;
        writeln!(
            out,
            "import * as diplomatRuntime from \"./diplomat-runtime.mjs\""
        )?;
        for custom_type in imports.js_imports.iter() {
            if let ast::CustomType::Enum(enm) = custom_type {
                writeln!(
                    out,
                    "import {{ {0}_js_to_rust, {0}_rust_to_js }} from \"./{0}.mjs\"",
                    enm.name
                )?;
            } else {
                writeln!(
                    out,
                    "import {{ {0} }} from \"./{0}.mjs\"",
                    custom_type.name()
                )?;
            }
        }
        writeln!(out)?;

        gen_struct(out, custom_type, in_path, env)?;

        // == Declaration file ==

        let out = outs
            .entry(format!("{}.d.ts", custom_type.name()))
            .or_default();
        if !imports.ts_primitives.is_empty() {
            writeln!(
                out,
                "import {{ {} }} from \"./diplomat-runtime\"",
                Csv(imports.ts_primitives.iter().map(|prim| match prim {
                    ast::PrimitiveType::i8 => "i8",
                    ast::PrimitiveType::u8 => "u8",
                    ast::PrimitiveType::i16 => "i16",
                    ast::PrimitiveType::u16 => "u16",
                    ast::PrimitiveType::i32 => "i32",
                    ast::PrimitiveType::u32 => "u32",
                    ast::PrimitiveType::i64 => "i64",
                    ast::PrimitiveType::u64 => "u64",
                    ast::PrimitiveType::i128 => panic!("i128 is unsupported"),
                    ast::PrimitiveType::u128 => panic!("u128 is unsupported"),
                    ast::PrimitiveType::isize => "isize",
                    ast::PrimitiveType::usize => "usize",
                    ast::PrimitiveType::f32 => "f32",
                    ast::PrimitiveType::f64 => "f64",
                    ast::PrimitiveType::char => "char",
                    ast::PrimitiveType::bool =>
                        unreachable!("bools aren't added because TypeScript has `boolean`"),
                }))
            )?;
        }
        if imports.ts_ffierror {
            writeln!(out, "import {{ FFIError }} from \"./diplomat-runtime\"")?;
        }
        for custom_type in imports.ts_imports.iter() {
            writeln!(out, "import {{ {0} }} from \"./{0}\";", custom_type.name())?;
        }
        writeln!(out)?;

        gen_ts_custom_type_declaration(out, custom_type, in_path, env, docs_url_gen)?;
    }

    Ok(())
}

/// A struct for detecting all the the imports required for .mjs and d.ts files.
#[derive(Default)]
struct Imports<'env> {
    /// Type that show up in a type's fields, or as a parameter or return value.
    /// Nested enums are also included since enums are converted at the boundary.
    js_imports: Vec<&'env ast::CustomType>,

    /// Types that show up in a type's fields, or as a parameter or return value
    ts_imports: Vec<&'env ast::CustomType>,

    /// Numeric primitive types and `char`, for more specific aliases to TypeScript's
    /// `number` and `string` types.
    ts_primitives: Vec<ast::PrimitiveType>,

    /// Whether or not a method returns a `Result`, which translates to potentially
    /// throwing an error in TypeScript/JavaScript.
    ts_ffierror: bool,
}

/// The context that a type appears in relation to a struct/opaque/enum.
///
/// When determining what types to import via the [`Imports`] type, JavaScript
/// and TypeScript care about different things. For example, JavaScript has to
/// construct struct fields in the constructor (if the type is a struct),
/// convert enums at the boundary in methods, and construct a return type, so it
/// has to import those types. In TypeScript's `.d.ts` files, we want to import
/// struct fields (if the type is a struct), parameter types, and return types
/// that are of the `Ok` variant, but _not_ of the `Err` variant (which gets
/// documented in TSDoc instead of the type signature).
#[derive(Copy, Clone)]
enum TypePosition {
    Return,
    Field,
    Param,
    Inner,
}

impl<'env> Imports<'env> {
    fn new(custom_type: &'env ast::CustomType, in_path: &ast::Path, env: &'env Env) -> Self {
        let mut this = Imports::default();

        if let ast::CustomType::Struct(strct) = custom_type {
            for (_, typ, _) in strct.fields.iter() {
                this.collect_usages(typ, in_path, env, TypePosition::Field);
            }
        }

        for method in custom_type.methods() {
            for param in method.params.iter() {
                this.collect_usages(&param.ty, in_path, env, TypePosition::Param);
            }

            if let Some(ref return_type) = method.return_type {
                this.collect_usages(return_type, in_path, env, TypePosition::Return);
            }
        }

        this.js_imports.retain(|t| *t != custom_type);
        this.js_imports.sort_unstable_by_key(|t| t.name());
        this.js_imports.dedup_by_key(|t| t.name());

        this.ts_imports.retain(|t| *t != custom_type);
        this.ts_imports.sort_unstable_by_key(|t| t.name());
        this.ts_imports.dedup_by_key(|t| t.name());

        this.ts_primitives.sort_by_key(|p| *p as u8);
        this.ts_primitives.dedup_by_key(|p| *p as u8);

        this
    }

    fn collect_usages(
        &mut self,
        typ: &'env ast::TypeName,
        in_path: &ast::Path,
        env: &'env Env,
        state: TypePosition,
    ) {
        match typ {
            ast::TypeName::Named(path_type) | ast::TypeName::SelfType(path_type) => {
                let (ty_in_path, custom) = path_type.resolve_with_path(in_path, env);
                // JS wants: return type, fields, _all_ enums.
                // TS wants: return type, fields, params.
                match state {
                    TypePosition::Return | TypePosition::Field => {
                        self.ts_imports.push(custom);
                        self.js_imports.push(custom);
                    }
                    TypePosition::Param => {
                        self.ts_imports.push(custom);
                    }
                    TypePosition::Inner => {}
                }

                match custom {
                    ast::CustomType::Opaque(_) => {}
                    ast::CustomType::Enum(_) => {
                        self.js_imports.push(custom);
                    }
                    ast::CustomType::Struct(strct) => {
                        for (_, typ, _) in strct.fields.iter() {
                            self.collect_usages(typ, &ty_in_path, env, TypePosition::Inner)
                        }
                    }
                    &_ => unreachable!("unknown AST/HIR variant"),
                }
            }
            ast::TypeName::Reference(.., typ)
            | ast::TypeName::Box(typ)
            | ast::TypeName::Option(typ) => {
                self.collect_usages(typ, in_path, env, state);
            }
            ast::TypeName::Result(ok, err, _) => {
                self.collect_usages(ok, in_path, env, state);
                self.collect_usages(err, in_path, env, state);
                self.ts_ffierror = true;
            }
            ast::TypeName::Primitive(ast::PrimitiveType::bool) => {}
            ast::TypeName::Primitive(prim) => {
                if !matches!(state, TypePosition::Inner) {
                    self.ts_primitives.push(*prim);
                }
            }
            _ => {}
        }
    }
}
