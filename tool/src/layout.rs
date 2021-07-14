use core::panic;
use std::{cmp::max, collections::HashMap};

use diplomat_core::ast;

// TODO(shadaj): support non-32-bit platforms
// TODO(shadaj): consider special types instead of tuples

pub fn struct_size_offsets_max_align(
    strct: &ast::Struct,
    env: &HashMap<String, ast::CustomType>,
) -> (usize, Vec<usize>, usize) {
    let mut max_align = 0;
    let mut next_offset = 0;
    let mut offsets = vec![];

    for (_, typ, _) in &strct.fields {
        let (size, align) = type_size_alignment(typ, env);
        max_align = max(max_align, align);
        let padding = (align - (next_offset % align)) % align;
        next_offset += padding;
        offsets.push(next_offset);
        next_offset += size;
    }

    (next_offset, offsets, max_align)
}

pub fn type_size_alignment(
    typ: &ast::TypeName,
    env: &HashMap<String, ast::CustomType>,
) -> (usize, usize) {
    match typ {
        ast::TypeName::Box(_) => (4, 4),
        ast::TypeName::Reference(_, _) => (4, 4),
        ast::TypeName::Named(_) => match typ.resolve(env) {
            ast::CustomType::Struct(strct) => {
                let (size, _, max_align) = struct_size_offsets_max_align(strct, env);
                (size, max_align)
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
    }
}
