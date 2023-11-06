use super::DartContext;
use super::DartFormatter;
use askama::Template;
use diplomat_core::hir::{
    self, OpaqueOwner, ReturnType, SelfType, SuccessType, TyPosition, Type, TypeDef, TypeId,
};
use heck::ToLowerCamelCase;
use std::borrow::Cow;
use std::collections::BTreeMap;
use std::collections::BTreeSet;

#[derive(PartialEq, Ord, PartialOrd, Clone, Eq, Debug)]
pub struct Class<'tcx> {
    // Only for sorting
    name: Cow<'tcx, str>,
    body: String,
    imports: BTreeSet<Import>,
    helper_classes: BTreeMap<String, String>,
}

impl<'tcx> Class<'tcx> {
    pub fn init() -> Self {
        Self {
            name: Default::default(),
            body: include_str!("../../templates/dart/init.dart").into(),
            imports: [Import {
                path: "dart:ffi".into(),
                suffix: " as ffi".into(),
            }]
            .into_iter()
            .collect(),
            helper_classes: Default::default(),
        }
    }

    pub fn append(mut self, other: Self) -> Self {
        self.body.push_str("\n\n");
        self.body.push_str(&other.body);
        self.imports.extend(other.imports);
        self.helper_classes.extend(other.helper_classes);
        self
    }

    pub fn render(self) -> String {
        #[derive(askama::Template)]
        #[template(path = "dart/base.dart.jinja", escape = "none")]
        struct ClassTemplate {
            imports: BTreeSet<Import>,
            body: String,
            helper_classes: BTreeMap<String, String>,
        }

        let Self {
            body,
            imports,
            helper_classes,
            ..
        } = self;

        ClassTemplate {
            body,
            imports,
            helper_classes,
        }
        .render()
        .unwrap()
    }
}

impl<'tcx> DartContext<'tcx> {
    pub fn gen_ty(&self, id: TypeId) -> Class<'tcx> {
        let ty = self.tcx.resolve_type(id);

        let mut imports = BTreeSet::new();
        let mut helper_classes = BTreeMap::new();
        let _guard = self.errors.set_context_ty(ty.name().as_str().into());

        let mut tgcx = TyGenContext {
            imports: &mut imports,
            helper_classes: &mut helper_classes,
            cx: self,
        };

        let name = self.formatter.fmt_type_name(id);

        let body = match ty {
            TypeDef::Enum(o) => tgcx.gen_enum(o, id, &name),
            TypeDef::Opaque(o) => tgcx.gen_opaque_def(o, id, &name),
            TypeDef::Struct(s) => tgcx.gen_struct_def(s, id, &name),
            TypeDef::OutStruct(s) => tgcx.gen_struct_def(s, id, &name),
            _ => unreachable!("unknown AST/HIR variant"),
        };

        Class {
            name,
            body,
            imports,
            helper_classes,
        }
    }
}

pub struct TyGenContext<'a, 'dartcx, 'tcx> {
    cx: &'dartcx DartContext<'tcx>,
    imports: &'a mut BTreeSet<Import>,
    helper_classes: &'a mut BTreeMap<String, String>,
}

impl<'a, 'dartcx, 'tcx: 'dartcx> TyGenContext<'a, 'dartcx, 'tcx> {
    fn gen_enum(&mut self, ty: &'tcx hir::EnumDef, id: TypeId, type_name: &str) -> String {
        #[derive(Template)]
        #[template(path = "dart/enum.dart.jinja", escape = "none")]
        struct ImplTemplate<'a> {
            ty: &'a hir::EnumDef,
            fmt: &'a DartFormatter<'a>,
            type_name: &'a str,
            methods: &'a [MethodInfo<'a>],
            docs: String,
        }

        let methods = ty
            .methods
            .iter()
            .flat_map(|method| self.gen_method_info(id, method, type_name))
            .collect::<Vec<_>>();

        ImplTemplate {
            ty,
            fmt: &self.cx.formatter,
            type_name,
            methods: methods.as_slice(),
            docs: self.cx.formatter.fmt_docs(&ty.docs),
        }
        .render()
        .unwrap()
    }

    fn gen_opaque_def(&mut self, ty: &'tcx hir::OpaqueDef, id: TypeId, type_name: &str) -> String {
        #[derive(Template)]
        #[template(path = "dart/opaque.dart.jinja", escape = "none")]
        struct ImplTemplate<'a> {
            type_name: &'a str,
            methods: &'a [MethodInfo<'a>],
            docs: String,
            destructor: String,
        }

        self.imports.extend([Import {
            path: "dart:ffi".into(),
            suffix: " as ffi".into(),
        }]);

        let methods = ty
            .methods
            .iter()
            .flat_map(|method| self.gen_method_info(id, method, type_name))
            .collect::<Vec<_>>();

        let destructor = self.cx.formatter.fmt_destructor_name(id);

        ImplTemplate {
            type_name,
            methods: methods.as_slice(),
            destructor,
            docs: self.cx.formatter.fmt_docs(&ty.docs),
        }
        .render()
        .unwrap()
    }

    fn gen_struct_def<P: TyPosition>(
        &mut self,
        ty: &'tcx hir::StructDef<P>,
        id: TypeId,
        type_name: &str,
    ) -> String {
        #[derive(Template)]
        #[template(path = "dart/struct.dart.jinja", escape = "none")]
        struct ImplTemplate<'a> {
            type_name: &'a str,
            fields: &'a [FieldInfo<'a>],
            methods: &'a [MethodInfo<'a>],
            docs: String,
        }

        struct FieldInfo<'a> {
            name: Cow<'a, str>,
            annotation: Option<&'static str>,
            ffi_cast_type_name: Cow<'a, str>,
            dart_type_name: Cow<'a, str>,
            get_expression: Cow<'a, str>,
            set_expression: Cow<'a, str>,
            set_slice_conversions: Vec<Cow<'a, str>>,
        }

        self.imports.insert(Import {
            path: "dart:ffi".into(),
            suffix: " as ffi".into(),
        });

        let fields = ty
            .fields
            .iter()
            .map(|field| {
                let name = self.cx.formatter.fmt_param_name(field.name.as_str());

                let annotation = match field.ty {
                    hir::Type::Primitive(p) => {
                        Some(self.cx.formatter.fmt_primitive_as_ffi(p, false))
                    }
                    hir::Type::Enum(_) => Some(self.cx.formatter.fmt_enum_as_ffi(false)),
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

                let mut set_slice_conversions = Vec::new();

                let set_expression = self.gen_dart_to_c_for_type(
                    &field.ty,
                    name.clone(),
                    &mut set_slice_conversions,
                );

                FieldInfo {
                    name,
                    annotation,
                    ffi_cast_type_name,
                    dart_type_name,
                    get_expression,
                    set_expression,
                    set_slice_conversions,
                }
            })
            .collect::<Vec<_>>();

        let methods = ty
            .methods
            .iter()
            .flat_map(|method| self.gen_method_info(id, method, type_name))
            .collect::<Vec<_>>();

        ImplTemplate {
            type_name,
            fields: &fields,
            methods: methods.as_slice(),
            docs: self.cx.formatter.fmt_docs(&ty.docs),
        }
        .render()
        .unwrap()
    }

    fn gen_method_info(
        &mut self,
        id: TypeId,
        method: &'tcx hir::Method,
        type_name: &str,
    ) -> Option<MethodInfo<'dartcx>> {
        if method.attrs.disable {
            return None;
        }

        let _guard = self.cx.errors.set_context_method(
            self.cx.formatter.fmt_type_name_diagnostics(id),
            method.name.as_str().into(),
        );

        self.imports.insert(Import {
            path: "dart:ffi".into(),
            suffix: " as ffi".into(),
        });

        let c_method_name = self.cx.formatter.fmt_c_method_name(id, method);

        let mut param_decls_dart = Vec::new();
        let mut param_types_ffi = Vec::new();
        let mut param_types_ffi_cast = Vec::new();
        let mut dart_to_ffi_params = Vec::new();

        if let Some(param_self) = method.param_self.as_ref() {
            param_types_ffi.push(self.gen_self_type_ffi(&param_self.ty, false));
            param_types_ffi_cast.push(self.gen_self_type_ffi(&param_self.ty, true));
            dart_to_ffi_params.push(self.gen_dart_to_c_self(&param_self.ty));
        }

        let mut slice_conversions = Vec::new();

        for param in method.params.iter() {
            param_decls_dart.push(self.gen_ty_decl(&param.ty, param.name.as_str()));

            let param_type_ffi = self.gen_type_name_ffi(&param.ty, false);
            let param_type_ffi_cast = self.gen_type_name_ffi(&param.ty, true);
            let conversion = self.gen_dart_to_c_for_type(
                &param.ty,
                self.cx.formatter.fmt_param_name(param.name.as_str()),
                &mut slice_conversions,
            );

            if matches!(param.ty, hir::Type::Slice(..)) {
                param_types_ffi.push(self.cx.formatter.fmt_pointer(&param_type_ffi).into());
                param_types_ffi.push(self.cx.formatter.fmt_usize(false).into());

                param_types_ffi_cast
                    .push(self.cx.formatter.fmt_pointer(&param_type_ffi_cast).into());
                param_types_ffi_cast.push(self.cx.formatter.fmt_usize(true).into());

                dart_to_ffi_params.push(format!("{conversion}._bytes").into());
                dart_to_ffi_params.push(format!("{conversion}._length").into());
            } else {
                param_types_ffi.push(param_type_ffi);
                param_types_ffi_cast.push(param_type_ffi_cast);
                dart_to_ffi_params.push(conversion);
            }
        }

        if method.is_writeable() {
            dart_to_ffi_params.push("writeable._underlying".into());
            param_types_ffi.push(
                self.cx
                    .formatter
                    .fmt_pointer(self.cx.formatter.fmt_opaque())
                    .into(),
            );
            param_types_ffi_cast.push(
                self.cx
                    .formatter
                    .fmt_pointer(self.cx.formatter.fmt_opaque())
                    .into(),
            );
            self.helper_classes.insert(
                "writeable".into(),
                include_str!("../../templates/dart/writeable.dart").into(),
            );
        }

        let ffi_return_ty = self.gen_ffi_return_type_name(&method.output, false);
        let ffi_cast_return_ty = self.gen_ffi_return_type_name(&method.output, true);

        let dart_return_expression: Option<Cow<str>> =
            self.gen_c_to_dart_for_return_type(&method.output, "result".into());

        let params = param_decls_dart
            .iter()
            .map(|param| format!("{} {}", param.type_name, param.var_name))
            .collect::<Vec<_>>()
            .join(", ");

        let return_ty = self.gen_dart_return_type_name(&method.output);
        let method_name = self.cx.formatter.fmt_method_name(method);

        let declaration = if method.param_self.is_none() {
            if return_ty == type_name {
                let mut method_name = &*method_name;
                for prefix in ["create", "new", "default"] {
                    method_name = method_name.strip_prefix(prefix).unwrap_or(method_name);
                }
                if method_name.is_empty() {
                    format!("factory {type_name}({params})")
                } else {
                    format!(
                        "factory {type_name}.{}({params})",
                        method_name.to_lower_camel_case()
                    )
                }
            } else if params.is_empty()
                && (return_ty != "bool" || method_name.starts_with("is"))
                && !matches!(method.output, hir::ReturnType::Fallible(..))
            {
                format!(
                    "static final {return_ty} {method_name} = \
                        _capi<ffi.NativeFunction<{ffi_return_ty} Function()>>('{c_method_name}')\
                        .asFunction<{ffi_cast_return_ty} Function()>(isLeaf: true)();"
                )
            } else if method_name == "new" {
                format!("static {return_ty} new_({params})")
            } else {
                format!("static {return_ty} {method_name}({params})")
            }
        } else if method.params.is_empty()
            // Returns some value
            && method.output.return_type().is_some()
            // If it returns a bool it has be a `isFoo`, otherwise the bool might be a success flag of a side effect
            && (return_ty != "bool" || method_name.starts_with("is"))
            // Conversions are not getters according to the style guide
            && !(method_name.starts_with("to") || method_name.starts_with("into"))
            // Mutates
            && !method_name.starts_with("enable")
            // Clone and build are not getters according to the style guide, and next is usually not pure
            && !["clone", "next", "build"].contains(&&*method_name)
        {
            format!("{return_ty} get {method_name}",)
        } else if method_name.starts_with("set")
            && method.params.len() == 1
            && method.output.return_type().is_none()
            // The corresponding getter exists, as required by the style guide
            && self
                .cx
                .tcx
                .resolve_type(id)
                .methods()
                .iter()
                .any(|m| m.name.as_str() == method.name.as_str().strip_prefix("set_").unwrap())
        {
            format!(
                "set {}({params})",
                method_name
                    .strip_prefix("set")
                    .unwrap()
                    .to_lower_camel_case()
            )
        } else if method_name == "toString" && method.output.is_writeable() && params.is_empty() {
            "@override\n  String toString()".to_string()
        } else {
            format!("{return_ty} {method_name}({params})")
        };

        let docs = self.cx.formatter.fmt_docs(&method.docs);

        Some(MethodInfo {
            method,
            docs,
            declaration,
            method_name,
            c_method_name,
            param_types_ffi,
            param_types_ffi_cast,
            ffi_return_ty,
            ffi_cast_return_ty,
            dart_to_ffi_params,
            dart_return_expression,
            slice_conversions,
        })
    }

    /// Generates Dart code for referencing a particular type with a given name.
    fn gen_ty_decl<'b, P: TyPosition>(&mut self, ty: &Type<P>, var_name: &'b str) -> NamedType<'b>
    where
        'dartcx: 'b,
    {
        let var_name = self.cx.formatter.fmt_param_name(var_name);
        let type_name = self.gen_type_name(ty);

        NamedType {
            var_name,
            type_name,
        }
    }

    /// Generates Dart code for referencing a particular Dart type.
    ///
    /// This function adds the necessary type imports to the decl and impl files.
    fn gen_type_name<P: TyPosition>(&mut self, ty: &Type<P>) -> Cow<'dartcx, str> {
        match *ty {
            Type::Primitive(prim) => self.cx.formatter.fmt_primitive_as_ffi(prim, true).into(),
            Type::Opaque(ref op) => {
                let op_id = op.tcx_id.into();
                let type_name = self.cx.formatter.fmt_type_name(op_id);

                if self.cx.tcx.resolve_type(op_id).attrs().disable {
                    self.cx
                        .errors
                        .push_error(format!("Found usage of disabled type {type_name}"))
                }
                let ret = if op.is_optional() {
                    self.cx.formatter.fmt_nullable(&type_name)
                } else {
                    type_name
                };

                ret.into_owned().into()
            }
            Type::Struct(ref st) => {
                let id = P::id_for_path(st);
                let type_name = self.cx.formatter.fmt_type_name(id);
                if self.cx.tcx.resolve_type(id).attrs().disable {
                    self.cx
                        .errors
                        .push_error(format!("Found usage of disabled type {type_name}"))
                }
                type_name
            }
            Type::Enum(ref e) => {
                let id = e.tcx_id.into();
                let type_name = self.cx.formatter.fmt_type_name(id);
                if self.cx.tcx.resolve_type(id).attrs().disable {
                    self.cx
                        .errors
                        .push_error(format!("Found usage of disabled type {type_name}"))
                }
                type_name
            }
            Type::Slice(hir::Slice::Str(_lifetime)) => self.cx.formatter.fmt_string().into(),
            Type::Slice(hir::Slice::Primitive(_, p)) => {
                self.imports
                    .insert(Import::simple("dart:typed_data".into()));
                self.cx.formatter.fmt_primitive_list_type(p).into()
            }
            _ => unreachable!("unknown AST/HIR variant"),
        }
    }

    /// Generates Dart code for referencing a particular FFI type with a given name.
    fn gen_type_name_ffi<'b, P: TyPosition>(&mut self, ty: &Type<P>, cast: bool) -> Cow<'b, str>
    where
        'dartcx: 'b,
    {
        match *ty {
            Type::Primitive(prim) => self.cx.formatter.fmt_primitive_as_ffi(prim, cast).into(),
            Type::Opaque(ref op) => {
                let op_id = op.tcx_id.into();
                let type_name = self.cx.formatter.fmt_type_name(op_id);
                if self.cx.tcx.resolve_type(op_id).attrs().disable {
                    self.cx
                        .errors
                        .push_error(format!("Found usage of disabled type {type_name}"))
                }
                self.cx
                    .formatter
                    .fmt_pointer(self.cx.formatter.fmt_opaque())
                    .into()
            }
            Type::Struct(ref st) => {
                let id = P::id_for_path(st);
                let type_name = self.cx.formatter.fmt_type_name(id);
                if self.cx.tcx.resolve_type(id).attrs().disable {
                    self.cx
                        .errors
                        .push_error(format!("Found usage of disabled type {type_name}"))
                }
                format!("_{type_name}Ffi").into()
            }
            Type::Enum(ref e) => {
                let id = e.tcx_id.into();
                let type_name = self.cx.formatter.fmt_type_name(id);
                if self.cx.tcx.resolve_type(id).attrs().disable {
                    self.cx
                        .errors
                        .push_error(format!("Found usage of disabled type {type_name}"))
                }
                self.cx.formatter.fmt_enum_as_ffi(cast).into()
            }
            Type::Slice(hir::Slice::Str(_lifetime)) => {
                self.cx.formatter.fmt_utf8_primitive().into()
            }
            Type::Slice(hir::Slice::Primitive(_, p)) => {
                self.cx.formatter.fmt_primitive_as_ffi(p, false).into()
            }
            _ => unreachable!("unknown AST/HIR variant"),
        }
    }

    fn gen_self_type_ffi(&self, ty: &SelfType, cast: bool) -> Cow<'tcx, str> {
        match ty {
            SelfType::Opaque(_) => self
                .cx
                .formatter
                .fmt_pointer(self.cx.formatter.fmt_opaque())
                .into(),
            SelfType::Struct(s) => format!("_{}Ffi", s.resolve(self.cx.tcx).name.as_str()).into(),
            SelfType::Enum(_) => self.cx.formatter.fmt_enum_as_ffi(cast).into(),
            _ => unreachable!("unknown AST/HIR variant"),
        }
    }

    /// Generates Dart code for referencing a particular FFI type.
    ///
    /// This function adds the necessary type imports to the decl and impl files.

    /// Generates a C++ expression that converts from the C++ self type to the corresponding C self type.
    fn gen_dart_to_c_self(&self, ty: &SelfType) -> Cow<'static, str> {
        match *ty {
            SelfType::Opaque(..) => "_underlying".into(),
            SelfType::Struct(..) => "_underlying".into(),
            SelfType::Enum(..) => "_id".into(),
            _ => unreachable!("unknown AST/HIR variant"),
        }
    }

    /// Generates one or two C++ expressions that convert from a C++ type to the corresponding C type.
    ///
    /// Returns `PartiallyNamedExpression`s whose `suffix` is either empty, `Data`, or `Size` for
    /// referencing fields of the C struct.
    fn gen_dart_to_c_for_type<'b, P: TyPosition>(
        &mut self,
        ty: &Type<P>,
        dart_name: Cow<'b, str>,
        slice_conversions: &mut Vec<Cow<'b, str>>,
    ) -> Cow<'b, str> {
        match *ty {
            Type::Primitive(..) => dart_name.clone(),
            Type::Opaque(ref op) if op.is_optional() => {
                // TODO(rb): Is `null` a valid `ffi.Pointer<T>`?
                format!("{dart_name}._underlying").into()
            }
            Type::Opaque(..) => format!("{dart_name}._underlying").into(),
            Type::Struct(..) => format!("{dart_name}._underlying").into(),
            Type::Enum(..) => format!("{dart_name}._id").into(),
            Type::Slice(s) => {
                let name = format!("{dart_name}Slice");
                slice_conversions.push(
                    format!(
                        "final {name} = {}._fromDart({dart_name}, alloc);",
                        &self.gen_slice(&s)
                    )
                    .into(),
                );
                name.into()
            }
            _ => unreachable!("unknown AST/HIR variant"),
        }
    }

    /// Generates the Dart type name of a return type.
    fn gen_dart_return_type_name(&mut self, result_ty: &ReturnType) -> Cow<'dartcx, str> {
        match *result_ty {
            ReturnType::Infallible(None) => self.cx.formatter.fmt_void().into(),
            ReturnType::Infallible(Some(ref ty)) => match ty {
                SuccessType::Writeable => self.cx.formatter.fmt_string().into(),
                SuccessType::OutType(o) => self.gen_type_name(o),
                &_ => unreachable!("unknown AST/HIR variant"),
            },
            ReturnType::Fallible(ref ok, _) => match ok {
                Some(SuccessType::Writeable) => self.cx.formatter.fmt_string().into(),
                None => self.cx.formatter.fmt_void().into(),
                Some(SuccessType::OutType(o)) => self.gen_type_name(o),
                &Some(_) => unreachable!("unknown AST/HIR variant"),
            },
        }
    }

    fn gen_slice(&mut self, slice: &hir::Slice) -> &'static str {
        #[derive(askama::Template)]
        #[template(path = "dart/slice.dart.jinja", escape = "none")]
        struct SliceTemplate {
            ffi_type: &'static str,
            slice_ty: &'static str,
            dart_ty: &'static str,
            to_dart: &'static str,
            from_dart: &'static str,
        }

        self.imports.insert(Import {
            path: "package:ffi/ffi.dart".into(),
            suffix: " as ffi2".into(),
        });

        let dart_ty = match slice {
            hir::Slice::Str(..) => self.cx.formatter.fmt_string(),
            hir::Slice::Primitive(_, p) => {
                self.imports
                    .insert(Import::simple("dart:typed_data".into()));
                self.cx.formatter.fmt_primitive_list_type(*p)
            }
            _ => todo!("{slice:?}"),
        };

        let slice_ty = match slice {
            hir::Slice::Str(..) => self.cx.formatter.fmt_str_slice_type(),
            hir::Slice::Primitive(_, p) => self.cx.formatter.fmt_slice_type(*p),
            _ => todo!("{slice:?}"),
        };

        let ffi_type = match slice {
            hir::Slice::Str(..) => self.cx.formatter.fmt_utf8_primitive(),
            hir::Slice::Primitive(_, p) => self.cx.formatter.fmt_primitive_as_ffi(*p, false),
            _ => todo!("{slice:?}"),
        };

        let to_dart = match slice {
            hir::Slice::Str(..) => {
                self.imports.insert(Import::simple("dart:convert".into()));
                "Utf8Decoder().convert(_bytes.cast<ffi.Uint8>().asTypedList(_length))"
            }
            // TODO: How to read ffi.Size?
            hir::Slice::Primitive(_, hir::PrimitiveType::IntSize(_)) => "this",
            _ => "_bytes.asTypedList(_length)",
        };

        let from_dart = match slice {
            hir::Slice::Str(..) => concat!(
                "    final units = Utf8Encoder().convert(value);\n",
                "    slice._length = units.length;\n",
                // TODO: Figure out why Pointer<Utf8> cannot be allocated
                "    slice._bytes = allocator<ffi.Uint8>(slice._length).cast();\n",
                "    slice._bytes.cast<ffi.Uint8>().asTypedList(slice._length).setAll(0, units);\n"
            ),
            hir::Slice::Primitive(_, hir::PrimitiveType::IntSize(_)) => "",
            _ => concat!(
                "    slice._length = value.length;\n",
                "    slice._bytes = allocator(slice._length);\n",
                "    slice._bytes.asTypedList(slice._length).setAll(0, value);\n"
            ),
        };

        self.helper_classes.insert(
            slice_ty.into(),
            SliceTemplate {
                ffi_type,
                slice_ty,
                dart_ty,
                to_dart,
                from_dart,
            }
            .render()
            .unwrap(),
        );

        slice_ty
    }

    /// Generates the Dart FFI type name of a return type.
    fn gen_ffi_return_type_name(
        &mut self,
        result_ty: &ReturnType,
        cast: bool,
    ) -> Cow<'dartcx, str> {
        match *result_ty {
            ReturnType::Infallible(None) => if cast {
                self.cx.formatter.fmt_void()
            } else {
                self.cx.formatter.fmt_ffi_void()
            }
            .into(),
            ReturnType::Infallible(Some(ref ty)) => match ty {
                SuccessType::Writeable => if cast {
                    self.cx.formatter.fmt_void()
                } else {
                    self.cx.formatter.fmt_ffi_void()
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
            ReturnType::Fallible(ref ok, ref err) => {
                #[derive(askama::Template)]
                #[template(path = "dart/result.dart.jinja", escape = "none")]
                struct ResultTemplate {
                    name: String,
                    decls: Vec<String>,
                }

                let ok = match ok {
                    None | Some(SuccessType::Writeable) => None,
                    Some(SuccessType::OutType(o)) => Some(o),
                    &Some(_) => unreachable!("unknown AST/HIR variant"),
                };

                let err = err.as_ref();

                let name = format!(
                    "_Result{}{}",
                    &self
                        .cx
                        .formatter
                        .fmt_type_as_ident(ok.map(|o| self.gen_type_name_ffi(o, false)).as_deref()),
                    &self.cx.formatter.fmt_type_as_ident(
                        err.map(|o| self.gen_type_name_ffi(o, false)).as_deref()
                    )
                );

                let decls =
                    [ok.map(|o| (o, "ok")), err.map(|o| (o, "err"))]
                        .into_iter()
                        .flatten()
                        .map(|(o, field_name)| {
                            format!(
                                "{}external {} {field_name};\n",
                                match o {
                                    hir::OutType::Primitive(p) => {
                                        format!(
                                            "@{}()\n\t\t",
                                            self.cx.formatter.fmt_primitive_as_ffi(*p, false)
                                        )
                                    }
                                    hir::OutType::Enum(_) => format!(
                                        "@{}()\n\t\t",
                                        self.cx.formatter.fmt_enum_as_ffi(false)
                                    ),
                                    _ => String::new(),
                                },
                                { self.gen_type_name_ffi(o, true) }
                            )
                        })
                        .collect();

                self.helper_classes.insert(
                    name.clone(),
                    ResultTemplate {
                        name: name.clone(),
                        decls,
                    }
                    .render()
                    .unwrap(),
                );

                name.into()
            }
        }
    }

    /// Generates a C++ expression that converts from a C type to the corresponding C++ type.
    ///
    /// If the type is a slice, this function assumes that `{var_name}_data` and `{var_name}_size` resolve
    /// to valid expressions referencing the two different C variables for the pointer and the length.
    fn gen_c_to_dart_for_type<'b, P: TyPosition>(
        &mut self,
        ty: &Type<P>,
        var_name: Cow<'b, str>,
    ) -> Cow<'b, str> {
        match *ty {
            Type::Primitive(..) => var_name,
            Type::Opaque(ref op) => {
                let id = op.tcx_id.into();
                let type_name = self.cx.formatter.fmt_type_name(id);

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
                let type_name = self.cx.formatter.fmt_type_name(id);
                format!("{type_name}._({var_name})").into()
            }
            Type::Enum(ref e) => {
                let id = e.tcx_id.into();
                let type_name = self.cx.formatter.fmt_type_name(id);
                format!("{type_name}._({var_name})").into()
            }
            Type::Slice(..) => format!("{var_name}._asDart").into(),
            _ => unreachable!("unknown AST/HIR variant"),
        }
    }

    /// Generates a C++ expression that converts from a C return type to the corresponding C++ return type.
    ///
    /// If the type is `Writeable`, this function assumes that there is a variable named `writeable` in scope.
    fn gen_c_to_dart_for_return_type<'b>(
        &mut self,
        result_ty: &ReturnType,
        var_name: Cow<'b, str>,
    ) -> Option<Cow<'b, str>> {
        match *result_ty {
            ReturnType::Infallible(None) => None,
            ReturnType::Infallible(Some(SuccessType::Writeable)) => {
                Some("return writeable.finalize();".into())
            }
            ReturnType::Infallible(Some(SuccessType::OutType(ref out_ty))) => {
                Some(format!("return {};", self.gen_c_to_dart_for_type(out_ty, var_name)).into())
            }
            ReturnType::Fallible(ref ok, ref err) => {
                let ok_path = format!("{var_name}.union.ok");
                let err_path = format!("{var_name}.union.err");
                let err_conversion = match err {
                    Some(o) => self.gen_c_to_dart_for_type(o, err_path.into()),
                    None => {
                        self.helper_classes.insert(
                            "voiderror".into(),
                            "/// An unspecified error value\nclass VoidError {}".into(),
                        );
                        "VoidError()".into()
                    }
                };
                let ok_conversion = match ok {
                    // Note: the `writeable` variable is a string initialized in the template
                    Some(SuccessType::Writeable) => "writeable.finalize()".into(),
                    Some(SuccessType::OutType(o)) => self.gen_c_to_dart_for_type(o, ok_path.into()),
                    None => {
                        return Some(
                            format!("if (!{var_name}.isOk) {{ throw {err_conversion}; }}").into(),
                        )
                    }
                    &Some(_) => unreachable!("unknown AST/HIR variant"),
                };
                Some(
                    format!("return {var_name}.isOk ? {ok_conversion} : throw {err_conversion};")
                        .into(),
                )
            }
            ReturnType::Infallible(Some(_)) => unreachable!("unknown AST/HIR variant"),
        }
    }
}

/// A type name with a corresponding variable name, such as a struct field or a function parameter.
struct NamedType<'a> {
    var_name: Cow<'a, str>,
    type_name: Cow<'a, str>,
}

/// Everything needed for rendering a method.
struct MethodInfo<'a> {
    /// HIR of the method being rendered
    method: &'a hir::Method,
    ///
    docs: String,
    /// The declaration (everything before the parameter list)
    declaration: String,
    /// The Dart method name
    method_name: Cow<'a, str>,
    /// The C method name
    c_method_name: Cow<'a, str>,

    param_types_ffi: Vec<Cow<'a, str>>,
    param_types_ffi_cast: Vec<Cow<'a, str>>,
    ffi_return_ty: Cow<'a, str>,
    ffi_cast_return_ty: Cow<'a, str>,

    slice_conversions: Vec<Cow<'a, str>>,

    /// Dart conversion code for each parameter of the C function
    dart_to_ffi_params: Vec<Cow<'a, str>>,
    /// If the function has a return value, the Dart code for the conversion. Assumes that
    /// the C function return value is saved to a variable named `result` or that the
    /// writeable, if present, is saved to a variable named `writeable`.
    dart_return_expression: Option<Cow<'a, str>>,
}

#[derive(PartialEq, Ord, PartialOrd, Clone, Eq, Debug)]
struct Import {
    path: Cow<'static, str>,
    suffix: Cow<'static, str>,
}

impl Import {
    fn simple(path: Cow<'static, str>) -> Self {
        Import {
            path,
            suffix: "".into(),
        }
    }
}

#[derive(PartialEq, Ord, PartialOrd, Clone, Eq, Debug)]
struct ResultClass {
    ok_name: String,
    err_name: String,
    ok_decl: Option<String>,
    err_decl: Option<String>,
}
