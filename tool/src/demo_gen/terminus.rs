use std::collections::BTreeSet;

use diplomat_core::hir::{self, DemoInfo, Method, Type, TypeContext};

use crate::{js::formatter::JSFormatter, ErrorStore};

use askama::{self, Template};

#[derive(Clone)]
pub struct ParamInfo {
    /// Either the name of the parameter (i.e, when a primitive is created as an argument for the render terminus), or the javascript that represents this parameter.
    pub js: String,
    /// The label to give this parameter. Used only in the `RenderInfo` out object. Can be blank if not intended to be used there.
    pub label: String,
    /// For typescript and RenderInfo output. Type that this parameter is.
    pub type_name: String,
}

/// Represents a function that we'll be using when constructing the ultimate output of a RenderTerminus function. See [`TerminusInfo`] for full output.
/// 
/// But this represents one step in the building block, so something like:
/// 
/// ```typescript
/// FixedDecimalFormatter.tryNew.apply(null, [...])
/// ```
/// 
/// Where we expand `...` with further MethodDependencies.
#[derive(Template)]
#[template(path = "demo_gen/method_dependency.js.jinja", escape = "none")]
struct MethodDependency {
    /// Javascript to invoke for this method.
    method_js: String,

    /// Parameters to pass into the method.
    params: Vec<ParamInfo>,
}

pub struct RenderTerminusContext<'ctx, 'tcx> {
    pub tcx: &'tcx TypeContext,
    pub formatter: &'ctx JSFormatter<'tcx>,
    pub errors: &'ctx ErrorStore<'tcx, String>,
    pub terminus_info: TerminusInfo,
}

impl MethodDependency {
    pub fn new(method_js: String) -> Self {
        MethodDependency {
            method_js,
            params: Vec::new(),
        }
    }
}

/// A terminus represents a function in the diplomat FFI that is meant to be called by an HTML rendering engine's JS.
/// (per our design doc: https://docs.google.com/document/d/1xRTmK0YtOfuAe7ClN6kqDaHyv5HpdIRIYQW6Zc_KKFU/edit?usp=sharing)
/// 
/// Termini are (as of right now) automagically generated from every valid FFI function that `diplomat-tool demo_gen` can detect.
/// Valid termini functions are determined in [`RenderTerminusContext::is_valid_terminus`]
/// 
/// The template outputs the structure of a JS function that is meant to directly demonstrate how a diplomat FFI function could be used with direct user input.
/// 
/// The text output will be something akin to the ends up working is a chain of [`MethodDependency`]s that construct every necessary struct and opaque type, until we reach primitive components that we can require direct user input for.
/// 
/// ## Example
/// 
/// To look at the `example` folder, we have `FixedDecimalFormatter`, which has ```rs format_write(&self, value: &FixedDecimal, write: &mut DiplomatWrite)```. Per [`RenderTerminusContext::is_valid_terminus`], this is a render terminus. So we want to generate a Javascript function that calls `formatWrite` for us.
///
/// So, step by step:
/// 
/// - formatWrite represents our root.
/// - formatWrite requires `FixedDecimal` as a parameter, and so we need to call `FixedDecimal.new()`
/// - We then add ICU4XFixedDecimal.new() as a child of our root, and add it as a parameter to be called by formatWrite.
///
/// The final render looks something like this:
/// ```typescript
/// function formatWrite(locale : Locale, provider : DataProvider, options : FixedDecimalFormatterOptions, v : number) {
///     return function(...args) { let self = args[0]; self.formatWrite(...args.slice(1)); }
///     .apply(
///         null,
///         [
///             FixedDecimalFormatter.tryNew.apply(
///                 null,
///                 [locale,
///                 provider,
///                 options]
///             ),
///             FixedDecimal.new_.apply(
///                 null,
///                 [v]
///             ),
///         ]
///     );
/// }
/// ```
#[derive(Template)]
#[template(path = "demo_gen/terminus.js.jinja", escape = "none")]
pub(super) struct TerminusInfo {
    /// Name of the function for the render engine to call
    pub function_name: String,

    /// Parameters that we require explicit user input from the render engine
    pub out_params: Vec<ParamInfo>,

    /// The type name of the type that this function belongs to.
    pub type_name: String,

    pub js_file_name: String,

    /// Final result of recursively calling [`RenderTerminusContext::evaluate_constructor`] on [`MethodDependency`]
    pub node_call_stack: String,

    /// Are we a typescript file? Set by [`super::WebDemoGenerationContext::init`]
    pub typescript: bool,

    /// List of JS imports that this terminus needs.
    pub imports: BTreeSet<String>,
}

impl<'ctx, 'tcx> RenderTerminusContext<'ctx, 'tcx> {
    /// See [`TerminusInfo`] for more information on termini.
    /// 
    /// Right now, we only check for the existence of `&mut DiplomatWrite` in the function parameters to determine a valid render termini.
    /// That is, if there exists a string/buffer output. (Also called "returning a writeable")
    pub fn is_valid_terminus(method: &Method) -> bool {
        method.output.success_type().is_write()
    }

    /// Create a Render Terminus .js file from a method.
    /// We define this (for now) as any function that outputs [`hir::SuccessType::Write`]
    pub fn evaluate(&mut self, type_name: String, method: &Method) {
        // TODO: I think it would be nice to have a stack of the current namespace a given parameter.
        // For instance, ICU4XFixedDecimalFormatter.formatWrite() needs a constructed ICU4XFixedDecimal, which takes an i32 called v as input.
        // Someone just trying to read the .d.ts file will only see function formatWrite(v: number); which doesn't really help them figure out where that's from or why it's there.

        // Not making this as part of the RenderTerminusContext because we want each evaluation to have a specific node,
        // which I find easier easier to represent as a parameter to each function than something like an updating the current node in the struct.
        let mut root =
            MethodDependency::new(self.get_constructor_js(type_name.to_string(), method));

        // And then we just treat the terminus as a regular constructor method:
        self.terminus_info.node_call_stack = self.evaluate_constructor(method, &mut root);

        let type_n = type_name.clone();
        let format = self
            .formatter
            .fmt_import_statement(&type_n, false, "./js/".into());

        self.terminus_info.imports.insert(format);
    }

    /// Currently unused, plan to hopefully use this in the future for quickly grabbing parameter information.
    fn _get_type_demo_attrs(&self, ty: &Type) -> Option<DemoInfo> {
        ty.id()
            .map(|id| self.tcx.resolve_type(id).attrs().demo_attrs.clone())
    }

    /// Helper function for quickly passing a parameter to both our node and the render terminus.
    fn append_out_param(&mut self, param_name: String, type_name: String, node : &mut MethodDependency, attrs : Option<DemoInfo>) {
        // This only works for enums, since otherwise we break the type into its component parts.
        let label = attrs
        .and_then(|attrs| {
            let label = attrs
                .input_cfg
                .get(&param_name)
                .map(|cfg| cfg.label.clone())
                .unwrap_or_default();

            if label.is_empty() {
                None
            } else {
                Some(label)
            }
        })
        .unwrap_or(heck::AsUpperCamelCase(param_name.clone()).to_string());

        let mut param_info = ParamInfo {
        js: param_name,
        label,
        type_name,
        };

        self.terminus_info.out_params.push(param_info.clone());

        // Grab arguments without having to name them
        param_info.js = format!("terminusArgs[{}]", self.terminus_info.out_params.len() - 1);
        node.params.push(param_info);
    }

    /// Take a parameter passed to a terminus (or a constructor), and either:
    /// 1. Add it to the list of parameters that the terminus function takes for the render engine to call.
    /// 2. Go a step deeper and look at its possible constructors to call evaluate_param on.
    ///
    /// `node` - Represents the current function of the parameter we're evaluating. See [`MethodDependency`] for more on its purpose.
    fn evaluate_param(
        &mut self,
        param_type: &Type,
        param_name: String,
        node: &mut MethodDependency,
        method_attrs: DemoInfo,
    ) {
        let attrs = Some(method_attrs);

        // TODO: I think we need to check for struct and opaque types as to whether or not these have attributes that label them as provided as a parameter.
        match param_type {
            Type::Primitive(_) | Type::Enum(_) | Type::Slice(_) => {
                let type_name = match param_type {
                    Type::Primitive(p) => self.formatter.fmt_primitive_as_ffi(*p).to_string(),
                    Type::Slice(hir::Slice::Str(..)) => self.formatter.fmt_string().to_string(),
                    Type::Slice(hir::Slice::Primitive(_, p)) => {
                        self.formatter.fmt_primitive_list_type(*p).to_string()
                    }
                    Type::Slice(hir::Slice::Strs(..)) => "Array<String>".to_string(),
                    Type::Enum(e) => {
                        let type_name = self.formatter.fmt_type_name(e.tcx_id.into()).to_string();

                        if e.resolve(self.tcx).attrs.disable {
                            self.errors
                                .push_error(format!("Found usage of disabled type {type_name}"))
                        }

                        type_name
                    }
                    _ => unreachable!("Unknown primitive type {:?}", param_type),
                };

                self.append_out_param(param_name, type_name, node, attrs);
            }
            Type::Opaque(o) => {
                let op = o.resolve(self.tcx);
                let type_name = self.formatter.fmt_type_name(o.tcx_id.into());

                let all_attrs = &o.resolve(self.tcx).attrs;
                if all_attrs.disable {
                    self.errors
                        .push_error(format!("Found usage of disabled type {type_name}"))
                }

                if all_attrs.demo_attrs.external {
                    self.append_out_param(param_name, type_name.into(), node, attrs);
                    return;
                }

                // We need to find a constructor that we can call.
                // Piggybacking off of the #[diplomat::attr(constructor)] macro for now as well as test attributes in attrs.rs
                let mut usable_constructor = false;

                for method in op.methods.iter() {
                    if usable_constructor {
                        break;
                    }

                    let method_attrs = &method.attrs.demo_attrs;
                    usable_constructor |= method_attrs.default_constructor;
                    if let Some(diplomat_core::hir::SpecialMethod::Constructor) =
                        method.attrs.special_method
                    {
                        usable_constructor |= true;
                    }

                    if usable_constructor {
                        self.terminus_info
                            .imports
                            .insert(self.formatter.fmt_import_statement(
                                &type_name.clone(),
                                false,
                                "./js/".into(),
                            ));

                        let mut child = MethodDependency::new(
                            self.get_constructor_js(type_name.to_string(), method),
                        );

                        let call = self.evaluate_constructor(method, &mut child);
                        node.params.push(ParamInfo {
                            js: call,
                            label: "".into(),
                            type_name: String::default(),
                        });
                    }
                }
                if !usable_constructor {
                    self.errors.push_error(
                        format!(
                            "You must set a default constructor for the opaque type {}, \
                            as it is required for the function {}. \
                            Try adding #[diplomat::demo(default_constructor)] \
                            above a method that you wish to be the default constructor.\
                            You may also disable the type {0} in the backend: `#[diplomat::attr(demo_gen, disable)]`.", 
                            op.name.as_str(), node.method_js)
                    );
                    node.params.push(ParamInfo {
                        js: format!(
                            "null \
                            /*Could not find a usable constructor for {}. \
                            Try adding #[diplomat::demo(default_constructor)]*/",
                            op.name.as_str()
                        ),
                        label: String::default(),
                        type_name: String::default(),
                    });
                }
            }
            Type::Struct(s) => {
                let st = s.resolve(self.tcx);

                let type_name = self.formatter.fmt_type_name(s.tcx_id.into());
                if st.attrs.disable {
                    self.errors
                        .push_error(format!("Found usage of disabled type {type_name}"))
                }

                self.terminus_info
                    .imports
                    .insert(
                        self.formatter
                            .fmt_import_statement(&type_name, false, "./js/".into()),
                    );

                let mut child = MethodDependency::new("".to_string());

                #[derive(Template)]
                #[template(path = "demo_gen/struct.js.jinja", escape = "none")]
                struct StructInfo {
                    fields: Vec<String>,
                    type_name: String,
                }

                let mut fields = Vec::new();

                for field in st.fields.iter() {
                    fields.push(
                        self.formatter
                            .fmt_param_name(field.name.as_str())
                            .to_string(),
                    );

                    self.evaluate_param(
                        &field.ty,
                        field.name.to_string(),
                        &mut child,
                        st.attrs.demo_attrs.clone(),
                    );
                }

                child.method_js = StructInfo {
                    type_name: type_name.to_string(),
                    fields,
                }
                .render()
                .unwrap();

                node.params.push(ParamInfo {
                    type_name: type_name.to_string(),
                    js: child.render().unwrap(),
                    label: "".into(),
                });
            }
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
    fn get_constructor_js(&self, owner_type_name: String, method: &Method) -> String {
        let method_name = self.formatter.fmt_method_name(method);
        if method.param_self.is_some() {
            // We represent as function () instead of () => since closures ignore the `this` args applied to them for whatever reason.

            // TODO: Currently haven't run into other methods that require special syntax to be called in this way, but this might change.
            let is_getter = matches!(
                method.attrs.special_method,
                Some(hir::SpecialMethod::Getter(_))
            );

            format!(
                "(function (...args) {{ return args[0].{method_name}{} }})",
                if !is_getter { "(...args.slice(1))" } else { "" }
            )
        } else {
            format!("{owner_type_name}.{method_name}")
        }
    }

    /// Read a constructor that will be created by our terminus, and add any parameters we might need.
    fn evaluate_constructor(&mut self, method: &Method, node: &mut MethodDependency) -> String {
        let param_self = method.param_self.as_ref();

        if param_self.is_some() {
            let ty = param_self.unwrap().ty.clone().into();
            self.evaluate_param(&ty, "self".into(), node, method.attrs.demo_attrs.clone());
        }

        for param in method.params.iter() {
            self.evaluate_param(
                &param.ty,
                self.formatter.fmt_param_name(param.name.as_str()).into(),
                node,
                method.attrs.demo_attrs.clone(),
            );
        }

        // The node that is awaiting this node as a child needs the rendered output:
        node.render().unwrap()
    }
}
