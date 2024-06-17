use std::rc::Rc;

use diplomat_core::hir::{self, Method, Type};

use super::{attrs::MarkupOutCFGAttr, WebDemoGenerationContext};
use askama::{self, Template};

#[derive(Clone)]
struct ParamInfo {
    pub name : String,
    pub type_name : String,
}


/// Represents a function that we'll be using when constructing the ultimate output of a RenderTerminus function.
/// So because formatWrite is a render terminus, we'll need to actuall call the real formatWrite() in the body of the function.
/// 
/// formatWrite represents our root.
/// formatWrite requires ICU4XFixedDecimal as a parameter, and so we need to call ICU4XFixedDecimal.new().
/// We then add ICU4XFixedDecimal.new() as a child of our root, and add it as a parameter to be called by formatWrite.
/// 
/// I think the final render should look something like this:
/// ```typescript
/// function formatWrite(locale : ICU4XLocale, provider : ICU4XDataProvider, options : ICU4XFixedDecimalFormatterOptions, v : number) {
///     return ICU4XFixedDecimalFormatter.formatWrite
///     .call(
///         ICU4XFixedDecimalFormatter.tryNew.call(
///             null,
///             locale,
///             provider,
///             options
///         ), 
///         ICU4XFixedDecimal.new_.call(
///             null,
///             v
///         ),
///     );
/// }
/// ```
/// TODO: Is this even necessary? I think this could be accomplished with the recursive structure that we already have. Just return this with no keeping track of children needed.
#[derive(Template)]
#[template(path="demo-gen/method_dependency.js.jinja", escape="none")]
struct MethodDependency {
    children: Vec<MethodDependency>,

    /// JS name to invoke for this method.
    method_name: String,

    /// Parameters to pass into the method.
    params : Vec<ParamInfo>,
}

pub struct RenderTerminusContext<'a, 'tcx> {
    pub ctx: &'a WebDemoGenerationContext<'tcx>,
    pub terminus_info : TerminusInfo,
    
}

impl<'a> MethodDependency {
    pub fn new(method_name : String) -> Self {
        MethodDependency {
            children: Vec::new(),
            method_name,
            params : Vec::new(),
        }
    }
}


#[derive(Clone, Template)]
#[template(path="demo-gen/method.js.jinja", escape="none")]
pub(super) struct TerminusInfo {
    /// Name of the function for the render engine to call
    function_name : String,
    /// Parameters that we require explicit user input from the render engine
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

        // Not making this as part of the RenderTerminusContext because we want each evaluation to have a specific node,
        // which I find easier easier to represent as a parameter to each function than something like an updating the current node in the struct.
        let mut root = MethodDependency::new(method_name);

        // And then we just treat the terminus as a regular constructor method:
        this.evaluate_constructor(method, &mut root);

        Some(this.terminus_info)
    }

    /// Take a parameter passed to a terminus (or a constructor), and either:
    /// 1. Add it to the list of parameters that the terminus function takes for the render engine to call.
    /// 2. Go a step deeper and look at its possible constructors to call evaluate_param on.
    /// 
    /// `node` - Represents the current function of the parameter we're evaluating. See [`MethodDependency`] for more on its purpose.
    fn evaluate_param(&mut self, param_type : Type, param_name : String, node : &mut MethodDependency) {
        // Helper function for quickly passing a parameter to both our node and the render terminus.
        let out_param = |type_name| {
            let param_info = ParamInfo {
                name: param_name,
                type_name
            };

            self.terminus_info.params.push(param_info.clone());
            node.params.push(param_info);
        };

        // TODO: I think we need to check for struct and opaque types as to whether or not these have attributes that label them as provided as a parameter.
        match param_type {
            Type::Primitive(_) | Type::Enum(_) | Type::Slice(_) => {
                let type_name = match param_type {
                    Type::Primitive(p) => self.ctx.formatter.fmt_primitive_as_ffi(p, true).to_string(),
                    Type::Slice(hir::Slice::Str(..)) => self.ctx.formatter.fmt_string().to_string(),
                    Type::Slice(hir::Slice::Primitive(_, p)) => self.ctx.formatter.fmt_primitive_list_type(p).to_string(),
                    Type::Slice(hir::Slice::Strs(..)) => "Array<String>".to_string(),
                    Type::Enum(e) => self.ctx.formatter.fmt_type_name(e.tcx_id.into()).to_string(),
                    _ => unreachable!("Unknown primitive type {:?}", param_type)
                };

                out_param(type_name);
            },
            Type::Opaque(o) => {
                // We need to find a constructor that we can call.
                // TODO: I'm not sure where I could start setting up attributes? So maybe this is a discussion point for later.
                // Piggybacking off of the #[diplomat::attr(constructor)] macro for now. 
                let op = o.resolve(self.ctx.tcx);

                let mut attrs = op.attrs.demo_attrs.as_ref().unwrap()
                .iter().map(|attr| { MarkupOutCFGAttr::from_demo_attr(attr.clone()) });
                if attrs.any(|attr| {attr == MarkupOutCFGAttr::External}) {
                    let type_name = self.ctx.formatter.fmt_type_name(o.tcx_id.into());
                    out_param(type_name.into());
                    return;
                }

                let mut usable_constructor = false;

                for method in op.methods.iter() {
                    if usable_constructor {
                        break;
                    }

                    let mut attrs = method.attrs.demo_attrs.as_ref().unwrap()
                    .iter().map(|attr| { MarkupOutCFGAttr::from_demo_attr(attr.clone()) });
                
                    usable_constructor |= attrs.any(|attr| { attr == MarkupOutCFGAttr::DefaultConstructor });
                    if let Some(diplomat_core::hir::SpecialMethod::Constructor) = method.attrs.special_method {
                        usable_constructor |= true;
                    }

                    if usable_constructor {
                        let method_name = self.ctx.formatter.fmt_method_name(method);
                        let child = MethodDependency::new(method_name);
                        node.children.push(child);
                        let i = node.children.len() - 1;
                        self.evaluate_constructor(method, &mut node.children.get_mut(i).unwrap());
                    } else {
                        panic!("You must set a default constructor for the opaque type {}, as it is required for the function {}. Try adding #[diplomat::attr(default_constructor)] above a method that you wish to be the default constructor.", op.name.as_str(), node.method_name);
                    }
                }
            },
            Type::Struct(s) => {
                let st = s.resolve(&self.ctx.tcx);

                for ty in st.fields.iter() {
                    // TODO: I think creating a struct purely from its fields needs to be implemented in the JS2 backend first.
                }
            },
            _ => unreachable!("Unknown HIR type {:?}", param_type),
        }
    }

    /// Read a constructor that will be created by our terminus, and add any parameters we might need.
    fn evaluate_constructor(&mut self, method : &Method, node : &mut MethodDependency) {
        method.param_self.as_ref().inspect(|s| {
            self.evaluate_param(s.ty.clone().into(), "self".into(), node);
        }).or_else(|| {
            // Insert null as our self type when we do jsFunction.call(self, arg1, arg2, ...);
            node.params.push(ParamInfo {
                name: self.ctx.formatter.fmt_null().into(),
                type_name: self.ctx.formatter.fmt_null().into(),
            });
            None
        });
        
        for param in method.params.iter() {
            self.evaluate_param(param.ty.clone(), self.ctx.formatter.fmt_param_name(param.name.as_str()).into(), node);
        }

        // Then add our call to the stack:
        self.terminus_info.setup_stack.push(node.render().unwrap());
    }
}