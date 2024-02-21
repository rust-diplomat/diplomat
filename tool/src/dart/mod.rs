use crate::common::{ErrorStore, FileMap};
use askama::Template;
use diplomat_core::ast::DocsUrlGenerator;
use diplomat_core::hir::TypeContext;
use diplomat_core::hir::{
    self, Lifetime, LifetimeEnv, Lifetimes, MaybeStatic, OpaqueOwner, ReturnType, SelfType,
    StructPathLike, SuccessType, TyPosition, Type, TypeDef, TypeId,
};
use formatter::DartFormatter;
use std::borrow::Cow;
use std::collections::{BTreeMap, BTreeSet};
use std::fmt::{Display, Write};

mod formatter;

/// Run file generation
pub fn run<'cx>(
    tcx: &'cx TypeContext,
    docs_url_generator: &'cx DocsUrlGenerator,
    strip_prefix: Option<String>,
) -> Result<FileMap, Vec<(impl Display + 'cx, String)>> {
    let formatter = DartFormatter::new(tcx, docs_url_generator, strip_prefix);

    let files = FileMap::default();
    let errors = ErrorStore::default();

    let mut directives = BTreeSet::default();
    let mut helper_classes = BTreeMap::default();

    for (id, ty) in tcx.all_types() {
        if ty.attrs().disable {
            continue;
        }

        let (file_name, body) = TyGenContext {
            tcx,
            errors: &errors,
            helper_classes: &mut helper_classes,
            formatter: &formatter,
        }
        .gen(id);

        directives.insert(formatter.fmt_part(&file_name));

        files.add_file(
            file_name,
            render_class(
                body,
                BTreeSet::from_iter([formatter.fmt_part_of_lib()]),
                Default::default(),
            ),
        );
    }

    directives.insert(formatter.fmt_import(
        "dart:core",
        Some("show int, double, bool, String, Object, override"),
    ));
    directives.insert(formatter.fmt_import("dart:convert", None));
    directives.insert(formatter.fmt_import("dart:math", None));
    directives.insert(formatter.fmt_import("dart:core", Some("as core")));
    directives.insert(formatter.fmt_import("dart:ffi", Some("as ffi")));
    directives
        .insert(formatter.fmt_import("package:ffi/ffi.dart", Some("as ffi2 show Arena, calloc")));
    directives.insert(formatter.fmt_import("dart:typed_data", None));
    files.add_file(
        formatter.fmt_file_name("lib"),
        render_class(
            include_str!("../../templates/dart/init.dart").into(),
            directives,
            helper_classes,
        ),
    );

    let errors = errors.take_all();
    if !errors.is_empty() {
        Err(errors)
    } else {
        Ok(files)
    }
}

fn render_class(
    body: String,
    directives: BTreeSet<Cow<'static, str>>,
    helper_classes: BTreeMap<String, String>,
) -> String {
    #[derive(askama::Template)]
    #[template(path = "dart/base.dart.jinja", escape = "none")]
    struct ClassTemplate {
        directives: BTreeSet<Cow<'static, str>>,
        body: String,
        helper_classes: BTreeMap<String, String>,
    }

    ClassTemplate {
        body,
        directives,
        helper_classes,
    }
    .render()
    .unwrap()
}

struct TyGenContext<'a, 'cx> {
    tcx: &'cx TypeContext,
    formatter: &'a DartFormatter<'cx>,
    errors: &'a ErrorStore<'cx, String>,
    helper_classes: &'a mut BTreeMap<String, String>,
}

impl<'a, 'cx> TyGenContext<'a, 'cx> {
    fn gen(&mut self, id: TypeId) -> (String, String) {
        let ty = self.tcx.resolve_type(id);

        let _guard = self.errors.set_context_ty(ty.name().as_str().into());

        let name = self.formatter.fmt_type_name(id);
        (
            self.formatter.fmt_file_name(&name),
            match ty {
                TypeDef::Enum(e) => self.gen_enum(e, id, &name),
                TypeDef::Opaque(o) => self.gen_opaque_def(o, id, &name),
                TypeDef::Struct(s) => self.gen_struct_def(s, id, false, &name, true),
                TypeDef::OutStruct(s) => self.gen_struct_def(s, id, true, &name, false),
                _ => unreachable!("unknown AST/HIR variant"),
            },
        )
    }

    fn gen_enum(&mut self, ty: &'cx hir::EnumDef, id: TypeId, type_name: &str) -> String {
        let methods = ty
            .methods
            .iter()
            .flat_map(|method| self.gen_method_info(id, method, type_name))
            .collect::<Vec<_>>();

        #[derive(Template)]
        #[template(path = "dart/enum.dart.jinja", escape = "none")]
        struct ImplTemplate<'a> {
            ty: &'a hir::EnumDef,
            fmt: &'a DartFormatter<'a>,
            type_name: &'a str,
            methods: &'a [MethodInfo<'a>],
            docs: String,
            is_contiguous: bool,
        }

        ImplTemplate {
            ty,
            fmt: self.formatter,
            type_name,
            methods: methods.as_slice(),
            docs: self.formatter.fmt_docs(&ty.docs),
            is_contiguous: is_contiguous_enum(ty),
        }
        .render()
        .unwrap()
    }

    fn gen_opaque_def(&mut self, ty: &'cx hir::OpaqueDef, id: TypeId, type_name: &str) -> String {
        let methods = ty
            .methods
            .iter()
            .flat_map(|method| self.gen_method_info(id, method, type_name))
            .collect::<Vec<_>>();

        let destructor = self.formatter.fmt_destructor_name(id);

        #[derive(Template)]
        #[template(path = "dart/opaque.dart.jinja", escape = "none")]
        struct ImplTemplate<'a> {
            type_name: &'a str,
            methods: &'a [MethodInfo<'a>],
            docs: String,
            destructor: String,
            lifetimes: &'a LifetimeEnv,
        }

        ImplTemplate {
            type_name,
            methods: methods.as_slice(),
            destructor,
            docs: self.formatter.fmt_docs(&ty.docs),
            lifetimes: &ty.lifetimes,
        }
        .render()
        .unwrap()
    }

    fn gen_struct_def<P: TyPosition>(
        &mut self,
        ty: &'cx hir::StructDef<P>,
        id: TypeId,
        is_out: bool,
        type_name: &str,
        mutable: bool,
    ) -> String {
        let fields = ty
            .fields
            .iter()
            .map(|field| {
                let name = self.formatter.fmt_param_name(field.name.as_str());

                let annotation = match field.ty {
                    hir::Type::Primitive(p) => Some(self.formatter.fmt_primitive_as_ffi(p, false)),
                    hir::Type::Enum(_) => Some(self.formatter.fmt_enum_as_ffi(false)),
                    _ => None,
                };

                let ffi_cast_type_name = if let hir::Type::Slice(s) = field.ty {
                    self.gen_slice(&s).into()
                } else {
                    self.gen_type_name_ffi(&field.ty, true)
                };

                let dart_type_name = self.gen_type_name(&field.ty);

                let c_to_dart = self.gen_c_to_dart_for_type(
                    &field.ty,
                    format!("underlying.{name}").into(),
                    &ty.lifetimes,
                );

                let dart_to_c = if let hir::Type::Slice(..) = &field.ty {
                    let view_expr = self.gen_dart_to_c_for_type(&field.ty, name.clone());
                    vec![
                        format!("final {name}View = {view_expr};"),
                        format!("pointer.ref.{name}._pointer = {name}View.pointer(temp);"),
                        format!("pointer.ref.{name}._length = {name}View.length;"),
                    ]
                } else {
                    vec![format!(
                        "pointer.ref.{name} = {};",
                        self.gen_dart_to_c_for_type(&field.ty, name.clone())
                    )]
                };

                FieldInfo {
                    name,
                    ty: &field.ty,
                    annotation,
                    ffi_cast_type_name,
                    dart_type_name,
                    c_to_dart,
                    dart_to_c,
                }
            })
            .collect::<Vec<_>>();

        let mut methods = ty
            .methods
            .iter()
            .flat_map(|method| self.gen_method_info(id, method, type_name))
            .collect::<Vec<_>>();

        // Non-out structs need to be constructible in Dart
        let default_constructor = if !is_out {
            if let Some(constructor) = methods
                .iter_mut()
                .find(|m| m.declaration.contains(&format!("{type_name}()")))
            {
                // If there's an existing zero-arg constructor, we repurpose it with optional arguments for all fields
                let args = fields
                    .iter()
                    .map(|field| format!("{}? {}", field.dart_type_name, field.name))
                    .collect::<Vec<_>>();
                constructor.declaration =
                    format!("factory {type_name}({{{args}}})", args = args.join(", "));

                let mut r = String::new();
                writeln!(&mut r, "final dart = {type_name}._(result);").unwrap();
                for field in &fields {
                    let name = &field.name;
                    writeln!(&mut r, "if ({name} != null) {{").unwrap();
                    writeln!(&mut r, "  dart.{name} = {name};").unwrap();
                    writeln!(&mut r, "}}").unwrap();
                }
                write!(&mut r, "return dart;").unwrap();
                constructor.return_expression = Some(r.into());

                None
            } else {
                // Otherwise we create a constructor with required values for all fields.
                let args = fields
                    .iter()
                    .map(|field| format!("required this.{}", field.name))
                    .collect::<Vec<_>>();

                Some(format!("{type_name}({{{args}}});", args = args.join(", ")))
            }
        } else {
            None
        };

        #[derive(Template)]
        #[template(path = "dart/struct.dart.jinja", escape = "none")]
        struct ImplTemplate<'a, P: TyPosition> {
            type_name: &'a str,
            default_constructor: Option<String>,
            mutable: bool,
            fields: Vec<FieldInfo<'a, P>>,
            methods: Vec<MethodInfo<'a>>,
            docs: String,
            lifetimes: &'a LifetimeEnv,
        }

        ImplTemplate {
            type_name,
            default_constructor,
            mutable,
            fields,
            methods,
            docs: self.formatter.fmt_docs(&ty.docs),
            lifetimes: &ty.lifetimes,
        }
        .render()
        .unwrap()
    }

    fn gen_method_info(
        &mut self,
        id: TypeId,
        method: &'cx hir::Method,
        type_name: &str,
    ) -> Option<MethodInfo<'cx>> {
        if method.attrs.disable {
            return None;
        }

        // Lifetime handling code
        //
        // XXXManishearth aside from the actual parameter-edge-creation code this
        // code can *probably* be generalized and made into an HIR utility. The struct
        // stuff makes this complicated

        // Lifetimes actually used in the output
        let used_method_lifetimes = method.output.used_method_lifetimes();
        let mut method_lifetimes_map: BTreeMap<_, _> = used_method_lifetimes
            .iter()
            .map(|lt| {
                (
                    *lt,
                    LifetimeInfo {
                        incoming_edges: Vec::new(),
                        all_longer_lifetimes: method
                            .lifetime_env
                            .all_longer_lifetimes(lt)
                            .collect(),
                    },
                )
            })
            .collect();

        // Add a parameter to the method_lifetimes map for any lifetimes it references.
        //
        // This basically boils down to: For each lifetime that is actually relevant to borrowing in this method, check if that
        // lifetime or lifetimes longer than it are used by this parameter. In other words, check if
        // it is possible for data in the return type with this lifetime to have been borrowed from this parameter.
        // If so, add code that will yield the ownership-relevant parts of this object to incoming_edges for that lifetime.
        let mut add_param_to_map = |ty: &hir::Type, param_name: &str| {
            if used_method_lifetimes.is_empty() {
                return;
            }

            // Structs have special handling: structs are purely Dart-side, so if you borrow
            // from a struct, you really are borrowing from the internal fields.
            if let hir::Type::Struct(s) = ty {
                let def = s.resolve(self.tcx);
                for method_lifetime in method_lifetimes_map.values_mut() {
                    // Note that ty.lifetimes()/s.lifetimes() is lifetimes
                    // in the *use* context, i.e. lifetimes on the Type that reference the
                    // indices of the method's lifetime arrays. Their *order* references
                    // the indices of the underlying struct def. We need to link the two,
                    // since the _fields_for_lifetime_foo() methods are named after
                    // the *def* context lifetime.
                    //
                    // Concretely, if we have struct `Foo<'a, 'b>` and our method
                    // accepts `Foo<'x, 'y>`, we need to output _fields_for_lifetime_a()/b not x/y.
                    let def_lifetimes = def.lifetimes.all_lifetimes();
                    let use_lifetimes = s.lifetimes.lifetimes();
                    assert_eq!(def_lifetimes.len(), use_lifetimes.len(), "lifetimes array found on struct def must match lifetime parameters accepted by struct");
                    // the type lifetimes array
                    for (def_lt, use_lt) in def_lifetimes.zip(use_lifetimes) {
                        if let MaybeStatic::NonStatic(use_lt) = use_lt {
                            if method_lifetime.all_longer_lifetimes.contains(&use_lt) {
                                let edge = format!(
                                    "...{param_name}._fields_for_lifetime_{}()",
                                    def.lifetimes.fmt_lifetime(def_lt)
                                );
                                method_lifetime.incoming_edges.push(edge);
                                // Do *not* break the inner loop here: even if we found *one* matching lifetime
                                // in this struct that may not be all of them, there may be some other fields that are borrowed
                            }
                        }
                    }
                }
            } else {
                for method_lifetime in method_lifetimes_map.values_mut() {
                    for lt in ty.lifetimes() {
                        if let MaybeStatic::NonStatic(lt) = lt {
                            if method_lifetime.all_longer_lifetimes.contains(&lt) {
                                let edge = if let hir::Type::Slice(..) = ty {
                                    // Slices make a temporary view type that needs to be attached
                                    // XXXManishearth: this is the wrong variable. We need to grab on to the arena
                                    // and also ensure it's not destroyed.
                                    format!("{param_name}View")
                                } else {
                                    // Everything else makes a direct edge
                                    param_name.into()
                                };
                                method_lifetime.incoming_edges.push(edge);
                                // Break the inner loop: we've already determined this
                                break;
                            }
                        }
                    }
                }
            }
        };

        let _guard = self.errors.set_context_method(
            self.formatter.fmt_type_name_diagnostics(id),
            method.name.as_str().into(),
        );

        let c_method_name = self.formatter.fmt_c_method_name(id, method);

        let mut param_decls_dart = Vec::new();
        let mut param_types_ffi = Vec::new();
        let mut param_types_ffi_cast = Vec::new();
        let mut param_names_ffi = Vec::new();
        let mut param_conversions = Vec::new();

        let mut needs_arena = false;

        if let Some(param_self) = method.param_self.as_ref() {
            add_param_to_map(&param_self.ty.clone().into(), "this");

            param_types_ffi.push(self.gen_self_type_name_ffi(&param_self.ty, false));
            param_types_ffi_cast.push(self.gen_self_type_name_ffi(&param_self.ty, true));
            param_conversions.push(self.gen_dart_to_c_self(&param_self.ty));
            param_names_ffi.push("self".into());
            if matches!(param_self.ty, hir::SelfType::Struct(..)) {
                needs_arena = true;
            }
        }

        let mut slice_params = Vec::new();

        for param in method.params.iter() {
            let param_name = self.formatter.fmt_param_name(param.name.as_str());
            add_param_to_map(&param.ty, &param_name);

            param_decls_dart.push(format!("{} {param_name}", self.gen_type_name(&param.ty)));

            let param_type_ffi = self.gen_type_name_ffi(&param.ty, false);
            let param_type_ffi_cast = self.gen_type_name_ffi(&param.ty, true);

            if let hir::Type::Slice(..) = &param.ty {
                // Two args on the ABI: pointer and size
                param_types_ffi.push(self.formatter.fmt_pointer(&param_type_ffi).into());
                param_types_ffi_cast.push(self.formatter.fmt_pointer(&param_type_ffi_cast).into());
                param_names_ffi.push(format!("{param_name}Data").into());

                param_types_ffi.push(self.formatter.fmt_usize(false).into());
                param_types_ffi_cast.push(self.formatter.fmt_usize(true).into());
                param_names_ffi.push(format!("{param_name}Length").into());

                let view_expr = self.gen_dart_to_c_for_type(&param.ty, param_name.clone());

                param_conversions.push(format!("{param_name}View.pointer(temp)").into());
                param_conversions.push(format!("{param_name}View.length").into());
                needs_arena = true;
                slice_params.push((param_name, view_expr));
            } else {
                if matches!(param.ty, hir::Type::Struct(..)) {
                    needs_arena = true;
                }
                param_types_ffi.push(param_type_ffi);
                param_types_ffi_cast.push(param_type_ffi_cast);
                param_conversions.push(self.gen_dart_to_c_for_type(&param.ty, param_name.clone()));
                param_names_ffi.push(param_name);
            }
        }

        if method.is_writeable() {
            param_conversions.push("writeable._underlying".into());
            param_types_ffi.push(
                self.formatter
                    .fmt_pointer(self.formatter.fmt_opaque())
                    .into(),
            );
            param_types_ffi_cast.push(
                self.formatter
                    .fmt_pointer(self.formatter.fmt_opaque())
                    .into(),
            );
            param_names_ffi.push("writeable".into());
            self.helper_classes.insert(
                "writeable".into(),
                include_str!("../../templates/dart/writeable.dart").into(),
            );
        }

        let return_ty = self.gen_return_type_name(&method.output);
        let return_type_ffi = self.gen_return_type_name_ffi(&method.output, false);
        let return_type_ffi_cast = self.gen_return_type_name_ffi(&method.output, true);

        let return_expression =
            self.gen_c_to_dart_for_return_type(&method.output, &method.lifetime_env);

        let params = param_decls_dart.join(", ");

        let declaration = if method.param_self.is_none() {
            // Constructor
            if return_ty == type_name {
                if let Some(name) = self.formatter.fmt_constructor_name(method) {
                    format!("factory {type_name}.{name}({params})")
                } else {
                    format!("factory {type_name}({params})")
                }
            // Static field
            } else if params.is_empty()
                && !matches!(
                    method.output,
                    hir::ReturnType::Fallible(..) | hir::ReturnType::Infallible(None)
                )
                && return_ty != "bool"
            {
                let method_name = self
                    .formatter
                    .fmt_constructor_name(method)
                    .unwrap_or("singleton".into());
                format!("static final {return_ty} {method_name} = ()")
            // Static method
            } else {
                let method_name = self.formatter.fmt_method_name(method);
                format!("static {return_ty} {method_name}({params})")
            }
        // Getter
        } else if method.params.is_empty()
            // Returns some value
            && method.output.return_type().is_some()
            // If it returns a bool it has be a `isFoo`, otherwise the bool might be a success flag of a side effect
            && (return_ty != "bool" || method.name.as_str().starts_with("is"))
            // Conversions are not getters according to the style guide
            && !(method.name.as_str().starts_with("to") || method.name.as_str().starts_with("into"))
            // Mutates
            && !method.name.as_str().starts_with("enable")
            // Clone and build are not getters according to the style guide, and next is usually not pure
            && !["clone", "next", "build"].contains(&method.name.as_str())
        {
            let method_name = self.formatter.fmt_method_name(method);
            format!("{return_ty} get {method_name}")
        // Setter
        } else if method.name.as_str().starts_with("set_")
            && method.params.len() == 1
            && method.output.return_type().is_none()
            // The corresponding getter exists, as required by the style guide
            && self
                .tcx
                .resolve_type(id)
                .methods()
                .iter()
                .any(|m| m.name.as_str() == method.name.as_str().strip_prefix("set_").unwrap())
        {
            let method_name = self.formatter.fmt_setter_name(method);
            format!("set {method_name}({params})")
        } else if method.name.as_str() == "to_string"
            && method.output.is_writeable()
            && params.is_empty()
        {
            "@override\n  String toString()".to_string()
        } else if method.name.as_str() == "get"
            && method.output.return_type().is_some()
            && method.params.len() == 1
        {
            format!("{return_ty} operator []({params})")
        } else {
            let method_name = self.formatter.fmt_method_name(method);
            format!("{return_ty} {method_name}({params})")
        };

        let mut docs = self.formatter.fmt_docs(&method.docs);

        if let hir::ReturnType::Fallible(_, e) = &method.output {
            write!(
                &mut docs,
                "\n///\n/// Throws [{}] on failure.",
                &if let Some(e) = e {
                    self.gen_type_name(e)
                } else {
                    "VoidError".into()
                },
            )
            .unwrap();
        }

        Some(MethodInfo {
            method,
            docs,
            declaration,
            c_method_name,
            param_types_ffi,
            param_types_ffi_cast,
            param_names_ffi,
            return_type_ffi,
            return_type_ffi_cast,
            slice_params,
            needs_arena,
            param_conversions,
            return_expression,
            lifetimes: &method.lifetime_env,
            method_lifetimes_map,
        })
    }

    /// Generates a type's Dart type.
    fn gen_type_name<P: TyPosition>(&mut self, ty: &Type<P>) -> Cow<'cx, str> {
        match *ty {
            Type::Primitive(prim) => self.formatter.fmt_primitive_as_ffi(prim, true).into(),
            Type::Opaque(ref op) => {
                let op_id = op.tcx_id.into();
                let type_name = self.formatter.fmt_type_name(op_id);

                if self.tcx.resolve_type(op_id).attrs().disable {
                    self.errors
                        .push_error(format!("Found usage of disabled type {type_name}"))
                }
                let ret = if op.is_optional() {
                    self.formatter.fmt_nullable(&type_name)
                } else {
                    type_name
                };

                ret.into_owned().into()
            }
            Type::Struct(ref st) => {
                let id = st.id();
                let type_name = self.formatter.fmt_type_name(id);
                if self.tcx.resolve_type(id).attrs().disable {
                    self.errors
                        .push_error(format!("Found usage of disabled type {type_name}"))
                }
                type_name
            }
            Type::Enum(ref e) => {
                let id = e.tcx_id.into();
                let type_name = self.formatter.fmt_type_name(id);
                if self.tcx.resolve_type(id).attrs().disable {
                    self.errors
                        .push_error(format!("Found usage of disabled type {type_name}"))
                }
                type_name
            }
            Type::Slice(hir::Slice::Str(..)) => self.formatter.fmt_string().into(),
            Type::Slice(hir::Slice::Primitive(_, p)) => {
                self.formatter.fmt_primitive_list_type(p).into()
            }
            _ => unreachable!("unknown AST/HIR variant"),
        }
    }

    /// Generates a return type's Dart type.
    fn gen_return_type_name(&mut self, result_ty: &ReturnType) -> Cow<'cx, str> {
        match *result_ty {
            ReturnType::Infallible(None) => self.formatter.fmt_void().into(),
            ReturnType::Infallible(Some(ref ty)) => match ty {
                SuccessType::Writeable => self.formatter.fmt_string().into(),
                SuccessType::OutType(o) => self.gen_type_name(o),
                &_ => unreachable!("unknown AST/HIR variant"),
            },
            ReturnType::Fallible(ref ok, _) => match ok {
                Some(SuccessType::Writeable) => self.formatter.fmt_string().into(),
                None => self.formatter.fmt_void().into(),
                Some(SuccessType::OutType(o)) => self.gen_type_name(o),
                &Some(_) => unreachable!("unknown AST/HIR variant"),
            },
        }
    }

    /// Generates a type's Dart FFI type.
    fn gen_type_name_ffi<P: TyPosition>(&mut self, ty: &Type<P>, cast: bool) -> Cow<'cx, str> {
        match *ty {
            Type::Primitive(prim) => self.formatter.fmt_primitive_as_ffi(prim, cast).into(),
            Type::Opaque(ref op) => {
                let op_id = op.tcx_id.into();
                let type_name = self.formatter.fmt_type_name(op_id);
                if self.tcx.resolve_type(op_id).attrs().disable {
                    self.errors
                        .push_error(format!("Found usage of disabled type {type_name}"))
                }
                self.formatter
                    .fmt_pointer(self.formatter.fmt_opaque())
                    .into()
            }
            Type::Struct(ref st) => {
                let id = st.id();
                let type_name = self.formatter.fmt_type_name(id);
                if self.tcx.resolve_type(id).attrs().disable {
                    self.errors
                        .push_error(format!("Found usage of disabled type {type_name}"))
                }
                format!("_{type_name}Ffi").into()
            }
            Type::Enum(ref e) => {
                let id = e.tcx_id.into();
                let type_name = self.formatter.fmt_type_name(id);
                if self.tcx.resolve_type(id).attrs().disable {
                    self.errors
                        .push_error(format!("Found usage of disabled type {type_name}"))
                }
                self.formatter.fmt_enum_as_ffi(cast).into()
            }
            Type::Slice(hir::Slice::Str(
                _,
                hir::StringEncoding::UnvalidatedUtf8 | hir::StringEncoding::Utf8,
            )) => self.formatter.fmt_utf8_primitive().into(),
            Type::Slice(hir::Slice::Str(_, hir::StringEncoding::UnvalidatedUtf16)) => {
                self.formatter.fmt_utf16_primitive().into()
            }
            Type::Slice(hir::Slice::Primitive(_, p)) => {
                self.formatter.fmt_primitive_as_ffi(p, false).into()
            }
            _ => unreachable!("unknown AST/HIR variant"),
        }
    }

    /// Generates the Dart FFI type name of a return type.
    fn gen_return_type_name_ffi(&mut self, result_ty: &ReturnType, cast: bool) -> Cow<'cx, str> {
        match *result_ty {
            ReturnType::Infallible(None) => if cast {
                self.formatter.fmt_void()
            } else {
                self.formatter.fmt_ffi_void()
            }
            .into(),
            ReturnType::Infallible(Some(ref ty)) => match ty {
                SuccessType::Writeable => if cast {
                    self.formatter.fmt_void()
                } else {
                    self.formatter.fmt_ffi_void()
                }
                .into(),
                SuccessType::OutType(o) => {
                    if let hir::OutType::Slice(s) = o {
                        self.gen_slice(s).into()
                    } else {
                        self.gen_type_name_ffi(o, cast)
                    }
                }
                &_ => unreachable!("unknown AST/HIR variant"),
            },
            ReturnType::Fallible(ref ok, ref err) => self
                .gen_result(ok.as_ref().and_then(SuccessType::as_type), err.as_ref())
                .into(),
        }
    }

    /// Generates a self type's Dart FFI type.
    fn gen_self_type_name_ffi(&self, ty: &SelfType, cast: bool) -> Cow<'cx, str> {
        match ty {
            SelfType::Opaque(_) => self
                .formatter
                .fmt_pointer(self.formatter.fmt_opaque())
                .into(),
            SelfType::Struct(s) => format!("_{}Ffi", s.resolve(self.tcx).name.as_str()).into(),
            SelfType::Enum(_) => self.formatter.fmt_enum_as_ffi(cast).into(),
            _ => unreachable!("unknown AST/HIR variant"),
        }
    }

    /// Generates an FFI expression for a self type.
    fn gen_dart_to_c_self(&self, ty: &SelfType) -> Cow<'static, str> {
        match *ty {
            SelfType::Enum(ref e) if is_contiguous_enum(e.resolve(self.tcx)) => "index".into(),
            SelfType::Struct(..) => "_pointer(temp)".into(),
            SelfType::Opaque(..) | SelfType::Enum(..) => "_underlying".into(),
            _ => unreachable!("unknown AST/HIR variant"),
        }
    }

    /// Generates an FFI expression for a type.
    fn gen_dart_to_c_for_type<P: TyPosition>(
        &mut self,
        ty: &Type<P>,
        dart_name: Cow<'cx, str>,
    ) -> Cow<'cx, str> {
        match *ty {
            Type::Primitive(..) => dart_name.clone(),
            Type::Opaque(ref op) if op.is_optional() => format!(
                // Note: {dart_name} == null ? 0 : {dart_name}.underlying
                // will not work for struct fields since Dart can't guarantee the
                // null check will be unchanged between the two accesses.
                "{dart_name}?._underlying ?? ffi.Pointer.fromAddress(0)"
            )
            .into(),
            Type::Enum(ref e) if is_contiguous_enum(e.resolve(self.tcx)) => {
                format!("{dart_name}.index").into()
            }
            Type::Struct(..) => format!("{dart_name}._pointer(temp)").into(),
            Type::Opaque(..) | Type::Enum(..) => format!("{dart_name}._underlying").into(),
            Type::Slice(hir::Slice::Str(
                _,
                hir::StringEncoding::UnvalidatedUtf8 | hir::StringEncoding::Utf8,
            )) => format!("{dart_name}.utf8View").into(),
            Type::Slice(hir::Slice::Str(_, hir::StringEncoding::UnvalidatedUtf16)) => {
                format!("{dart_name}.utf16View").into()
            }
            Type::Slice(hir::Slice::Primitive(_, p)) => format!(
                "{dart_name}{view}",
                view = self.formatter.fmt_primitive_list_view(p)
            )
            .into(),
            _ => unreachable!("unknown AST/HIR variant"),
        }
    }

    /// Generate the name of a single lifetime edge array
    ///
    /// FIXME(Manishearth): this may need to belong in  fmt.rs
    fn gen_single_edge(&self, lifetime: Lifetime, lifetime_env: &LifetimeEnv) -> Cow<'static, str> {
        format!("edge_{}", lifetime_env.fmt_lifetime(lifetime)).into()
    }

    /// Make a list of edge arrays, one for every lifetime in a Lifetimes
    ///
    /// Will generate with a leading `, `, so will look something like `, edge_a, edge_b, ...`
    fn gen_lifetimes_edge_list(&self, lifetimes: &Lifetimes, lifetime_env: &LifetimeEnv) -> String {
        let mut ret = String::new();
        for lt in lifetimes.lifetimes() {
            if let MaybeStatic::NonStatic(lt) = lt {
                // We only generate a single edge in the list per lifetime, despite transitivity
                //
                // This is because we plan to handle transitivity when constructing these edge arrays,
                // e.g. if `'a: 'b`, `edge_a` will already contain the relevant bits from `edge_b`.
                //
                // This lets us do things like not generate edge_b if it's not actually relevant for returning.
                write!(ret, ", {}", self.gen_single_edge(lt, lifetime_env)).unwrap();
            } else {
                write!(ret, ", []").unwrap();
            }
        }
        ret
    }

    /// Generates a Dart expression for a type.
    fn gen_c_to_dart_for_type<P: TyPosition>(
        &mut self,
        ty: &Type<P>,
        var_name: Cow<'cx, str>,
        lifetime_env: &LifetimeEnv,
    ) -> Cow<'cx, str> {
        match *ty {
            Type::Primitive(..) => var_name,
            Type::Opaque(ref op) => {
                let id = op.tcx_id.into();
                let type_name = self.formatter.fmt_type_name(id);

                let (owned, self_edge) = if let Some(lt) = op.owner.lifetime() {
                    if let MaybeStatic::NonStatic(lt) = lt {
                        (false, self.gen_single_edge(lt, lifetime_env))
                    } else {
                        // 'statics are still not owned and should not register finalizers
                        // but they also have no lifetime edges
                        (false, "[]".into())
                    }
                } else {
                    (true, "[]".into())
                };
                let edges = self.gen_lifetimes_edge_list(&op.lifetimes, lifetime_env);
                if op.is_optional() {
                    format!("{var_name}.address == 0 ? null : {type_name}._({var_name}, {owned}, {self_edge}{edges})").into()
                } else {
                    format!("{type_name}._({var_name}, {owned}, {self_edge}{edges})").into()
                }
            }
            Type::Struct(ref st) => {
                let id = st.id();
                let type_name = self.formatter.fmt_type_name(id);
                // TODO (#406) use correct edges here
                let edges = st.lifetimes().lifetimes().map(|_| ", []").collect::<Vec<_>>().join("");

                format!("{type_name}._({var_name}{edges})").into()
            }
            Type::Enum(ref e) if is_contiguous_enum(e.resolve(self.tcx)) => {
                let id = e.tcx_id.into();
                let type_name = self.formatter.fmt_type_name(id);
                format!("{type_name}.values[{var_name}]").into()
            }
            Type::Enum(ref e) => {
                let id = e.tcx_id.into();
                let type_name = self.formatter.fmt_type_name(id);
                format!("{type_name}.values.firstWhere((v) => v._underlying == {var_name})").into()
            }
            // As we only get borrowed slices from the FFI, we always have to copy.
            Type::Slice(hir::Slice::Str(_, hir::StringEncoding::UnvalidatedUtf8 | hir::StringEncoding::Utf8)) =>
                format!("Utf8Decoder().convert({var_name}._pointer.asTypedList({var_name}._length))").into(),
            Type::Slice(hir::Slice::Str(_, hir::StringEncoding::UnvalidatedUtf16)) =>
                format!("core.String.fromCharCodes({var_name}._pointer.asTypedList({var_name}._length))").into(),
            Type::Slice(hir::Slice::Primitive(_, hir::PrimitiveType::IntSize(_))) =>
                format!("core.Iterable.generate({var_name}._length).map((i) => {var_name}._pointer[i]).toList(growable: false)").into(),
            Type::Slice(..) => format!("{var_name}._pointer.asTypedList({var_name}._length)").into(),
            _ => unreachable!("unknown AST/HIR variant"),
        }
    }

    /// Generates a Dart expressions for a return type.
    fn gen_c_to_dart_for_return_type(
        &mut self,
        result_ty: &ReturnType,
        lifetime_env: &LifetimeEnv,
    ) -> Option<Cow<'cx, str>> {
        match *result_ty {
            ReturnType::Infallible(None) => None,
            ReturnType::Infallible(Some(SuccessType::Writeable)) => {
                // Note: the `writeable` variable is initialized in the template
                Some("return writeable.finalize();".into())
            }
            ReturnType::Infallible(Some(SuccessType::OutType(ref out_ty))) => Some(
                format!(
                    "return {};",
                    self.gen_c_to_dart_for_type(out_ty, "result".into(), lifetime_env)
                )
                .into(),
            ),
            ReturnType::Fallible(ref ok, ref err) => {
                let err_conversion = match err {
                    Some(o) => {
                        self.gen_c_to_dart_for_type(o, "result.union.err".into(), lifetime_env)
                    }
                    None => "VoidError()".into(),
                };
                let err_check =
                    format!("if (!result.isOk) {{\n  throw {err_conversion};\n}}").into();
                let ok_conversion = match ok {
                    // Note: the `writeable` variable is initialized in the template
                    Some(SuccessType::Writeable) => "writeable.finalize()".into(),
                    Some(SuccessType::OutType(o)) => {
                        self.gen_c_to_dart_for_type(o, "result.union.ok".into(), lifetime_env)
                    }
                    None => return Some(err_check),
                    &Some(_) => unreachable!("unknown AST/HIR variant"),
                };
                Some(format!("{err_check}\nreturn {ok_conversion};").into())
            }
            ReturnType::Infallible(Some(_)) => unreachable!("unknown AST/HIR variant"),
        }
    }

    /// Generates a Dart helper class for a slice type.
    fn gen_slice(&mut self, slice: &hir::Slice) -> &'static str {
        let slice_ty = match slice {
            hir::Slice::Str(
                _,
                hir::StringEncoding::UnvalidatedUtf8 | hir::StringEncoding::Utf8,
            ) => self.formatter.fmt_utf8_slice_type(),
            hir::Slice::Str(_, hir::StringEncoding::UnvalidatedUtf16) => {
                self.formatter.fmt_utf16_slice_type()
            }
            hir::Slice::Primitive(_, p) => self.formatter.fmt_slice_type(*p),
            _ => unreachable!("unknown AST/HIR variant"),
        };

        let ffi_type = match slice {
            hir::Slice::Str(
                _,
                hir::StringEncoding::UnvalidatedUtf8 | hir::StringEncoding::Utf8,
            ) => self.formatter.fmt_utf8_primitive(),
            hir::Slice::Str(_, hir::StringEncoding::UnvalidatedUtf16) => {
                self.formatter.fmt_utf16_primitive()
            }
            hir::Slice::Primitive(_, p) => self.formatter.fmt_primitive_as_ffi(*p, false),
            _ => unreachable!("unknown AST/HIR variant"),
        };

        #[derive(askama::Template)]
        #[template(path = "dart/slice.dart.jinja", escape = "none")]
        struct SliceTemplate {
            ffi_type: &'static str,
            slice_ty: &'static str,
        }

        self.helper_classes.insert(
            slice_ty.into(),
            SliceTemplate { ffi_type, slice_ty }.render().unwrap(),
        );

        slice_ty
    }

    /// Generates a Dart helper class for a result type.
    fn gen_result(&mut self, ok: Option<&hir::OutType>, err: Option<&hir::OutType>) -> String {
        let name = format!(
            "_Result{}{}",
            &self
                .formatter
                .fmt_type_as_ident(ok.map(|o| self.gen_type_name_ffi(o, false)).as_deref()),
            &self
                .formatter
                .fmt_type_as_ident(err.map(|o| self.gen_type_name_ffi(o, false)).as_deref())
        );

        if self.helper_classes.contains_key(&name) {
            return name;
        }

        let decls = [ok.map(|o| (o, "ok")), err.map(|o| (o, "err"))]
            .into_iter()
            .flatten()
            .map(|(o, field_name)| {
                format!(
                    "{}external {} {field_name};",
                    match o {
                        hir::OutType::Primitive(p) => {
                            format!("@{}()\n", self.formatter.fmt_primitive_as_ffi(*p, false))
                        }
                        hir::OutType::Enum(_) =>
                            format!("@{}()\n", self.formatter.fmt_enum_as_ffi(false)),
                        _ => String::new(),
                    },
                    { self.gen_type_name_ffi(o, true) }
                )
            })
            .collect();

        #[derive(askama::Template)]
        #[template(path = "dart/result.dart.jinja", escape = "none")]
        struct ResultTemplate {
            name: String,
            decls: Vec<String>,
        }

        self.helper_classes.insert(
            name.clone(),
            ResultTemplate {
                name: name.clone(),
                decls,
            }
            .render()
            .unwrap(),
        );

        name
    }
}

fn is_contiguous_enum(ty: &hir::EnumDef) -> bool {
    ty.variants
        .iter()
        .enumerate()
        .all(|(i, v)| i as isize == v.discriminant)
}

/// Everything needed for rendering a method.
struct MethodInfo<'a> {
    /// HIR of the method being rendered
    method: &'a hir::Method,
    /// Docs
    docs: String,
    /// The declaration (everything before the parameter list)
    declaration: String,
    /// The C method name
    c_method_name: Cow<'a, str>,

    // The types for the FFI declaration. The uncast types are the types
    // from the `dart:ffi` package, the cast types are native Dart types.
    param_types_ffi: Vec<Cow<'a, str>>,
    param_types_ffi_cast: Vec<Cow<'a, str>>,
    param_names_ffi: Vec<Cow<'a, str>>,
    return_type_ffi: Cow<'a, str>,
    return_type_ffi_cast: Cow<'a, str>,

    /// All slice parameters, and their conversion code
    slice_params: Vec<(Cow<'a, str>, Cow<'a, str>)>,
    /// The invocation of the Rust method might need temporary allocations,
    /// for which we use a Dart Arena type.
    needs_arena: bool,

    /// Conversion code for each parameter
    param_conversions: Vec<Cow<'a, str>>,

    /// If the function has a return value, the Dart code for the conversion. Assumes that
    /// the C function return value is saved to a variable named `result` or that the
    /// writeable, if present, is saved to a variable named `writeable`.
    return_expression: Option<Cow<'a, str>>,

    lifetimes: &'a LifetimeEnv,
    /// Maps each (used in the output) method lifetime to a list of parameters
    /// it borrows from. The parameter list may contain the parameter name,
    /// an internal slice View that was temporarily constructed, or
    /// a spread of a struct's `_fields_for_lifetime_foo()` method.
    method_lifetimes_map: BTreeMap<Lifetime, LifetimeInfo>,
}

struct LifetimeInfo {
    // Initializers for all inputs to the edge array from
    incoming_edges: Vec<String>,
    // All lifetimes longer than this. When this lifetime is borrowed from, data corresponding to
    // the other lifetimes may also be borrowed from.
    all_longer_lifetimes: BTreeSet<Lifetime>,
}

struct FieldInfo<'a, P: TyPosition> {
    name: Cow<'a, str>,
    ty: &'a Type<P>,
    annotation: Option<&'static str>,
    ffi_cast_type_name: Cow<'a, str>,
    dart_type_name: Cow<'a, str>,
    c_to_dart: Cow<'a, str>,
    dart_to_c: Vec<String>,
}

// Helpers used in templates (Askama has restrictions on Rust syntax)

/// Convert an iterator to btreeset
fn iterator_to_btreeset<T: Ord>(i: impl Iterator<Item = T>) -> BTreeSet<T> {
    i.collect()
}

/// Turn a set of lifetimes into a nice comma separated list
fn display_lifetime_list(env: &LifetimeEnv, set: &BTreeSet<Lifetime>) -> String {
    if set.len() <= 1 {
        String::new()
    } else {
        set.iter()
            .map(|i| env.fmt_lifetime(i))
            .collect::<Vec<_>>()
            .join(", ")
    }
}

/// Iterate over fields, filtering by fields that actually use lifetimes from `lifetimes`
fn iter_fields_with_lifetimes_from_set<'a, P: TyPosition>(
    fields: &'a [FieldInfo<'a, P>],
    lifetimes: &'a BTreeSet<Lifetime>,
) -> impl Iterator<Item = &'a FieldInfo<'a, P>> + 'a {
    /// Does `ty` use any lifetime from `lifetimes`?
    fn does_type_use_lifetime_from_set<P: TyPosition>(
        ty: &Type<P>,
        lifetimes: &BTreeSet<Lifetime>,
    ) -> bool {
        for lt in ty.lifetimes() {
            if let MaybeStatic::NonStatic(lt) = lt {
                if lifetimes.contains(&lt) {
                    return true;
                }
            }
        }
        false
    }

    fields
        .iter()
        .filter(move |f| does_type_use_lifetime_from_set(f.ty, lifetimes))
}
