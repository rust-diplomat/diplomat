use std::borrow::Cow;

use diplomat_core::hir::{Method, OpaquePath, SelfType, Type};

use crate::js2::FileType;

use super::WebDemoGenerationContext;
use askama::{self, Template};

pub struct RenderTerminusContext<'a, 'tcx> {
	pub ctx: &'a WebDemoGenerationContext<'tcx>,
}


#[derive(Clone, Template)]
#[template(path="demo-gen/method.js.jinja", escape="none")]
pub(super) struct TerminusInfo {
	/// Name of the function for the render engine to call
	function_name : String,
	/// Parameters that we require explicit input from the render engine for
	params: Vec<String>,

	/// Are we a typescript file? Set by [`super::WebDemoGenerationContext::init`]
	pub typescript: bool,
}

impl<'a, 'tcx> RenderTerminusContext<'a, 'tcx> {
    /// Create a Render Terminus .js file from a method.
    /// We define this (for now) as any function that outputs [`hir::SuccessType::Write`]
    pub fn evaluate_terminus(&self, method : &Method) -> Option<TerminusInfo> {
        if !method.output.success_type().is_write() {
            return None;
        }

        let mut terminus_info = TerminusInfo {
            function_name : self.ctx.formatter.fmt_method_name(method),
            params: Vec::new(),

			// We set this in the init function of WebDemoGenerationContext.
            typescript: false,
        };

        // Start with our self variable:
        method.param_self.as_ref().inspect(|s| {
            self.evaluate_param(s.ty.clone().into());
        });

        // if method.param_self.is_some() {
        //     method_info.params.push("self".into());
        // }
        for param in method.params.iter() {
            self.evaluate_param(param.ty.clone());
        }

        Some(terminus_info)
    }

	/// Take a parameter passed to a terminus (or a constructor), and either:
	/// 1. Add it to the list of parameters that the terminus function takes for the render engine to call.
	/// 2. Go a step deeper and look at its possible constructors to call evaluate_param on.
    pub fn evaluate_param(&self, param_type : Type) {
        match param_type {
            Type::Primitive(p) => todo!(),
            Type::Enum(e) => todo!(),
            Type::Slice(s) => todo!(),
            Type::Opaque(o) => {
				// We need to find a constructor that we can call.
			   // TODO: I'm not sure where I could start setting up attributes? So maybe this is a discussion point for later.
				let op = o.resolve(self.ctx.tcx);
				for method in op.methods.iter() {
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
	pub fn evaluate_constructor(&self, method : &Method) {
		for param in method.params.iter() {
			self.evaluate_param(param.ty.clone());
		}
	}
}