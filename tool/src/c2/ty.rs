use super::header::Header;
use super::CContext;
use diplomat_core::hir::{self, OpaqueOwner, TyPosition, Type, TypeDef, TypeId};
use std::borrow::Cow;

pub fn gen_ty(cx: &CContext, id: TypeId, ty: TypeDef) {
    let header_name = cx.fmt_header_name(id);
    let header_path = format!("{header_name}.h");
    let mut header = Header::new(header_name.to_owned().into());
    let ty_name = cx.fmt_type_name(id);
    let mut context = TyGenContext::new(cx, id, ty_name, &mut header);
    match ty {
        TypeDef::Enum(e) => context.gen_enum_def(e),
        TypeDef::Opaque(o) => context.gen_opaque_def(o),
        TypeDef::Struct(s) => context.gen_struct_def(s),
        TypeDef::OutStruct(s) => context.gen_struct_def(s),
    }

    context.header.body += "\n\n\n";

    for method in ty.methods() {
        context.gen_method(method);
    }

    // In some cases like generating decls for `self` parameters,
    // a header will get its own forwards and includes. Instead of
    // trying to avoid pushing them, it's cleaner to just pull them out
    // once done
    context.header.forwards.remove(&*context.ty_name);
    context.header.includes.remove(&*header_name);

    cx.files.add_file(header_path, header.to_string());
}

/// Context for generating a particular type's header
pub struct TyGenContext<'cx, 'tcx, 'header> {
    id: TypeId,
    ty_name: Cow<'tcx, str>,
    cx: &'cx CContext<'tcx>,
    header: &'header mut Header,
}

impl<'cx, 'tcx: 'cx, 'header> TyGenContext<'cx, 'tcx, 'header> {
    pub fn new(
        cx: &'cx CContext<'tcx>,
        id: TypeId,
        ty_name: Cow<'tcx, str>,
        header: &'header mut Header,
    ) -> Self {
        TyGenContext {
            cx,
            id,
            ty_name,
            header,
        }
    }

    pub fn gen_enum_def(&mut self, def: &'tcx hir::EnumDef) {
        let enum_name = &self.ty_name;
        self.header.body += &format!("typedef enum {enum_name} {{\n");
        for variant in def.variants.iter() {
            let variant_name = self.cx.fmt_enum_variant(variant);
            let discriminant = variant.discriminant;
            self.header.body += &format!("\t{enum_name}_{variant_name} = {discriminant},\n")
        }
        self.header.body += &format!("}} {enum_name};\n");
    }

    pub fn gen_opaque_def(&mut self, _def: &'tcx hir::OpaqueDef) {
        let opaque_name = &self.ty_name;
        self.header.body += &format!("typedef struct {opaque_name} {opaque_name};\n");
    }

    pub fn gen_struct_def<P: TyPosition>(&mut self, def: &'tcx hir::StructDef<P>) {
        let struct_name = &self.ty_name;
        self.header.body += &format!("typedef struct {struct_name} {{\n");
        for field in def.fields.iter() {
            let decls = self.gen_ty_decl(&field.ty, field.name.as_str(), true);
            for (decl_ty, decl_name) in decls {
                self.header.body += &format!("\t{decl_ty} {decl_name};\n")
            }
        }
        // reborrow to avoid borrowing across mutation
        let struct_name = &self.ty_name;
        self.header.body += &format!("}} {struct_name};\n");
    }

    pub fn gen_method(&mut self, method: &'tcx hir::Method) {
        use diplomat_core::hir::{ReturnFallability, ReturnType};
        let method_name = self.cx.fmt_method_name(self.id, method);
        let mut param_decls = Vec::new();
        if let Some(ref self_ty) = method.param_self {
            let self_ty = self_ty.ty.clone().into();
            param_decls = self.gen_ty_decl(&self_ty, "self", false);
        }

        for param in &method.params {
            let decls = self.gen_ty_decl(&param.ty, param.name.as_str(), false);
            param_decls.extend(decls);
        }

        let return_ty: Cow<str> = match method.output {
            ReturnFallability::Infallible(None) => "void".into(),
            ReturnFallability::Infallible(Some(ref ty)) => match ty {
                ReturnType::Writeable => {
                    param_decls.push(("DiplomatWriteable*".into(), "writeable".into()));
                    "void".into()
                }
                ReturnType::OutType(o) => self.gen_ty_name(o),
            },
            ReturnFallability::Fallible(ref ok, ref err) => {
                let ok_ty = match ok {
                    Some(ReturnType::Writeable) => {
                        param_decls.push(("DiplomatWriteable*".into(), "writeable".into()));
                        "void".into()
                    }
                    None => "void".into(),
                    Some(ReturnType::OutType(o)) => self.cx.fmt_type_name_uniquely(o),
                };
                let err_ty = match err {
                    Some(o) => self.cx.fmt_type_name_uniquely(o),
                    None => "void".into(),
                };
                // todo push to results set
                format!("diplomat_result_{ok_ty}_{err_ty}").into()
            }
        };

        let mut params = String::new();
        let mut first = true;
        for (decl_ty, decl_name) in param_decls {
            let comma = if first {
                first = false;
                ""
            } else {
                ", "
            };
            params += &format!("{comma}{decl_ty} {decl_name}");
        }

        self.header.body += &format!("{return_ty} {method_name}({params});\n");
    }

    /// Generates a list of decls for a given type, returned as (type, name)
    ///
    /// Might return multiple in the case of slices and strings. The `is_struct` parameter
    /// affects whether the decls are generated for a struct field or method
    pub fn gen_ty_decl<'a, P: TyPosition>(
        &mut self,
        ty: &Type<P>,
        ident: &'a str,
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
                let constness = self.cx.fmt_constness(b.mutability);
                let prim = self.cx.fmt_primitive_as_c(*p);
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
            Type::Primitive(prim) => self.cx.fmt_primitive_as_c(prim),
            Type::Opaque(ref op) => {
                let op_id = op.tcx_id.into();
                let name = self.cx.fmt_type_name(op_id);
                // unwrap_or(mut) since owned pointers need to not be const
                let mutability = op.owner.mutability().unwrap_or(hir::Mutability::Mutable);
                let constness = self.cx.fmt_constness(mutability);
                let ret = format!("{constness}{name}*");
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
}
