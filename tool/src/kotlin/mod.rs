use askama::Template;
use diplomat_core::hir::borrowing_param::{
    BorrowedLifetimeInfo, LifetimeEdge, LifetimeEdgeKind, ParamBorrowInfo, StructBorrowInfo,
};
use diplomat_core::hir::{
    self, BackendAttrSupport, Borrow, Callback, DocsUrlGenerator, InputOnly, Lifetime, LifetimeEnv,
    Lifetimes, MaybeOwn, MaybeStatic, Method, Mutability, OpaquePath, Optional, OutType, Param,
    PrimitiveType, ReturnableStructDef, ReturnableStructPath, SelfType, Slice, SpecialMethod,
    StringEncoding, StructPath, StructPathLike, TraitIdGetter, TyPosition, Type, TypeContext,
    TypeDef,
};
use diplomat_core::hir::{ReturnType, SuccessType};

use std::borrow::Cow;
use std::cell::RefCell;
use std::collections::{BTreeMap, BTreeSet};
use std::iter::once;

mod formatter;
use formatter::KotlinFormatter;

use crate::{Config, ErrorStore, FileMap};
use serde::{Deserialize, Serialize};

pub(crate) fn attr_support() -> BackendAttrSupport {
    let mut a = BackendAttrSupport::default();

    a.namespacing = false; // TODO
    a.memory_sharing = false;
    a.non_exhaustive_structs = true;
    a.method_overloading = true;
    a.utf8_strings = false;
    a.utf16_strings = true;
    a.static_slices = true;
    a.option = true;

    a.constructors = false; // TODO
    a.named_constructors = false; // TODO
    a.fallible_constructors = false; // TODO
    a.accessors = false;
    a.static_accessors = false;
    a.stringifiers = true;
    a.comparators = false; // TODO
    a.iterators = true;
    a.iterables = true;
    a.indexing = true;
    a.callbacks = true;
    a.traits = true;
    a.custom_errors = true;
    a.traits_are_send = true;
    a.traits_are_sync = true;
    a.generate_mocking_interface = true;
    a.owned_slices = true;

    a
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct KotlinConfig {
    domain: Option<String>,
    /// An optional override for the dylib name
    /// By default this will look for a dylib named lib{lib-name}.so
    dylib_name: Option<String>,
    use_finalizers_not_cleaners: Option<bool>,
    scaffold: Option<bool>,
}

impl KotlinConfig {
    pub fn set(&mut self, key: &str, value: toml::Value) {
        match key {
            "domain" => {
                if value.is_str() {
                    self.domain = value.as_str().map(|s| s.to_string());
                }
            }
            "use_finalizers_not_cleaners" => {
                self.use_finalizers_not_cleaners = value.as_str().map(|val| val == "true");
            }
            "scaffold" => {
                self.scaffold = value.as_str().map(|val| val == "true");
            }
            "dylib_name" => {
                self.dylib_name = value.as_str().map(|val| val.to_string());
            }
            _ => {}
        }
    }
}

pub(crate) fn run<'tcx>(
    tcx: &'tcx TypeContext,
    conf: Config,
    docs_url_gen: &'tcx DocsUrlGenerator,
) -> (FileMap, ErrorStore<'tcx, String>) {
    let KotlinConfig {
        domain,
        use_finalizers_not_cleaners,
        scaffold,
        dylib_name,
    } = conf.kotlin_config;

    let domain = domain.expect("Failed to parse Kotlin config. Missing required field `domain`.");

    let lib_name = conf
        .shared_config
        .lib_name
        .expect("Failed to parse Kotlin config. Missing required field `lib_name`.");

    let dylib_name = dylib_name.as_deref().unwrap_or(&lib_name);

    let use_finalizers_not_cleaners = use_finalizers_not_cleaners.unwrap_or(false);
    let formatter = KotlinFormatter::new(tcx, None, docs_url_gen);

    let files = FileMap::default();
    let errors = ErrorStore::default();
    let mut callback_params = Vec::new();

    let mut ty_gen_cx = ItemGenContext {
        tcx,
        errors: &errors,
        result_types: RefCell::new(BTreeSet::new()),
        formatter: &formatter,
        callback_params: &mut callback_params,
        lib_name: &lib_name,
        dylib_name,
        domain: &domain,
        use_finalizers_not_cleaners,
    };

    for (_id, ty) in tcx.all_types() {
        ty_gen_cx.callback_params.clear(); // specific to each type in a file
        let _guard = ty_gen_cx.errors.set_context_ty(ty.name().as_str().into());
        if ty.attrs().disable {
            continue;
        }
        match ty {
            TypeDef::Opaque(o) => {
                let type_name = o.name.to_string();

                let (file_name, body) = ty_gen_cx.gen_opaque_def(o, &type_name);

                files.add_file(format!("src/main/kotlin/{file_name}"), body);
            }

            TypeDef::OutStruct(o) => {
                let type_name = o.name.to_string();

                let (file_name, body) = ty_gen_cx.gen_struct_def(o, &type_name);

                files.add_file(format!("src/main/kotlin/{file_name}"), body);
            }

            TypeDef::Struct(struct_def) => {
                let type_name = struct_def.name.to_string();

                let (file_name, body) = ty_gen_cx.gen_struct_def(struct_def, &type_name);

                files.add_file(format!("src/main/kotlin/{file_name}"), body);
            }

            TypeDef::Enum(enum_def) => {
                let type_name = enum_def.name.to_string();

                let (file_name, body) = ty_gen_cx.gen_enum_def(enum_def, &type_name);

                files.add_file(format!("src/main/kotlin/{file_name}"), body);
            }
            ty_def => panic!("Received unknown type definition: {ty_def:?}"),
        }
    }

    for (_id, trt_def) in tcx.all_traits() {
        ty_gen_cx.callback_params.clear(); // specific to each type in a file
        let _guard = ty_gen_cx
            .errors
            .set_context_ty(trt_def.name.as_str().into());
        if trt_def.attrs.disable {
            continue;
        }
        let trait_name = trt_def.name.to_string();

        let (file_name, body) = ty_gen_cx.gen_trait_def(trt_def, &trait_name);

        files.add_file(format!("src/main/kotlin/{file_name}"), body);
    }

    if scaffold.unwrap_or(false) {
        #[derive(Template)]
        #[template(path = "kotlin/build.gradle.kts.jinja", escape = "none")]
        struct Build<'a> {
            domain: &'a str,
            lib_name: &'a str,
        }

        let build = Build {
            domain: &domain,
            lib_name: &lib_name,
        }
        .render()
        .expect("Failed to render build file");

        files.add_file("build.gradle.kts".to_string(), build);

        #[derive(Template)]
        #[template(path = "kotlin/settings.gradle.kts.jinja", escape = "none")]
        struct Settings<'a> {
            lib_name: &'a str,
        }
        let settings = Settings {
            lib_name: &lib_name,
        }
        .render()
        .expect("Failed to render settings file");

        files.add_file("settings.gradle.kts".to_string(), settings);
    }
    let native_results = ty_gen_cx
        .result_types
        .borrow()
        .iter()
        .map(|result_type| result_type.render().expect("failed to render result type"))
        .collect::<Vec<_>>();

    // The map may contain entries that resolve to the same underlying native types
    // In this case, we don't want to generate multiple copies of those types, as
    // that will error. Make sure we're not doing that.

    let mut native_results_found = BTreeSet::new();

    for ty in &*ty_gen_cx.result_types.borrow() {
        let combined = (&ty.ok.type_name, &ty.err.type_name);

        let inserted = native_results_found.insert(combined);
        if !inserted {
            panic!(
                "Found duplicate native Result type for Result<{}, {}> (defaults: ({:?}, {:?}))",
                ty.ok.type_name, ty.err.type_name, ty.ok.default, ty.err.default
            );
        }
    }

    #[derive(Template)]
    #[template(path = "kotlin/init.kt.jinja", escape = "none")]
    struct Init<'a> {
        domain: &'a str,
        native_results: &'a [String],
        lib_name: &'a str,
        dylib_name: &'a str,
        use_finalizers_not_cleaners: bool,
    }

    let init = Init {
        domain: &domain,
        lib_name: &lib_name,
        dylib_name,
        native_results: native_results.as_slice(),
        use_finalizers_not_cleaners,
    }
    .render()
    .expect("Failed to lib top level file");

    files.add_file(
        format!(
            "src/main/kotlin/{}/{lib_name}/Lib.kt",
            domain.replace('.', "/")
        ),
        init,
    );

    (files, errors)
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash)]
struct TypeForResult<'d> {
    type_name: Cow<'d, str>,
    default: Option<Cow<'d, str>>,
}

#[derive(Template, PartialEq, Eq, PartialOrd, Ord, Clone, Hash, Debug)]
#[template(path = "kotlin/Result.kt.jinja")]
struct NativeResult<'d> {
    ok: TypeForResult<'d>,
    err: TypeForResult<'d>,
}

struct ItemGenContext<'a, 'cx> {
    tcx: &'cx TypeContext,
    lib_name: &'a str,
    dylib_name: &'a str,
    domain: &'a str,
    formatter: &'a KotlinFormatter<'cx>,
    result_types: RefCell<BTreeSet<NativeResult<'cx>>>,
    errors: &'a ErrorStore<'cx, String>,
    callback_params: &'a mut Vec<CallbackParamInfo>,
    use_finalizers_not_cleaners: bool,
}

/// Format a `val aEdges = mutableListOf(..)` edges array initializer
fn display_lifetime_edge<'a>(edge: &'a LifetimeEdge) -> Option<Cow<'a, str>> {
    let param_name = &edge.param_name;
    match edge.kind {
        // Opaque parameters are just retained as edges
        LifetimeEdgeKind::OpaqueParam => Some(param_name.into()),
        // Slice parameters make an arena which is retained as an edge
        LifetimeEdgeKind::SliceParam => Some(format!("{param_name}TODO").into()),
        // This is handled via append arrays
        LifetimeEdgeKind::StructLifetime(..) => None,
        _ => unreachable!("Unknown lifetime edge kind {:?}", edge.kind),
    }
}

/// Context about a struct being borrowed when doing kotlin-to-c conversions
struct StructBorrowContext<'tcx> {
    use_env: &'tcx LifetimeEnv,
    param_info: StructBorrowInfo<'tcx>,
}

impl<'cx> ItemGenContext<'_, 'cx> {
    fn gen_infallible_return_type_name(&self, success_type: &SuccessType) -> Cow<'cx, str> {
        match success_type {
            SuccessType::Unit => self.formatter.fmt_void().into(),
            SuccessType::Write => self.formatter.fmt_string().into(),
            SuccessType::OutType(ref o) => self.gen_type_name(o, None),
            _ => panic!("Unsupported success type"),
        }
    }
    fn gen_return_type_name(&self, result_ty: &ReturnType) -> Cow<'cx, str> {
        match *result_ty {
            ReturnType::Infallible(ref success) => self.gen_infallible_return_type_name(success),
            ReturnType::Fallible(ref ok, _) => {
                let ok_type = self.gen_infallible_return_type_name(ok);
                format!("Result<{ok_type}>").into()
            }
            ReturnType::Nullable(ref success) => self
                .formatter
                .fmt_nullable(self.gen_infallible_return_type_name(success).as_ref())
                .into(),
        }
    }

    fn gen_kt_to_c_for_struct(
        &self,
        s: &StructPath,
        name: Cow<str>,
        struct_borrow_info: Option<StructBorrowContext<'cx>>,
        mut needs_temporary: Option<&mut bool>,
    ) -> String {
        use std::fmt::Write;
        let struct_def = s.resolve(self.tcx);
        let mut params = String::new();
        let struct_borrow_info = struct_borrow_info.as_ref();
        if struct_def.lifetimes.num_lifetimes() != 0 {
            let mut maybe_comma_outer = "";
            for def_lt in struct_def.lifetimes.all_lifetimes() {
                write!(
                    &mut params,
                    "{maybe_comma_outer}{}AppendArray = ",
                    struct_def.lifetimes.fmt_lifetime(def_lt)
                )
                .unwrap();

                // Check if this lifetime
                if let Some(use_lts) = struct_borrow_info
                    .and_then(|i| i.param_info.borrowed_struct_lifetime_map.get(&def_lt))
                {
                    // Optimization: don't generate arrayOf(*fooAppendArray) when you can just
                    // directly use fooAppendArray
                    if needs_temporary.is_none() && use_lts.len() == 1 {
                        let lt = struct_borrow_info
                            .unwrap()
                            .use_env
                            .fmt_lifetime(use_lts.iter().next().unwrap());
                        write!(&mut params, "{lt}AppendArray",).unwrap();
                    } else {
                        write!(&mut params, "arrayOf(").unwrap();
                        let mut maybe_comma = "";
                        for use_lt in use_lts {
                            // Generate stuff like `, aEdges` or for struct fields, `, *aAppendArray`
                            let lt = struct_borrow_info.unwrap().use_env.fmt_lifetime(use_lt);
                            // Params use edges, structs use append arrays
                            if needs_temporary.is_some() {
                                write!(&mut params, "{maybe_comma}{lt}Edges").unwrap();
                            } else {
                                write!(&mut params, "{maybe_comma}*{lt}AppendArray").unwrap();
                            }
                            maybe_comma = ", ";
                        }
                        write!(&mut params, ")").unwrap();
                    }
                } else {
                    if let Some(ref mut needs_temporary) = needs_temporary {
                        **needs_temporary = true;
                    } else {
                        panic!("Struct borrow info MUST reference all lifetimes");
                    }
                    write!(&mut params, "arrayOf(temporaryEdgeArena)").unwrap();
                }

                maybe_comma_outer = ", ";
            }
        }
        format!("{name}.toNative({params})")
    }

    /// needs_temporary should be set to an outparam boolean ONLY in the parameter case,
    /// and will get set to true if thise needs a temporary arena.
    ///
    /// Booleans are represented differently at the native layer in struct fields and params
    /// so the presence of needs_temporary is also used to tell that.
    fn gen_kt_to_c_for_type<P: TyPosition<OpaqueOwnership = Borrow, StructPath = StructPath>>(
        &self,
        ty: &Type<P>,
        name: Cow<str>,
        struct_borrow_info: Option<StructBorrowContext<'cx>>,
        needs_temporary: Option<&mut bool>,
    ) -> Cow<'cx, str> {
        let is_param = needs_temporary.is_some();
        match *ty {
            Type::Primitive(prim) => self
                .formatter
                .fmt_primitive_to_native_conversion(name.as_ref(), prim, is_param)
                .into(),
            Type::Opaque(ref op @ OpaquePath { owner, .. }) => {
                let optional = if op.is_optional() { "?" } else { "" };
                match owner.mutability {
                    Mutability::Immutable => format!("{name}{optional}.handle").into(),
                    Mutability::Mutable => format!("{name}{optional}.handle /* note this is a mutable reference. Think carefully about using, especially concurrently */" ).into(),
                }
            }
            Type::Struct(ref s) => self
                .gen_kt_to_c_for_struct(s, name, struct_borrow_info, needs_temporary)
                .into(),
            Type::ImplTrait(ref trt) => {
                let trait_id = trt.id();
                let resolved = self.tcx.resolve_trait(trait_id);
                let trait_name = resolved.name.to_string();
                format!("DiplomatTrait_{trait_name}_Wrapper.fromTraitObj({name}).nativeStruct")
                    .into()
            }
            Type::Enum(_) => format!("{name}.toNative()").into(),
            Type::Slice(ref s) => {
                if is_param {
                    format!("{name}SliceMemory.slice").into()
                } else {
                    // TODO(#1003) this is incorrect, since it won't handle the borrow (the Memory object is discarded)
                    let slice_method = self.slice_method_for(s);
                    format!("PrimitiveArrayTools.{slice_method}({name}).slice").into()
                }
            }
            Type::Callback(_) => {
                let real_param_name = name[name.rfind('_').unwrap() + 1..].to_string(); // past last _
                format!("{name}.fromCallback({real_param_name}).nativeStruct").into()
            }
            Type::DiplomatOption(ref inner) => {
                // We pass false for is_params here the type is a struct field
                let inner_expr = self.gen_kt_to_c_for_type(
                    inner,
                    "it".into(),
                    struct_borrow_info,
                    needs_temporary,
                );
                let ffi_option = format!(
                    "Option{}",
                    self.formatter.fmt_struct_field_type_native(inner)
                );
                format!("{name}?.let {{ {ffi_option}.some({inner_expr}) }} ?: {ffi_option}.none()")
                    .into()
            }
            _ => todo!(),
        }
    }

    fn gen_infallible_return_type_ffi(&self, success: &SuccessType) -> Cow<'cx, str> {
        match success {
            SuccessType::Unit => self.formatter.fmt_void().into(),
            SuccessType::Write => self.formatter.fmt_void().into(),
            SuccessType::OutType(ref o) => self.gen_type_name_ffi(o, None),
            _ => panic!("Unsupported success type"),
        }
    }

    fn gen_return_type_name_ffi(&self, out: &ReturnType) -> Cow<'cx, str> {
        match *out {
            ReturnType::Infallible(ref s) => self.gen_infallible_return_type_ffi(s),
            ReturnType::Fallible(ref ok, ref err) => {
                let ok_type = self.gen_infallible_return_type_ffi(ok);
                let err_type = err
                    .as_ref()
                    .map(|err| self.gen_type_name_ffi(err, None))
                    .unwrap_or_else(|| "Unit".into());

                let ok_default = match ok {
                    SuccessType::OutType(ref o) => Some(o)
                        .filter(|t| {
                            let Type::Struct(s) = t else {
                                return true;
                            };
                            match s.resolve(self.tcx) {
                                ReturnableStructDef::Struct(s) => !s.fields.is_empty(),
                                ReturnableStructDef::OutStruct(s) => !s.fields.is_empty(),
                                _ => unreachable!("unknown AST/HIR variant"),
                            }
                        })
                        .map(|t| self.formatter.fmt_field_default(t, true)),
                    _ => None,
                };
                let err_default = err
                    .as_ref()
                    .filter(|t| {
                        let Type::Struct(s) = t else {
                            return true;
                        };
                        match s.resolve(self.tcx) {
                            ReturnableStructDef::Struct(s) => !s.fields.is_empty(),
                            ReturnableStructDef::OutStruct(s) => !s.fields.is_empty(),
                            _ => unreachable!("unknown AST/HIR variant"),
                        }
                    })
                    .map(|t| self.formatter.fmt_field_default(t, true));
                let result_type = NativeResult {
                    ok: TypeForResult {
                        type_name: ok_type.clone(),
                        default: ok_default,
                    },
                    err: TypeForResult {
                        type_name: err_type.clone(),
                        default: err_default,
                    },
                };
                let mut result_types = self.result_types.borrow_mut();
                result_types.insert(result_type);

                format!("Result{ok_type}{err_type}").into()
            }
            ReturnType::Nullable(SuccessType::Unit | SuccessType::Write) => "OptionUnit".into(),
            ReturnType::Nullable(
                ref success @ SuccessType::OutType(
                    Type::Struct(..) | Type::Enum(..) | Type::Primitive(..),
                ),
            ) => {
                let infallible_return = self.gen_infallible_return_type_ffi(success);
                format!("Option{infallible_return}").into()
            }
            ReturnType::Nullable(SuccessType::OutType(Type::Opaque(..))) => "Pointer?".into(),
            ReturnType::Nullable(SuccessType::OutType(Type::Slice(..))) => "OptionSlice".into(),
            _ => panic!("unsupported return type"),
        }
    }

    fn gen_type_name_ffi<P: TyPosition>(
        &self,
        ty: &Type<P>,
        additional_name: Option<String>,
    ) -> Cow<'cx, str> {
        match *ty {
            Type::Primitive(prim) => self.formatter.fmt_primitive_type_native(prim).into(),
            Type::Opaque(ref op) => {
                let optional = if op.is_optional() { "?" } else { "" };
                format!("Pointer{optional}").into()
            }

            Type::Struct(ref strct) => {
                let type_id = strct.id();
                let resolved = self.tcx.resolve_type(type_id);
                format!("{}Native", resolved.name()).into()
            }
            Type::Enum(_) => "Int".into(),
            Type::Slice(_) => "Slice".into(),
            Type::Callback(_) => {
                format!("DiplomatCallback_{}_Native", additional_name.unwrap()).into()
            }
            Type::ImplTrait(ref trt) => {
                let trait_id = trt.id();
                let resolved = self.tcx.resolve_trait(trait_id);
                format!("DiplomatTrait_{}_Wrapper_Native", resolved.name).into()
            }

            Type::DiplomatOption(ref inner) => {
                assert!(additional_name.is_none());
                format!("Option{}", self.gen_type_name_ffi(inner, None)).into()
            }
            _ => unreachable!("unknown AST/HIR variant"),
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn gen_opaque_return_conversion<'d>(
        &'d self,
        opaque_path: &'d OpaquePath<Optional, MaybeOwn>,
        method_lifetimes_map: &'d MethodLtMap<'d>,
        lifetime_env: &'d LifetimeEnv,
        cleanups: &[Cow<'d, str>],
        val_name: &'d str,
        return_type_modifier: &str,
    ) -> String {
        let opaque_def = opaque_path.resolve(self.tcx);

        let ownership = opaque_path.owner;
        let lifetimes = &opaque_path.lifetimes;
        let optional = opaque_path.is_optional();
        #[derive(Template)]
        #[template(path = "kotlin/OpaqueReturn.kt.jinja", escape = "none")]
        struct OpaqueReturn<'a, 'b> {
            return_type_name: Cow<'b, str>,
            named_lifetimes: Vec<Cow<'b, str>>,
            is_owned: bool,
            self_edges: Vec<Cow<'b, str>>,
            cleanups: &'a [Cow<'b, str>],
            optional: bool,
            val_name: &'a str,
            return_type_modifier: &'a str,
            use_finalizers_not_cleaners: bool,
        }

        let return_type_name = opaque_def.name.to_string().into();
        let self_edges = || match ownership {
            MaybeOwn::Borrow(Borrow {
                lifetime: MaybeStatic::NonStatic(lt),
                ..
            }) => Some(
                method_lifetimes_map
                    .get(&lt)
                    .iter()
                    .flat_map(|param| param.incoming_edges.iter())
                    .map(move |edge| self.formatter.fmt_borrow(edge))
                    .collect(),
            ),
            _ => None,
        };

        let self_edges = self_edges();
        let is_owned = self_edges.is_none();
        let self_edges = self_edges.unwrap_or_else(Vec::new);

        let named_lifetimes = lifetimes
            .lifetimes()
            .filter_map(|lt| match lt {
                MaybeStatic::Static => None,
                MaybeStatic::NonStatic(lt) => Some(lifetime_env.fmt_lifetime(lt)),
            })
            .collect::<Vec<_>>();

        let opaque_return = OpaqueReturn {
            return_type_name,
            named_lifetimes,
            is_owned,
            self_edges,
            cleanups,
            optional,
            val_name,
            return_type_modifier,
            use_finalizers_not_cleaners: self.use_finalizers_not_cleaners,
        };
        opaque_return
            .render()
            .expect("Failed to render opaque return block")
    }

    fn write_return(return_type_modifier: &str) -> String {
        format!(
            r#"
val returnString = DW.writeToString(write)
return returnString{return_type_modifier}"#
        )
    }

    fn boxed_slice_return(encoding: &str, val_name: &str, return_type_modifier: &str) -> String {
        format!(
            r#"val string = PrimitiveArrayTools.get{encoding}({val_name})
Native.free(Pointer.nativeValue({val_name}.data))
return string{return_type_modifier}"#
        )
    }

    fn gen_slice_return_conversion<'d, P: TyPosition>(
        &'d self,
        slice_ty: &'d Slice<P>,
        val_name: &'d str,
        return_type_modifier: &str,
    ) -> String {
        match slice_ty {
            Slice::Str(Some(_), enc) => match enc {
                StringEncoding::UnvalidatedUtf16 => {
                    format!("    return PrimitiveArrayTools.getUtf16({val_name})")
                }
                StringEncoding::UnvalidatedUtf8 => {
                    format!("    return PrimitiveArrayTools.getUtf8({val_name})")
                }
                StringEncoding::Utf8 => {
                    format!("    return PrimitiveArrayTools.getUtf8({val_name})")
                }
                _ => todo!(),
            },
            Slice::Str(None, enc) => match enc {
                StringEncoding::UnvalidatedUtf16 => {
                    Self::boxed_slice_return("Utf16", val_name, return_type_modifier)
                }
                StringEncoding::UnvalidatedUtf8 => {
                    Self::boxed_slice_return("Utf8", val_name, return_type_modifier)
                }
                StringEncoding::Utf8 => {
                    Self::boxed_slice_return("Utf8", val_name, return_type_modifier)
                }
                _ => todo!(),
            },
            Slice::Primitive(MaybeOwn::Borrow(_), prim_ty) => {
                let prim_ty = self.formatter.fmt_primitive_as_kt(*prim_ty);
                format!("    return PrimitiveArrayTools.get{prim_ty}Array({val_name}){return_type_modifier}")
            }
            Slice::Primitive(MaybeOwn::Own, prim_ty) => {
                let prim_ty = self.formatter.fmt_primitive_as_kt(*prim_ty);
                let prim_ty_array = format!("{prim_ty}Array");
                Self::boxed_slice_return(prim_ty_array.as_str(), val_name, return_type_modifier)
            }

            _ => todo!(),
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn gen_struct_return_conversion<'d>(
        &'d self,
        struct_def: &'d ReturnableStructDef,
        lifetimes: &'d Lifetimes,
        lifetime_env: &'d LifetimeEnv,
        cleanups: &[Cow<'d, str>],
        val_name: &'d str,
        return_type_modifier: &str,
    ) -> String {
        let is_zst = match struct_def {
            ReturnableStructDef::Struct(s) => s.fields.is_empty(),
            ReturnableStructDef::OutStruct(s) => s.fields.is_empty(),
            _ => false,
        };

        let return_type_name = match struct_def {
            ReturnableStructDef::Struct(strct) => strct.name.to_string().into(),
            ReturnableStructDef::OutStruct(out_strct) => out_strct.name.to_string().into(),
            _ => todo!(),
        };

        if is_zst {
            return format!("return {return_type_name}(){return_type_modifier}");
        }

        let named_lifetimes = lifetimes
            .lifetimes()
            .filter_map(|lt| match lt {
                MaybeStatic::Static => None,
                MaybeStatic::NonStatic(lt) => Some(lifetime_env.fmt_lifetime(lt)),
            })
            .collect::<Vec<_>>();

        #[derive(Template)]
        #[template(path = "kotlin/StructReturn.kt.jinja", escape = "none")]
        struct StructReturn<'a, 'b> {
            return_type_name: Cow<'b, str>,
            named_lifetimes: Vec<Cow<'b, str>>,
            cleanups: &'a [Cow<'b, str>],
            val_name: &'a str,
            return_type_modifier: &'a str,
        }
        StructReturn {
            return_type_name,
            named_lifetimes,
            cleanups,
            val_name,
            return_type_modifier,
        }
        .render()
        .expect("Failed to render opaque return block")
    }

    #[allow(clippy::too_many_arguments)]
    fn gen_out_type_return_conversion<'d>(
        &'d self,
        method: &'d Method,
        method_lifetimes_map: &'d MethodLtMap<'d>,
        cleanups: &[Cow<'d, str>],
        val_name: &'d str,
        return_type_modifier: &'d str,
        err_cast: &'d str,
        o: &'d OutType,
    ) -> String {
        match o {
            Type::Primitive(prim) => {
                let maybe_unsized_modifier = self.formatter.fmt_unsized_conversion(*prim, false);
                format!(
                    "return {err_cast}({val_name}{maybe_unsized_modifier}){return_type_modifier}"
                )
            }
            Type::Opaque(opaque_path) => self.gen_opaque_return_conversion(
                opaque_path,
                method_lifetimes_map,
                &method.lifetime_env,
                cleanups,
                val_name,
                return_type_modifier,
            ),
            Type::Struct(strct) => {
                let lifetimes = strct.lifetimes();
                self.gen_struct_return_conversion(
                    &strct.resolve(self.tcx),
                    lifetimes,
                    &method.lifetime_env,
                    cleanups,
                    val_name,
                    return_type_modifier,
                )
            }
            Type::Enum(enm) => {
                let return_type = enm.resolve(self.tcx);
                format!(
                    "return {err_cast}({}.fromNative({val_name})){return_type_modifier}",
                    return_type.name
                )
            }
            Type::Slice(slc) => {
                // Slices do NOT  need to worry about borrows when being returned since they're just copied
                self.gen_slice_return_conversion(slc, val_name, return_type_modifier)
            }
            _ => todo!(),
        }
    }

    fn gen_nullable_return_conversion<'d>(
        &'d self,
        method: &'d Method,
        method_lifetimes_map: &'d MethodLtMap<'d>,
        cleanups: &[Cow<'d, str>],
        val_name: &'d str,
        o: &'d OutType,
    ) -> String {
        match o {
            Type::Primitive(prim) => {
                let maybe_unsized_modifier = self.formatter.fmt_unsized_conversion(*prim, true);
                format!("return {val_name}.option(){maybe_unsized_modifier}")
            }
            Type::Opaque(opaque_path) => self.gen_opaque_return_conversion(
                opaque_path,
                method_lifetimes_map,
                &method.lifetime_env,
                cleanups,
                val_name,
                ".?",
            ),
            Type::Struct(strct) => {
                let lifetimes = strct.lifetimes();
                format!(
                    r#"
val intermediateOption = {val_name}.option() ?: return null
{}
                        "#,
                    self.gen_struct_return_conversion(
                        &strct.resolve(self.tcx),
                        lifetimes,
                        &method.lifetime_env,
                        cleanups,
                        "intermediateOption",
                        "",
                    )
                )
            }
            Type::Enum(enm) => {
                let return_type = enm.resolve(self.tcx);
                format!(
                    r#"
val intermediateOption = {val_name}.option() ?: return null
return {}.fromNative(intermediateOption)"#,
                    return_type.name
                )
            }
            Type::Slice(slc) => {
                format!(
                    r#"
val intermediateOption = {val_name}.option() ?: return null
{}
                        "#,
                    self.gen_slice_return_conversion(slc, "intermediateOption", "")
                )
            }
            _ => todo!(),
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn gen_success_return_conversion<'d>(
        &'d self,
        res: &'d SuccessType,
        method: &'d Method,
        method_lifetimes_map: &'d MethodLtMap<'d>,
        cleanups: &[Cow<'d, str>],
        val_name: &'d str,
        return_type_postfix: &str,
    ) -> String {
        match res {
            SuccessType::Write => Self::write_return(return_type_postfix),
            SuccessType::OutType(ref o) => self.gen_out_type_return_conversion(
                method,
                method_lifetimes_map,
                cleanups,
                val_name,
                return_type_postfix,
                "", // error cast
                o,
            ),
            SuccessType::Unit if return_type_postfix.is_empty() => "".into(),
            SuccessType::Unit => format!("return Unit{return_type_postfix}"),
            _ => todo!(),
        }
    }

    fn gen_return_conversion<'d>(
        &'d self,
        method: &'d Method,
        method_lifetimes_map: &MethodLtMap<'d>,
        cleanups: &[Cow<'d, str>],
    ) -> String {
        match &method.output {
            ReturnType::Infallible(res) => self.gen_success_return_conversion(
                res,
                method,
                method_lifetimes_map,
                cleanups,
                "returnVal",
                "",
            ),
            ReturnType::Fallible(ok, err) => {
                let ok_path = self.gen_success_return_conversion(
                    ok,
                    method,
                    method_lifetimes_map,
                    cleanups,
                    "returnVal.union.ok",
                    ".ok()",
                );

                let err_path = err
                    .as_ref()
                    .map(|err| {
                        match err {
                            OutType::Opaque(OpaquePath{tcx_id: id, ..}) => {
                                let resolved = self.tcx.resolve_opaque(*id);
                                if !resolved.attrs.custom_errors {
                                    panic!("Opaque type {:?} must have the `error` attribute to be used as an error result", resolved.name);
                                }
                            },
                            OutType::Struct(ReturnableStructPath::Struct(path)) => {
                                let resolved = self.tcx.resolve_struct(path.tcx_id);
                                if !resolved.attrs.custom_errors {
                                    panic!("Struct type {:?} must have the `error` attribute to be used as an error result", resolved.name);
                                }
                            },
                            OutType::Struct(ReturnableStructPath::OutStruct(path)) => {
                                let resolved = self.tcx.resolve_out_struct(path.tcx_id);
                                if !resolved.attrs.custom_errors {
                                    panic!("Struct type {:?} must have the `error` attribute to be used as an error result", resolved.name);
                                }
                            }
                            Type::Enum(enm) => {
                                let resolved = enm.resolve(self.tcx);
                                    if !resolved.attrs.custom_errors {
                                        panic!("Struct type {:?} must have the `error` attribute to be used as an error result", resolved.name);
                                    }
                            }
                            _ => {}
                        }
                        let err_converter = ".err()";
                        let err_cast = if let Type::Primitive(prim) = err {
                            self.formatter.fmt_primitive_error_type(*prim)
                        } else if let Type::Enum(enm) = err {
                            let return_type = enm.resolve(self.tcx);
                            (return_type.name.to_string() + "Error").into()
                        } else {
                            "".into()
                        };

                        self.gen_out_type_return_conversion(
                            method,
                            method_lifetimes_map,
                            cleanups,
                            "returnVal.union.err",
                            err_converter,
                            &err_cast,
                            err,
                        )
                    })
                    .unwrap_or_else(|| "return UnitError().err()".into());

                #[derive(Template)]
                #[template(path = "kotlin/ResultReturn.kt.jinja", escape = "none")]
                struct ResultReturn<'d> {
                    ok_path: &'d str,
                    err_path: &'d str,
                }
                ResultReturn {
                    ok_path: ok_path.as_str(),
                    err_path: err_path.as_str(),
                }
                .render()
                .expect("Failed to render result return")
            }
            ReturnType::Nullable(SuccessType::OutType(ref res)) => self
                .gen_nullable_return_conversion(
                    method,
                    method_lifetimes_map,
                    cleanups,
                    "returnVal",
                    res,
                ),

            ReturnType::Nullable(SuccessType::Write) => format!(
                r#"
returnVal.option() ?: return null
{}
                        "#,
                Self::write_return("")
            ),
            ReturnType::Nullable(SuccessType::Unit) => "return returnVal.option()".into(),
            _ => panic!("unsupported type"),
        }
    }

    fn slice_method_for<P: TyPosition>(&self, slice_type: &Slice<P>) -> &'static str {
        match slice_type {
            Slice::Str(Some(_), StringEncoding::UnvalidatedUtf16) => "borrowUtf16",
            Slice::Str(None, StringEncoding::UnvalidatedUtf16) => "moveUtf16",
            Slice::Str(Some(_), _) => "borrowUtf8",
            Slice::Str(None, _) => "moveUtf8",
            Slice::Primitive(MaybeOwn::Borrow(_), _) => "borrow",
            Slice::Primitive(_, _) => "move",
            Slice::Strs(StringEncoding::UnvalidatedUtf16) => "borrowUtf16s",
            Slice::Strs(_) => "borrowUtf8s",
            _ => {
                self.errors
                    .push_error("Found unsupported slice type".into());
                ""
            }
        }
    }

    fn gen_slice_conversion<P: TyPosition>(
        &self,
        kt_param_name: Cow<'cx, str>,
        slice_type: Slice<P>,
    ) -> Cow<'cx, str> {
        #[derive(Template)]
        #[template(path = "kotlin/SliceConversion.kt.jinja", escape = "none")]
        struct SliceConv<'d> {
            slice_method: Cow<'d, str>,
            kt_param_name: Cow<'d, str>,
        }
        let slice_method = self.slice_method_for(&slice_type).into();

        SliceConv {
            kt_param_name,
            slice_method,
        }
        .render()
        .expect("Failed to render slice method")
        .into()
    }

    fn gen_cleanup<P: TyPosition>(
        &self,
        param_name: Cow<'cx, str>,
        slice: Slice<P>,
    ) -> Option<Cow<'cx, str>> {
        // TODO(#1003) Is this actually needed?
        match slice {
            Slice::Str(Some(_), _) => Some(format!("{param_name}SliceMemory?.close()").into()),
            Slice::Str(_, _) => None,
            Slice::Primitive(MaybeOwn::Borrow(_), _) => {
                Some(format!("{param_name}SliceMemory?.close()").into())
            }
            Slice::Primitive(_, _) => None,
            Slice::Strs(_) => Some(format!("{param_name}SliceMemory?.close()").into()),
            _ => todo!(),
        }
    }

    fn gen_method(
        &mut self,
        special_methods: &mut SpecialMethods,
        method: &'cx hir::Method,
        self_type: Option<&'cx SelfType>,
        struct_name: Option<&str>,
        add_override_specifier_for_opaque_self_methods: bool,
    ) -> MethodInfo {
        let _guard = self.errors.set_context_method(method.name.as_str().into());
        if method.attrs.disable {
            return MethodInfo::default();
        }

        let mut visitor = method.borrowing_param_visitor(self.tcx, false);
        let native_method_name = method.abi_name.as_str();

        let mut param_decls_kt = Vec::with_capacity(method.params.len());
        let mut param_types_ffi = Vec::with_capacity(method.params.len());
        let mut param_conversions = Vec::with_capacity(method.params.len());
        let mut slice_conversions = Vec::with_capacity(method.params.len());
        let mut cleanups = Vec::with_capacity(method.params.len());

        let mut needs_temporary = false;
        match self_type {
            Some(st @ SelfType::Opaque(_)) => {
                let param_type = "Pointer".into();
                let param_name: Cow<'_, str> = "handle".into();
                visitor.visit_param(&st.clone().into(), "this");

                param_types_ffi.push(param_type);
                param_conversions.push(param_name.clone());
            }
            Some(st @ SelfType::Struct(s)) => {
                let param_type =
                    format!("{}Native", self.tcx.resolve_struct(s.tcx_id).name.as_str()).into();
                let param_borrow_kind = visitor.visit_param(&st.clone().into(), "this");
                let struct_borrow_info =
                    if let ParamBorrowInfo::Struct(param_info) = param_borrow_kind {
                        Some(StructBorrowContext {
                            use_env: &method.lifetime_env,
                            param_info,
                        })
                    } else {
                        None
                    };
                param_types_ffi.push(param_type);
                param_conversions.push(
                    self.gen_kt_to_c_for_struct(
                        s,
                        "this".into(),
                        struct_borrow_info,
                        Some(&mut needs_temporary),
                    )
                    .into(),
                );
            }
            Some(SelfType::Enum(_)) => {
                let param_type = "Int".into();
                let param_conversion: Cow<'_, str> = "this.toNative()".into();
                param_types_ffi.push(param_type);
                param_conversions.push(param_conversion.clone());
            }
            None => (),
            _ => todo!(),
        };
        for param in method.params.iter() {
            let param_name = self.formatter.fmt_param_name(param.name.as_str());
            let mut additional_name = None;

            let param_borrow_kind = visitor.visit_param(&param.ty, &param_name);

            match &param.ty {
                Type::Slice(slice) => {
                    slice_conversions
                        .push(self.gen_slice_conversion(param_name.clone(), slice.clone()));

                    match param_borrow_kind {
                        ParamBorrowInfo::Struct(_) => (),
                        ParamBorrowInfo::TemporarySlice => {
                            if let Some(cleanup) =
                                self.gen_cleanup(param_name.clone(), slice.clone())
                            {
                                cleanups.push(cleanup)
                            }
                        }
                        ParamBorrowInfo::BorrowedSlice => self.errors.push_error("Kotlin backend does not support borrowing slices across functions (#1003)".into()),
                        ParamBorrowInfo::BorrowedOpaque => (),
                        ParamBorrowInfo::NotBorrowed => (),
                        _ => todo!(),
                    };
                }
                Type::Callback(Callback {
                    param_self: _,
                    params,
                    output,
                    ..
                }) => {
                    let param_name = "diplomatCallback_".to_owned() + &param_name;
                    additional_name = Some(
                        struct_name.unwrap().to_owned()
                            + "_"
                            + method.name.as_ref()
                            + "_"
                            + &param_name,
                    );
                    let param_input_types: Vec<String> = params
                        .iter()
                        .map(|param| self.gen_type_name(&param.ty, None).into())
                        .collect();
                    let param_names: Vec<String> = params
                        .iter()
                        .enumerate()
                        .map(|(index, _)| format!("arg{index}"))
                        .collect();
                    let (native_input_names, native_input_params_and_types): (
                        Vec<String>,
                        Vec<String>,
                    ) = params
                        .iter()
                        .zip(param_input_types.iter())
                        .zip(param_names.iter())
                        .map(|((in_param, in_ty), in_name)| match in_param.ty {
                            Type::Struct(_) | Type::Enum(_) => {
                                if let Type::Struct(ref s) = in_param.ty {
                                    assert!(
                                        s.lifetimes().lifetimes().len() == 0,
                                        "Code did not expect structs with lifetimes"
                                    );
                                }
                                // named types have a _Native wrapper, this needs to be passed as the "native"
                                // version of the argument
                                (
                                    format!("{in_ty}.fromNative({in_name})"),
                                    format!("{in_name}: {in_ty}Native"),
                                )
                            }
                            Type::Slice(Slice::Primitive(_, _)) => {
                                // slices need to be passed as Slice type
                                // and only primitive slices are allowed
                                (
                                    format!("PrimitiveArrayTools.get{in_ty}({in_name})"),
                                    format!("{in_name}: Slice"),
                                )
                            }
                            Type::Slice(_) => {
                                panic!("Non-primitive slices are not allowed as callback args")
                            }
                            Type::Opaque(_) => (
                                format!("{in_ty}({in_name}, listOf())"),
                                format!("{in_name}: Pointer"),
                            ),
                            _ => (in_name.clone(), format!("{in_name}: {in_ty}")),
                        })
                        .unzip();
                    let (native_output_type, return_modification) = match &**output {
                        ReturnType::Infallible(success) => match success {
                            SuccessType::OutType(ty) => (
                                self.gen_native_type_name(ty, None).into(),
                                match ty {
                                    Type::Enum(..) | Type::Struct(..) => ".toNative()",
                                    _ => "",
                                }
                                .into(),
                            ),
                            SuccessType::Unit => ("Unit".into(), "".into()),
                            _ => panic!("Unsupported success type {success:?}"),
                        },
                        _ => panic!("Unsupported return type {output:?}. Results and Options are not supported."),
                    };

                    self.callback_params.push(CallbackParamInfo {
                        name: "DiplomatCallback_".to_owned() + &additional_name.clone().unwrap(),
                        input_types: param_input_types.join(", "),
                        output_type: match &**output {
                            ReturnType::Infallible(success) => match success {
                                SuccessType::OutType(ty) => self.gen_type_name(ty, None).into(),
                                SuccessType::Unit => "Unit".into(),
                                _ => panic!("Unsupported success type {success:?}"),
                            },
                            _ => panic!("Unsupported return type {output:?}. Results and Options are not supported."),
                        },
                        native_input_params_and_types: native_input_params_and_types.join(", "),
                        native_input_names: native_input_names.join(", "),
                        native_output_type,
                        return_modification,
                    })
                }
                _ => (),
            }
            let param_type_ffi = self.gen_type_name_ffi(&param.ty, additional_name.clone());
            param_decls_kt.push(format!(
                "{param_name}: {}",
                self.gen_non_wrapped_type_name(&param.ty, additional_name.clone())
            ));
            let param_name_to_pass = if let Type::Callback(_) = &param.ty {
                self.gen_type_name(&param.ty, additional_name)
            } else {
                param_name.clone()
            };
            param_types_ffi.push(param_type_ffi);

            let struct_borrow_info = if let ParamBorrowInfo::Struct(param_info) = param_borrow_kind
            {
                Some(StructBorrowContext {
                    use_env: &method.lifetime_env,
                    param_info,
                })
            } else {
                None
            };

            param_conversions.push(self.gen_kt_to_c_for_type(
                &param.ty,
                param_name_to_pass,
                struct_borrow_info,
                Some(&mut needs_temporary),
            ));
        }
        let write_return = matches!(
            &method.output,
            ReturnType::Infallible(SuccessType::Write)
                | ReturnType::Fallible(SuccessType::Write, _)
                | ReturnType::Nullable(SuccessType::Write)
        );
        if write_return {
            param_conversions.push("write".into());
        }
        let params = param_decls_kt.join(", ");

        let return_ty = self.gen_return_type_name(&method.output);

        let method_lifetimes_map = visitor.borrow_map();
        let return_expression = self
            .gen_return_conversion(method, &method_lifetimes_map, cleanups.as_ref())
            .into();

        // this should only be called in the special method generation below
        let non_option_type_name = |return_type: &ReturnType| match return_type {
            ReturnType::Infallible(ok) | ReturnType::Nullable(ok) => {
                self.gen_infallible_return_type_name(ok)
            }
            ReturnType::Fallible(_, _) => panic!(
                "non_option_type_name should only be called for a return type that is optional"
            ),
        };
        let declaration = match method.attrs.special_method {
            Some(SpecialMethod::Iterator) => {
                if special_methods.iterator_type.is_none() {
                    let non_option_ty = non_option_type_name(&method.output);
                    special_methods.iterator_type = Some(non_option_ty.into());
                    format!("internal fun nextInternal({params}): {return_ty}")
                } else {
                    panic!("Can only have one iterator method per opaque struct")
                }
            }
            Some(SpecialMethod::Indexer) => {
                if special_methods.indexer_type.is_none() {
                    let non_option_ty = non_option_type_name(&method.output);
                    let index_type = match &method.params.first() {
                        Some(Param {
                            ty:
                                Type::Primitive(
                                    prim @ PrimitiveType::Int(..)
                                    | prim @ PrimitiveType::IntSize(..),
                                ),
                            ..
                        }) => self.formatter.fmt_primitive_as_kt(*prim),
                        _ => panic!("index type must be an integer type"),
                    };
                    special_methods.indexer_type = Some(IndexerType {
                        index_type: index_type.into(),
                        item_type: non_option_ty.into(),
                    });
                    format!("internal fun getInternal({params}): {return_ty}")
                } else {
                    panic!("Can only have one indexer method per opaque struct")
                }
            }
            Some(SpecialMethod::Iterable) => {
                if special_methods.iterable_type.is_none() {
                    special_methods.iterable_type = Some(return_ty.to_string());
                    format!("override fun iterator(): {return_ty}")
                } else {
                    panic!("Can only have one iterable method per opaque struct")
                }
            }
            Some(SpecialMethod::Stringifier) => {
                if !special_methods.has_stringifier {
                    special_methods.has_stringifier = true;
                    "override fun toString(): String".to_string()
                } else {
                    panic!("Can only have one stringifier method per opaque struct")
                }
            }
            _ => {
                format!(
                    "fun {}({}): {return_ty}",
                    self.formatter.fmt_method_name(method),
                    params
                )
            }
        };

        let definition = MethodTpl {
            // todo: comment,
            declaration: declaration.clone(),
            native_method_name,
            param_conversions,
            return_expression,
            write_return,
            slice_conversions,
            docs: self.formatter.fmt_docs(&method.docs),
            add_override_specifier_for_opaque_self_methods,
            lifetimes: &method.lifetime_env,
            method_lifetimes_map,
            needs_temporary,
        }
        .render()
        .expect("Failed to render string for method");
        MethodInfo {
            declaration,
            definition,
        }
    }

    fn gen_native_method_info(
        &mut self,
        method: &'cx hir::Method,
        type_name: &str,
    ) -> NativeMethodInfo {
        let mut param_decls = Vec::with_capacity(method.params.len());

        let mut visitor = method.borrowing_param_visitor(self.tcx, false);
        let mut additional_name = None;

        if let Some(param_self) = method.param_self.as_ref() {
            match &param_self.ty {
                SelfType::Opaque(_) => param_decls.push("handle: Pointer".into()),
                SelfType::Struct(s) => param_decls.push(format!(
                    "nativeStruct: {}Native",
                    self.tcx.resolve_struct(s.tcx_id).name.as_str()
                )),
                SelfType::Enum(_) => param_decls.push("inner: Int".into()),
                _ => todo!(),
            }
        };
        for param in method.params.iter() {
            let param_name = self.formatter.fmt_param_name(param.name.as_str());

            visitor.visit_param(&param.ty, &param_name);
            if let Type::Callback(_) = &param.ty {
                additional_name = Some(
                    type_name.to_owned()
                        + "_"
                        + method.name.as_ref()
                        + "_diplomatCallback_"
                        + &param_name
                        + "_Native",
                )
            }

            param_decls.push(format!(
                "{param_name}: {}",
                self.gen_native_type_name(&param.ty, additional_name.clone()),
            ));
        }
        if let ReturnType::Infallible(SuccessType::Write)
        | ReturnType::Fallible(SuccessType::Write, _)
        | ReturnType::Nullable(SuccessType::Write) = method.output
        {
            param_decls.push("write: Pointer".into())
        }
        let params = param_decls.join(", ");
        let native_method = &method.abi_name;
        let return_ty = self.gen_return_type_name_ffi(&method.output);

        NativeMethodInfo {
            declaration: format!("fun {native_method}({params}): {return_ty}"),
        }
    }

    fn gen_opaque_def(&mut self, ty: &'cx hir::OpaqueDef, type_name: &str) -> (String, String) {
        let native_methods = ty
            .methods
            .iter()
            .filter(|m| !m.attrs.disable)
            .map(|method| self.gen_native_method_info(method, type_name))
            .collect::<Vec<_>>();

        let mut special_methods = SpecialMethods::default();
        let self_methods = ty
            .methods
            .iter()
            .filter(|m| !m.attrs.disable)
            .filter_map(|method| {
                method
                    .param_self
                    .as_ref()
                    .map(|self_param| (&self_param.ty, method))
            })
            .map(|(self_param, method)| {
                self.gen_method(
                    &mut special_methods,
                    method,
                    Some(self_param),
                    None,
                    ty.attrs.generate_mocking_interface, // Add override specifier when interface is generated for opaque self methods
                )
            })
            .collect::<Vec<_>>();

        let mut unused_special_methods = SpecialMethods::default();
        let companion_methods = ty
            .methods
            .iter()
            .filter(|m| !m.attrs.disable)
            .filter(|method| method.param_self.is_none())
            .map(|method| {
                self.gen_method(
                    &mut unused_special_methods,
                    method,
                    None,
                    Some(type_name),
                    false, // Add override specifier when interface is generated for opaque self methods
                )
            })
            .collect::<Vec<_>>();

        let lifetimes = ty
            .lifetimes
            .lifetimes()
            .lifetimes()
            .filter_map(|lt| match lt {
                MaybeStatic::Static => None,
                MaybeStatic::NonStatic(lt) => Some(lt),
            })
            .map(|lt| ty.lifetimes.fmt_lifetime(lt))
            .collect();

        #[derive(Template)]
        #[template(path = "kotlin/Opaque.kt.jinja", escape = "none")]
        struct ImplTemplate<'a> {
            domain: &'a str,
            lib_name: &'a str,
            dylib_name: &'a str,
            type_name: &'a str,
            dtor_abi_name: &'a str,
            self_methods: &'a [MethodInfo],
            companion_methods: &'a [MethodInfo],
            native_methods: &'a [NativeMethodInfo],
            lifetimes: Vec<Cow<'a, str>>,
            special_methods: SpecialMethodsImpl,
            callback_params: &'a [CallbackParamInfo],
            use_finalizers_not_cleaners: bool,
            docs: String,
            is_custom_error: bool,
            generate_mocking_interface: bool,
        }

        (
            format!(
                "{}/{lib_name}/{type_name}.kt",
                self.domain.replace('.', "/"),
                lib_name = self.lib_name
            ),
            ImplTemplate {
                domain: self.domain,
                lib_name: self.lib_name,
                dylib_name: self.dylib_name,
                type_name,
                dtor_abi_name: ty.dtor_abi_name.as_str(),
                self_methods: self_methods.as_ref(),
                companion_methods: companion_methods.as_ref(),
                native_methods: native_methods.as_ref(),
                lifetimes,
                special_methods: SpecialMethodsImpl::new(special_methods),
                callback_params: self.callback_params.as_ref(),
                use_finalizers_not_cleaners: self.use_finalizers_not_cleaners,
                docs: self.formatter.fmt_docs(&ty.docs),
                is_custom_error: ty.attrs.custom_errors,
                generate_mocking_interface: (ty.attrs.generate_mocking_interface
                    && !self_methods.is_empty()),
            }
            .render()
            .expect("failed to generate struct"),
        )
    }

    fn gen_struct_def<P: TyPosition>(
        &mut self,
        ty: &'cx hir::StructDef<P>,
        type_name: &str,
    ) -> (String, String) {
        let native_methods = ty
            .methods
            .iter()
            .filter(|m| !m.attrs.disable)
            .map(|method| self.gen_native_method_info(method, type_name))
            .collect::<Vec<_>>();

        let mut unused_special_methods = SpecialMethods::default();
        let self_methods = ty
            .methods
            .iter()
            .filter_map(|method| {
                method
                    .param_self
                    .as_ref()
                    .map(|self_param| (&self_param.ty, method))
            })
            .map(|(self_param, method)| {
                self.gen_method(
                    &mut unused_special_methods,
                    method,
                    Some(self_param),
                    Some(type_name),
                    false, // Add override specifier when interface is generated for opaque self methods
                )
            })
            .collect::<Vec<_>>();

        let companion_methods = ty
            .methods
            .iter()
            .filter(|method| method.param_self.is_none())
            .map(|method| {
                self.gen_method(
                    &mut unused_special_methods,
                    method,
                    None,
                    Some(type_name),
                    false, // Add override specifier when interface is generated for opaque self methods
                )
            })
            .collect::<Vec<_>>();

        let lifetimes = ty
            .lifetimes
            .lifetimes()
            .lifetimes()
            .filter_map(|lt| match lt {
                MaybeStatic::Static => None,
                MaybeStatic::NonStatic(lt) => Some(lt),
            })
            .map(|lt| ty.lifetimes.fmt_lifetime(lt))
            .collect();

        struct StructFieldDef<'d> {
            name: Cow<'d, str>,
            ffi_type_default: Cow<'d, str>,
            ffi_cast_type_name: Cow<'d, str>,
            field_type: Cow<'d, str>,
            native_to_kt: Cow<'d, str>,
            kt_to_native: Option<Cow<'d, str>>,
            docs: String,
        }

        #[derive(Template)]
        #[template(path = "kotlin/Struct.kt.jinja", escape = "none")]
        struct ImplTemplate<'a> {
            domain: &'a str,
            lib_name: &'a str,
            dylib_name: &'a str,
            type_name: &'a str,
            fields: Vec<StructFieldDef<'a>>,
            self_methods: &'a [MethodInfo],
            companion_methods: &'a [MethodInfo],
            native_methods: &'a [NativeMethodInfo],
            callback_params: &'a [CallbackParamInfo],
            lifetimes: Vec<Cow<'a, str>>,
            docs: String,
            is_custom_error: bool,
            is_out_struct: bool,
        }

        let non_out_struct = if let TypeDef::Struct(s) = P::wrap_struct_def(ty) {
            Some(s)
        } else {
            None
        };

        let fields = ty
            .fields
            .iter()
            .enumerate()
            .map(|(i, field)| {
                let field_name = self.formatter.fmt_field_name(field.name.as_str());
                let field_access = format!("nativeStruct.{field_name}");
                let field_access = &field_access;

                let struct_borrow_info = if let hir::Type::Struct(path) = &field.ty {
                    StructBorrowInfo::compute_for_struct_field(ty, path, self.tcx).map(
                        |param_info| StructBorrowContext {
                            use_env: &ty.lifetimes,
                            param_info,
                        },
                    )
                } else {
                    None
                };

                let kt_to_native = non_out_struct.map(|nonout| {
                    self.gen_kt_to_c_for_type(
                        &nonout.fields[i].ty,
                        format!("this.{field_name}").into(),
                        struct_borrow_info,
                        None,
                    )
                });

                StructFieldDef {
                    name: field_name.clone(),
                    ffi_type_default: self
                        .formatter
                        .fmt_field_default(&field.ty, /* for_results */ false),
                    ffi_cast_type_name: self.formatter.fmt_struct_field_type_native(&field.ty),
                    field_type: self.formatter.fmt_struct_field_type_kt(&field.ty),
                    native_to_kt: self.formatter.fmt_struct_field_native_to_kt(
                        field_access,
                        &ty.lifetimes,
                        &field.ty,
                    ),
                    kt_to_native,
                    docs: self.formatter.fmt_docs(&field.docs),
                }
            })
            .collect();

        (
            format!(
                "{}/{lib_name}/{type_name}.kt",
                self.domain.replace('.', "/"),
                lib_name = self.lib_name
            ),
            ImplTemplate {
                domain: self.domain,
                lib_name: self.lib_name,
                dylib_name: self.dylib_name,
                type_name,
                fields,
                self_methods: self_methods.as_ref(),
                companion_methods: companion_methods.as_ref(),
                native_methods: native_methods.as_ref(),
                callback_params: self.callback_params.as_ref(),
                lifetimes,
                docs: self.formatter.fmt_docs(&ty.docs),
                is_custom_error: ty.attrs.custom_errors,
                is_out_struct: non_out_struct.is_none(),
            }
            .render()
            .expect("Failed to render struct template"),
        )
    }

    fn gen_trait_method_info(&self, method: &Callback) -> TraitMethodInfo {
        if method.name.is_none() {
            panic!("Trait methods need a name");
        }
        let method_name = self.formatter.fmt_trait_method_name(method).into();
        let param_input_types: Vec<String> = method
            .params
            .iter()
            .map(|param| self.gen_type_name(&param.ty, None).into())
            .collect();
        let param_names: Vec<String> = method
            .params
            .iter()
            .enumerate()
            .map(|(index, param)| {
                if let Some(param_name) = &param.name {
                    param_name.to_string()
                } else {
                    format!("arg{index}")
                }
            })
            .collect();
        let (native_input_names, native_input_params_and_types): (Vec<String>, Vec<String>) =
            method
                .params
                .iter()
                .zip(param_input_types.iter())
                .zip(param_names.iter())
                .map(|((in_param, in_ty), in_name)| match in_param.ty {
                    Type::Struct(_) | Type::Enum(_) => {
                        // named types have a _Native wrapper, this needs to be passed as the "native"
                        // version of the argument
                        (
                            format!("{in_ty}.fromNative({in_name})"),
                            format!("{in_name}: {in_ty}Native"),
                        )
                    }
                    Type::Slice(Slice::Primitive(_, _)) => {
                        // slices need to be passed as Slice type
                        (
                            format!("PrimitiveArrayTools.get{in_ty}({in_name})"),
                            format!("{in_name}: Slice"),
                        )
                    }
                    Type::Slice(_) => {
                        panic!("Non-primitive slices are not allowed as callback args")
                    }
                    Type::Opaque(_) => (
                        format!("{in_ty}({in_name}, listOf())"),
                        format!("{in_name}: Pointer"),
                    ),
                    _ => (in_name.clone(), format!("{in_name}: {in_ty}")),
                })
                .unzip();
        let non_native_params_and_types = method
            .params
            .iter()
            .zip(param_input_types.iter())
            .zip(param_names.iter())
            .fold("".to_string(), |cur, ((_, in_ty), in_name)| {
                cur.clone()
                    + (if !cur.is_empty() { ", " } else { "" })
                    + &format!("{in_name}: {in_ty}")
            });
        let (native_output_type, return_modification, return_cast) = match &*method.output {
            ReturnType::Infallible(success) => match success {
                SuccessType::OutType(ty) => (
                    self.gen_native_type_name(ty, None).into(),
                    match ty {
                        Type::Enum(..) | Type::Struct(..) => ".toNative()",
                        _ => "",
                    }
                    .into(),
                    match ty {
                        Type::Primitive(prim) => {
                            self.formatter.fmt_unsigned_primitive_ffi_cast(prim)
                        }
                        _ => "",
                    }
                    .into(),
                ),
                SuccessType::Unit => ("Unit".into(), "".into(), "".into()),
                _ => panic!("Unsupported success type {success:?}"),
            },
            _ => panic!(
                "Unsupported return type {:?}. Results and Options are not supported.",
                method.output
            ),
        };
        TraitMethodInfo {
            name: method_name,
            output_type: match &*method.output {
                ReturnType::Infallible(success) => match success {
                    SuccessType::OutType(ty) => self.gen_type_name(ty, None).into(),
                    SuccessType::Unit => "Unit".into(),
                    _ => panic!("Unsupported success type {success:?}"),
                },
                _ => panic!(
                    "Unsupported return type {:?}. Results and Options are not supported.",
                    method.output
                ),
            },
            native_output_type,
            return_modification,
            return_cast,
            input_params_and_types: native_input_params_and_types.join(", "),
            non_native_params_and_types,
            input_params: native_input_names.join(", "),
            docs: match &method.docs {
                Some(method_docs) => self.formatter.fmt_docs(method_docs),
                None => "".to_string(),
            },
        }
    }

    fn gen_trait_def(&mut self, trt: &'cx hir::TraitDef, trait_name: &str) -> (String, String) {
        let trait_methods = trt
            .methods
            .iter()
            .filter(|m| {
                if let Some(m_attrs) = &m.attrs {
                    !m_attrs.disable
                } else {
                    true
                }
            })
            .map(|method| self.gen_trait_method_info(method))
            .collect::<Vec<_>>();
        let trait_method_names = trait_methods
            .iter()
            .map(|m| format!("\"run_{}_callback\"", m.name))
            .collect::<Vec<_>>();

        #[derive(Template)]
        #[template(path = "kotlin/Trait.kt.jinja", escape = "none")]
        struct ImplTemplate<'a> {
            domain: &'a str,
            lib_name: &'a str,
            trait_name: &'a str,
            trait_methods: &'a [TraitMethodInfo],
            trait_method_names: &'a str,
            callback_params: &'a [CallbackParamInfo],
            docs: String,
        }

        (
            format!(
                "{}/{lib_name}/{trait_name}.kt",
                self.domain.replace('.', "/"),
                lib_name = self.lib_name
            ),
            ImplTemplate {
                domain: self.domain,
                lib_name: self.lib_name,
                trait_name,
                trait_methods: trait_methods.as_ref(),
                callback_params: self.callback_params.as_ref(),
                trait_method_names: &trait_method_names.join(", "),
                docs: self.formatter.fmt_docs(&trt.docs),
            }
            .render()
            .expect("Failed to render trait template"),
        )
    }

    fn gen_enum_def(&mut self, ty: &'cx hir::EnumDef, type_name: &str) -> (String, String) {
        let native_methods = ty
            .methods
            .iter()
            .filter(|m| !m.attrs.disable)
            .map(|method| self.gen_native_method_info(method, type_name))
            .collect::<Vec<_>>();

        let mut special_methods = SpecialMethods::default();
        let self_methods = ty
            .methods
            .iter()
            .filter(|m| !m.attrs.disable)
            .filter_map(|method| {
                method
                    .param_self
                    .as_ref()
                    .map(|self_param| (&self_param.ty, method))
            })
            .map(|(self_param, method)| {
                self.gen_method(
                    &mut special_methods,
                    method,
                    Some(self_param),
                    None,
                    false, // Add override specifier when interface is generated for opaque self methods
                )
            })
            .collect::<Vec<_>>();

        let companion_methods = ty
            .methods
            .iter()
            .filter(|m| !m.attrs.disable)
            .filter(|method| method.param_self.is_none())
            .map(|method| {
                self.gen_method(
                    &mut special_methods,
                    method,
                    None,
                    None,
                    false, // Add override specifier when interface is generated for opaque self methods
                )
            })
            .collect::<Vec<_>>();

        #[derive(Clone, Debug)]
        struct NonContiguousEnumVariant<'d> {
            index: i32,
            name: Cow<'d, str>,
        }

        #[derive(Clone, Debug)]
        enum EnumVariants<'d> {
            Contiguous(Vec<Cow<'d, str>>),
            NonContiguous(Vec<NonContiguousEnumVariant<'d>>),
        }

        impl<'d> EnumVariants<'d> {
            fn new(ty: &'d hir::EnumDef) -> Self {
                let n_variants = ty.variants.len();
                ty.variants.iter().enumerate().fold(
                    EnumVariants::Contiguous(Vec::with_capacity(n_variants)),
                    |variants, (i, v)| match variants {
                        EnumVariants::Contiguous(mut vec) if i as isize == v.discriminant => {
                            vec.push(v.name.as_str().into());
                            EnumVariants::Contiguous(vec)
                        }

                        EnumVariants::Contiguous(vec) => {
                            let new_vec = vec
                                .into_iter()
                                .enumerate()
                                .map(|(index, name)| NonContiguousEnumVariant {
                                    name,
                                    index: index as i32,
                                })
                                .chain(once(NonContiguousEnumVariant {
                                    name: v.name.as_str().into(),
                                    index: v.discriminant as i32,
                                }))
                                .collect();

                            EnumVariants::NonContiguous(new_vec)
                        }
                        EnumVariants::NonContiguous(mut vec) => {
                            vec.push(NonContiguousEnumVariant {
                                index: v.discriminant as i32,
                                name: v.name.as_str().into(),
                            });
                            EnumVariants::NonContiguous(vec)
                        }
                    },
                )
            }
        }

        #[derive(Template)]
        #[template(path = "kotlin/Enum.kt.jinja", escape = "none")]
        struct EnumDef<'d> {
            lib_name: &'d str,
            dylib_name: &'d str,
            domain: &'d str,
            type_name: Cow<'d, str>,
            variants: &'d EnumVariants<'d>,
            self_methods: &'d [MethodInfo],
            companion_methods: &'d [MethodInfo],
            native_methods: &'d [NativeMethodInfo],
            callback_params: &'d [CallbackParamInfo],
            is_custom_error: bool,
            docs: String,
        }

        let variants = EnumVariants::new(ty);

        let enum_def = EnumDef {
            lib_name: self.lib_name,
            dylib_name: self.dylib_name,
            domain: self.domain,
            type_name: type_name.into(),
            variants: &variants,
            self_methods: self_methods.as_ref(),
            companion_methods: companion_methods.as_ref(),
            native_methods: native_methods.as_ref(),
            callback_params: self.callback_params.as_ref(),
            docs: self.formatter.fmt_docs(&ty.docs),
            is_custom_error: ty.attrs.custom_errors,
        }
        .render()
        .unwrap_or_else(|err| panic!("Failed to render Enum {{type_name}}\n\tcause: {err}"));

        (
            format!(
                "{}/{lib_name}/{type_name}.kt",
                self.domain.replace('.', "/"),
                lib_name = self.lib_name
            ),
            enum_def,
        )
    }

    fn gen_native_type_name<P: TyPosition>(
        &self,
        ty: &Type<P>,
        additional_name: Option<String>,
    ) -> Cow<'cx, str> {
        match *ty {
            Type::Primitive(prim) => self.formatter.fmt_primitive_as_ffi(prim).into(),
            Type::Opaque(ref op) => {
                let optional = if op.is_optional() { "?" } else { "" };
                format!("Pointer{optional}").into()
            }
            Type::Struct(ref strct) => {
                let op_id = strct.id();
                format!("{}Native", self.formatter.fmt_type_name(op_id)).into()
            }
            Type::Enum(_) => "Int".into(),
            Type::Slice(_) => "Slice".into(),
            Type::Callback(_) => self.gen_type_name(ty, additional_name),
            Type::ImplTrait(ref trt) => {
                let op_id = trt.id();
                format!(
                    "DiplomatTrait_{}_Wrapper_Native",
                    self.formatter.fmt_trait_name(op_id)
                )
                .into()
            }
            Type::DiplomatOption(ref inner) => {
                assert!(additional_name.is_none());
                format!("Option{}", self.gen_native_type_name(inner, None)).into()
            }
            _ => unreachable!("unknown AST/HIR variant"),
        }
    }

    fn gen_type_name<P: TyPosition>(
        &self,
        ty: &Type<P>,
        additional_name: Option<String>,
    ) -> Cow<'cx, str> {
        match *ty {
            Type::Primitive(prim) => self.formatter.fmt_primitive_as_kt(prim).into(),
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
            Type::Struct(ref strct) => {
                let op_id = strct.id();
                self.formatter.fmt_type_name(op_id)
            }
            Type::ImplTrait(ref trt) => {
                let op_id = trt.id();
                format!(
                    "DiplomatTrait_{}_Wrapper",
                    self.formatter.fmt_trait_name(op_id)
                )
                .into()
            }
            Type::Enum(ref enum_def) => self.formatter.fmt_type_name(enum_def.tcx_id.into()),
            Type::Slice(hir::Slice::Str(_, _)) => self.formatter.fmt_string().into(),
            Type::Slice(hir::Slice::Primitive(_, ty)) => {
                self.formatter.fmt_primitive_slice(ty).into()
            }
            Type::Callback(_) => format!("DiplomatCallback_{}", additional_name.unwrap()).into(),
            Type::DiplomatOption(ref inner) => {
                assert!(additional_name.is_none());
                format!("{}?", self.gen_type_name(inner, None)).into()
            }
            Type::Slice(hir::Slice::Strs(_)) => self.formatter.fmt_str_slices().into(),
            _ => unreachable!("unknown AST/HIR variant"),
        }
    }

    /// Generate the non-diplomat name for a type -- this only applies to
    /// callback and trait types. So: for a callback, instead of returning `DiplomatCallback_...`
    /// it returns `(input types)->output type`.
    /// And for traits instead of returning `DiplomatTrait_<TraitName>...` it returns TraitName
    /// For all non-callback types it returns the same result as `gen_type_name`.
    fn gen_non_wrapped_type_name(
        &self,
        ty: &Type<InputOnly>,
        additional_name: Option<String>,
    ) -> Cow<'cx, str> {
        match ty {
            Type::Callback(Callback {
                param_self: _,
                params,
                output,
                ..
            }) => {
                let in_type_string = params
                    .iter()
                    .map(|param| self.gen_type_name(&param.ty, None).into())
                    .collect::<Vec<String>>()
                    .join(", ");
                let out_type_string: String = match &**output {
                    ReturnType::Infallible(success) => match success {
                        SuccessType::OutType(out_ty) => self.gen_type_name(out_ty, None).into(),
                        SuccessType::Unit => "Unit".into(),
                        _ => panic!("Unsupported success type {success:?}"),
                    },
                    _ => panic!("Unsupported return type {output:?}"),
                };
                format!("({in_type_string})->{out_type_string}").into()
            }
            Type::ImplTrait(trt) => {
                let trait_id = trt.id();
                let resolved = self.tcx.resolve_trait(trait_id);
                resolved.name.to_string().into()
            }
            _ => self.gen_type_name(ty, additional_name),
        }
    }
}

type MethodLtMap<'a> = BTreeMap<Lifetime, BorrowedLifetimeInfo<'a>>;

#[derive(Default)]
struct SpecialMethods {
    iterator_type: Option<String>,
    indexer_type: Option<IndexerType>,
    iterable_type: Option<String>,
    has_stringifier: bool,
}

struct IndexerType {
    index_type: String,
    item_type: String,
}

#[derive(Default)]
struct SpecialMethodsImpl {
    iterator_type: Option<String>,
    indexer_type: Option<IndexerType>,
    interfaces: Vec<String>,
}

impl SpecialMethodsImpl {
    fn new(
        SpecialMethods {
            iterator_type,
            indexer_type,
            iterable_type,
            has_stringifier: _,
        }: SpecialMethods,
    ) -> Self {
        let interfaces = iterator_type
            .iter()
            .map(|ty| format!("Iterator<{ty}>"))
            .chain(
                iterable_type
                    .iter()
                    .map(|iterable_type| format!("Iterable<{iterable_type}IteratorItem>")),
            )
            .collect();
        Self {
            iterator_type,
            indexer_type,
            interfaces,
        }
    }
}

#[derive(Template)]
#[template(path = "kotlin/Method.kt.jinja", escape = "none")]
struct MethodTpl<'a> {
    // todo: comment: String,
    declaration: String,
    /// The C method name
    native_method_name: &'a str,

    /// Conversion code for each parameter
    param_conversions: Vec<Cow<'a, str>>,
    return_expression: Cow<'a, str>,
    write_return: bool,
    slice_conversions: Vec<Cow<'a, str>>,
    docs: String,
    add_override_specifier_for_opaque_self_methods: bool,

    lifetimes: &'a LifetimeEnv,
    /// Maps each (used in the output) method lifetime to a list of parameters
    /// it borrows from. The parameter list may contain the parameter name, or
    /// a spread of a struct's `_fiellsForLifetimeFoo` getter.
    method_lifetimes_map: MethodLtMap<'a>,
    needs_temporary: bool,
}

#[derive(Default)]
struct MethodInfo {
    declaration: String,
    definition: String,
}

struct NativeMethodInfo {
    declaration: String,
}

struct TraitMethodInfo {
    name: String,
    input_params_and_types: String,
    input_params: String,
    output_type: String,
    native_output_type: String,
    return_modification: String,
    return_cast: String,
    non_native_params_and_types: String,
    docs: String,
}

#[derive(Template)]
#[template(path = "kotlin/Callback.kt.jinja", escape = "none")]
struct CallbackParamInfo {
    name: String,
    input_types: String,
    native_input_params_and_types: String,
    native_input_names: String,
    output_type: String,
    native_output_type: String,
    return_modification: String,
}

#[cfg(test)]
mod test {

    use std::cell::RefCell;
    use std::collections::{BTreeSet, HashMap};

    use diplomat_core::hir::TypeDef;
    use quote::quote;

    use crate::ErrorStore;

    use super::formatter::test::new_tcx;
    use super::{formatter::KotlinFormatter, ItemGenContext};

    #[test]
    fn test_enum() {
        let tk_stream = quote! {
            #[diplomat::bridge]
            mod ffi {

                pub enum Cont {
                    A,
                    B,
                    C,
                    D,
                }

                pub enum ContNumbered {
                    Alpha=0,
                    Beta=1,
                    Gamma=2,
                }

                pub enum NonCont {
                    Aleph=0,
                    Bet=1,
                    Tav=22,
                }

                pub enum Neg {
                    Neg3=-3,
                    Neg1=-1,
                    Thirteen=13,
                }

            }
        };

        let tcx = new_tcx(tk_stream);
        let mut all_types = tcx.all_types();
        if let (_id, TypeDef::Enum(enum_def)) = all_types
            .next()
            .expect("Failed to generate first opaque def")
        {
            let error_store = ErrorStore::default();
            let docs_urls = HashMap::new();
            let docs_generator =
                diplomat_core::hir::DocsUrlGenerator::with_base_urls(None, docs_urls);
            let formatter = KotlinFormatter::new(&tcx, None, &docs_generator);
            let mut callback_params = Vec::new();
            let mut ty_gen_cx = ItemGenContext {
                tcx: &tcx,
                formatter: &formatter,
                result_types: RefCell::new(BTreeSet::new()),
                errors: &error_store,
                callback_params: &mut callback_params,
                lib_name: "somelib",
                dylib_name: "somelib",
                domain: "dev.diplomattest",
                use_finalizers_not_cleaners: false,
            };
            let type_name = enum_def.name.to_string();
            // test that we can render and that it doesn't panic
            let (_, enum_code) = ty_gen_cx.gen_enum_def(enum_def, &type_name);
            insta::assert_snapshot!(enum_code)
        }
    }

    #[test]
    fn test_struct() {
        let tk_stream = quote! {
            #[diplomat::bridge]
            mod ffi {
                use diplomat_runtime::DiplomatSlice;


                #[diplomat::opaque]
                pub struct Opaque {
                    string: String
                }

                pub struct OtherNariveStruct {
                    i: i32,
                }

                /// Documentation for the struct
                pub struct MyNativeStruct<'b> {
                    a: bool,
                    b: i8,
                    c: u8,
                    d: i16,
                    e: u16,
                    f: i32,
                    g: u32,
                    h: i64,
                    /// Documentation for the struct field `i`
                    i: u64,
                    j: DiplomatChar,
                    k: f32,
                    l: f64,
                    m: DiplomatSlice<'b, f64>,
                    n: &'b Opaque,
                }

                impl<'b> MyNativeStruct<'b> {
                    pub fn new() -> MyNativeStruct<'b> {
                        todo!()
                    }

                    pub fn test_multi_arg_callback(f: impl Fn(i32, i32, i32) -> i32, x: i32) -> i32 {
                        f(10 + x, 5, 5)
                    }

                    pub fn get_u_byte_slice<'a>() -> &'a [u8] {
                        todo!()
                    }

                    pub fn boolean_result() -> Result<bool, ()> {
                        todo!()
                    }

                    pub fn ubyte_result() -> Result<u8, ()> {
                        todo!()
                    }
                }
            }
        };

        let tcx = new_tcx(tk_stream);
        let mut all_types = tcx.all_types();
        if let (_id, TypeDef::Struct(strct)) = all_types
            .next()
            .expect("Failed to generate first opaque def")
        {
            let error_store = ErrorStore::default();
            let docs_urls = HashMap::new();
            let docs_generator =
                diplomat_core::hir::DocsUrlGenerator::with_base_urls(None, docs_urls);
            let formatter = KotlinFormatter::new(&tcx, None, &docs_generator);
            let mut callback_params = Vec::new();
            let mut ty_gen_cx = ItemGenContext {
                tcx: &tcx,
                formatter: &formatter,
                result_types: RefCell::new(BTreeSet::new()),
                errors: &error_store,
                callback_params: &mut callback_params,
                lib_name: "somelib",
                dylib_name: "somelib",
                domain: "dev.diplomattest",
                use_finalizers_not_cleaners: false,
            };
            let type_name = strct.name.to_string();
            // test that we can render and that it doesn't panic
            let (_, struct_code) = ty_gen_cx.gen_struct_def(strct, &type_name);
            insta::assert_snapshot!(struct_code)
        }
    }

    #[test]
    fn test_opaque_gen_multiple_ref_args() {
        let tk_stream = quote! {
            #[diplomat::bridge]
            mod ffi {
                #[diplomat::opaque]
                struct RustOwnedBytes {
                    my_bytes: Vec<u8>,
                }

                #[diplomat::opaque]
                struct AnotherOpaque {
                    my_bytes: Vec<u8>,
                }

                impl AnotherOpaque {
                    // we need the 2 referenced inputs to make sure the cleaner
                    // code is not all on the same line
                    pub fn get_rust_owned_bytes(&self, a: &[u8], b: &[u8]) -> Box<RustOwnedBytes> {
                        return Box::new(RustOwnedBytes{my_bytes: a.to_vec()})
                    }
                }
            }
        };
        let tcx = new_tcx(tk_stream);
        let mut all_types = tcx.all_types();
        if let (_id, TypeDef::Opaque(opaque_def)) = all_types
            .next()
            .expect("Failed to generate first opaque def")
        {
            let eror_store = ErrorStore::default();
            let docs_urls = HashMap::new();
            let docs_generator =
                diplomat_core::hir::DocsUrlGenerator::with_base_urls(None, docs_urls);
            let formatter = KotlinFormatter::new(&tcx, None, &docs_generator);
            let mut callback_params = Vec::new();
            let mut ty_gen_cx = ItemGenContext {
                tcx: &tcx,
                formatter: &formatter,
                result_types: RefCell::new(BTreeSet::new()),
                errors: &eror_store,
                callback_params: &mut callback_params,
                lib_name: "somelib",
                dylib_name: "somelib",
                domain: "dev.diplomattest",
                use_finalizers_not_cleaners: false,
            };
            let type_name = opaque_def.name.to_string();
            // test that we can render and that it doesn't panic
            let (_, result) = ty_gen_cx.gen_opaque_def(opaque_def, &type_name);
            insta::assert_snapshot!(result)
        }
    }

    #[test]
    fn test_opaque_gen() {
        let tk_stream = quote! {
            #[diplomat::bridge]
            mod ffi {
                #[diplomat::opaque]
                struct MyOpaqueStruct<'b> {
                    a: SomeExternalType
                }

                #[diplomat::opaque]
                struct InputStruct {
                }

                #[diplomat::opaque]
                struct BorrowWrapper<'a, 'b> {
                    my_opaque: &'b MyOpaqueStruct<'a>

                }

                impl<'b> MyOpaqueStruct<'b> {

                    pub fn get_byte() -> u8 {
                        unimplemented!()
                    }

                    pub fn get_string_wrapper(in1: i32) -> i32 {
                        unimplemented!()
                    }

                    pub fn copy(&self, borrow: &MyOpaqueStruct<'b>) -> i32 {
                        unimplemented!()
                    }

                    pub fn borrow_other<'a>(inp_1: &'a InputStruct, inp_2: &'a InputStruct, borrow: &'a MyOpaqueStruct<'b>) -> &'a MyOpaqueStruct<'b> {
                        unimplemented!()
                    }

                    pub fn create(in1: i32) -> Box<MyOpaqueStruct<'b>> {
                        unimplemented!()
                    }


                    pub fn do_stuff(&self, in1: i32) -> f64 {
                        unimplemented!()
                    }

                    pub fn borrow<'a>(&'a self ) -> Box<BorrowWrapper<'b, 'a>> {
                        Box::new(BorrowWrapper {
                            my_opaque: self.as_ref()
                        })
                    }

                    pub fn borrow2<'a>(&'a self ) -> &'a MyOpaqueStruct<'b> {
                        self
                    }


                    pub fn borrow3<'a>(&'a self, other: &'a mut DiplomatWrite) {
                        todo!()
                    }

                    pub fn borrow<'a>(other: &'a MyOpaqueStruct<'b>) -> Box<BorrowWrapper<'b, 'a>> {
                        Box::new(BorrowWrapper {
                            my_opaque: other.as_ref()
                        })
                    }


                    pub fn string_stuff<'a, 'c>(&'a self,  some_str: &'c DiplomatStr)  -> &'c MyOpaqueStruct<'b> {
                        self.0.as_ref()
                    }


                    pub fn string_stuff_2<'a, 'c>(&'a self,  some_str: &'c DiplomatStr)  -> &'a MyOpaqueStruct<'b> {
                        self.0.as_ref()
                    }
                }

            }
        };
        let tcx = new_tcx(tk_stream);
        let mut all_types = tcx.all_types();
        if let (_id, TypeDef::Opaque(opaque_def)) = all_types
            .next()
            .expect("Failed to generate first opaque def")
        {
            let eror_store = ErrorStore::default();
            let docs_urls = HashMap::new();
            let docs_generator =
                diplomat_core::hir::DocsUrlGenerator::with_base_urls(None, docs_urls);
            let formatter = KotlinFormatter::new(&tcx, None, &docs_generator);
            let mut callback_params = Vec::new();
            let mut ty_gen_cx = ItemGenContext {
                tcx: &tcx,
                formatter: &formatter,
                result_types: RefCell::new(BTreeSet::new()),
                errors: &eror_store,
                callback_params: &mut callback_params,
                lib_name: "somelib",
                dylib_name: "somelib",
                domain: "dev.diplomattest",
                use_finalizers_not_cleaners: false,
            };
            let type_name = opaque_def.name.to_string();
            // test that we can render and that it doesn't panic
            let (_, result) = ty_gen_cx.gen_opaque_def(opaque_def, &type_name);
            insta::assert_snapshot!(result)
        }
    }

    #[test]
    fn test_opaque_gen_with_finalizers() {
        let tk_stream = quote! {
            #[diplomat::bridge]
            mod ffi {
                #[diplomat::opaque]
                struct MyOpaqueStruct<'b> {
                    a: SomeExternalType
                }

                impl<'b> MyOpaqueStruct<'b> {

                    pub fn get_byte() -> u8 {
                        unimplemented!()
                    }

                    pub fn get_string_wrapper(in1: i32) -> i32 {
                        unimplemented!()
                    }
                }

            }
        };
        let tcx = new_tcx(tk_stream);
        let mut all_types = tcx.all_types();
        if let (_id, TypeDef::Opaque(opaque_def)) = all_types
            .next()
            .expect("Failed to generate first opaque def")
        {
            let eror_store = ErrorStore::default();
            let docs_urls = HashMap::new();
            let docs_generator =
                diplomat_core::hir::DocsUrlGenerator::with_base_urls(None, docs_urls);
            let formatter = KotlinFormatter::new(&tcx, None, &docs_generator);
            let mut callback_params = Vec::new();
            let mut ty_gen_cx = ItemGenContext {
                tcx: &tcx,
                formatter: &formatter,
                result_types: RefCell::new(BTreeSet::new()),
                errors: &eror_store,
                callback_params: &mut callback_params,
                lib_name: "somelib",
                dylib_name: "somelib",
                domain: "dev.diplomattest",
                use_finalizers_not_cleaners: true,
            };
            let type_name = opaque_def.name.to_string();
            // test that we can render and that it doesn't panic
            let (_, result) = ty_gen_cx.gen_opaque_def(opaque_def, &type_name);
            insta::assert_snapshot!(result)
        }
    }

    #[test]
    fn test_trait_gen() {
        let tk_stream = quote! {
            #[diplomat::bridge]
            mod ffi {

                pub struct TraitTestingStruct {
                    x: i32,
                    y: i32,
                }
                pub enum TraitTestingEnum {
                    One,
                    Two,
                }
                /// Documentation for this trait!
                pub trait TesterTrait {
                    /// Trait function docs
                    fn test_trait_fn(&self, x: i32, y: i32, z: u8) -> i32;
                    fn test_void_trait_fn(&self);
                    fn test_struct_trait_fn(&self, s: TraitTestingStruct) -> i32;
                    fn test_with_slices(&mut self, a: &[u8], b: &[i16]) -> i32;
                    fn test_struct_return(&self) -> TraitTestingStruct;
                    fn test_enum_return(&self) -> TraitTestingEnum;
                }
                pub struct Wrapper {
                    cant_be_empty: bool,
                }
                impl Wrapper {
                    pub fn test_with_trait(t: impl TesterTrait, x: i32, y: i32, z: u8) -> i32 {
                        t.test_void_trait_fn();
                        t.test_trait_fn(x, y, z)
                    }

                    pub fn test_trait_with_struct(t: impl TesterTrait) -> i32 {
                        let arg = TraitTestingStruct { x: 1, y: 5 };
                        t.test_struct_trait_fn(arg)
                    }
                }
            }
        };
        let tcx = new_tcx(tk_stream);
        let mut all_traits = tcx.all_traits();
        let (_id, trait_def) = all_traits.next().expect("Failed to generate trait");
        let error_store = ErrorStore::default();
        let docs_urls = HashMap::new();
        let docs_generator = diplomat_core::hir::DocsUrlGenerator::with_base_urls(None, docs_urls);
        let formatter = KotlinFormatter::new(&tcx, None, &docs_generator);
        let mut callback_params = Vec::new();
        let mut ty_gen_cx = ItemGenContext {
            tcx: &tcx,
            formatter: &formatter,
            result_types: RefCell::new(BTreeSet::new()),
            errors: &error_store,
            callback_params: &mut callback_params,
            lib_name: "somelib",
            dylib_name: "somelib",
            domain: "dev.diplomattest",
            use_finalizers_not_cleaners: false,
        };
        let trait_name = trait_def.name.to_string();
        // test that we can render and that it doesn't panic
        let (_, result) = ty_gen_cx.gen_trait_def(trait_def, &trait_name);
        insta::assert_snapshot!(result)
    }

    #[test]
    fn test_opaque_gen_with_mocking_interface() {
        let tk_stream = quote! {
            #[diplomat::bridge]
            mod ffi {
                #[diplomat::opaque]
                #[diplomat::attr(kotlin_test, generate_mocking_interface)]
                struct MyOpaqueStruct<'b> {
                    a: SomeExternalType
                }

                impl<'b> MyOpaqueStruct<'b> {

                    pub fn get_byte() -> u8 {
                        unimplemented!()
                    }

                    pub fn get_string_wrapper(in1: i32) -> i32 {
                        unimplemented!()
                    }
                }

            }
        };
        let tcx = new_tcx(tk_stream);
        let mut all_types = tcx.all_types();
        if let (_id, TypeDef::Opaque(opaque_def)) = all_types
            .next()
            .expect("Failed to generate first opaque def")
        {
            let eror_store = ErrorStore::default();
            let docs_urls = HashMap::new();
            let docs_generator =
                diplomat_core::hir::DocsUrlGenerator::with_base_urls(None, docs_urls);
            let formatter = KotlinFormatter::new(&tcx, None, &docs_generator);
            let mut callback_params = Vec::new();
            let mut ty_gen_cx = ItemGenContext {
                tcx: &tcx,
                formatter: &formatter,
                result_types: RefCell::new(BTreeSet::new()),
                errors: &eror_store,
                callback_params: &mut callback_params,
                lib_name: "somelib",
                dylib_name: "somelib",
                domain: "dev.diplomattest",
                use_finalizers_not_cleaners: true,
            };
            let type_name = opaque_def.name.to_string();
            // test that we can render and that it doesn't panic
            let (_, result) = ty_gen_cx.gen_opaque_def(opaque_def, &type_name);
            insta::assert_snapshot!(result)
        }
    }
}
