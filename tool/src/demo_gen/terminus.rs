use std::collections::{BTreeSet, HashMap};

use diplomat_core::hir::{
    self, DemoInfo, Method, OpaqueDef, StructDef, StructPath, TyPosition, Type, TypeContext,
};

use crate::{js::formatter::JSFormatter, ErrorStore};

use askama::{self, Template};

#[derive(Clone)]
pub struct ParamInfo {
    /// The javascript that represents this parameter.
    pub js: String,
}

pub struct OutParam {
    /// Param JS representation (i.e., `arg_1`)
    pub param_name: String,
    /// Full string name of the param.
    pub label: String,
    pub default_value: String,
    /// For typescript and RenderInfo output. Type that this parameter is. We get this directly from Rust.
    pub type_name: String,
    /// Also for typescript and RenderInfo output. This is used for types where we might want to know more information, like if it's an enumerator, or a custom type to be set by the default renderer.
    pub type_use: String,
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

    /// The Rust parameter that we're attempting to construct with this method. Currently used by [`OutParam`] for better default parameter names.
    owning_param: Option<String>,
}

pub(super) struct RenderTerminusContext<'ctx, 'tcx> {
    pub tcx: &'tcx TypeContext,
    pub formatter: &'ctx JSFormatter<'tcx>,
    pub errors: &'ctx ErrorStore<'tcx, String>,
    pub terminus_info: TerminusInfo,

    /// To avoid similar parameter names while we're collecting [`OutParam`]s.
    pub out_param_collision: HashMap<String, i32>,

    pub relative_import_path: String,
    pub module_name: String,
}

impl MethodDependency {
    pub fn new(method_js: String, owning_param: Option<String>) -> Self {
        MethodDependency {
            method_js,
            params: Vec::new(),
            owning_param,
        }
    }
}

/// A terminus represents a function in the diplomat FFI that is meant to be called by an HTML rendering engine's JS.
/// (per our design doc: docs/demo_gen.md)
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

    /// Parameters provided to us by the rendering engine.
    ///
    /// These EITHER:
    ///
    /// a. Require user input that the render engine provides to us. These are primitives and slices, like strings or floats.
    ///
    /// b. Are too complicated for us to automagically setup ourselves. These are opaque types tagged with `#[diplomat::demo(external)]`.
    /// The current use case is for say, a singleton or single source of data that must not be repeated. But I'm sure there are other instances
    /// where you don't want us to guess how to construct an opaque, and wish to do it yourself.
    pub out_params: Vec<OutParam>,

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
            MethodDependency::new(self.get_constructor_js(type_name.clone(), method), None);

        // And then we just treat the terminus as a regular constructor method:
        self.terminus_info.node_call_stack = self.evaluate_constructor(method, &mut root);

        let type_n = type_name.clone();
        let format = self.formatter.fmt_import_module(
            &type_n,
            self.module_name.clone(),
            self.relative_import_path.clone(),
        );

        self.terminus_info.imports.insert(format);
    }

    /// Currently unused, plan to hopefully use this in the future for quickly grabbing parameter information.
    fn _get_type_demo_attrs(&self, ty: &Type) -> Option<DemoInfo> {
        ty.id()
            .map(|id| self.tcx.resolve_type(id).attrs().demo_attrs.clone())
    }

    /// Helper function for quickly passing a parameter to both our node and the render terminus.
    /// Appends to [TerminusInfo::out_params]
    fn append_out_param<P: TyPosition<StructPath = StructPath>>(
        &mut self,
        param_name: String,
        type_info: &Type<P>,
        node: &mut MethodDependency,
        attrs: Option<DemoInfo>,
    ) {
        let attrs_default = attrs.unwrap_or_default();
        // This only works for enums, since otherwise we break the type into its component parts.
        let label = if attrs_default.input_cfg.label.is_empty() {
            let owning_str = node
                .owning_param
                .as_ref()
                .map(|p| format!("{}:", heck::AsUpperCamelCase(p)))
                .unwrap_or_default();
            format!(
                "{}{}",
                owning_str,
                heck::AsUpperCamelCase(param_name.clone())
            )
            .to_string()
        } else {
            attrs_default.input_cfg.label
        };

        let default_value = attrs_default.input_cfg.default_value;

        let type_name = match type_info {
            Type::Primitive(p) => self.formatter.fmt_primitive_as_ffi(*p).to_string(),
            Type::Enum(e) => self.formatter.fmt_type_name(e.tcx_id.into()).to_string(),
            Type::Slice(hir::Slice::Str(..)) => self.formatter.fmt_string().to_string(),
            Type::Slice(hir::Slice::Primitive(.., p)) => {
                self.formatter.fmt_primitive_list_type(*p).to_string()
            }
            Type::Slice(hir::Slice::Strs(..)) => "Array<string>".to_string(),
            _ => {
                if let Some(i) = type_info.id() {
                    self.formatter.fmt_type_name(i).to_string()
                } else {
                    panic!("Type {type_info:?} not recognized.");
                }
            }
        };

        let type_use = if attrs_default.external {
            "external".into()
        } else {
            match type_info {
                Type::Enum(..) => "enumerator".into(),
                _ => type_name.clone(),
            }
        };

        let (p, n) = if self.out_param_collision.contains_key(&param_name) {
            let n = self.out_param_collision.get(&param_name).unwrap();

            (format!("{param_name}_{n}"), n + 1)
        } else {
            (param_name.clone(), 1)
        };

        self.out_param_collision.insert(param_name, n);

        let out_param = OutParam {
            param_name: p.clone(),
            label,
            type_name: type_name.clone(),
            type_use,
            default_value,
        };

        self.terminus_info.out_params.push(out_param);

        let param_info = ParamInfo { js: p };

        node.params.push(param_info);
    }

    /// Take a parameter passed to a terminus (or a constructor), and either:
    /// 1. Add it to the list of parameters that the terminus function takes for the render engine to call.
    /// 2. Go a step deeper and look at its possible constructors to call evaluate_param on.
    ///
    /// `node` - Represents the current function of the parameter we're evaluating. See [`MethodDependency`] for more on its purpose.
    fn evaluate_param<P: TyPosition<StructPath = StructPath>>(
        &mut self,
        param_type: &Type<P>,
        param_name: String,
        node: &mut MethodDependency,
        param_attrs: DemoInfo,
    ) {
        // TODO: I think we need to check for struct and opaque types as to whether or not these have attributes that label them as provided as a parameter.
        match param_type {
            // Types we can easily coerce into out parameters (i.e., get easy user input from):
            Type::Primitive(..) => {
                self.append_out_param(param_name, param_type, node, Some(param_attrs));
            }
            Type::Enum(e) => {
                let type_name = self.formatter.fmt_type_name(e.tcx_id.into()).to_string();

                if e.resolve(self.tcx).attrs.disable {
                    self.errors
                        .push_error(format!("Found usage of disabled type {type_name}"))
                }

                self.append_out_param(param_name, param_type, node, Some(param_attrs));
            }
            Type::Slice(..) => {
                self.append_out_param(param_name, param_type, node, Some(param_attrs));
            }
            // Types we can't easily coerce into out parameters:
            Type::Opaque(o) => {
                let op = o.resolve(self.tcx);
                let type_name = self.formatter.fmt_type_name(o.tcx_id.into());

                let all_attrs = &o.resolve(self.tcx).attrs;
                if all_attrs.disable {
                    self.errors
                        .push_error(format!("Found usage of disabled type {type_name}"))
                }

                if all_attrs.demo_attrs.external {
                    self.append_out_param(param_name, param_type, node, Some(param_attrs));
                    return;
                }

                self.evaluate_op_constructors(op, type_name.to_string(), param_name, node);
            }
            Type::Struct(s) => {
                let st = s.resolve(self.tcx);

                let type_name = self.formatter.fmt_type_name(s.tcx_id.into());
                if st.attrs.disable {
                    self.errors
                        .push_error(format!("Found usage of disabled type {type_name}"))
                }

                self.evaluate_struct_fields(st, type_name.to_string(), param_name, node);
            }
            Type::DiplomatOption(ref inner) => {
                self.evaluate_param(inner, param_name, node, param_attrs)
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

    /// Find an opaque constructor that suits our purposes (see the `usable_constructor` variable), then evaluate it with [`RenderTerminusContext::evaluate_constructor`].
    fn evaluate_op_constructors(
        &mut self,
        op: &OpaqueDef,
        type_name: String,
        param_name: String,
        node: &mut MethodDependency,
    ) {
        let mut usable_constructor = false;

        for method in op.methods.iter() {
            let method_attrs = &method.attrs.demo_attrs;

            usable_constructor = method_attrs.default_constructor;

            // Piggybacking off of the #[diplomat::attr(constructor)] macro for now as well as test attributes in attrs.rs
            if let Some(diplomat_core::hir::SpecialMethod::Constructor) =
                method.attrs.special_method
            {
                usable_constructor |= true;
            }

            if usable_constructor {
                self.terminus_info
                    .imports
                    .insert(self.formatter.fmt_import_module(
                        &type_name.clone(),
                        self.module_name.clone(),
                        self.relative_import_path.clone(),
                    ));

                let mut child = MethodDependency::new(
                    self.get_constructor_js(type_name.to_string(), method),
                    Some(param_name),
                );

                let call = self.evaluate_constructor(method, &mut child);
                node.params.push(ParamInfo { js: call });
                break;
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
            });
        }
    }

    /// Search through each field in the struct, and find constructors for each.
    fn evaluate_struct_fields(
        &mut self,
        st: &StructDef,
        type_name: String,
        param_name: String,
        node: &mut MethodDependency,
    ) {
        self.terminus_info
            .imports
            .insert(self.formatter.fmt_import_module(
                &type_name,
                self.module_name.clone(),
                self.relative_import_path.clone(),
            ));

        let mut child = MethodDependency::new("".to_string(), Some(param_name));

        #[derive(Template)]
        #[template(path = "demo_gen/struct.js.jinja", escape = "none")]
        struct StructInfo {
            type_name: String,
            fields: Vec<String>,
        }

        let mut fields = Vec::new();

        for field in st.fields.iter() {
            self.evaluate_param(
                &field.ty,
                field.name.to_string(),
                &mut child,
                field.attrs.demo_attrs.clone(),
            );
            fields.push(self.formatter.fmt_param_name(field.name.as_ref()).into());
        }

        child.method_js = StructInfo {
            type_name: type_name.clone(),
            fields,
        }
        .render()
        .unwrap();

        node.params.push(ParamInfo {
            js: child.render().unwrap(),
        });
    }

    /// Read a constructor that will be created by our terminus, and add any parameters we might need.
    fn evaluate_constructor(&mut self, method: &Method, node: &mut MethodDependency) -> String {
        let param_self = method.param_self.as_ref();

        if param_self.is_some() {
            let s = param_self.unwrap();

            let ty = s.ty.clone().into();
            self.evaluate_param(&ty, "self".into(), node, s.attrs.demo_attrs.clone());
        }

        for param in method.params.iter() {
            self.evaluate_param(
                &param.ty,
                self.formatter.fmt_param_name(param.name.as_str()).into(),
                node,
                param.attrs.demo_attrs.clone(),
            );
        }

        // The node that is awaiting this node as a child needs the rendered output:
        node.render().unwrap()
    }
}
