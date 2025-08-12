use std::borrow::Cow;

use askama::Template;
use diplomat_core::hir;

use crate::c::{Header, TyGenContext};


#[derive(Template)]
#[template(path = "c/impl.h.jinja", escape = "none")]
pub(super) struct ImplTemplate<'a> {
    pub(super) methods: Vec<MethodTemplate>,
    pub(super) cb_structs_and_defs: Vec<CallbackAndStructDef>,
    pub(super) is_for_cpp: bool,
    pub(super) ty_name: Option<Cow<'a, str>>,
    pub(super) dtor_name: Option<&'a str>,
}


#[derive(Clone)]
pub(super) struct CallbackAndStructDef {
    pub(super) name: String,
    pub(super) params_types: String,
    pub(super) return_type: String,
    pub(super) return_struct: Option<String>,
}

pub(super) struct MethodTemplate {
    return_ty: String,
    params: String,
    abi_name: String,
}

pub struct ImplGenContext<'tcx> {
    pub header : Header,
    template : ImplTemplate<'tcx>,
}

impl<'tcx> ImplGenContext<'tcx> {
    pub(crate) fn new(header : Header, is_for_cpp : bool) -> Self {
        ImplGenContext { header,
            template: ImplTemplate { 
                methods: Vec::new(), cb_structs_and_defs: Vec::new(), 
                is_for_cpp, ty_name: None, dtor_name: None } }
    }

    pub(crate) fn render(&mut self, ty_name : Option<Cow<'tcx, str>>, dtor_name : Option<&'tcx str>) -> Result<(), askama::Error> {
        self.template.ty_name = ty_name;
        self.template.dtor_name = dtor_name;

        self.template.render_into(&mut self.header)?;
        Ok(())
    }

    pub (crate) fn render_into(&mut self, ty_name : Option<Cow<'tcx, str>>, dtor_name : Option<&'tcx str>, header : &mut Header) -> Result<(), askama::Error> {
        self.template.ty_name = ty_name;
        self.template.dtor_name = dtor_name;

        self.template.render_into(header)?;
        Ok(())
    }

    pub(crate) fn gen_method(
        &mut self,
        method: &'tcx hir::Method,
        context : &TyGenContext,
    ) {
        use diplomat_core::hir::{ReturnType, SuccessType};
        let abi_name = method.abi_name.to_string();
        // Right now these are the same, but we may eventually support renaming
        // and should be sure to use method_name when naming the result type
        let method_name = abi_name.clone();
        let mut param_decls = Vec::new();
        let mut cb_structs_and_defs = vec![];
        if let Some(ref self_ty) = method.param_self {
            let self_ty = self_ty.ty.clone().into();
            param_decls.push(context.gen_ty_decl(
                &self_ty,
                "self",
                &mut self.header,
                Some(abi_name.clone()),
                &mut cb_structs_and_defs,
            ))
        }

        for param in &method.params {
            param_decls.push(context.gen_ty_decl(
                &param.ty,
                param.name.as_str(),
                &mut self.header,
                Some(abi_name.clone()),
                &mut cb_structs_and_defs,
            ));
        }

        let return_ty: Cow<str> = match method.output {
            ReturnType::Infallible(SuccessType::Unit) => "void".into(),
            ReturnType::Infallible(SuccessType::Write) => {
                param_decls.push((
                    format!("{}*", context.formatter.fmt_write_name()).into(),
                    "write".into(),
                ));
                "void".into()
            }
            ReturnType::Infallible(SuccessType::OutType(ref o)) => context.gen_ty_name(o, &mut self.header),
            ReturnType::Fallible(ref ok, _) | ReturnType::Nullable(ref ok) => {
                // Result<T, ()> and Option<T> are the same on the ABI
                let err = if let ReturnType::Fallible(_, Some(ref e)) = method.output {
                    Some(e)
                } else {
                    None
                };
                let ok_ty = match ok {
                    SuccessType::Write => {
                        param_decls.push((
                            format!("{}*", context.formatter.fmt_write_name()).into(),
                            "write".into(),
                        ));
                        None
                    }
                    SuccessType::Unit => None,
                    SuccessType::OutType(o) => Some(o),
                    _ => unreachable!("unknown AST/HIR variant"),
                };
                context.gen_result_ty(&method_name, ok_ty, err, &mut self.header).into()
            }
            _ => unreachable!("unknown AST/HIR variant"),
        };

        use itertools::Itertools;
        let params = if !param_decls.is_empty() {
            param_decls
                .into_iter()
                .map(|(ty, name)| {
                    format!("{ty} {name}", name = context.formatter.fmt_identifier(name))
                })
                .join(", ")
        } else {
            "void".to_owned()
        };

        
        self.template.methods.push(MethodTemplate { abi_name, return_ty: return_ty.to_string(), params });
        self.template.cb_structs_and_defs.extend_from_slice(&cb_structs_and_defs);
    }
}