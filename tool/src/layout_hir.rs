use std::{alloc::Layout, cmp::max};

use diplomat_core::hir::{self, Everywhere, FloatType, Int128Type, IntSizeType, IntType, PrimitiveType, StructPathLike, TyPosition, Type, TypeContext};

// TODO(#58): support non-32-bit platforms
use u32 as usize_target;

pub fn struct_offsets_size_max_align<'a>(
	types : impl Iterator<Item = &'a Type>,
	tcx : &'a TypeContext
) -> (Vec<usize>, Layout) {
	
    let mut max_align = 0;
    let mut next_offset = 0;
    let mut offsets = vec![];

	for typ in types {
		let size_align = type_size_alignment(&typ, tcx);
		let size = size_align.size();
		let align = size_align.align();

		max_align = max(max_align, align);
		let padding = (align - (next_offset % align)) % align;
		next_offset += padding;
		offsets.push(next_offset);
		next_offset += size;
	}
	(offsets, Layout::from_size_align(next_offset, max_align).unwrap())
}

pub fn type_size_alignment(typ : &Type, tcx : &TypeContext) -> Layout {
	match *typ {
		Type::Enum(..) => Layout::new::<usize_target>(),
		Type::Opaque(..) => panic!("Size of opaque types is unknown."),
		Type::Slice(..) => Layout::new::<(usize_target, usize_target)>(),
		Type::Primitive(p) => primitive_size_alignment(p),
		Type::Struct(struct_path) => {
			let s = struct_path.resolve(tcx);
			let (_, size_max_align) = struct_offsets_size_max_align(
				s.fields.iter().map(|f| &f.ty),
				tcx
			);
			size_max_align
		},
		_ => unreachable!("Unknown AST/HIR variant {:?}", typ)
	}
}

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
        PrimitiveType::Int128(Int128Type::I128) | PrimitiveType::Int128(Int128Type::U128) => Layout::new::<u128>(),
        PrimitiveType::IntSize(IntSizeType::Isize) | PrimitiveType::IntSize(IntSizeType::Usize) => Layout::new::<usize_target>(),
        PrimitiveType::Float(FloatType::F32) => Layout::new::<f32>(),
        PrimitiveType::Float(FloatType::F64) => Layout::new::<f64>(),
    }
}