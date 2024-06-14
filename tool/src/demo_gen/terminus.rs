use std::rc::Rc;

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

/// Represents a function that we'll be using when constructing the ultimate output of a RenderTerminus function.
/// So because formatWrite is a render terminus, we'll need to actuall call the real formatWrite() in the body of the function.
/// 
/// formatWrite represents our root.
/// formatWrite is based on ICU4XFixedDecimal.new(), so we add that as a child of the root.
struct MethodDependency {
    children: Vec<MethodDependency>,

    method_name: String,
}

impl<'a> MethodDependency {
    pub fn new(method_name : String) -> Self {
        MethodDependency {
            children: Vec::new(),
            method_name
        }
    }
}


#[derive(Clone, Template)]
#[template(path="demo-gen/method.js.jinja", escape="none")]
pub(super) struct TerminusInfo {
    /// Name of the function for the render engine to call
    function_name : String,
    /// Parameters that we require explicit input from the render engine for
    params: Vec<ParamInfo>,

    /// Stack of setup statements for creating the body of the JS function. Created from a [`MethodDependency`] tree.
    setup_stack : Vec<String>,

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

        // TODO: I think it would be nice to have a stack of the current namespace a given parameter.
        // For instance, ICU4XFixedDecimalFormatter.formatWrite() needs a constructed ICU4XFixedDecimal, which takes an i32 called v as input.
        // Someone just trying to read the .d.ts file will only see function formatWrite(v: number); which doesn't really help them figure out where that's from or why it's there.  
        let mut this = RenderTerminusContext {
            ctx,
            terminus_info: TerminusInfo {
                function_name : ctx.formatter.fmt_method_name(method),
                params: Vec::new(),

                setup_stack: Vec::new(),
    
                // We set this in the init function of WebDemoGenerationContext.
                typescript: false,
            },
        };

        let method_name = this.ctx.formatter.fmt_method_name(method);

        // We don't include this as part of the RenderTerminusContext because that's something we want to render later.
        // Easier to make the MethodDependency tree now, then turn it into something that the template can read then clone it all and set it up later. 
        let mut root = MethodDependency::new(method_name);

        // if method.param_self.is_some() {
        //     method_info.params.push("self".into());
        // }

        // And then we just treat the terminus as a regular constructor method:
        this.evaluate_constructor(method, &mut root);

        Some(this.terminus_info)
    }

    /// Take a parameter passed to a terminus (or a constructor), and either:
    /// 1. Add it to the list of parameters that the terminus function takes for the render engine to call.
    /// 2. Go a step deeper and look at its possible constructors to call evaluate_param on.
    /// 
    /// `node` - Represents the 
    fn evaluate_param(&mut self, param_type : Type, param_name : String, node : &mut MethodDependency) {
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
                // Piggybacking off of the #[diplomat::attr(constructor)] macro for now. 
                let op = o.resolve(self.ctx.tcx);
                for method in op.methods.iter() {
                    println!("{:?} {:?} {:?}", self.ctx.formatter.fmt_type_name(o.tcx_id.into()), method.name, method.attrs.special_method);
                    if let Some(diplomat_core::hir::SpecialMethod::Constructor) = method.attrs.special_method {
                        let method_name = self.ctx.formatter.fmt_method_name(method);
                        
                        let child = MethodDependency::new(method_name);
                        node.children.push(child);
                        let i = node.children.len() - 1;
                        self.evaluate_constructor(method, &mut node.children.get_mut(i).unwrap());
                    }
                }
            },
            Type::Struct(s) => todo!(),
            _ => unreachable!("Unknown HIR type {:?}", param_type),
        }
    }

    /// Read a constructor that will be created by our terminus, and add any parameters we might need.
    fn evaluate_constructor(&mut self, method : &Method, node : &mut MethodDependency) {
        method.param_self.as_ref().inspect(|s| {
            self.evaluate_param(s.ty.clone().into(), "self".into(), node);
        });
        
        for param in method.params.iter() {
            self.evaluate_param(param.ty.clone(), self.ctx.formatter.fmt_param_name(param.name.as_str()).into(), node);
        }

        // TODO: Then add our call to the stack:
    }
}