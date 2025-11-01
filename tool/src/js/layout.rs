//! The corpse of the old AST backend, wearing a fresh coat of paint. AST used to have this  `layout.rs` file for figuring out how types would look in memory.
//!
//! Every backend needed this. But now only Javascript does. And we pretty much only use it for structs; WASM sometimes requires us to create an appropriately sized buffer for a struct. It sometimes also requires us to pad method signatures when inserting a flattened structure (see [`super::type_generation::ItemGenContext::gen_c_to_js_for_return_type`] or [`super::type_generation::ItemGenContext::generate_fields`] for more).
use std::ops::{Add, AddAssign};
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
    ///
    /// Note that this is NOT the total amount of padding: padding fields can be of different
    /// sizes, see docs/wasm_abi_quirks.md
    pub padding_count: usize,
    /// The width of an individual padding field
    pub padding_field_width: usize,
}

pub struct StructFieldsInfo {
    /// Layout details for individual fields
    pub fields: Vec<StructFieldLayout>,
    /// The layout of the struct overall
    pub struct_layout: Layout,
    /// The number of scalar (integer primitive) fields in this struct, transitively. Does not count padding fields.
    pub scalar_count: ScalarCount,
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
    let mut scalar_count = ScalarCount::Zst;

    let types = types.collect::<Vec<_>>();
    if types.is_empty() {
        return StructFieldsInfo {
            fields: vec![],
            struct_layout: unit_size_alignment(),
            scalar_count: ScalarCount::Zst,
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
            // The padding field width is the alignment of the previous field, see docs/wasm_abi_quirks.md
            fields[fields_len - 1].padding_count = padding / prev_align;
            fields[fields_len - 1].padding_field_width = prev_align;
        }

        fields.push(StructFieldLayout {
            offset: next_offset,
            padding_count: 0,
            padding_field_width: 1,
        });
        prev_align = align;
        next_offset += size;
    }

    // Structs can have padding at the end, too
    if next_offset % max_align != 0 {
        let fields_len = fields.len();
        let padding = (max_align - (next_offset % max_align)) % max_align;
        fields[fields_len - 1].padding_count = padding / prev_align;
        fields[fields_len - 1].padding_field_width = prev_align;
        next_offset += padding;
    }

    debug_assert!(
        next_offset % max_align == 0,
        "Size {next_offset} must be a multiple of alignment {max_align}"
    );

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
) -> (Layout, ScalarCount) {
    match typ {
        // repr(C) fieldless enums use the default platform representation: isize
        Type::Enum(..) => (Layout::new::<usize_target>(), ScalarCount::Scalars(1)),
        Type::Opaque(..) => (opaque_size_alignment(), ScalarCount::Scalars(1)),
        Type::Slice(..) => (
            Layout::new::<(usize_target, usize_target)>(),
            ScalarCount::Scalars(2),
        ),
        Type::Primitive(p) => (primitive_size_alignment(*p), ScalarCount::Scalars(1)),
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
        Type::DiplomatOption(inner) => {
            let (layout, inner_scalar) = type_size_alignment_and_scalar_count(inner, tcx);

            if inner_scalar == ScalarCount::Zst {
                unimplemented!("Option<ZST> has quirky wasm ABI behavior that is not implemented")
            }
            let size = layout.size();
            let align = layout.align();
            debug_assert!(size % align == 0, "Found inner type {typ:?} with size {size} that is not a multiple of its alignment {align}");
            // A DiplomatOption will always add a new, aligned-to-T boolean field and requisite padding, which just increases the size by `align`
            let layout = Layout::from_size_align(size + align, align).unwrap();
            // See wasm_abi_quirks.md section "unions", union types never partake in anything other than the "indirect" pass mode,
            // which
            (layout, ScalarCount::Memory)
        }
        _ => unreachable!("Unknown AST/HIR variant {:?}", typ),
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub(crate) enum ScalarCount {
    /// This is a zero-sized type and has no scalars
    Zst,
    /// This contains n scalar fields
    Scalars(usize),
    /// This contains unions (and potentially other scalar fields)
    /// (corresponds to Rust `BackendRepr::Memory`)
    Memory,
}

impl Add<ScalarCount> for ScalarCount {
    type Output = Self;
    fn add(self, other: ScalarCount) -> Self {
        match (self, other) {
            (_, ScalarCount::Memory) | (ScalarCount::Memory, _) => ScalarCount::Memory,
            (a, ScalarCount::Zst) | (ScalarCount::Zst, a) => a,
            (ScalarCount::Scalars(a), ScalarCount::Scalars(b)) => ScalarCount::Scalars(a + b),
        }
    }
}

impl AddAssign<ScalarCount> for ScalarCount {
    fn add_assign(&mut self, other: ScalarCount) {
        *self = *self + other;
    }
}

/// Get the [`Layout`] for a specific [`PrimitiveType`].
pub fn primitive_size_alignment(prim: PrimitiveType) -> Layout {
    match prim {
        PrimitiveType::Bool => Layout::new::<bool>(),
        PrimitiveType::Char => Layout::new::<char>(),
        PrimitiveType::Int(IntType::I8)
        | PrimitiveType::Int(IntType::U8)
        | PrimitiveType::Byte
        | PrimitiveType::Ordering => Layout::new::<u8>(),
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
