use crate::common::{ErrorStore, FileMap};
use askama::Template;
use diplomat_core::ast::DocsUrlGenerator;
use diplomat_core::hir::TypeContext;
use diplomat_core::hir::{
    self, OpaqueOwner, ReturnType, SelfType, SuccessType, TyPosition, Type, TypeDef, TypeId,
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
                TypeDef::Struct(s) => self.gen_struct_def(s, id, &name, true),
                TypeDef::OutStruct(s) => self.gen_struct_def(s, id, &name, false),
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
        }

        ImplTemplate {
            type_name,
            methods: methods.as_slice(),
            destructor,
            docs: self.formatter.fmt_docs(&ty.docs),
        }
        .render()
        .unwrap()
    }

    fn gen_struct_def<P: TyPosition>(
        &mut self,
        ty: &'cx hir::StructDef<P>,
        id: TypeId,
        type_name: &str,
        mutable: bool,
    ) -> String {
        struct FieldInfo<'a> {
            name: Cow<'a, str>,
            annotation: Option<&'static str>,
            ffi_cast_type_name: Cow<'a, str>,
            dart_type_name: Cow<'a, str>,
            get_expression: Cow<'a, str>,
            set_expressions: Vec<String>,
        }

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

                let get_expression =
                    self.gen_c_to_dart_for_type(&field.ty, format!("_underlying.{name}").into());

                let set_expressions = if !mutable {
                    vec![]
                } else if let hir::Type::Slice(..) = &field.ty {
                    let view_expr = self.gen_dart_to_c_for_type(&field.ty, name.clone());
                    vec![
                        // Free the existing slice, which is owned by us. After construction, the slice
                        // is (null, 0), which is valid to free.
                        format!("ffi2.calloc.free(_underlying.{name}._pointer);"),
                        format!("final {name}View = {view_expr};"),
                        format!("_underlying.{name}._pointer = {name}View.pointer(ffi2.calloc);"),
                        format!("_underlying.{name}._length = {name}View.length;"),
                    ]
                } else {
                    vec![format!(
                        "_underlying.{name} = {};",
                        self.gen_dart_to_c_for_type(&field.ty, name.clone())
                    )]
                };

                FieldInfo {
                    name,
                    annotation,
                    ffi_cast_type_name,
                    dart_type_name,
                    get_expression,
                    set_expressions,
                }
            })
            .collect::<Vec<_>>();

        let methods = ty
            .methods
            .iter()
            .flat_map(|method| self.gen_method_info(id, method, type_name))
            .collect::<Vec<_>>();

        #[derive(Template)]
        #[template(path = "dart/struct.dart.jinja", escape = "none")]
        struct ImplTemplate<'a> {
            type_name: &'a str,
            needs_constructor: bool,
            mutable: bool,
            fields: Vec<FieldInfo<'a>>,
            methods: Vec<MethodInfo<'a>>,
            docs: String,
        }

        ImplTemplate {
            type_name,
            needs_constructor: !methods
                .iter()
                .any(|m| m.declaration.contains(&format!("{type_name}()"))),
            mutable,
            fields,
            methods,
            docs: self.formatter.fmt_docs(&ty.docs),
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

        let _guard = self.errors.set_context_method(
            self.formatter.fmt_type_name_diagnostics(id),
            method.name.as_str().into(),
        );

        let c_method_name = self.formatter.fmt_c_method_name(id, method);

        let mut param_decls_dart = Vec::new();
        let mut param_types_ffi = Vec::new();
        let mut param_types_ffi_cast = Vec::new();
        let mut param_conversions = Vec::new();

        if let Some(param_self) = method.param_self.as_ref() {
            param_types_ffi.push(self.gen_self_type_name_ffi(&param_self.ty, false));
            param_types_ffi_cast.push(self.gen_self_type_name_ffi(&param_self.ty, true));
            param_conversions.push(self.gen_dart_to_c_self(&param_self.ty));
        }

        let mut slice_conversions = Vec::new();
        let mut needs_arena = false;

        for param in method.params.iter() {
            let var_name = self.formatter.fmt_param_name(param.name.as_str());

            param_decls_dart.push(format!("{} {var_name}", self.gen_type_name(&param.ty)));

            let param_type_ffi = self.gen_type_name_ffi(&param.ty, false);
            let param_type_ffi_cast = self.gen_type_name_ffi(&param.ty, true);

            if let hir::Type::Slice(..) = &param.ty {
                // Two args on the ABI: pointer and size
                param_types_ffi.push(self.formatter.fmt_pointer(&param_type_ffi).into());
                param_types_ffi_cast.push(self.formatter.fmt_pointer(&param_type_ffi_cast).into());

                param_types_ffi.push(self.formatter.fmt_usize(false).into());
                param_types_ffi_cast.push(self.formatter.fmt_usize(true).into());

                let view_expr = self.gen_dart_to_c_for_type(&param.ty, var_name.clone());

                slice_conversions.push(format!("final {var_name}View = {view_expr};"));
                needs_arena = true;
                param_conversions.push(format!("{var_name}View.pointer(temp)").into());
                param_conversions.push(format!("{var_name}View.length").into());
            } else {
                param_types_ffi.push(param_type_ffi);
                param_types_ffi_cast.push(param_type_ffi_cast);
                param_conversions.push(self.gen_dart_to_c_for_type(&param.ty, var_name.clone()));
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
            self.helper_classes.insert(
                "writeable".into(),
                include_str!("../../templates/dart/writeable.dart").into(),
            );
        }

        let return_ty = self.gen_return_type_name(&method.output);
        let return_type_ffi = self.gen_return_type_name_ffi(&method.output, false);
        let return_type_ffi_cast = self.gen_return_type_name_ffi(&method.output, true);

        let return_expression = self.gen_c_to_dart_for_return_type(&method.output);

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
            return_type_ffi,
            return_type_ffi_cast,
            slice_conversions,
            needs_arena,
            param_conversions,
            return_expression,
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
                let id = P::id_for_path(st);
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
                let id = P::id_for_path(st);
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
            SelfType::Opaque(..) | SelfType::Struct(..) | SelfType::Enum(..) => {
                "_underlying".into()
            }
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
                "{dart_name} == null ? ffi.Pointer.fromAddress(0) : {dart_name}._underlying"
            )
            .into(),
            Type::Enum(ref e) if is_contiguous_enum(e.resolve(self.tcx)) => {
                format!("{dart_name}.index").into()
            }
            Type::Opaque(..) | Type::Struct(..) | Type::Enum(..) => {
                format!("{dart_name}._underlying").into()
            }
            Type::Slice(hir::Slice::Str(
                _,
                hir::StringEncoding::UnvalidatedUtf8 | hir::StringEncoding::Utf8,
            )) => format!("{dart_name}.utf8View").into(),
            Type::Slice(hir::Slice::Str(_, hir::StringEncoding::UnvalidatedUtf16)) => {
                format!("{dart_name}.utf16View").into()
            }
            Type::Slice(hir::Slice::Primitive(
                _,
                hir::PrimitiveType::Int(_) | hir::PrimitiveType::Float(_),
            )) => dart_name,
            // usize doesn't have a typed list: https://github.com/dart-lang/sdk/issues/54119
            Type::Slice(hir::Slice::Primitive(
                _,
                hir::PrimitiveType::IntSize(hir::IntSizeType::Usize),
            )) => format!("_SizeListView({dart_name})").into(),
            // Other primitive slices are not supported in Dart
            _ => unreachable!("unknown AST/HIR variant"),
        }
    }

    /// Generates a Dart expression for a type.
    fn gen_c_to_dart_for_type<P: TyPosition>(
        &mut self,
        ty: &Type<P>,
        var_name: Cow<'cx, str>,
    ) -> Cow<'cx, str> {
        match *ty {
            Type::Primitive(..) => var_name,
            Type::Opaque(ref op) => {
                let id = op.tcx_id.into();
                let type_name = self.formatter.fmt_type_name(id);

                match (op.owner.is_owned(), op.is_optional()) {
                    (false, _) => unimplemented!(),
                    (true, false) => format!("{type_name}._({var_name})").into(),
                    (true, true) => {
                        format!("{var_name}.address == 0 ? null : {type_name}._({var_name})").into()
                    }
                }
            }
            Type::Struct(ref st) => {
                let id = P::id_for_path(st);
                let type_name = self.formatter.fmt_type_name(id);
                format!("{type_name}._({var_name})").into()
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
    fn gen_c_to_dart_for_return_type(&mut self, result_ty: &ReturnType) -> Option<Cow<'cx, str>> {
        match *result_ty {
            ReturnType::Infallible(None) => None,
            ReturnType::Infallible(Some(SuccessType::Writeable)) => {
                // Note: the `writeable` variable is initialized in the template
                Some("return writeable.finalize();".into())
            }
            ReturnType::Infallible(Some(SuccessType::OutType(ref out_ty))) => Some(
                format!(
                    "return {};",
                    self.gen_c_to_dart_for_type(out_ty, "result".into())
                )
                .into(),
            ),
            ReturnType::Fallible(ref ok, ref err) => {
                let err_conversion = match err {
                    Some(o) => self.gen_c_to_dart_for_type(o, "result.union.err".into()),
                    None => "VoidError()".into(),
                };
                let err_check =
                    format!("if (!result.isOk) {{\n  throw {err_conversion};\n}}").into();
                let ok_conversion = match ok {
                    // Note: the `writeable` variable is initialized in the template
                    Some(SuccessType::Writeable) => "writeable.finalize()".into(),
                    Some(SuccessType::OutType(o)) => {
                        self.gen_c_to_dart_for_type(o, "result.union.ok".into())
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
    return_type_ffi: Cow<'a, str>,
    return_type_ffi_cast: Cow<'a, str>,

    /// Conversion code for Dart arguments to slice helper structs
    slice_conversions: Vec<String>,
    /// The invocation of the Rust method might need temporary allocations,
    /// for which we use a Dart Arena type.
    needs_arena: bool,

    /// Conversion code for each parameter
    param_conversions: Vec<Cow<'a, str>>,

    /// If the function has a return value, the Dart code for the conversion. Assumes that
    /// the C function return value is saved to a variable named `result` or that the
    /// writeable, if present, is saved to a variable named `writeable`.
    return_expression: Option<Cow<'a, str>>,
}
