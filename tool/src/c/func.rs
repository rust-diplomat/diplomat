use std::borrow::Cow;

use askama::Template;
use diplomat_core::hir;

use crate::c::{Header, ItemGenContext};

#[derive(Template)]
#[template(path = "c/func_block.h.jinja", escape = "none")]
/// Represents a block of functions. Can belong to a method, or just a list of free functions.
pub(super) struct FuncBlockTemplate<'a> {
    pub(super) methods: Vec<MethodTemplate>,
    pub(super) cb_structs_and_defs: Vec<CallbackAndStructDef>,
    pub(super) is_for_cpp: bool,
    pub(super) ty_name: Option<Cow<'a, str>>,
    pub(super) dtor_name: Option<&'a str>,
}

#[derive(Clone)]
/// Information for constructing a C struct required for callback return informatioon.
pub(super) struct CallbackAndStructDef {
    pub(super) name: String,
    pub(super) params_types: String,
    pub(super) return_type: String,
    pub(super) return_struct: Option<String>,
}

/// The C representation of a Rust method.
/// Created for [`FuncBlockTemplate`] to use in generation.
pub(super) struct MethodTemplate {
    return_ty: String,
    params: String,
    abi_name: String,
}

/// Helper for creating and rendering to a [`FuncBlockTemplate`].
/// Used either for creating functions that belong to structs, or for free functions that belong to no structs.
pub struct FuncGenContext<'tcx> {
    pub header: Header,
    template: FuncBlockTemplate<'tcx>,
}

impl<'tcx> FuncGenContext<'tcx> {
    pub(crate) fn new(header: Header, is_for_cpp: bool) -> Self {
        FuncGenContext {
            header,
            template: FuncBlockTemplate {
                methods: Vec::new(),
                cb_structs_and_defs: Vec::new(),
                is_for_cpp,
                ty_name: None,
                dtor_name: None,
            },
        }
    }

    pub(crate) fn render(
        &mut self,
        ty_name: Option<Cow<'tcx, str>>,
        dtor_name: Option<&'tcx str>,
    ) -> Result<(), askama::Error> {
        self.template.ty_name = ty_name;
        self.template.dtor_name = dtor_name;

        self.template.render_into(&mut self.header)?;
        Ok(())
    }

    pub(crate) fn render_into(
        &mut self,
        ty_name: Option<Cow<'tcx, str>>,
        dtor_name: Option<&'tcx str>,
        header: &mut Header,
    ) -> Result<(), askama::Error> {
        self.template.ty_name = ty_name;
        self.template.dtor_name = dtor_name;

        self.template.render_into(header)?;
        Ok(())
    }

    pub(crate) fn gen_method(&mut self, method: &'tcx hir::Method, context: &ItemGenContext) {
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
            ReturnType::Infallible(SuccessType::OutType(ref o)) => {
                context.gen_ty_name(o, &mut self.header)
            }
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
                context
                    .gen_result_ty(&method_name, ok_ty, err, &mut self.header)
                    .into()
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

        self.template.methods.push(MethodTemplate {
            abi_name,
            return_ty: return_ty.to_string(),
            params,
        });
        self.template
            .cb_structs_and_defs
            .extend_from_slice(&cb_structs_and_defs);
    }
}
