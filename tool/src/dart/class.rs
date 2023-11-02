use super::DartContext;
use super::DartFormatter;
use askama::Template;
use diplomat_core::hir::{
    self, OpaqueOwner, ReturnType, SelfType, SuccessType, TyPosition, Type, TypeDef, TypeId,
};
use std::borrow::Cow;
use std::collections::BTreeMap;
use std::collections::BTreeSet;

#[derive(PartialEq, Ord, PartialOrd, Clone, Eq, Debug)]
pub struct Class {
    // Only for sorting
    name: String,
    body: String,
    imports: BTreeSet<Import>,
    helper_classes: BTreeMap<String, String>,
}

impl Class {
    pub fn init() -> Self {
        Self {
            name: Default::default(),
            body: include_str!("init.dart").into(),
            imports: [
                Import::simple("dart:convert".into()),
                Import {
                    path: "dart:ffi".into(),
                    suffix: " as ffi".into(),
                },
            ]
            .into_iter()
            .collect(),
            helper_classes: Default::default(),
        }
    }

    pub fn append(mut self, other: Self) -> Self {
        self.body.push_str("\n\n");
        self.body.push_str(&other.body);
        self.imports.extend(other.imports.into_iter());
        self.helper_classes.extend(other.helper_classes.into_iter());
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
    pub fn gen_ty(&self, id: TypeId) -> Class {
        let ty = self.tcx.resolve_type(id);

        let mut imports = BTreeSet::new();
        let mut helper_classes = BTreeMap::new();
        let _guard = self.errors.set_context_ty(ty.name().as_str().into());

        let mut tgcx = TyGenContext {
            imports: &mut imports,
            helper_classes: &mut helper_classes,
            cx: self,
        };

        let name = ty.name().as_str();

        let body = match ty {
            TypeDef::Enum(o) => tgcx.gen_enum(o, id, name.into()),
            TypeDef::Opaque(o) => tgcx.gen_opaque_def(o, id, name.into()),
            TypeDef::Struct(s) => tgcx.gen_struct_def(s, id, name.into()),
            TypeDef::OutStruct(s) => tgcx.gen_struct_def(s, id, name.into()),
            _ => unreachable!("unknown AST/HIR variant"),
        };

        Class {
            name: name.into(),
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
    fn gen_enum(
        &mut self,
        ty: &'tcx hir::EnumDef,
        id: TypeId,
        type_name: Cow<'tcx, str>,
    ) -> String {
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
            .flat_map(|method| self.gen_method_info(id, method, &type_name))
            .collect::<Vec<_>>();

        ImplTemplate {
            ty,
            fmt: &self.cx.formatter,
            type_name: &type_name,
            methods: methods.as_slice(),
            docs: self.cx.formatter.fmt_docs(&ty.docs),
        }
        .render()
        .unwrap()
    }

    fn gen_opaque_def(
        &mut self,
        ty: &'tcx hir::OpaqueDef,
        id: TypeId,
        type_name: Cow<'tcx, str>,
    ) -> String {
        #[derive(Template)]
        #[template(path = "dart/opaque.dart.jinja", escape = "none")]
        struct ImplTemplate<'a> {
            type_name: &'a str,
            methods: &'a [MethodInfo<'a>],
            docs: String,
        }

        self.imports.extend([
            Import {
                path: "dart:ffi".into(),
                suffix: " as ffi".into(),
            },
        ]);

        let methods = ty
            .methods
            .iter()
            .flat_map(|method| self.gen_method_info(id, method, &type_name))
            .collect::<Vec<_>>();

        ImplTemplate {
            type_name: &type_name,
            methods: methods.as_slice(),
            docs: self.cx.formatter.fmt_docs(&ty.docs),
        }
        .render()
        .unwrap()
    }

    fn gen_struct_def<P: TyPosition>(
        &mut self,
        ty: &'tcx hir::StructDef<P>,
        id: TypeId,
        type_name: Cow<'tcx, str>,
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
            annotation: Option<Cow<'a, str>>,
            ffi_cast_type_name: Cow<'a, str>,
            dart_type_name: Cow<'a, str>,
            get_expression: Cow<'a, str>,
            set_cleanups: Vec<String>,
            set_slice_conversions: Vec<Cow<'a, str>>,
            set_expressions: Vec<NamedExpression<'a>>,
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
                    hir::Type::Primitive(p) => Some(self.cx.formatter.fmt_primitive_as_ffi(p)),
                    hir::Type::Enum(_) => Some("ffi.Int32".into()),
                    _ => None,
                };

                let ffi_cast_type_name = self.gen_type_name_ffi(&field.ty, true);

                let ffi_cast_type_name = if ffi_cast_type_name.starts_with("Slice:") {
                    self.helper_classes.insert("slice".into(), include_str!("slice.dart").into());
                    "_Slice".into()
                } else {
                    ffi_cast_type_name
                };

                let dart_type_name = self.gen_type_name(&field.ty);

                let get_expression = self
                    .gen_c_to_dart_for_type(&field.ty, format!("this._underlying.{name}").into());

                let set_cleanups = if ffi_cast_type_name == "_Slice" {
                    vec![format!("this._underlying.{name}.bytes")]
                } else if ffi_cast_type_name.starts_with("ffi.Pointer") {
                    vec![format!("this._underlying.{name}")]
                } else {
                    vec![]
                };

                let mut set_slice_conversions = Vec::new();

                let set_expressions = self
                    .gen_dart_to_c_for_type(&field.ty, name.clone(), &mut set_slice_conversions)
                    .into_iter()
                    .map(
                        |PartiallyNamedExpression { suffix, expression }| NamedExpression {
                            name: format!("{name}{suffix}").into(),
                            expression,
                        },
                    )
                    .collect();

                FieldInfo {
                    name,
                    annotation,
                    ffi_cast_type_name,
                    dart_type_name,
                    get_expression,
                    set_cleanups,
                    set_slice_conversions,
                    set_expressions,
                }
            })
            .collect::<Vec<_>>();

        let methods = ty
            .methods
            .iter()
            .flat_map(|method| self.gen_method_info(id, method, &type_name))
            .collect::<Vec<_>>();

        ImplTemplate {
            type_name: &type_name,
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
            if let Some(element_type_ffi) = param_type_ffi.strip_prefix("Slice:") {
                param_types_ffi.push(format!("ffi.Pointer<{element_type_ffi}>").into());
                param_types_ffi.push("ffi.Size".into());
            } else {
                param_types_ffi.push(param_type_ffi);
            }
            let param_type_ffi_cast = self.gen_type_name_ffi(&param.ty, true);
            if let Some(element_type_ffi_cast) = param_type_ffi_cast.strip_prefix("Slice:") {
                param_types_ffi_cast.push(format!("ffi.Pointer<{element_type_ffi_cast}>").into());
                param_types_ffi_cast.push("int".into());
            } else {
                param_types_ffi_cast.push(param_type_ffi_cast);
            }
            let conversions = self.gen_dart_to_c_for_type(
                &param.ty,
                self.cx.formatter.fmt_param_name(param.name.as_str()),
                &mut slice_conversions,
            );
            dart_to_ffi_params.extend(
                conversions
                    .into_iter()
                    .map(|PartiallyNamedExpression { expression, .. }| expression),
            );
        }

        if method.is_writeable() {
            dart_to_ffi_params.push("writeable._underlying".into());
            param_types_ffi.push("ffi.Pointer<ffi.Opaque>".into());
            param_types_ffi_cast.push("ffi.Pointer<ffi.Opaque>".into());
            self.helper_classes.insert("writeable".into(), include_str!("writeable.dart").into());
        }

        let ffi_return_ty = self.gen_ffi_return_type_name(&method.output, false);
        let ffi_cast_return_ty = self.gen_ffi_return_type_name(&method.output, true);

        let dart_return_expression: Option<Cow<str>> =
            self.gen_c_to_dart_for_return_type(&method.output, "result".into());

        let return_ty = self.gen_dart_return_type_name(&method.output);
        let method_name = self.cx.formatter.fmt_method_name(method);
        let declaration = if method.param_self.is_none() {
            if return_ty == type_name {
                format!("factory {type_name}.{method_name}")
            } else {
                format!("static {return_ty} {method_name}")
            }
        } else {
            format!("{return_ty} {method_name}")
        };

        let docs = self.cx.formatter.fmt_docs(&method.docs);

        Some(MethodInfo {
            method,
            docs,
            declaration,
            method_name,
            c_method_name,
            param_decls_dart,
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
            Type::Primitive(prim) => self.cx.formatter.fmt_primitive_as_dart(prim),
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
            Type::Slice(hir::Slice::Str(_lifetime)) => self.cx.formatter.fmt_string(),
            Type::Slice(hir::Slice::Primitive(_, p)) => {
                self.imports
                    .insert(Import::simple("dart:typed_data".into()));
                self.cx.formatter.fmt_primitive_list(p)
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
            Type::Primitive(prim) => {
                if cast {
                    self.cx.formatter.fmt_primitive_as_dart(prim)
                } else {
                    self.cx.formatter.fmt_primitive_as_ffi(prim)
                }
            }
            Type::Opaque(ref op) => {
                let op_id = op.tcx_id.into();
                let type_name = self.cx.formatter.fmt_type_name(op_id);
                if self.cx.tcx.resolve_type(op_id).attrs().disable {
                    self.cx
                        .errors
                        .push_error(format!("Found usage of disabled type {type_name}"))
                }
                "ffi.Pointer<ffi.Opaque>".into()
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
                if cast { "int" } else { "ffi.Uint32" }.into()
            }
            Type::Slice(hir::Slice::Str(_lifetime)) => "Slice:ffi.Char".into(),
            Type::Slice(hir::Slice::Primitive(_, p)) => {
                format!("Slice:{}", self.cx.formatter.fmt_primitive_as_ffi(p)).into()
            }
            _ => unreachable!("unknown AST/HIR variant"),
        }
    }

    fn gen_self_type_ffi(&self, ty: &SelfType, cast: bool) -> Cow<'tcx, str> {
        match ty {
            SelfType::Opaque(_) => "ffi.Pointer<ffi.Opaque>".into(),
            SelfType::Struct(s) => format!("_{}Ffi", s.resolve(self.cx.tcx).name.as_str()).into(),
            SelfType::Enum(_) => if cast { "int" } else { "ffi.Uint32" }.into(),
            _ => unreachable!("unknown AST/HIR variant"),
        }
    }

    /// Generates Dart code for referencing a particular FFI type.
    ///
    /// This function adds the necessary type imports to the decl and impl files.

    /// Generates a C++ expression that converts from the C++ self type to the corresponding C self type.
    fn gen_dart_to_c_self(&self, ty: &SelfType) -> Cow<'static, str> {
        match *ty {
            SelfType::Opaque(..) => "this._underlying".into(),
            SelfType::Struct(..) => "this._underlying".into(),
            SelfType::Enum(..) => "this._id".into(),
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
    ) -> Vec<PartiallyNamedExpression<'b>> {
        match *ty {
            Type::Primitive(..) => {
                vec![PartiallyNamedExpression {
                    suffix: "".into(),
                    expression: dart_name.clone(),
                }]
            }
            Type::Opaque(ref op) if op.is_optional() => {
                vec![PartiallyNamedExpression {
                    suffix: "".into(),
                    // TODO(rb): Is `null` a valid `ffi.Pointer<T>`?
                    expression: format!("{dart_name}._underlying").into(),
                }]
            }
            Type::Opaque(..) => {
                vec![PartiallyNamedExpression {
                    suffix: "".into(),
                    expression: format!("{dart_name}._underlying").into(),
                }]
            }
            Type::Struct(..) => {
                vec![PartiallyNamedExpression {
                    suffix: "".into(),
                    expression: format!("{dart_name}._underlying").into(),
                }]
            }
            Type::Enum(..) => {
                vec![PartiallyNamedExpression {
                    suffix: "".into(),
                    expression: format!("{dart_name}._id").into(),
                }]
            }
            Type::Slice(hir::Slice::Str(..)) => {
                self.imports.insert(Import {
                    path: "package:ffi/ffi.dart".into(),
                    suffix: " as allocators".into(),
                });
                self.imports.insert(Import::simple("dart:convert".into()));
                slice_conversions.push(
                    format!("\
                         final {dart_name}List = Utf8Encoder().convert({dart_name});\n\t\t\
                         final {dart_name}Bytes = alloc.call<ffi.Char>({dart_name}List.length);\n\t\t\
                         {dart_name}Bytes.cast<ffi.Uint8>().asTypedList({dart_name}List.length).setAll(0, {dart_name}List);\n\
                    ").into()
                );
                vec![
                    PartiallyNamedExpression {
                        suffix: ".bytes".into(),
                        expression: format!("{dart_name}Bytes.cast()").into(),
                    },
                    PartiallyNamedExpression {
                        suffix: ".length".into(),
                        expression: format!("{dart_name}List.length").into(),
                    },
                ]
            }
            Type::Slice(hir::Slice::Primitive(_, p)) => {
                self.imports.insert(Import {
                    path: "package:ffi/ffi.dart".into(),
                    suffix: " as allocators".into(),
                });
                let native_prim = self.cx.formatter.fmt_primitive_as_ffi(p);
                slice_conversions.push(
                    format!(
                        "\
                         final {dart_name}Bytes = alloc.call<{native_prim}>({dart_name}.length);\n\
                         {dart_name}Bytes.asTypedList({dart_name}.length).setAll(0, {dart_name});\n\
                    "
                    )
                    .into(),
                );
                vec![
                    PartiallyNamedExpression {
                        suffix: ".bytes".into(),
                        expression: format!("{dart_name}Bytes.cast()").into(),
                    },
                    PartiallyNamedExpression {
                        suffix: ".length".into(),
                        expression: format!("{dart_name}.length").into(),
                    },
                ]
            }
            _ => unreachable!("unknown AST/HIR variant"),
        }
    }

    /// Generates the Dart type name of a return type.
    fn gen_dart_return_type_name(&mut self, result_ty: &ReturnType) -> Cow<'dartcx, str> {
        match *result_ty {
            ReturnType::Infallible(None) => "void".into(),
            ReturnType::Infallible(Some(ref ty)) => match ty {
                SuccessType::Writeable => self.cx.formatter.fmt_string(),
                SuccessType::OutType(o) => self.gen_type_name(o),
                &_ => unreachable!("unknown AST/HIR variant"),
            },
            ReturnType::Fallible(ref ok, _) => match ok {
                Some(SuccessType::Writeable) => self.cx.formatter.fmt_string(),
                None => "void".into(),
                Some(SuccessType::OutType(o)) => self.gen_type_name(o),
                &Some(_) => unreachable!("unknown AST/HIR variant"),
            },
        }
    }

    /// Generates the Dart FFI type name of a return type.
    fn gen_ffi_return_type_name(
        &mut self,
        result_ty: &ReturnType,
        cast: bool,
    ) -> Cow<'dartcx, str> {
        match *result_ty {
            ReturnType::Infallible(None) => if cast { "void" } else { "ffi.Void" }.into(),
            ReturnType::Infallible(Some(ref ty)) => match ty {
                SuccessType::Writeable => if cast { "void" } else { "ffi.Void" }.into(),
                SuccessType::OutType(o) => {
                    let out = self.gen_type_name_ffi(o, cast);

                    if out.starts_with("Slice:") {
                        self.helper_classes.insert("slice".into(), include_str!("slice.dart").into());
                        "_Slice".into()
                    } else {
                        out
                    }
                }
                &_ => unreachable!("unknown AST/HIR variant"),
            },
            ReturnType::Fallible(ref ok, ref err) => {
                let ok = match ok {
                    None | Some(SuccessType::Writeable) => None,
                    Some(SuccessType::OutType(o)) => Some(o),
                    &Some(_) => unreachable!("unknown AST/HIR variant"),
                };

                let ok_name = ok
                    .as_ref()
                    .map(|o| self.gen_type_name_ffi(o, false))
                    .as_deref()
                    .unwrap_or("Void")
                    .replace("ffi.Pointer<ffi.Opaque>", "Opaque")
                    .replace("ffi.", "");

                let err_name = err
                    .as_ref()
                    .map(|o| self.gen_type_name_ffi(o, false))
                    .as_deref()
                    .unwrap_or("Void")
                    .replace("ffi.Pointer<ffi.Opaque>", "Opaque")
                    .replace("ffi.", "");

                fn decl(
                    selff: &mut TyGenContext,
                    field_name: &str,
                    o: &Type<hir::OutputOnly>,
                ) -> String {
                    format!(
                        "{}external {} {field_name};\n",
                        match o {
                            hir::OutType::Primitive(p) => {
                                format!("@{}()\n\t\t", selff.cx.formatter.fmt_primitive_as_ffi(*p))
                            }
                            hir::OutType::Enum(_) => "@ffi.Int32()\n\t\t".into(),
                            _ => String::new(),
                        },
                        { selff.gen_type_name_ffi(o, true) }
                    )
                }
                let ok_decl = ok.map(|o| decl(self, "ok", o));
                let err_decl = err.as_ref().map(|o| decl(self, "err", o));

                #[derive(askama::Template)]
                #[template(path = "dart/result.dart.jinja", escape = "none")]
                struct ResultTemplate<'a> {
                    ok_name: &'a str,
                    err_name: &'a str,
                    ok_decl: Option<String>,
                    err_decl: Option<String>,
                }

                let name = format!("_Result{ok_name}{err_name}");

                self.helper_classes.insert(
                    name.clone(),
                    ResultTemplate {
                        ok_name: &ok_name,
                        err_name: &err_name,
                        ok_decl,
                        err_decl,
                    }.render().unwrap(),
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
            Type::Slice(hir::Slice::Str(..)) => {
                self.imports.insert(Import::simple("dart:convert".into()));
                self.imports
                    .insert(Import::simple("dart:typed_data".into()));
                format!("Utf8Decoder(allowMalformed: false).convert({var_name}.bytes.cast<ffi.Uint8>().asTypedList({var_name}.length))").into()
            }
            Type::Slice(hir::Slice::Primitive(_, p)) => {
                self.imports
                    .insert(Import::simple("dart:typed_data".into()));
                let prim_ty = self.cx.formatter.fmt_primitive_as_ffi(p);
                format!("{var_name}.bytes.cast<{prim_ty}>().asTypedList({var_name}.length)").into()
            }
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
                Some("return writeable.toString();".into())
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
                        self.helper_classes.insert("voiderror".into(), "class VoidError {}".into());
                        "VoidError()".into()
                    }
                };
                let ok_conversion = match ok {
                    // Note: the `writeable` variable is a string initialized in the template
                    Some(SuccessType::Writeable) => "writeable.toString()".into(),
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

/// An expression with a corresponding variable name, such as a struct field or a function parameter.
struct NamedExpression<'a> {
    name: Cow<'a, str>,
    expression: Cow<'a, str>,
}

/// An expression associated with a variable name having the given suffix.
struct PartiallyNamedExpression<'a> {
    suffix: Cow<'a, str>,
    expression: Cow<'a, str>,
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
    /// Type declarations for the Dart parameters
    param_decls_dart: Vec<NamedType<'a>>,
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
