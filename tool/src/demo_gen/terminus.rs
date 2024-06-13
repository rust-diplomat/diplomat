use std::borrow::Cow;

use diplomat_core::hir::{Method, SelfType, Type};

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

	pub typescript: bool,
}

impl<'a, 'tcx> RenderTerminusContext<'a, 'tcx> {
    /// Create a Render Terminus .js file from a method.
    /// We define this (for now) as any function that outputs [`hir::SuccessType::Write`]
    pub fn evaluate_method(&self, method : &Method) -> Option<TerminusInfo> {
        if !method.output.success_type().is_write() {
            return None;
        }

        let mut terminus_info = TerminusInfo {
            function_name : self.ctx.formatter.fmt_method_name(method),
            params: Vec::new(),

			// We set this 
            typescript: false,
        };

        // Start with our self variable:
        method.param_self.as_ref().inspect(|s| {
            self.evaluate_self_param(s.ty.clone());
        });

        // if method.param_self.is_some() {
        //     method_info.params.push("self".into());
        // }
        for param in method.params.iter() {
            self.evaluate_param(param.ty.clone());
        }

        Some(terminus_info)
    }

    pub fn evaluate_self_param(&self, self_type : SelfType) {
        match self_type {
            SelfType::Enum(e) => todo!(),
            SelfType::Opaque(o) => todo!(),
            SelfType::Struct(s) => todo!(),
            _ => unreachable!("Unknown HIR type {:?}", self_type),
        }
    }

    pub fn evaluate_param(&self, param_type : Type) {
        match param_type {
            Type::Primitive(p) => todo!(),
            Type::Enum(e) => todo!(),
            Type::Slice(s) => todo!(),
            Type::Opaque(o) => todo!(),
            Type::Struct(s) => todo!(),
            _ => unreachable!("Unknown HIR type {:?}", param_type),
        }
    }
}