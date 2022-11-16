use super::header::Header;
use diplomat_core::hir::{self, TypeDef, TypeId};

impl super::CContext {
    pub fn gen_ty(&self, id: TypeId, ty: TypeDef) {
        let header_name = self.fmt_header_name(id);
        let header_path = format!("{header_name}.h");
        let mut header = Header::new(header_name.into());
        match ty {
            TypeDef::Enum(e) => self.gen_enum_def(id, e, &mut header),
            _ => {
                eprintln!("Todo: handle other kinds of types");
            }
        }

        // todo: methods
        self.files.add_file(header_path, header.to_string());
    }

    pub fn gen_enum_def<'tcx>(&self, id: TypeId, def: &'tcx hir::EnumDef, header: &mut Header) {
        let enum_name = self.fmt_type_name(id);
        header.body += &format!("typedef enum {enum_name} {{\n");
        for variant in def.variants.iter() {
            let variant_name = self.fmt_enum_variant(variant);
            let discriminant = variant.discriminant;
            header.body += &format!("\t{enum_name}_{variant_name} = {discriminant},\n")
        }
        header.body += &format!("}} {enum_name};\n");
    }
}
