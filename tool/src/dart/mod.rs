use std::borrow::Cow;
use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write;

use crate::{ErrorStore, FileMap};
use diplomat_core::hir::OutputOnly;
use diplomat_core::hir::{
    self,
    borrowing_param::{
        BorrowedLifetimeInfo, LifetimeEdge, LifetimeEdgeKind, ParamBorrowInfo, StructBorrowInfo,
    },
    BackendAttrSupport, DocsUrlGenerator, Lifetime, LifetimeEnv, MaybeStatic, OpaqueOwner,
    ReturnType, SelfType, SpecialMethod, SpecialMethodPresence, StructPathLike, SuccessType,
    TyPosition, Type, TypeContext, TypeDef, TypeId,
};

use askama::Template;

mod formatter;
use formatter::DartFormatter;

pub(crate) fn attr_support() -> BackendAttrSupport {
    let mut a = BackendAttrSupport::default();

    a.namespacing = false;
    a.memory_sharing = false;
    a.non_exhaustive_structs = true;
    a.method_overloading = false;
    a.utf8_strings = false;
    a.utf16_strings = true;
    a.static_slices = false;

    a.constructors = true;
    a.named_constructors = true;
    a.fallible_constructors = true;
    a.accessors = true;
    a.stringifiers = true;
    a.comparators = true;
    a.iterators = true;
    a.iterables = true;
    a.indexing = true;
    a.option = true;
    a.callbacks = false;
    a.traits = false;
    a.traits_are_send = false;
    a.traits_are_sync = false;

    a
}

pub(crate) fn run<'cx>(
    tcx: &'cx TypeContext,
    docs_url_gen: &'cx DocsUrlGenerator,
) -> (FileMap, ErrorStore<'cx, String>) {
    let formatter = DartFormatter::new(tcx, docs_url_gen);

    let files = FileMap::default();
    let errors = ErrorStore::default();

    let mut directives = BTreeSet::default();
    let mut helper_classes = BTreeMap::default();

    let mut context = TyGenContext {
        tcx,
        errors: &errors,
        helper_classes: &mut helper_classes,
        formatter: &formatter,
    };

    for (id, ty) in tcx.all_types() {
        if ty.attrs().disable {
            continue;
        }

        let (file_name, body) = context.gen(id);

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
        Some("unused_shown_name"),
    ));
    directives.insert(formatter.fmt_import("dart:core", Some("as core"), Some("unused_import")));

    directives.insert(formatter.fmt_import("dart:ffi", Some("as ffi"), None));
    directives.insert(formatter.fmt_import(
        "package:ffi/ffi.dart",
        Some("as ffi2 show Arena, calloc"),
        None,
    ));
    directives.insert(formatter.fmt_import("package:meta/meta.dart", Some("as meta"), None));

    // For DiplomatWrite and slices
    directives.insert(formatter.fmt_import("dart:convert", None, None));

    // For slices
    directives.insert(formatter.fmt_import("dart:typed_data", None, Some("unused_import")));

    files.add_file(
        formatter.fmt_file_name("lib"),
        render_class(
            include_str!("../../templates/dart/init.dart").into(),
            directives,
            helper_classes,
        ),
    );

    (files, errors)
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

impl<'cx> TyGenContext<'_, 'cx> {
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
            .filter(|m| !m.attrs.disable)
            .flat_map(|method| self.gen_method_info(id, method, type_name))
            .collect::<Vec<_>>();

        let special = self.gen_special_method_info(&ty.special_method_presence);

        #[derive(Template)]
        #[template(path = "dart/enum.dart.jinja", escape = "none")]
        struct ImplTemplate<'a> {
            ty: &'a hir::EnumDef,
            fmt: &'a DartFormatter<'a>,
            type_name: &'a str,
            methods: &'a [MethodInfo<'a>],
            docs: String,
            is_contiguous: bool,
            special: SpecialMethodGenInfo<'a>,
        }

        ImplTemplate {
            ty,
            fmt: self.formatter,
            type_name,
            methods: methods.as_slice(),
            docs: self.formatter.fmt_docs(&ty.docs),
            is_contiguous: is_contiguous_enum(ty),
            special,
        }
        .render()
        .unwrap()
    }

    fn gen_opaque_def(&mut self, ty: &'cx hir::OpaqueDef, id: TypeId, type_name: &str) -> String {
        let methods = ty
            .methods
            .iter()
            .filter(|m| !m.attrs.disable)
            .flat_map(|method| self.gen_method_info(id, method, type_name))
            .collect::<Vec<_>>();

        let destructor = &ty.dtor_abi_name;
        let special = self.gen_special_method_info(&ty.special_method_presence);

        #[derive(Template)]
        #[template(path = "dart/opaque.dart.jinja", escape = "none")]
        struct ImplTemplate<'a> {
            type_name: &'a str,
            methods: &'a [MethodInfo<'a>],
            docs: String,
            destructor: &'a str,
            lifetimes: &'a LifetimeEnv,
            special: SpecialMethodGenInfo<'a>,
        }

        ImplTemplate {
            type_name,
            methods: methods.as_slice(),
            destructor: destructor.as_str(),
            docs: self.formatter.fmt_docs(&ty.docs),
            lifetimes: &ty.lifetimes,
            special,
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

                let ffi_cast_type_name = self.gen_type_name_ffi(&field.ty, true);

                let dart_type_name = self.gen_type_name(&field.ty);

                let c_to_dart = self.gen_c_to_dart_for_type(
                    &field.ty,
                    format!("ffi.{name}").into(),
                    &ty.lifetimes,
                );


                // We do not need to handle lifetime transitivity here: Methods already resolve
                // lifetime transitivity, and we have an HIR validity pass ensuring that struct lifetime bounds
                // are explicitly specified on methods.

                /// Get the name/initializer of the allocator needed for a particular type
                fn alloc_name<P: TyPosition>(ty: &hir::StructDef<P>, field_ty: &Type<P>) -> Option<String> {
                    if let &hir::Type::Slice(slice) = field_ty {
                        if let Some(lt) = slice.lifetime() {
                            let MaybeStatic::NonStatic(lt) = lt else {
                                panic!("'static not supported in Dart");
                            };
                            Some(format!(
                                "{lt_name}AppendArray.isNotEmpty ? _FinalizedArena.withLifetime({lt_name}AppendArray).arena : temp",
                                lt_name = ty.lifetimes.fmt_lifetime(lt),
                            ))
                        } else {
                            None
                        }
                    } else if let &hir::Type::Struct(..) = field_ty {
                        Some("temp".into())
                    } else if let hir::Type::DiplomatOption(inner) = field_ty {
                        alloc_name(ty, inner)
                    } else {
                        None
                    }
                }

                let alloc = alloc_name(ty, &field.ty);

                let struct_borrow_info = if let hir::Type::Struct(path) = &field.ty {
                    StructBorrowInfo::compute_for_struct_field(ty, path, self.tcx).map(
                        |param_info| StructBorrowContext {
                            use_env: &ty.lifetimes,
                            param_info,
                            is_method: false,
                        },
                    )
                } else {
                    None
                };


                let dart_to_c = self.gen_dart_to_c_for_type(&field.ty, name.clone(), struct_borrow_info.as_ref(), alloc.as_deref());

                FieldInfo {
                    name,
                    ty: &field.ty,
                    annotation,
                    ffi_cast_type_name,
                    dart_type_name,
                    c_to_dart,
                    dart_to_c,
                    param_info: struct_borrow_info.map(|i| i.param_info),
                }
            })
            .collect::<Vec<_>>();

        let mut methods = ty
            .methods
            .iter()
            .filter(|m| !m.attrs.disable)
            .flat_map(|method| self.gen_method_info(id, method, type_name))
            .collect::<Vec<_>>();
        let special = self.gen_special_method_info(&ty.special_method_presence);

        // Non-out structs need to be constructible in Dart
        let default_constructor = if !is_out {
            if let Some(constructor) = methods
                .iter_mut()
                .find(|m| m.declaration.contains(&format!("{type_name}()")))
            {
                // If there's an existing zero-arg constructor, we repurpose it with optional arguments for all fields
                let args = fields
                    .iter()
                    .map(|field| {
                        format!(
                            "{} {}",
                            self.formatter.fmt_nullable(&field.dart_type_name),
                            field.name
                        )
                    })
                    .collect::<Vec<_>>();
                constructor.declaration =
                    format!("factory {type_name}({{{args}}})", args = args.join(", "));

                let mut r = String::new();
                writeln!(&mut r, "final dart = {type_name}._fromFfi(result);").unwrap();
                for field in &fields {
                    let name = &field.name;
                    writeln!(&mut r, "if ({name} != null) {{").unwrap();
                    writeln!(&mut r, "  dart.{name} = {name};").unwrap();
                    writeln!(&mut r, "}}").unwrap();
                }
                write!(&mut r, "return dart;").unwrap();
                constructor.return_expression = Some(r.into());

                None
            } else if fields.is_empty() {
                // ZST
                Some(format!("{type_name}();"))
            } else {
                // Otherwise we create a constructor with required values for all non-optional fields.
                let args = fields
                    .iter()
                    .map(|field| {
                        format!(
                            "{}this.{}",
                            if field.ty.is_option() {
                                ""
                            } else {
                                "required "
                            },
                            field.name
                        )
                    })
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
            special: SpecialMethodGenInfo<'a>,
        }

        ImplTemplate {
            type_name,
            default_constructor,
            mutable,
            fields,
            methods,
            docs: self.formatter.fmt_docs(&ty.docs),
            lifetimes: &ty.lifetimes,
            special,
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

        let mut visitor = method.borrowing_param_visitor(self.tcx);

        let _guard = self.errors.set_context_method(
            self.tcx.fmt_type_name_diagnostics(id),
            method.name.as_str().into(),
        );

        let abi_name = method.abi_name.as_str();

        let mut param_decls_dart_required = Vec::new();
        let mut param_decls_dart_optional = Vec::new();
        let mut param_types_ffi = Vec::new();
        let mut param_types_ffi_cast = Vec::new();
        let mut param_names_ffi = Vec::new();
        let mut param_conversions = Vec::new();

        let mut needs_temp_arena = false;

        if let Some(param_self) = method.param_self.as_ref() {
            visitor.visit_param(&param_self.ty.clone().into(), "this");

            param_types_ffi.push(self.gen_self_type_name_ffi(&param_self.ty, false));
            param_types_ffi_cast.push(self.gen_self_type_name_ffi(&param_self.ty, true));
            param_conversions.push(self.gen_dart_to_c_self(&param_self.ty, "temp.arena"));
            param_names_ffi.push("self".into());
            if matches!(param_self.ty, hir::SelfType::Struct(..)) {
                needs_temp_arena = true;
            }
        }

        let mut arenas = Vec::new();

        for param in method.params.iter() {
            let param_name = self.formatter.fmt_param_name(param.name.as_str());
            if param.ty.is_option() {
                &mut param_decls_dart_optional
            } else {
                &mut param_decls_dart_required
            }
            .push(format!("{} {param_name}", self.gen_type_name(&param.ty),));
            param_names_ffi.push(param_name.clone());
            param_types_ffi.push(self.gen_type_name_ffi(&param.ty, false));
            param_types_ffi_cast.push(self.gen_type_name_ffi(&param.ty, true));

            let param_borrow_kind = visitor.visit_param(&param.ty, &param_name);

            /// Get the name/initializer of the allocator needed for a particular type
            fn alloc_name(
                param_ty: &hir::Type<hir::InputOnly>,
                param_name: &str,
                param_borrow_kind: &ParamBorrowInfo,
                needs_temp_arena: &mut bool,
                arenas: &mut Vec<Cow<str>>,
            ) -> Option<String> {
                if let hir::Type::Struct(..) = param_ty {
                    *needs_temp_arena = true;
                    Some("temp.arena".to_string())
                } else if let hir::Type::Slice(s) = param_ty {
                    match param_borrow_kind {
                        ParamBorrowInfo::BorrowedSlice => {
                            // Slices borrowed in the return value use a custom arena
                            arenas.push(
                                format!("final {param_name}Arena = _FinalizedArena();").into(),
                            );
                            Some(format!("{param_name}Arena.arena"))
                        }
                        // Owned slices use the Rust allocator
                        ParamBorrowInfo::TemporarySlice if s.lifetime().is_none() => None,
                        ParamBorrowInfo::TemporarySlice => {
                            // Everyone else uses the temporary arena that keeps stuff alive until the method is called
                            *needs_temp_arena = true;
                            Some("temp.arena".into())
                        }
                        _ => unreachable!(
                            "Slices must produce slice ParamBorrowInfo, found {param_borrow_kind:?}"
                        ),
                    }
                } else if let hir::Type::DiplomatOption(ref inner) = param_ty {
                    alloc_name(
                        inner,
                        param_name,
                        param_borrow_kind,
                        needs_temp_arena,
                        arenas,
                    )
                } else {
                    None
                }
            }
            let alloc = alloc_name(
                &param.ty,
                &param_name,
                &param_borrow_kind,
                &mut needs_temp_arena,
                &mut arenas,
            );

            let struct_borrow_info = if let ParamBorrowInfo::Struct(param_info) = param_borrow_kind
            {
                Some(StructBorrowContext {
                    use_env: &method.lifetime_env,
                    param_info,
                    is_method: true,
                })
            } else {
                None
            };

            param_conversions.push(self.gen_dart_to_c_for_type(
                &param.ty,
                param_name,
                struct_borrow_info.as_ref(),
                alloc.as_deref(),
            ));
        }

        if needs_temp_arena {
            arenas.insert(0, "final temp = _FinalizedArena();".into());
        }

        if method.output.is_write() {
            param_conversions.push("write._ffi".into());
            param_types_ffi.push(self.formatter.fmt_opaque_as_ffi().into());
            param_types_ffi_cast.push(self.formatter.fmt_opaque_as_ffi().into());
            param_names_ffi.push("write".into());
            self.helper_classes.insert(
                "write".into(),
                include_str!("../../templates/dart/write.dart").into(),
            );
        }

        let return_ty = self.gen_return_type_name(&method.output);
        let return_type_ffi = self.gen_return_type_name_ffi(&method.output, false);
        let return_type_ffi_cast = self.gen_return_type_name_ffi(&method.output, true);

        let return_expression =
            self.gen_c_to_dart_for_return_type(&method.output, &method.lifetime_env);

        let params = match (
            param_decls_dart_required.len(),
            param_decls_dart_optional.len(),
        ) {
            (_, 0) => param_decls_dart_required.join(", "),
            (0, 1) => format!("[{}]", param_decls_dart_optional[0]),
            (_, 1) => format!(
                "{}, [{}]",
                param_decls_dart_required.join(", "),
                param_decls_dart_optional[0]
            ),
            (0, _) => format!("{{{}}}", param_decls_dart_optional.join(", ")),
            (_, _) => format!(
                "{}, {{{}}}",
                param_decls_dart_required.join(", "),
                param_decls_dart_optional.join(", ")
            ),
        };
        let declaration = match &method.attrs.special_method {
            Some(SpecialMethod::Constructor) => format!("factory {type_name}({params})"),
            Some(SpecialMethod::NamedConstructor(name)) => format!(
                "factory {type_name}.{}({params})",
                self.formatter.fmt_constructor_name(name, method)
            ),
            Some(SpecialMethod::Getter(name)) => format!(
                "{return_ty} get {}",
                self.formatter.fmt_accessor_name(name, method)
            ),
            Some(SpecialMethod::Setter(name)) => format!(
                "set {}({params})",
                self.formatter.fmt_accessor_name(name, method)
            ),
            Some(SpecialMethod::Stringifier) => "@core.override\n  String toString()".into(),
            Some(SpecialMethod::Comparison) => format!("int compareTo({type_name} other)"),
            Some(SpecialMethod::Iterator) => format!("{return_ty} _iteratorNext({params})"),
            Some(SpecialMethod::Iterable) => format!("{return_ty} get iterator"),
            Some(SpecialMethod::Indexer) => format!("{return_ty} operator []({params})"),
            None if method.param_self.is_none() => format!(
                "static {return_ty} {}({params})",
                self.formatter.fmt_method_name(method)
            ),
            None => format!(
                "{return_ty} {}({params})",
                self.formatter.fmt_method_name(method)
            ),
            Some(special) => unimplemented!("Found unknown special method type {special:?}"),
        };

        let mut docs = self.formatter.fmt_docs(&method.docs);

        if let hir::ReturnType::Fallible(_, Some(e)) = &method.output {
            write!(
                &mut docs,
                "\n///\n/// Throws [{}] on failure.",
                self.gen_type_name(e)
            )
            .unwrap();
        }

        Some(MethodInfo {
            method,
            docs,
            declaration,
            abi_name,
            param_types_ffi,
            param_types_ffi_cast,
            param_names_ffi,
            return_type_ffi,
            return_type_ffi_cast,
            arenas,
            param_conversions,
            return_expression,
            lifetimes: &method.lifetime_env,
            method_lifetimes_map: visitor.borrow_map(),
        })
    }

    fn gen_special_method_info(
        &mut self,
        special_method_presence: &SpecialMethodPresence,
    ) -> SpecialMethodGenInfo<'cx> {
        let mut info = SpecialMethodGenInfo {
            comparator: special_method_presence.comparator,
            ..Default::default()
        };

        if let Some(ref val) = special_method_presence.iterator {
            info.iterator = Some(self.gen_success_ty(val))
        }
        if let Some(ref iterator) = special_method_presence.iterable {
            let iterator_def = self.tcx.resolve_opaque(*iterator);
            let Some(ref val) = iterator_def.special_method_presence.iterator else {
                self.errors
                    .push_error("Found iterable not returning an iterator type".into());
                return info;
            };
            info.iterable = Some(self.gen_success_ty(val))
        }

        info
    }

    fn gen_success_ty(&mut self, out_ty: &SuccessType) -> Cow<'cx, str> {
        match out_ty {
            SuccessType::Write => self
                .formatter
                .fmt_string_type(hir::StringEncoding::UnvalidatedUtf8)
                .into(),
            SuccessType::OutType(o) => self.gen_type_name(o),
            SuccessType::Unit => self.formatter.fmt_void().into(),
            _ => unreachable!(),
        }
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
                    self.formatter.fmt_nullable(&type_name).into()
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
            Type::Slice(hir::Slice::Str(_, encoding)) => {
                self.formatter.fmt_string_type(encoding).into()
            }
            Type::Slice(hir::Slice::Primitive(_, p)) => {
                self.formatter.fmt_primitive_list_type(p).into()
            }
            Type::Slice(hir::Slice::Strs(..)) => "core.List<core.String>".into(),
            Type::DiplomatOption(ref inner) => {
                let inner = self.gen_type_name(inner);
                self.formatter.fmt_nullable(&inner).into()
            }
            _ => unreachable!("unknown AST/HIR variant"),
        }
    }

    /// Generates a return type's Dart type.
    fn gen_return_type_name(&mut self, result_ty: &ReturnType) -> Cow<'cx, str> {
        match *result_ty {
            ReturnType::Infallible(SuccessType::Unit)
            | ReturnType::Fallible(SuccessType::Unit, Some(_)) => self.formatter.fmt_void().into(),
            ReturnType::Infallible(SuccessType::Write)
            | ReturnType::Fallible(SuccessType::Write, Some(_)) => self
                .formatter
                .fmt_string_type(hir::StringEncoding::Utf8)
                .into(),
            ReturnType::Infallible(SuccessType::OutType(ref o))
            | ReturnType::Fallible(SuccessType::OutType(ref o), Some(_)) => self.gen_type_name(o),
            ReturnType::Fallible(SuccessType::Write, None)
            | ReturnType::Nullable(SuccessType::Write) => self
                .formatter
                .fmt_nullable(self.formatter.fmt_string_type(hir::StringEncoding::Utf8))
                .into(),
            ReturnType::Fallible(SuccessType::Unit, None)
            | ReturnType::Nullable(SuccessType::Unit) => self
                .formatter
                .fmt_primitive_as_ffi(hir::PrimitiveType::Bool, true)
                .into(),
            ReturnType::Fallible(SuccessType::OutType(ref o), None)
            | ReturnType::Nullable(SuccessType::OutType(ref o)) => {
                self.formatter.fmt_nullable(&self.gen_type_name(o)).into()
            }
            _ => unreachable!("unknown AST/HIR variant"),
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
                self.formatter.fmt_opaque_as_ffi().into()
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
            Type::Slice(s) => self.gen_slice(&s).into(),
            Type::DiplomatOption(ref inner) => self.gen_result(Some(inner), None).into(),
            _ => unreachable!("unknown AST/HIR variant"),
        }
    }

    /// Generates the Dart FFI type name of a return type.
    fn gen_return_type_name_ffi(&mut self, result_ty: &ReturnType, cast: bool) -> Cow<'cx, str> {
        match *result_ty {
            ReturnType::Infallible(SuccessType::Unit) => if cast {
                self.formatter.fmt_void()
            } else {
                self.formatter.fmt_ffi_void()
            }
            .into(),
            ReturnType::Infallible(SuccessType::Write) => if cast {
                self.formatter.fmt_void()
            } else {
                self.formatter.fmt_ffi_void()
            }
            .into(),
            ReturnType::Infallible(SuccessType::OutType(ref o)) => self.gen_type_name_ffi(o, cast),
            ReturnType::Fallible(ref ok, ref err) => {
                self.gen_result(ok.as_type(), err.as_ref()).into()
            }
            ReturnType::Nullable(ref ok) => self.gen_result(ok.as_type(), None).into(),
            _ => unreachable!("unknown AST/HIR variant"),
        }
    }

    /// Generates a self type's Dart FFI type.
    fn gen_self_type_name_ffi(&self, ty: &SelfType, cast: bool) -> Cow<'cx, str> {
        match ty {
            SelfType::Opaque(_) => self.formatter.fmt_opaque_as_ffi().into(),
            SelfType::Struct(s) => format!("_{}Ffi", s.resolve(self.tcx).name.as_str()).into(),
            SelfType::Enum(_) => self.formatter.fmt_enum_as_ffi(cast).into(),
            _ => unreachable!("unknown AST/HIR variant"),
        }
    }

    /// Generates an FFI expression for a self type.
    fn gen_dart_to_c_self(&self, ty: &SelfType, allocator: &str) -> Cow<'static, str> {
        match *ty {
            SelfType::Enum(ref e) if is_contiguous_enum(e.resolve(self.tcx)) => "index".into(),
            SelfType::Struct(..) => format!("_toFfi({allocator})").into(),
            SelfType::Opaque(..) | SelfType::Enum(..) => "_ffi".into(),
            _ => unreachable!("unknown AST/HIR variant"),
        }
    }

    /// Generates an FFI expression for a type.
    ///
    /// For struct parameters borrowed by the output, `struct_borrow_info` is a map of
    fn gen_dart_to_c_for_type<P: TyPosition>(
        &mut self,
        ty: &Type<P>,
        dart_name: Cow<'cx, str>,
        struct_borrow_info: Option<&StructBorrowContext<'cx>>,
        alloc: Option<&str>,
    ) -> Cow<'cx, str> {
        match *ty {
            Type::Primitive(..) => dart_name.clone(),
            Type::Opaque(ref op) if op.is_optional() => format!(
                // Use coalescing to only evaluate `{dart_name}` once
                "{dart_name}?._ffi ?? ffi.Pointer.fromAddress(0)"
            )
            .into(),
            Type::Enum(ref e) if is_contiguous_enum(e.resolve(self.tcx)) => {
                format!("{dart_name}.index").into()
            }
            Type::Struct(..) => {
                self.gen_dart_to_c_for_struct_type(dart_name, struct_borrow_info, alloc.unwrap())
            }
            Type::Opaque(..) | Type::Enum(..) => format!("{dart_name}._ffi").into(),
            Type::Slice(s) => {
                self.gen_slice(&s);
                let alloc_in = match s {
                    hir::Slice::Primitive(_, hir::PrimitiveType::Byte) => {
                        "asUint8List()._uint8AllocIn"
                    }
                    hir::Slice::Primitive(_, p) => self.formatter.fmt_primitive_alloc_in(p),
                    hir::Slice::Str(_, encoding) => self.formatter.fmt_str_alloc_in(encoding),
                    hir::Slice::Strs(encoding) => self.formatter.fmt_str_slice_alloc_in(encoding),
                    _ => unreachable!("unknown AST/HIR variant"),
                };
                let alloc = if s.lifetime().is_none() {
                    "_RustAlloc()"
                } else {
                    alloc.expect("need allocator for slice")
                };
                format!("{dart_name}.{alloc_in}({alloc})",).into()
            }
            Type::DiplomatOption(ref inner) => {
                let conversion = self.gen_dart_to_c_for_type(
                    inner,
                    dart_name.clone(),
                    struct_borrow_info,
                    alloc,
                );
                let result = self.gen_result(Some(inner), None);
                format!("{dart_name} != null ? {result}.ok({conversion}) : {result}.err()").into()
            }
            _ => unreachable!("unknown AST/HIR variant"),
        }
    }

    /// Generates an FFI expression for a struct
    fn gen_dart_to_c_for_struct_type(
        &mut self,
        dart_name: Cow<'cx, str>,
        struct_borrow_info: Option<&StructBorrowContext<'cx>>,
        allocator: &str,
    ) -> Cow<'cx, str> {
        let mut params = String::from(allocator);
        if let Some(info) = struct_borrow_info {
            for (def_lt, use_lts) in &info.param_info.borrowed_struct_lifetime_map {
                write!(
                    &mut params,
                    ", {}AppendArray: [",
                    info.param_info.env.fmt_lifetime(def_lt)
                )
                .unwrap();
                let mut maybe_comma = "";
                for use_lt in use_lts {
                    // Generate stuff like `, aEdges` or for struct fields, `, ...aAppendArray`
                    let lt = info.use_env.fmt_lifetime(use_lt);
                    if info.is_method {
                        write!(&mut params, "{maybe_comma}{lt}Edges",).unwrap();
                    } else {
                        write!(&mut params, "{maybe_comma}...{lt}AppendArray",).unwrap();
                    }
                    maybe_comma = ", ";
                }
                write!(&mut params, "]").unwrap();
            }
        }
        format!("{dart_name}._toFfi({params})").into()
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

                let mut edges = if let Some(lt) = op.owner.lifetime() {
                    let MaybeStatic::NonStatic(lt) = lt else {
                        panic!("'static not supported in Dart")
                    };
                    self.formatter
                        .fmt_lifetime_edge_array(lt, lifetime_env)
                        .into_owned()
                } else {
                    "[]".into()
                };

                for lt in op.lifetimes.lifetimes() {
                    let MaybeStatic::NonStatic(lt) = lt else {
                        panic!("'static not supported in Dart");
                    };
                    // We only generate a single edge in the list per lifetime, despite transitivity
                    //
                    // This is because we plan to handle transitivity when constructing these edge arrays,
                    // e.g. if `'a: 'b`, `aEdges` will already contain the relevant bits from `bEdges`.
                    //
                    // This lets us do things like not generate bEdges if it's not actually relevant for returning.
                    write!(
                        edges,
                        ", {}",
                        self.formatter.fmt_lifetime_edge_array(lt, lifetime_env)
                    )
                    .unwrap();
                }

                if op.is_optional() {
                    format!("{var_name}.address == 0 ? null : {type_name}._fromFfi({var_name}, {edges})").into()
                } else {
                    format!("{type_name}._fromFfi({var_name}, {edges})").into()
                }
            }
            Type::Struct(ref st) => {
                let id = st.id();
                let type_name = self.formatter.fmt_type_name(id);
                let is_zst = match self.tcx.resolve_type(id) {
                    TypeDef::Struct(def) => def.fields.is_empty(),
                    TypeDef::OutStruct(def) => def.fields.is_empty(),
                    _ => false,
                };
                if is_zst {
                    format!("{type_name}()").into()
                } else {
                    let mut edges = String::new();
                    for lt in st.lifetimes().lifetimes() {
                        let MaybeStatic::NonStatic(lt) = lt else {
                            panic!("'static not supported in Dart")
                        };
                        write!(&mut edges, ", {}Edges", lifetime_env.fmt_lifetime(lt)).unwrap();
                    }

                    format!("{type_name}._fromFfi({var_name}{edges})").into()
                }
            }
            Type::Enum(ref e) if is_contiguous_enum(e.resolve(self.tcx)) => {
                let id = e.tcx_id.into();
                let type_name = self.formatter.fmt_type_name(id);
                format!("{type_name}.values[{var_name}]").into()
            }
            Type::Enum(ref e) => {
                let id = e.tcx_id.into();
                let type_name = self.formatter.fmt_type_name(id);
                format!("{type_name}.values.firstWhere((v) => v._ffi == {var_name})").into()
            }
            Type::Slice(slice) => {
                if let Some(lt) = slice.lifetime() {
                    let MaybeStatic::NonStatic(lifetime) = lt else {
                        panic!("'static not supported in Dart");
                    };
                    format!(
                        "{var_name}._toDart({}Edges)",
                        lifetime_env.fmt_lifetime(lifetime)
                    )
                    .into()
                } else {
                    format!("{var_name}._toDart([])").into()
                }
            }
            Type::DiplomatOption(ref inner) => {
                let conversion = self.gen_c_to_dart_for_type(
                    inner,
                    format!("{var_name}.union.ok").into(),
                    lifetime_env,
                );
                format!("{var_name}.isOk ? {conversion} : null").into()
            }
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
            ReturnType::Infallible(SuccessType::Unit) => None,
            ReturnType::Infallible(SuccessType::Write) => {
                // Note: the `write` variable is initialized in the template
                Some("return write.finalize();".into())
            }
            ReturnType::Infallible(SuccessType::OutType(ref out_ty)) => Some(
                format!(
                    "return {};",
                    self.gen_c_to_dart_for_type(out_ty, "result".into(), lifetime_env)
                )
                .into(),
            ),
            // Special case Result<(), ()> and Option<()> to bool
            ReturnType::Fallible(SuccessType::Unit, None)
            | ReturnType::Nullable(SuccessType::Unit) => Some("return result.isOk;".into()),
            ReturnType::Fallible(ref ok, _) | ReturnType::Nullable(ref ok) => {
                let err_check = format!(
                    "if (!result.isOk) {{\n  {}\n}}\n",
                    match result_ty {
                        ReturnType::Fallible(_, Some(e)) => format!(
                            "throw {};",
                            self.gen_c_to_dart_for_type(e, "result.union.err".into(), lifetime_env)
                        ),
                        _ => "return null;".into(),
                    }
                );

                Some(
                    match ok {
                        // Note: the `write` variable is initialized in the template
                        SuccessType::Write => {
                            format!("{err_check}return write.finalize();")
                        }
                        SuccessType::OutType(o) => {
                            format!(
                                "{err_check}return {};",
                                self.gen_c_to_dart_for_type(
                                    o,
                                    "result.union.ok".into(),
                                    lifetime_env
                                )
                            )
                        }
                        SuccessType::Unit => err_check,
                        _ => unreachable!("unknown AST/HIR variant"),
                    }
                    .into(),
                )
            }
            _ => unreachable!("unknown AST/HIR variant"),
        }
    }

    fn gen_slice_element_ty(&mut self, slice: &hir::Slice) -> Cow<'cx, str> {
        match slice {
            hir::Slice::Str(_, encoding) => {
                self.formatter.fmt_string_element_as_ffi(*encoding).into()
            }
            hir::Slice::Primitive(_, p) => {
                self.gen_type_name_ffi(&Type::<OutputOnly>::Primitive(*p), false)
            }
            hir::Slice::Strs(encoding) => self.gen_type_name_ffi(
                &Type::<OutputOnly>::Slice(hir::Slice::Str(None, *encoding)),
                false,
            ),
            _ => unreachable!("unknown AST/HIR variant"),
        }
    }

    /// Generates a Dart helper class for a slice type.
    fn gen_slice(&mut self, slice: &hir::Slice) -> &'static str {
        let slice_ty = self.formatter.fmt_slice_type(slice);

        if self.helper_classes.contains_key(slice_ty) {
            return slice_ty;
        }

        #[derive(askama::Template)]
        #[template(path = "dart/slice.dart.jinja", escape = "none")]
        struct SliceTemplate<'a> {
            slice_ty: &'a str,
            ffi_element_type: &'a str,
            dart_ty: &'a str,
            to_dart: &'a str,
            owned_free: &'a str,
            borrowed_free: &'a str,
            alloc_in_ident: &'a str,
            from_dart: Vec<Cow<'a, str>>,
        }

        let ffi_element_type = &self.gen_slice_element_ty(slice);

        let dart_ty = match slice {
            hir::Slice::Primitive(_, p) => self.formatter.fmt_primitive_list_type(*p),
            hir::Slice::Str(.., encoding) => self.formatter.fmt_string_type(*encoding),
            hir::Slice::Strs(.., encoding) => self.formatter.fmt_string_list_type(*encoding),
            _ => unreachable!("unknown AST/HIR variant"),
        };

        let to_dart = &match slice {
            hir::Slice::Str(
                _,
                hir::StringEncoding::UnvalidatedUtf8 | hir::StringEncoding::Utf8,
            ) => "Utf8Decoder().convert(_data.asTypedList(_length))",
            hir::Slice::Str(_, hir::StringEncoding::UnvalidatedUtf16) => "core.String.fromCharCodes(_data.asTypedList(_length))",
            // special case: not typed lists for platform-specific integers, so cannot borrow
            hir::Slice::Primitive(_, hir::PrimitiveType::IntSize(_) | hir::PrimitiveType::Bool) => "core.Iterable.generate(_length).map((i) => _data[i]).toList(growable: false)",
            hir::Slice::Primitive(..) => "_data.asTypedList(_length)",
            hir::Slice::Strs(..) => "core.Iterable.generate(_length).map((i) => _data[i]._toDart(lifetimeEdges)).toList(growable: false)",
            _ => unreachable!("unknown AST/HIR variant"),
        };

        let alloc_in_ident = match slice {
            hir::Slice::Primitive(_, p) => self.formatter.fmt_primitive_alloc_in(*p),
            hir::Slice::Str(_, e) => self.formatter.fmt_str_alloc_in(*e),
            hir::Slice::Strs(e) => self.formatter.fmt_str_slice_alloc_in(*e),
            _ => unreachable!("unknown AST/HIR variant"),
        };

        let from_dart: Vec<Cow<str>> = match slice {
            // Strings
            hir::Slice::Str(
                _,
                hir::StringEncoding::UnvalidatedUtf8 | hir::StringEncoding::Utf8,
            ) => vec![
                    "final encoded = Utf8Encoder().convert(this);".into(), 
                    "slice._data = alloc(encoded.length)..asTypedList(encoded.length).setRange(0, encoded.length, encoded);".into(),
                    "slice._length = encoded.length;".into(),
                ],
            hir::Slice::Str(_, hir::StringEncoding::UnvalidatedUtf16) => vec![
                "slice._data = alloc(codeUnits.length)..asTypedList(codeUnits.length).setRange(0, codeUnits.length, codeUnits);".into(),
                "slice._length = length;".into(),
            ],
            // Typed lists
            hir::Slice::Primitive(_, hir::PrimitiveType::Int(hir::IntType::I8 | hir::IntType::I16 | hir::IntType::I32 | hir::IntType::I64) | hir::PrimitiveType::Float(..) | hir::PrimitiveType::Char) => vec![
                "slice._data = alloc(length)..asTypedList(length).setRange(0, length, this);".into(),
                "slice._length = length;".into(),
            ],
            hir::Slice::Primitive(_, hir::PrimitiveType::Int128(_)) => panic!("i128 not supported in Dart"),
            // Manual construction
            _ => vec![
                "slice._data = alloc(length);".into(),
                "for (var i = 0; i < length; i++) {".into(),
                format!(
                    "  slice._data[i] = {};",
                    match slice {
                        hir::Slice::Primitive(_, hir::PrimitiveType::Bool) =>  Cow::Borrowed("this[i]"),
                        hir::Slice::Primitive(_, hir::PrimitiveType::IntSize(hir::IntSizeType::Usize)) => "this[i] < 0 ? 0 : this[i]".into(),
                        hir::Slice::Primitive(_, hir::PrimitiveType::IntSize(hir::IntSizeType::Isize)) => "this[i]".into(),
                        hir::Slice::Primitive(_, hir::PrimitiveType::Int(hir::IntType::U8)) => format!("this[i].clamp(0, {})", u8::MAX).into(),
                        hir::Slice::Primitive(_, hir::PrimitiveType::Int(hir::IntType::U16)) => format!("this[i].clamp(0, {})", u16::MAX).into(),
                        hir::Slice::Primitive(_, hir::PrimitiveType::Int(hir::IntType::U32)) => format!("this[i].clamp(0, {})", u32::MAX).into(),
                        hir::Slice::Primitive(_, hir::PrimitiveType::Int(hir::IntType::U64)) => format!("this[i].clamp(0, {})", u64::MAX).into(),
                        hir::Slice::Strs(e) => {
                            self.gen_slice(&hir::Slice::Str(None, *e));
                            format!("this[i].{}(alloc);", self.formatter.fmt_str_alloc_in(*e)).into()
                        },
                        _ => unreachable!("unknown AST/HIR variant"),
                    }).into(),
                "}".into(),
                "slice._length = length;".into(),
            ],
        };

        let owned_free: Cow<str> = match slice {
        hir::Slice::Str(
            _,
            hir::StringEncoding::UnvalidatedUtf8 | hir::StringEncoding::Utf8,
        ) => "_diplomat_free(_data.cast(), _length, 1);".into(),
        hir::Slice::Str(_, hir::StringEncoding::UnvalidatedUtf16) => "_diplomat_free(_data.cast(), _length * 2, 2);".into(),
        hir::Slice::Primitive(_, hir::PrimitiveType::IntSize(_)) => "_diplomat_free(_data.cast(), _length * ffi.sizeOf<ffi.Size>(), ffi.sizeOf<ffi.Size>());".into(),
        hir::Slice::Primitive(_, p) => {
            let (size, align) = match p {
                hir::PrimitiveType::Bool | hir::PrimitiveType::Byte | hir::PrimitiveType::Char | hir::PrimitiveType::Int(hir::IntType::U8 | hir::IntType::I8) => ("", "1"),
                hir::PrimitiveType::Int(hir::IntType::U16 | hir::IntType::I16) => (" * 2", "2"),
                hir::PrimitiveType::Int(hir::IntType::U32 | hir::IntType::I32) | hir::PrimitiveType::Float(hir::FloatType::F32) => (" * 4", "4"),
                hir::PrimitiveType::Int(hir::IntType::U64 | hir::IntType::I64) | hir::PrimitiveType::Float(hir::FloatType::F64) => (" * 8", "8"),
                hir::PrimitiveType::IntSize(..) => ("* ffi.sizeOf<ffi.Size>()", "ffi.sizeOf<ffi.Size>()"),
                hir::PrimitiveType::Int128(_) => panic!("i128 not supported in Dart"),
            };
            format!("_rustFree.attach(r, (pointer: _data.cast(), bytes: _length{size}, align: {align}));").into()
        }
        hir::Slice::Strs(..) => "// unsupported".into(),
        _ => unreachable!("unknown AST/HIR variant"),
        };

        let borrowed_free = match slice {
            hir::Slice::Primitive(
                _,
                hir::PrimitiveType::Bool
                | hir::PrimitiveType::Char
                | hir::PrimitiveType::Int(..)
                | hir::PrimitiveType::Float(..),
            ) => "_nopFree.attach(r, lifetimeEdges); // Keep lifetime edges alive",
            _ => "// Lifetime edges will be cleaned up",
        };

        self.helper_classes.insert(
            slice_ty.to_string(),
            SliceTemplate {
                slice_ty,
                ffi_element_type,
                dart_ty,
                to_dart,
                owned_free: &owned_free,
                borrowed_free,
                from_dart,
                alloc_in_ident,
            }
            .render()
            .unwrap(),
        );

        slice_ty
    }

    /// Generates a Dart helper class for a result type.
    fn gen_result<P: TyPosition>(
        &mut self,
        ok: Option<&hir::Type<P>>,
        err: Option<&hir::Type<P>>,
    ) -> String {
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

        let ok = ok.filter(|t| {
            let Type::Struct(s) = t else {
                return true;
            };
            match self.tcx.resolve_type(s.id()) {
                TypeDef::Struct(s) => !s.fields.is_empty(),
                TypeDef::OutStruct(s) => !s.fields.is_empty(),
                _ => unreachable!("unknown AST/HIR variant"),
            }
        });

        let err = err.filter(|t| {
            let Type::Struct(s) = t else {
                return true;
            };
            match self.tcx.resolve_type(s.id()) {
                TypeDef::Struct(s) => !s.fields.is_empty(),
                TypeDef::OutStruct(s) => !s.fields.is_empty(),
                _ => unreachable!("unknown AST/HIR variant"),
            }
        });

        let mut gen_decl = |ty: &Type<P>| {
            let annotation = match *ty {
                hir::Type::Primitive(p) => {
                    format!("@{}()", self.formatter.fmt_primitive_as_ffi(p, false))
                }
                hir::Type::Enum(_) => format!("@{}()", self.formatter.fmt_enum_as_ffi(false)),
                _ => String::new(),
            };
            let ty = self.gen_type_name_ffi(ty, true);
            (annotation, ty)
        };

        let ok = ok.map(&mut gen_decl);
        let err = err.map(&mut gen_decl);

        #[derive(askama::Template)]
        #[template(path = "dart/result.dart.jinja", escape = "none")]
        struct ResultTemplate<'a> {
            name: String,
            ok: Option<(String, Cow<'a, str>)>,
            err: Option<(String, Cow<'a, str>)>,
        }

        self.helper_classes.insert(
            name.clone(),
            ResultTemplate {
                name: name.clone(),
                ok,
                err,
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
    /// The ABI name of the method
    abi_name: &'a str,

    // The types for the FFI declaration. The uncast types are the types
    // from the `dart:ffi` package, the cast types are native Dart types.
    param_types_ffi: Vec<Cow<'a, str>>,
    param_types_ffi_cast: Vec<Cow<'a, str>>,
    param_names_ffi: Vec<Cow<'a, str>>,
    return_type_ffi: Cow<'a, str>,
    return_type_ffi_cast: Cow<'a, str>,

    /// All slice parameters conversion code
    arenas: Vec<Cow<'a, str>>,

    /// Conversion code for each parameter
    param_conversions: Vec<Cow<'a, str>>,

    /// If the function has a return value, the Dart code for the conversion. Assumes that
    /// the C function return value is saved to a variable named `result` or that the
    /// `DiplomatWrite`, if present, is saved to a variable named `write`.
    return_expression: Option<Cow<'a, str>>,

    lifetimes: &'a LifetimeEnv,
    /// Maps each (used in the output) method lifetime to a list of parameters
    /// it borrows from. The parameter list may contain the parameter name, or
    /// a spread of a struct's `_fiellsForLifetimeFoo` getter.
    method_lifetimes_map: BTreeMap<Lifetime, BorrowedLifetimeInfo<'a>>,
}

struct FieldInfo<'a, P: TyPosition> {
    name: Cow<'a, str>,
    ty: &'a Type<P>,
    annotation: Option<&'static str>,
    ffi_cast_type_name: Cow<'a, str>,
    dart_type_name: Cow<'a, str>,
    c_to_dart: Cow<'a, str>,
    dart_to_c: Cow<'a, str>,
    /// If this is a struct field that borrows, the borrowing information for that field.
    param_info: Option<StructBorrowInfo<'a>>,
}

// Helpers used in templates (Askama has restrictions on Rust syntax)

fn display_lifetime_edge<'a>(edge: &'a LifetimeEdge) -> Cow<'a, str> {
    let param_name = &edge.param_name;
    match edge.kind {
        // Opaque parameters are just retained as edges
        LifetimeEdgeKind::OpaqueParam => param_name.into(),
        // Slice parameters make an arena which is retained as an edge
        LifetimeEdgeKind::SliceParam => format!("{param_name}Arena").into(),
        // We extract the edge-relevant fields for a borrowed struct lifetime
        LifetimeEdgeKind::StructLifetime(def_env, def_lt, is_option) => {
            let lt = def_env.fmt_lifetime(def_lt).to_uppercase();
            if is_option {
                format!("...?{param_name}?._fieldsForLifetime{lt}").into()
            } else {
                format!("...{param_name}._fieldsForLifetime{lt}").into()
            }
        }
        _ => unreachable!("Unknown lifetime edge kind {:?}", edge.kind),
    }
}

/// Iterate over fields, filtering by fields that actually use lifetimes from `lifetimes`
fn iter_fields_with_lifetimes_from_set<'a, P: TyPosition>(
    fields: &'a [FieldInfo<'a, P>],
    lifetime: &'a Lifetime,
) -> impl Iterator<Item = &'a FieldInfo<'a, P>> + 'a {
    /// Does `ty` use any lifetime from `lifetimes`?
    fn does_type_use_lifetime_from_set<P: TyPosition>(ty: &Type<P>, lifetime: &Lifetime) -> bool {
        ty.lifetimes().any(|lt| {
            let MaybeStatic::NonStatic(lt) = lt else {
                panic!("'static not supported in Dart");
            };
            lt == *lifetime
        })
    }

    fields
        .iter()
        .filter(move |f| does_type_use_lifetime_from_set(f.ty, lifetime))
}

fn iter_def_lifetimes_matching_use_lt<'a>(
    use_lt: &'a Lifetime,
    info: &'a StructBorrowInfo,
) -> impl Iterator<Item = Lifetime> + 'a {
    info.borrowed_struct_lifetime_map
        .iter()
        .filter(|(_def_lt, use_lts)| use_lts.contains(use_lt))
        .map(|(def_lt, _use_lts)| def_lt)
        .copied()
}

/// Context about a struct being borrowed when doing dart-to-c conversions
struct StructBorrowContext<'tcx> {
    /// Is this in a method or struct?
    ///
    /// Methods generate things like `[aEdges, bEdges]`
    /// whereas structs do `[...aAppendArray, ...bAppendArray]`
    is_method: bool,
    use_env: &'tcx LifetimeEnv,
    param_info: StructBorrowInfo<'tcx>,
}

#[derive(Default)]
struct SpecialMethodGenInfo<'a> {
    /// Whether it is a comparator
    comparator: bool,
    /// Whether it is an iterator, and the type it iterates over
    iterator: Option<Cow<'a, str>>,
    /// Whether it is an iterable, and the type it iterates over
    iterable: Option<Cow<'a, str>>,
}
