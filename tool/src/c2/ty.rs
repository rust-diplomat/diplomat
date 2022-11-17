use super::header::Header;
use super::CContext;
use diplomat_core::hir::{self, TyPosition, Type, TypeDef, TypeId};
use std::borrow::Cow;

pub fn gen_ty(cx: &CContext, id: TypeId, ty: TypeDef) {
    let header_name = cx.fmt_header_name(id);
    let header_path = format!("{header_name}.h");
    let mut header = Header::new(header_name.into());
    let mut context = TyGenContext::new(cx, &mut header);
    match ty {
        TypeDef::Enum(e) => context.gen_enum_def(id, e),
        TypeDef::Opaque(o) => context.gen_opaque_def(id, o),
        TypeDef::Struct(s) => context.gen_struct_def(id, s),
        TypeDef::OutStruct(s) => context.gen_struct_def(id, s),
    }

    // todo: methods
    cx.files.add_file(header_path, header.to_string());
}

/// Context for generating a particular type's header
pub struct TyGenContext<'cx, 'header> {
    cx: &'cx CContext,
    header: &'header mut Header,
}

impl<'cx, 'header> TyGenContext<'cx, 'header> {
    pub fn new(cx: &'cx CContext, header: &'header mut Header) -> Self {
        TyGenContext { cx, header }
    }

    pub fn gen_enum_def<'tcx>(&mut self, id: TypeId, def: &'tcx hir::EnumDef) {
        let enum_name = self.cx.fmt_type_name(id);
        self.header.body += &format!("typedef enum {enum_name} {{\n");
        for variant in def.variants.iter() {
            let variant_name = self.cx.fmt_enum_variant(variant);
            let discriminant = variant.discriminant;
            self.header.body += &format!("\t{enum_name}_{variant_name} = {discriminant},\n")
        }
        self.header.body += &format!("}} {enum_name};\n");
    }

    pub fn gen_opaque_def<'tcx>(&mut self, id: TypeId, _def: &'tcx hir::OpaqueDef) {
        let opaque_name = self.cx.fmt_type_name(id);
        self.header.body += &format!("typedef struct {opaque_name} {opaque_name};\n");
    }

    pub fn gen_struct_def<'tcx, P: TyPosition>(
        &mut self,
        id: TypeId,
        def: &'tcx hir::StructDef<P>,
    ) {
        let struct_name = self.cx.fmt_type_name(id);
        self.header.body += &format!("typedef struct {struct_name} {{\n");
        for field in def.fields.iter() {
            let decls = self.gen_ty_decl(&field.ty, &field.name, true);
            for (decl_ty, decl_name) in decls {
                self.header.body += &format!("\t{decl_ty} {decl_name};\n")
            }
        }
        self.header.body += &format!("}} {struct_name};\n");
    }

    /// Generates a list of decls for a given type, returned as (type, name)
    ///
    /// Might return multiple in the case of slices and strings. The `is_struct` parameter
    /// affects whether the decls are generated for a struct field or method
    pub fn gen_ty_decl<'a, P: TyPosition>(
        &mut self,
        ty: &Type<P>,
        ident: &'a hir::Ident,
        is_struct: bool,
    ) -> Vec<(Cow<'cx, str>, Cow<'a, str>)> {
        let param_name = self.cx.fmt_param_name(ident);
        match ty {
            Type::Slice(hir::Slice::Str(..)) if !is_struct => {
                vec![
                    ("const char*".into(), format!("{param_name}_data").into()),
                    ("size_t".into(), format!("{param_name}_len").into()),
                ]
            }
            Type::Slice(hir::Slice::Primitive(b, p)) => {
                let constness = if b.mutability.is_immutable() {
                    "const "
                } else {
                    ""
                };
                let prim = self.gen_primitive(*p);
                vec![
                    (
                        format!("{constness}{prim}*").into(),
                        format!("{param_name}_data").into(),
                    ),
                    ("size_t".into(), format!("{param_name}_len").into()),
                ]
            }
            _ => {
                let ty = self.gen_ty_name(ty);
                vec![(ty, param_name)]
            }
        }
    }

    // Generate the C code for referencing a particular type.
    // Handles adding imports and such as necessary
    fn gen_ty_name<P: TyPosition>(&mut self, ty: &Type<P>) -> Cow<'cx, str> {
        match *ty {
            Type::Primitive(prim) => self.gen_primitive(prim),
            Type::Opaque(ref op) => {
                let op_id = op.tcx_id.into();
                let name = self.cx.fmt_type_name(op_id);
                let ret = format!("{name}*");
                // should not be necessary for opaques, might be necessary for import compat
                // let header_name = self.cx.fmt_header_name(op_id);
                // self.header.includes.insert(header_name.into());
                self.header.forwards.insert(name.into());
                ret.into()
            }
            Type::Struct(ref st) => {
                let st_id = P::id_for_path(st);
                let name = self.cx.fmt_type_name(st_id);
                let ret = name.clone();
                let header_name = self.cx.fmt_header_name(st_id);
                self.header.includes.insert(header_name.into());
                self.header.forwards.insert(name.into());
                ret.into()
            }
            Type::Enum(ref e) => {
                let id = e.tcx_id.into();
                let header_name = self.cx.fmt_header_name(id);
                let enum_name = self.cx.fmt_type_name(id);
                self.header.includes.insert(header_name.into());
                enum_name
            }
            Type::Slice(ref s) => match s {
                // only reachable for structs, not methods
                hir::Slice::Str(..) => "DiplomatStringView".into(),
                hir::Slice::Primitive(_, p) => panic!("Attempted to gen_ty_name for slice of {}, should have been handled by gen_ty_decl", p.as_str())
            }
        }
    }

    fn gen_primitive(&self, prim: hir::PrimitiveType) -> Cow<'static, str> {
        use diplomat_core::hir::{FloatType, IntSizeType, IntType, PrimitiveType};
        let s = match prim {
            PrimitiveType::Bool => "bool",
            PrimitiveType::Char => "char32_t",
            PrimitiveType::Int(IntType::I8) => "int8_t",
            PrimitiveType::Int(IntType::U8) => "uint8_t",
            PrimitiveType::Int(IntType::I16) => "int16_t",
            PrimitiveType::Int(IntType::U16) => "uint16_t",
            PrimitiveType::Int(IntType::I32) => "int32_t",
            PrimitiveType::Int(IntType::U32) => "uint32_t",
            PrimitiveType::Int(IntType::I64) => "int64_t",
            PrimitiveType::Int(IntType::U64) => "uint64_t",
            PrimitiveType::Int128(_) => panic!("i128 not supported in C"),
            PrimitiveType::IntSize(IntSizeType::Isize) => "ssize_t",
            PrimitiveType::IntSize(IntSizeType::Usize) => "size_t",
            PrimitiveType::Float(FloatType::F32) => "float",
            PrimitiveType::Float(FloatType::F64) => "double",
        };
        s.into()
    }
}
