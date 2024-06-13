use diplomat_core::hir::{Method, Type};

use super::WebDemoGenerationContext;
use askama::{self, Template};

pub struct RenderTerminusContext<'a, 'tcx> {
    pub ctx: &'a WebDemoGenerationContext<'tcx>,
    pub terminus_info : TerminusInfo,
}

#[derive(Clone)]
struct ParamInfo {
    pub name : String,
    pub type_name : String,
}


#[derive(Clone, Template)]
#[template(path="demo-gen/method.js.jinja", escape="none")]
pub(super) struct TerminusInfo {
    /// Name of the function for the render engine to call
    function_name : String,
    /// Parameters that we require explicit input from the render engine for
    params: Vec<ParamInfo>,

    /// Are we a typescript file? Set by [`super::WebDemoGenerationContext::init`]
    pub typescript: bool,
}

impl<'a, 'tcx> RenderTerminusContext<'a, 'tcx> {
    /// Create a Render Terminus .js file from a method.
    /// We define this (for now) as any function that outputs [`hir::SuccessType::Write`]
    pub fn evaluate_terminus(ctx : &'a WebDemoGenerationContext<'tcx>, method : &Method) -> Option<TerminusInfo> {
        if !method.output.success_type().is_write() {
            return None;
        }
        let mut this = RenderTerminusContext {
            ctx,
            terminus_info: TerminusInfo {
                function_name : ctx.formatter.fmt_method_name(method),
                params: Vec::new(),
    
                // We set this in the init function of WebDemoGenerationContext.
                typescript: false,
            },
        };

        // Start with our self variable:
        method.param_self.as_ref().inspect(|s| {
            this.evaluate_param(s.ty.clone().into(), "self".into());
        });

        // if method.param_self.is_some() {
        //     method_info.params.push("self".into());
        // }
        for param in method.params.iter() {
            this.evaluate_param(param.ty.clone(), this.ctx.formatter.fmt_param_name(param.name.as_str()).into());
        }

        Some(this.terminus_info)
    }

    /// Take a parameter passed to a terminus (or a constructor), and either:
    /// 1. Add it to the list of parameters that the terminus function takes for the render engine to call.
    /// 2. Go a step deeper and look at its possible constructors to call evaluate_param on.
    pub fn evaluate_param(&mut self, param_type : Type, param_name : String) {
        match param_type {
            Type::Primitive(p) => {
                let type_name = self.ctx.formatter.fmt_primitive_as_ffi(p, true);
                self.terminus_info.params.push(ParamInfo {
                    name: param_name,
                    type_name: type_name.into()
                });
            },
            Type::Enum(e) => todo!(),
            Type::Slice(s) => todo!(),
            Type::Opaque(o) => {
                // We need to find a constructor that we can call.
                // TODO: I'm not sure where I could start setting up attributes? So maybe this is a discussion point for later.
                let op = o.resolve(self.ctx.tcx);
                for method in op.methods.iter() {
                    println!("{:?} {:?} {:?}", self.ctx.formatter.fmt_type_name(o.tcx_id.into()), method.name, method.attrs.special_method);
                    if let Some(diplomat_core::hir::SpecialMethod::Constructor) = method.attrs.special_method {
                        self.evaluate_constructor(method);
                    }
                }
            },
            Type::Struct(s) => todo!(),
            _ => unreachable!("Unknown HIR type {:?}", param_type),
        }
    }

    /// Read a constructor that will be created by our terminus, and add any parameters we might need.
    pub fn evaluate_constructor(&mut self, method : &Method) {
        for param in method.params.iter() {
            self.evaluate_param(param.ty.clone(), self.ctx.formatter.fmt_param_name(param.name.as_str()).into());
        }
    }
}