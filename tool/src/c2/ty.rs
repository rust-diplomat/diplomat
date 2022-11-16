use super::header::Header;
use super::CContext;
use diplomat_core::hir::{self, TypeDef, TypeId};

pub fn gen_ty(cx: &CContext, id: TypeId, ty: TypeDef) {
    let header_name = cx.fmt_header_name(id);
    let header_path = format!("{header_name}.h");
    let mut header = Header::new(header_name.into());
    let mut context = TyGenContext::new(cx, &mut header);
    match ty {
        TypeDef::Enum(e) => context.gen_enum_def(id, e),
        TypeDef::Opaque(o) => context.gen_opaque_def(id, o),
        _ => {
            eprintln!("Todo: handle other kinds of types");
        }
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
}
