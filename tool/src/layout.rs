use core::panic;
use std::{cmp::max, collections::HashMap};

use diplomat_core::ast::{self, PrimitiveType, TypeName};

// TODO(shadaj): support non-32-bit platforms
// TODO(shadaj): consider special types instead of tuples

pub fn struct_size_offsets_max_align(
    strct: &ast::Struct,
    in_path: &ast::Path,
    env: &HashMap<ast::Path, HashMap<String, ast::ModSymbol>>,
) -> (usize, Vec<usize>, usize) {
    let mut max_align = 0;
    let mut next_offset = 0;
    let mut offsets = vec![];

    for (_, typ, _) in &strct.fields {
        let (size, align) = type_size_alignment(typ, in_path, env);
        max_align = max(max_align, align);
        let padding = (align - (next_offset % align)) % align;
        next_offset += padding;
        offsets.push(next_offset);
        next_offset += size;
    }

    (next_offset, offsets, max_align)
}

pub fn result_size_ok_offset_align(
    ok: &TypeName,
    err: &TypeName,
    in_path: &ast::Path,
    env: &HashMap<ast::Path, HashMap<String, ast::ModSymbol>>,
) -> (usize, usize, usize) {
    let (ok_size, _) = type_size_alignment(ok, in_path, env);
    let (err_size, _) = type_size_alignment(err, in_path, env);
    let (size, offsets, max_align) = struct_size_offsets_max_align(
        &ast::Struct {
            name: "".to_string(),
            doc_lines: "".to_string(),
            fields: vec![
                if ok_size > err_size {
                    ("".to_string(), ok.clone(), "".to_string())
                } else {
                    ("".to_string(), err.clone(), "".to_string())
                },
                (
                    "".to_string(),
                    ast::TypeName::Primitive(PrimitiveType::bool),
                    "".to_string(),
                ),
            ],
            methods: vec![],
        },
        in_path,
        env,
    );
    (size, offsets[1], max_align)
}

pub fn type_size_alignment(
    typ: &ast::TypeName,
    in_path: &ast::Path,
    env: &HashMap<ast::Path, HashMap<String, ast::ModSymbol>>,
) -> (usize, usize) {
    match typ {
        ast::TypeName::Box(_) => (4, 4),
        ast::TypeName::Reference(_, _) => (4, 4),
        ast::TypeName::Option(underlying) => match underlying.as_ref() {
            ast::TypeName::Box(_) => type_size_alignment(underlying, in_path, env),
            _ => todo!(),
        },
        ast::TypeName::Result(ok, err) => {
            let (size, _, align) = result_size_ok_offset_align(ok, err, in_path, env);
            (size, align)
        }
        ast::TypeName::Named(_) => match typ.resolve(in_path, env) {
            ast::CustomType::Struct(strct) => {
                let (size, _, max_align) = struct_size_offsets_max_align(strct, in_path, env);
                (size, max_align)
            }

            ast::CustomType::Enum(_) => {
                // repr(C) fieldless enums use the default platform representation: isize
                (4, 4)
            }

            ast::CustomType::Opaque(_) => {
                panic!("Size of opaque types is unknown")
            }
        },
        ast::TypeName::Primitive(p) => match p {
            ast::PrimitiveType::bool => (1, 1),
            ast::PrimitiveType::char => (1, 1),
            ast::PrimitiveType::i8 | ast::PrimitiveType::u8 => (1, 1),
            ast::PrimitiveType::i16 | ast::PrimitiveType::u16 => (2, 2),
            ast::PrimitiveType::i32 | ast::PrimitiveType::u32 => (4, 4),
            ast::PrimitiveType::i64 | ast::PrimitiveType::u64 => (4, 4),
            ast::PrimitiveType::i128 | ast::PrimitiveType::u128 => (4, 4),
            ast::PrimitiveType::isize | ast::PrimitiveType::usize => (4, 4),
            ast::PrimitiveType::f32 => (4, 4),
            ast::PrimitiveType::f64 => (4, 4),
        },
        ast::TypeName::StrReference => (4, 4),
        ast::TypeName::Writeable => panic!(),
        ast::TypeName::Void => (0, 1),
    }
}
