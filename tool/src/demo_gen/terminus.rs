use std::collections::BTreeSet;

use diplomat_core::hir::{self, Method, Type};

use super::WebDemoGenerationContext;
use askama::{self, Template};

#[derive(Clone)]
pub struct ParamInfo {
    /// Either the name of the parameter (i.e, when a primitive is created as an argument for the render terminus), or the javascript that represents this parameter.
    pub js : String,
    /// For typescript only.
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
#[derive(Template)]
#[template(path="demo-gen/method_dependency.js.jinja", escape="none")]
struct MethodDependency {
    /// Javascript to invoke for this method.
    method_js: String,

    /// Parameters to pass into the method.
    params : Vec<ParamInfo>,
}

pub struct RenderTerminusContext<'a, 'tcx> {
    pub ctx: &'a WebDemoGenerationContext<'tcx>,
    pub terminus_info : TerminusInfo,
    
}

impl<'a> MethodDependency {
    pub fn new(method_js : String) -> Self {
        MethodDependency {
            method_js,
            params : Vec::new(),
        }
    }
}


#[derive(Template)]
#[template(path="demo-gen/terminus.js.jinja", escape="none")]
pub(super) struct TerminusInfo {
    /// Name of the function for the render engine to call
    pub function_name : String,

    /// Parameters that we require explicit user input from the render engine
    pub params: Vec<ParamInfo>,

    /// The type name of the type that this function belongs to.
    pub type_name : String,

    pub js_file_name : String,

    /// Final result of recursively calling [`RenderTerminusContext::evaluate_constructor`] on [`MethodDependency`]
    node_call_stack : String,

    /// Are we a typescript file? Set by [`super::WebDemoGenerationContext::init`]
    pub typescript: bool,
    
    /// List of JS imports that this terminus needs.
    imports: BTreeSet<String>,
}

impl<'a, 'tcx> RenderTerminusContext<'a, 'tcx> {
    /// Create a Render Terminus .js file from a method.
    /// We define this (for now) as any function that outputs [`hir::SuccessType::Write`]
    pub fn evaluate_terminus(ctx : &'a WebDemoGenerationContext<'tcx>, type_name: String, method : &Method) -> Option<TerminusInfo> {
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

                type_name: type_name.clone(),

                js_file_name : ctx.formatter.fmt_file_name(&type_name, &crate::js2::FileType::Module),

                node_call_stack: String::default(),
    
                // We set this in the init function of WebDemoGenerationContext.
                typescript: false,

                imports: BTreeSet::new(),
            },
        };

        // Not making this as part of the RenderTerminusContext because we want each evaluation to have a specific node,
        // which I find easier easier to represent as a parameter to each function than something like an updating the current node in the struct.
        let mut root = MethodDependency::new(this.get_constructor_js(type_name.to_string(), method));

        // And then we just treat the terminus as a regular constructor method:
        this.terminus_info.node_call_stack = this.evaluate_constructor(method, &mut root);

        this.terminus_info.imports.insert(this.ctx.formatter.fmt_import_statement(&type_name, false, "../".into()));

        Some(this.terminus_info)
    }

    /// Take a parameter passed to a terminus (or a constructor), and either:
    /// 1. Add it to the list of parameters that the terminus function takes for the render engine to call.
    /// 2. Go a step deeper and look at its possible constructors to call evaluate_param on.
    /// 
    /// `node` - Represents the current function of the parameter we're evaluating. See [`MethodDependency`] for more on its purpose.
    fn evaluate_param(&mut self, param_type : &Type, param_name : String, node : &mut MethodDependency) {
        // Helper function for quickly passing a parameter to both our node and the render terminus.
        let out_param = |type_name| {
            let mut param_info = ParamInfo {
                js: param_name,
                type_name
            };

            self.terminus_info.params.push(param_info.clone());

            // Grab arguments without having to name them
            param_info.js = format!("arguments[{}]", self.terminus_info.params.len() - 1);
            node.params.push(param_info);
        };

        // TODO: I think we need to check for struct and opaque types as to whether or not these have attributes that label them as provided as a parameter.
        match param_type {
            Type::Primitive(_) | Type::Enum(_) | Type::Slice(_) => {
                let type_name = match param_type {
                    Type::Primitive(p) => self.ctx.formatter.fmt_primitive_as_ffi(*p, true).to_string(),
                    Type::Slice(hir::Slice::Str(..)) => self.ctx.formatter.fmt_string().to_string(),
                    Type::Slice(hir::Slice::Primitive(_, p)) => self.ctx.formatter.fmt_primitive_list_type(*p).to_string(),
                    Type::Slice(hir::Slice::Strs(..)) => "Array<String>".to_string(),
                    Type::Enum(e) => self.ctx.formatter.fmt_type_name(e.tcx_id.into()).to_string(),
                    _ => unreachable!("Unknown primitive type {:?}", param_type)
                };

                out_param(type_name);
            },
            Type::Opaque(o) => {
                let attrs = &o.resolve(&self.ctx.tcx).attrs.demo_attrs;

                // We need to find a constructor that we can call.
                // Piggybacking off of the #[diplomat::attr(constructor)] macro for now as well as test attributes in attrs.rs
                let op = o.resolve(self.ctx.tcx);
                let type_name = self.ctx.formatter.fmt_type_name(o.tcx_id.into());

                if attrs.external {
                    out_param(type_name.into());
                    return;
                }

                let mut usable_constructor = false;

                for method in op.methods.iter() {
                    if usable_constructor {
                        break;
                    }
                
                
                    let method_attrs = &method.attrs.demo_attrs;
                    usable_constructor |= method_attrs.default_constructor;
                    if let Some(diplomat_core::hir::SpecialMethod::Constructor) = method.attrs.special_method {
                        usable_constructor |= true;
                    }

                    if usable_constructor {
                        self.terminus_info.imports.insert(self.ctx.formatter.fmt_import_statement(&type_name.clone(), false, "../".into()));

                        let mut child = MethodDependency::new(self.get_constructor_js(type_name.to_string(), method));
                        
                        let call = self.evaluate_constructor(method, &mut child);
                        node.params.push(ParamInfo {
                            js: call,
                            type_name: String::default(),
                        });
                    }
                }
                if !usable_constructor {
                    self.ctx.errors.push_error(format!("You must set a default constructor for the opaque type {}, as it is required for the function {}. Try adding #[diplomat::attr(default_constructor)] above a method that you wish to be the default constructor.", op.name.as_str(), node.method_js));
                }
            },
            Type::Struct(s) => {
                let st = s.resolve(&self.ctx.tcx);

                let type_name = self.ctx.formatter.fmt_type_name(s.tcx_id.into());
                
                self.terminus_info.imports.insert(self.ctx.formatter.fmt_import_statement(&type_name, false, "../".into()));
                
                let mut child = MethodDependency::new("".to_string());

                #[derive(Template)]
                #[template(path = "demo-gen/struct.js.jinja", escape = "none")]
                struct StructInfo {
                    fields: Vec<String>,
                    type_name : String,
                };

                let mut fields = Vec::new();

                for field in st.fields.iter() {
                    fields.push(self.ctx.formatter.fmt_param_name(field.name.as_str()).to_string());
                    self.evaluate_param(&field.ty, field.name.to_string(), &mut child);
                }

                child.method_js = StructInfo {
                    type_name: type_name.to_string(),
                    fields,
                }.render().unwrap();

                node.params.push(ParamInfo {
                    type_name: type_name.to_string(), 
                    js: child.render().unwrap(),
                });
            },
            _ => unreachable!("Unknown HIR type {:?}", param_type),
        }
    }

    /// Get the javascript that will be used to evaluate a constructor.
    /// 
    /// Could be something like:
    /// ClassName.new()
    /// or
    /// (...args) => { this.new(); }
    /// 
    /// `owner_type_name` - The type name of the owner for this method.
    /// 
    /// `method` - The method we're trying to call.
    fn get_constructor_js(&self, owner_type_name: String, method : &Method) -> String {
        let method_name = self.ctx.formatter.fmt_method_name(method);
        if method.param_self.is_some() {
            // We represent as function () instead of () => since closures ignore the `this` args applied to them for whatever reason.
            format!("(function (...args) {{ return this.{method_name}(...args) }})", )
        } else {
            format!("{owner_type_name}.{method_name}")
        }
    }

    /// Read a constructor that will be created by our terminus, and add any parameters we might need.
    fn evaluate_constructor(&mut self, method : &Method, node : &mut MethodDependency) -> String {
        method.param_self.as_ref().inspect(|s| {
            self.evaluate_param(&s.ty.clone().into(), "self".into(), node);
        }).or_else(|| {
            // Insert null as our self type when we do jsFunction.call(self, arg1, arg2, ...);
            node.params.push(ParamInfo {
                js: self.ctx.formatter.fmt_null().into(),
                type_name: self.ctx.formatter.fmt_null().into(),
            });
            None
        });
        
        for param in method.params.iter() {
            self.evaluate_param(&param.ty, self.ctx.formatter.fmt_param_name(param.name.as_str()).into(), node);
        }

        // The node that is awaiting this node as a child needs the rendered output:
        return node.render().unwrap();
    }
}