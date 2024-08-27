//! The corpse of the old AST backend, wearing a fresh coat of paint. AST used to have this  `layout.rs` file for figuring out how types would look in memory.
//!
//! Every backend needed this. But now only Javascript does. And we pretty much only use it for structs; WASM sometimes requires us to create an appropriately sized buffer for a struct. It sometimes also requires us to pad method signatures when inserting a flattened structure (see [`super::type_generation::TyGenContext::gen_c_to_js_for_return_type`] or [`super::type_generation::TyGenContext::generate_fields`] for more).
use std::{alloc::Layout, cmp::max};

use diplomat_core::hir::{
    self, FloatType, Int128Type, IntSizeType, IntType, PrimitiveType, StructPathLike, Type,
    TypeContext,
};

// TODO(#58): support non-32-bit platforms
use u32 as usize_target;

pub struct StructFieldLayout {
    /// The offset of this field in the struct
    pub offset: usize,
    /// The number of padding fields needed after this field
    pub padding_count: usize,
    /// The size of the padding field
    pub padding_size: usize,
    /// The number of scalar fields in this field
    pub scalar_count: usize,
}

pub struct StructFieldsInfo {
    /// Layout details for individual fields
    pub fields: Vec<StructFieldLayout>,
    /// The layout of the struct overall
    pub struct_layout: Layout,
    /// The number of scalar fields in this struct
    pub scalar_count: usize,
}
/// Given a struct, calculate where each of its fields is in memory.
///
/// ([`Vec<usize>`], _) is the list of offsets that each field is at in memory.
/// (_, [`Layout`]) represents the [`Layout`] of our structure, in full.
pub fn struct_field_info<'a, P: hir::TyPosition + 'a>(
    types: impl Iterator<Item = &'a Type<P>>,
    tcx: &'a TypeContext,
) -> StructFieldsInfo {
    let mut max_align = 0;
    let mut next_offset = 0;
    let mut fields: Vec<StructFieldLayout> = vec![];
    let mut scalar_count = 0;

    let types = types.collect::<Vec<_>>();
    if types.is_empty() {
        return StructFieldsInfo {
            fields: vec![],
            struct_layout: unit_size_alignment(),
            scalar_count: 0,
        };
    }

    let mut prev_align = 1;
    for typ in types {
        let (size_align, field_scalars) = type_size_alignment_and_scalar_count(typ, tcx);
        scalar_count += field_scalars;
        let size = size_align.size();
        let align = size_align.align();

        max_align = max(max_align, align);
        let padding = (align - (next_offset % align)) % align;
        next_offset += padding;

        // Tack padding on to previous field
        //
        // We don't know until we see the next field if padding is needed, but padding
        // belongs to the field before it, not the field after it (since there can be padding at the end, but never
        // padding at the beginning)
        if padding != 0 {
            assert!(padding % prev_align == 0, "Needed padding {padding} must be a perfect multiple of the previous field alignment {prev_align}");
            let fields_len = fields.len();
            assert!(
                fields_len != 0,
                "Padding can only be found after first field!"
            );

            fields[fields_len - 1].padding_count = padding / prev_align;
            fields[fields_len - 1].padding_size = prev_align;
        }

        fields.push(StructFieldLayout {
            offset: next_offset,
            padding_count: 0,
            padding_size: 1,
            scalar_count: field_scalars,
        });
        prev_align = align;
        next_offset += size;
    }

    // Structs can have padding at the end, too
    if next_offset % max_align != 0 {
        let fields_len = fields.len();
        let padding = (max_align - (next_offset % max_align)) % max_align;
        fields[fields_len - 1].padding_count = padding / prev_align;
        fields[fields_len - 1].padding_size = prev_align;
    }

    StructFieldsInfo {
        fields,
        struct_layout: Layout::from_size_align(next_offset, max_align).unwrap(),
        scalar_count,
    }
}

pub fn opaque_size_alignment() -> Layout {
    // TODO: Is this correct?
    Layout::new::<usize_target>()
}

pub fn unit_size_alignment() -> Layout {
    // TODO: Is this correct?
    Layout::new::<usize_target>()
}

/// Get the [`Layout`] for a specific type.
pub fn type_size_alignment<P: hir::TyPosition>(typ: &Type<P>, tcx: &TypeContext) -> Layout {
    type_size_alignment_and_scalar_count(typ, tcx).0
}

/// Get the [`Layout`] for a specific type, as well as the number of scalar fields it contains
pub fn type_size_alignment_and_scalar_count<P: hir::TyPosition>(
    typ: &Type<P>,
    tcx: &TypeContext,
) -> (Layout, usize) {
    match typ {
        // repr(C) fieldless enums use the default platform representation: isize
        Type::Enum(..) => (Layout::new::<usize_target>(), 1),
        Type::Opaque(..) => (opaque_size_alignment(), 1),
        Type::Slice(..) => (Layout::new::<(usize_target, usize_target)>(), 2),
        Type::Primitive(p) => (primitive_size_alignment(*p), 1),
        Type::Struct(struct_path) => {
            let def = tcx.resolve_type(struct_path.id());
            let info = match def {
                hir::TypeDef::OutStruct(out_struct) => {
                    struct_field_info(out_struct.fields.iter().map(|f| &f.ty), tcx)
                }
                hir::TypeDef::Struct(struct_def) => {
                    struct_field_info(struct_def.fields.iter().map(|f| &f.ty), tcx)
                }
                _ => panic!("Should be a struct TypeDef."),
            };
            (info.struct_layout, info.scalar_count)
        }
        _ => unreachable!("Unknown AST/HIR variant {:?}", typ),
    }
}

/// Get the [`Layout`] for a specific [`PrimitiveType`].
pub fn primitive_size_alignment(prim: PrimitiveType) -> Layout {
    match prim {
        PrimitiveType::Bool => Layout::new::<bool>(),
        PrimitiveType::Char => Layout::new::<char>(),
        PrimitiveType::Int(IntType::I8) | PrimitiveType::Int(IntType::U8) | PrimitiveType::Byte => {
            Layout::new::<u8>()
        }
        PrimitiveType::Int(IntType::I16) | PrimitiveType::Int(IntType::U16) => Layout::new::<u16>(),
        PrimitiveType::Int(IntType::I32) | PrimitiveType::Int(IntType::U32) => Layout::new::<u32>(),
        PrimitiveType::Int(IntType::I64) | PrimitiveType::Int(IntType::U64) => Layout::new::<u64>(),
        PrimitiveType::Int128(Int128Type::I128) | PrimitiveType::Int128(Int128Type::U128) => {
            Layout::new::<u128>()
        }
        PrimitiveType::IntSize(IntSizeType::Isize) | PrimitiveType::IntSize(IntSizeType::Usize) => {
            Layout::new::<usize_target>()
        }
        PrimitiveType::Float(FloatType::F32) => Layout::new::<f32>(),
        PrimitiveType::Float(FloatType::F64) => Layout::new::<f64>(),
    }
}
