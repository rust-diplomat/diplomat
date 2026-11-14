//! Rust code generation, one template per kind of type:
//!
//! * [`EnumTemplate`], [`StructTemplate`] and [`OpaqueTemplate`] render a single
//!   type definition.
//! * [`MethodInfo`] lowers the methods all three share into their safe wrapper and
//!   `extern "C"` declaration.
//! * [`ItemGenContext`] holds what all of them need.

use super::formatter::RustFormatter;
use crate::ErrorStore;
use askama::Template;
use diplomat_core::hir::{
    self, Lifetime, LifetimeEnv, MaybeOwn, MaybeStatic, Mutability, PrimitiveType, ReturnType,
    SelfType, Slice, StringEncoding, StructPathLike, SuccessType, TyPosition, Type, TypeContext,
    TypeId,
};
use std::borrow::Cow;
use std::cell::Cell;
use std::collections::BTreeSet;

/// The context used for generating a particular type
pub(super) struct ItemGenContext<'cx, 'tcx> {
    pub tcx: &'tcx TypeContext,
    pub formatter: &'cx RustFormatter<'tcx>,
    pub errors: &'cx ErrorStore<'tcx, String>,
    pub used_types: &'cx UsedRuntimeTypes,
}

/// Which of the types defined in `lib.rs` the generated methods use
#[derive(Default)]
pub(super) struct UsedRuntimeTypes {
    pub slice: Cell<bool>,
    pub slice_mut: Cell<bool>,
    pub result: Cell<bool>,
    /// `DiplomatOpaque` and `DiplomatBox`
    pub opaque: Cell<bool>,
}

impl UsedRuntimeTypes {
    fn mark(&self, ty: RuntimeType) {
        match ty {
            RuntimeType::Slice => self.slice.set(true),
            RuntimeType::SliceMut => self.slice_mut.set(true),
            RuntimeType::Result => self.result.set(true),
        }
    }
}

impl ItemGenContext<'_, '_> {
    /// Types that cross the ABI by value with the same layout on both sides
    fn gen_value_type<P: TyPosition>(&self, ty: &Type<P>) -> Option<String> {
        match ty {
            Type::Primitive(prim) => Self::gen_primitive(prim).map(String::from),
            Type::Enum(path) => Some(self.fmt_type_path(path.tcx_id.into())),
            Type::Struct(path) => Some(self.fmt_type_path(path.id())),
            _ => None,
        }
    }

    fn gen_primitive(prim: &PrimitiveType) -> Option<&'static str> {
        match prim {
            PrimitiveType::Bool
            | PrimitiveType::Int(_)
            | PrimitiveType::IntSize(_)
            | PrimitiveType::Float(_) => Some(prim.as_str()),
            PrimitiveType::Byte => Some("u8"),
            // `char` and orderings need validation before they become Rust values
            _ => None,
        }
    }

    /// Types are re-exported from the crate root
    fn fmt_type_path(&self, id: TypeId) -> String {
        format!("crate::{}", self.formatter.fmt_type_name(id))
    }

    pub fn gen_enum(&self, id: hir::EnumId) -> String {
        let def = self.tcx.resolve_enum(id);
        EnumTemplate {
            ty_name: self.formatter.fmt_type_name(id.into()),
            docs: self.formatter.fmt_docs(&def.docs, ""),
            variants: def
                .variants
                .iter()
                .map(|v| EnumVariant {
                    name: v.name.to_string(),
                    discriminant: v.discriminant,
                    docs: self.formatter.fmt_docs(&v.docs, "    "),
                })
                .collect(),
            methods: self.gen_methods(id.into(), &def.methods),
        }
        .to_string()
    }

    pub fn gen_struct<P: TyPosition>(&self, id: P::StructId) -> String {
        let def = P::resolve_struct(self.tcx, id);
        let fields = def
            .fields
            .iter()
            .map(|field| {
                let ty = self.gen_value_type(&field.ty).unwrap_or_else(|| {
                    self.errors.push_error(format!(
                        "Unsupported type for struct field `{}`",
                        field.name
                    ));
                    String::new()
                });
                StructField {
                    name: field.name.to_string(),
                    ty,
                    docs: self.formatter.fmt_docs(&field.docs, "    "),
                }
            })
            .collect();
        StructTemplate {
            ty_name: self.formatter.fmt_type_name(id.into()),
            docs: self.formatter.fmt_docs(&def.docs, ""),
            fields,
            methods: self.gen_methods(id.into(), &def.methods),
        }
        .to_string()
    }

    pub fn gen_opaque(&self, id: hir::OpaqueId) -> String {
        let def = self.tcx.resolve_opaque(id);
        self.used_types.opaque.set(true);
        OpaqueTemplate {
            ty_name: self.formatter.fmt_type_name(id.into()),
            docs: self.formatter.fmt_docs(&def.docs, ""),
            dtor_abi_name: def.dtor_abi_name.as_str(),
            methods: self.gen_methods(id.into(), &def.methods),
        }
        .to_string()
    }

    fn gen_methods(&self, self_id: TypeId, methods: &[hir::Method]) -> Vec<MethodInfo> {
        methods
            .iter()
            .filter(|m| !m.attrs.disable)
            .filter_map(|m| self.gen_method(self_id, m))
            .collect()
    }

    fn gen_method(&self, self_id: TypeId, method: &hir::Method) -> Option<MethodInfo> {
        let mut params = vec![];
        let mut abi_params = vec![];
        let mut args = vec![];

        // A returned borrow is tied to the inputs by its lifetime, so the signature
        // spells out every lifetime instead of relying on elision.
        let returns_borrow = returns_borrow(method);
        if returns_borrow && self_has_lifetimes(method) {
            // The generated type has no lifetime parameters to tie the borrow to
            return None;
        }
        let refs = RefFormatter {
            env: returns_borrow.then_some(&method.lifetime_env),
        };

        if let Some(param_self) = &method.param_self {
            match &param_self.ty {
                SelfType::Opaque(path) => {
                    let reference = refs.fmt_ref(&path.owner.lifetime, path.owner.mutability);
                    params.push(format!("{reference}self"));
                    let ty_name = self.formatter.fmt_type_name(self_id);
                    let ptr = fmt_ptr(path.owner.mutability);
                    abi_params.push(format!("this: {ptr} {ty_name}"));
                    args.push("self".to_string());
                }
                SelfType::Struct(_) | SelfType::Enum(_) => {
                    params.push("self".to_string());
                    abi_params.push(format!("this: {}", self.fmt_type_path(self_id)));
                    args.push("self".to_string());
                }
                _ => return None,
            }
        }

        let storable = storable_lifetimes(method);
        let mut runtime_types = vec![];
        for param in &method.params {
            let param = self.gen_param(param, &storable, &refs)?;
            params.push(param.decl);
            abi_params.push(param.abi_decl);
            args.push(param.arg);
            runtime_types.extend(param.runtime_type);
        }

        let call = format!("{}({})", method.abi_name, args.join(", "));
        let mut safety = BTreeSet::new();
        let (ret, abi_ret, opaque_ret, result) = match &method.output {
            ReturnType::Infallible(SuccessType::Unit) => (None, None, None, None),
            ReturnType::Infallible(SuccessType::OutType(ty)) => {
                let out = self.gen_out_type(ty, &refs)?;
                safety.extend(out.convert.as_ref().map(Convert::safety));
                let opaque_ret = out.convert.map(|convert| convert.apply(&call));
                (Some(out.ty), Some(out.abi_ty), opaque_ret, None)
            }
            ReturnType::Fallible(ok, err) => {
                let ok = match ok {
                    SuccessType::Unit => OutTypeInfo::unit(),
                    SuccessType::OutType(ty) => self.gen_out_type(ty, &refs)?,
                    _ => return None,
                };
                let err = match err {
                    Some(ty) => self.gen_out_type(ty, &refs)?,
                    None => OutTypeInfo::unit(),
                };
                for convert in [&ok.convert, &err.convert].into_iter().flatten() {
                    safety.insert(convert.safety());
                }
                runtime_types.push(RuntimeType::Result);
                (
                    Some(format!("Result<{}, {}>", ok.ty, err.ty)),
                    Some(format!(
                        "crate::{}<{}, {}>",
                        RuntimeType::Result.type_name(),
                        ok.abi_ty,
                        err.abi_ty
                    )),
                    None,
                    Some(ResultInfo {
                        ok_convert: ok.convert.map(|convert| convert.apply("ok")),
                        err_convert: err.convert.map(|convert| convert.apply("err")),
                    }),
                )
            }
            _ => return None,
        };

        // Only record the runtime types once the method is known to be generated
        for ty in runtime_types {
            self.used_types.mark(ty);
        }

        let (generics, where_clause) = refs.fmt_generics();
        Some(MethodInfo {
            docs: self.formatter.fmt_docs(&method.docs, "    "),
            name: method.name.to_string(),
            abi_name: method.abi_name.to_string(),
            generics,
            where_clause,
            params,
            abi_params,
            call,
            safety: safety.into_iter().collect(),
            ret,
            abi_ret,
            opaque_ret,
            result,
        })
    }

    /// Lowers a type returned by the library. An opaque comes back as a pointer that
    /// the safe method turns into a `DiplomatBox` or a reference; value types come
    /// back as-is.
    fn gen_out_type(&self, ty: &hir::OutType, refs: &RefFormatter) -> Option<OutTypeInfo> {
        match ty {
            // The generated type has no lifetime parameters, so it cannot hold on to a borrow
            Type::Opaque(path) if !path.is_optional() && path.lifetimes.lifetimes().len() == 0 => {
                let opaque = self.fmt_type_path(path.tcx_id.into());
                Some(match path.owner {
                    MaybeOwn::Own => OutTypeInfo {
                        ty: format!("crate::DiplomatBox<{opaque}>"),
                        abi_ty: format!("*mut {opaque}"),
                        convert: Some(Convert::Box),
                    },
                    MaybeOwn::Borrow(borrow) => OutTypeInfo {
                        ty: format!(
                            "{}{opaque}",
                            refs.fmt_ref(&borrow.lifetime, borrow.mutability)
                        ),
                        abi_ty: format!("{} {opaque}", fmt_ptr(borrow.mutability)),
                        convert: Some(Convert::Ref(borrow.mutability)),
                    },
                })
            }
            ty => {
                let ty = self.gen_value_type(ty)?;
                Some(OutTypeInfo {
                    ty: ty.clone(),
                    abi_ty: ty,
                    convert: None,
                })
            }
        }
    }

    /// Lowers a parameter of the safe method. Borrowed parameters are only supported
    /// when the borrow cannot outlive the call, i.e. when its lifetime is not in
    /// `storable` (see [`storable_lifetimes`]) and is not `'static`.
    fn gen_param(
        &self,
        param: &hir::Param,
        storable: &BTreeSet<Lifetime>,
        refs: &RefFormatter,
    ) -> Option<ParamInfo> {
        let call_only = |lifetime: &MaybeStatic<Lifetime>| match lifetime {
            MaybeStatic::Static => false,
            MaybeStatic::NonStatic(lifetime) => !storable.contains(lifetime),
        };
        let name = &param.name;
        let (ty, abi_ty, arg, runtime_type) = match &param.ty {
            Type::Opaque(path)
                if !path.is_optional()
                    && path.lifetimes.lifetimes().len() == 0
                    && call_only(&path.owner.lifetime) =>
            {
                let opaque = self.fmt_type_path(path.tcx_id.into());
                let reference = refs.fmt_ref(&path.owner.lifetime, path.owner.mutability);
                (
                    format!("{reference}{opaque}"),
                    format!("{} {opaque}", fmt_ptr(path.owner.mutability)),
                    name.to_string(),
                    None,
                )
            }
            Type::Slice(Slice::Str(Some(lifetime), encoding)) if call_only(lifetime) => {
                let (ty, elem, bytes) = match encoding {
                    StringEncoding::Utf8 => ("str", "u8", ".as_bytes()"),
                    StringEncoding::UnvalidatedUtf8 => ("[u8]", "u8", ""),
                    StringEncoding::UnvalidatedUtf16 => ("[u16]", "u16", ""),
                    _ => return None,
                };
                (
                    format!("{}{ty}", refs.fmt_ref(lifetime, Mutability::Immutable)),
                    format!("crate::DiplomatSlice<{elem}>"),
                    format!("crate::DiplomatSlice::new({name}{bytes})"),
                    Some(RuntimeType::Slice),
                )
            }
            Type::Slice(Slice::Primitive(MaybeOwn::Borrow(borrow), prim))
                if call_only(&borrow.lifetime) =>
            {
                let elem = Self::gen_primitive(prim)?;
                let reference = refs.fmt_ref(&borrow.lifetime, borrow.mutability);
                let kind = match borrow.mutability {
                    Mutability::Mutable => RuntimeType::SliceMut,
                    Mutability::Immutable => RuntimeType::Slice,
                };
                let slice = kind.type_name();
                (
                    format!("{reference}[{elem}]"),
                    format!("crate::{slice}<{elem}>"),
                    format!("crate::{slice}::new({name})"),
                    Some(kind),
                )
            }
            ty => {
                let ty = self.gen_value_type(ty)?;
                (ty.clone(), ty, name.to_string(), None)
            }
        };
        Some(ParamInfo {
            decl: format!("{name}: {ty}"),
            abi_decl: format!("{name}: {abi_ty}"),
            arg,
            runtime_type,
        })
    }
}

/// Lifetimes under which the library could keep a borrowed parameter after the call
/// returns: those of `self`'s type, and every lifetime that outlives one of them.
fn storable_lifetimes(method: &hir::Method) -> BTreeSet<Lifetime> {
    let self_lifetimes = match method.param_self.as_ref().map(|p| &p.ty) {
        Some(SelfType::Opaque(path)) => path.lifetimes.lifetimes(),
        Some(SelfType::Struct(path)) => path.lifetimes.lifetimes(),
        _ => return BTreeSet::new(),
    };
    self_lifetimes
        .filter_map(|lifetime| match lifetime {
            MaybeStatic::NonStatic(lifetime) => Some(lifetime),
            MaybeStatic::Static => None,
        })
        .flat_map(|lifetime| method.lifetime_env.all_longer_lifetimes(lifetime))
        .collect()
}

/// Whether the method returns a borrowed opaque, directly or in a `Result`
fn returns_borrow(method: &hir::Method) -> bool {
    let borrowed = |ty: &hir::OutType| matches!(ty, Type::Opaque(path) if !path.is_owned());
    match &method.output {
        ReturnType::Infallible(SuccessType::OutType(ty)) => borrowed(ty),
        ReturnType::Fallible(ok, err) => {
            matches!(ok, SuccessType::OutType(ty) if borrowed(ty))
                || err.as_ref().is_some_and(borrowed)
        }
        _ => false,
    }
}

/// Whether the type of `self` has lifetime parameters
fn self_has_lifetimes(method: &hir::Method) -> bool {
    match method.param_self.as_ref().map(|p| &p.ty) {
        Some(SelfType::Opaque(path)) => path.lifetimes.lifetimes().len() > 0,
        Some(SelfType::Struct(path)) => path.lifetimes.lifetimes().len() > 0,
        _ => false,
    }
}

/// The raw pointer an opaque reference crosses the ABI as
fn fmt_ptr(mutability: Mutability) -> &'static str {
    match mutability {
        Mutability::Mutable => "*mut",
        Mutability::Immutable => "*const",
    }
}

/// Formats the references in a method signature. With an `env`, every reference
/// names its lifetime and the method declares them all; without one, lifetimes are
/// left to elision.
struct RefFormatter<'a> {
    env: Option<&'a LifetimeEnv>,
}

impl RefFormatter<'_> {
    /// `&`, `&mut `, `&'a ` or `&'a mut `
    fn fmt_ref(&self, lifetime: &MaybeStatic<Lifetime>, mutability: Mutability) -> String {
        let lifetime = match (self.env, lifetime) {
            (None, _) => String::new(),
            (Some(_), MaybeStatic::Static) => "'static ".into(),
            (Some(env), MaybeStatic::NonStatic(lifetime)) => {
                format!("'{} ", env.fmt_lifetime(lifetime))
            }
        };
        match mutability {
            Mutability::Mutable => format!("&{lifetime}mut "),
            Mutability::Immutable => format!("&{lifetime}"),
        }
    }

    /// The method's lifetime parameters, e.g. `<'a, 'b>`, and the bounds between
    /// them, e.g. ` where 'b: 'a`
    fn fmt_generics(&self) -> (String, String) {
        let Some(env) = self.env else {
            return Default::default();
        };
        let names: Vec<_> = env
            .all_lifetimes()
            .map(|lifetime| format!("'{}", env.fmt_lifetime(lifetime)))
            .collect();
        let bounds: Vec<_> = env
            .all_lifetimes()
            .flat_map(|shorter| {
                env.all_longer_lifetimes(shorter)
                    .filter(move |&longer| longer != shorter)
                    .map(move |longer| {
                        format!(
                            "'{}: '{}",
                            env.fmt_lifetime(longer),
                            env.fmt_lifetime(shorter)
                        )
                    })
            })
            .collect();
        let generics = if names.is_empty() {
            String::new()
        } else {
            format!("<{}>", names.join(", "))
        };
        let where_clause = if bounds.is_empty() {
            String::new()
        } else {
            format!(" where {}", bounds.join(", "))
        };
        (generics, where_clause)
    }
}

/// How the safe method turns an opaque pointer from the library into its return type
enum Convert {
    /// A `Box<Opaque>`, owned by a `DiplomatBox`
    Box,
    /// A borrowed opaque
    Ref(Mutability),
}

impl Convert {
    /// Why the conversion is safe
    fn safety(&self) -> &'static str {
        match self {
            Convert::Box => "`Box` returns are never null, and the caller owns them.",
            Convert::Ref(_) => {
                "Borrowed returns are never null, and live as long as the signature says."
            }
        }
    }

    /// The conversion applied to the pointer `expr`
    fn apply(&self, expr: &str) -> String {
        match self {
            Convert::Box => format!("crate::DiplomatBox::from_raw({expr})"),
            Convert::Ref(Mutability::Immutable) => format!("&*{expr}"),
            Convert::Ref(Mutability::Mutable) => format!("&mut *{expr}"),
        }
    }
}

/// A type defined in `lib.rs` for crossing the ABI
#[derive(Clone, Copy)]
enum RuntimeType {
    Slice,
    SliceMut,
    Result,
}

impl RuntimeType {
    fn type_name(self) -> &'static str {
        match self {
            RuntimeType::Slice => "DiplomatSlice",
            RuntimeType::SliceMut => "DiplomatSliceMut",
            RuntimeType::Result => "DiplomatResult",
        }
    }
}

/// A parameter of a [`MethodInfo`]
struct ParamInfo {
    /// The parameter of the safe method
    decl: String,
    /// The parameter of the `extern "C"` declaration
    abi_decl: String,
    /// The argument passed to the `extern "C"` function
    arg: String,
    /// The type in `lib.rs` that the argument is passed as, if any
    runtime_type: Option<RuntimeType>,
}

/// A type returned by a [`MethodInfo`], or one side of its `Result`
struct OutTypeInfo {
    /// The type the safe method returns
    ty: String,
    /// The type the `extern "C"` function returns
    abi_ty: String,
    /// For an opaque, how the safe method converts the pointer
    convert: Option<Convert>,
}

impl OutTypeInfo {
    fn unit() -> Self {
        OutTypeInfo {
            ty: "()".into(),
            abi_ty: "()".into(),
            convert: None,
        }
    }
}

/// How a [`MethodInfo`] converts the `DiplomatResult` it gets from the library
struct ResultInfo {
    /// For an opaque in `Ok`, the conversion applied to the pointer `ok`
    ok_convert: Option<String>,
    /// For an opaque in `Err`, the conversion applied to the pointer `err`
    err_convert: Option<String>,
}

/// A method that can be called through the safe wrapper.
///
/// The safe signature and the `extern` declaration differ in how `self`, opaques and
/// slices are spelled; value types cross the ABI as-is.
struct MethodInfo {
    docs: String,
    name: String,
    abi_name: String,
    /// The lifetime parameters of the safe method, if it spells them out
    generics: String,
    /// The bounds between those lifetimes
    where_clause: String,
    /// The parameters of the safe method, including `self`
    params: Vec<String>,
    /// The parameters of the `extern "C"` declaration
    abi_params: Vec<String>,
    /// The call to the `extern "C"` function
    call: String,
    /// Why converting the returned pointers is safe, one line each
    safety: Vec<&'static str>,
    /// The return type of the safe method
    ret: Option<String>,
    /// The return type of the `extern "C"` declaration
    abi_ret: Option<String>,
    /// For an opaque return, the call converted into the return type
    opaque_ret: Option<String>,
    /// For a `Result` return, how to convert it
    result: Option<ResultInfo>,
}

#[derive(Template)]
#[template(path = "rust/enum.rs.jinja", escape = "none")]
struct EnumTemplate<'a> {
    ty_name: Cow<'a, str>,
    docs: String,
    variants: Vec<EnumVariant>,
    methods: Vec<MethodInfo>,
}

struct EnumVariant {
    name: String,
    discriminant: isize,
    docs: String,
}

#[derive(Template)]
#[template(path = "rust/struct.rs.jinja", escape = "none")]
struct StructTemplate<'a> {
    ty_name: Cow<'a, str>,
    docs: String,
    fields: Vec<StructField>,
    methods: Vec<MethodInfo>,
}

struct StructField {
    name: String,
    ty: String,
    docs: String,
}

#[derive(Template)]
#[template(path = "rust/opaque.rs.jinja", escape = "none")]
struct OpaqueTemplate<'a> {
    ty_name: Cow<'a, str>,
    docs: String,
    dtor_abi_name: &'a str,
    methods: Vec<MethodInfo>,
}

#[cfg(test)]
mod test {
    use super::super::formatter::RustFormatter;
    use super::{ItemGenContext, UsedRuntimeTypes};
    use crate::ErrorStore;
    use diplomat_core::hir::{self, DocsUrlGenerator, TypeContext};
    use proc_macro2::TokenStream;
    use quote::quote;

    /// Generates the type named `name` from a `#[diplomat::bridge]` module
    fn gen_type(tk_stream: TokenStream, name: &str) -> String {
        let file = syn::parse2::<syn::File>(tk_stream).expect("failed to parse test module");
        let mut attr_validator = hir::BasicAttributeValidator::new("rust");
        attr_validator.support = super::super::attr_support();
        let tcx = TypeContext::from_syn(
            &file,
            Default::default(),
            attr_validator,
            None,
            &diplomat_core::ast::SpanLocation::None,
        )
        .unwrap_or_else(|e| panic!("Failed to create context: {e:?}"));

        let docs_url_gen = DocsUrlGenerator::default();
        let formatter = RustFormatter::new(&tcx, &docs_url_gen);
        let errors = ErrorStore::default();
        let used_types = UsedRuntimeTypes::default();
        let context = ItemGenContext {
            tcx: &tcx,
            formatter: &formatter,
            errors: &errors,
            used_types: &used_types,
        };

        let (id, _) = tcx
            .all_types()
            .find(|(_, ty)| ty.name().as_str() == name)
            .expect("type not found");
        let code = match id {
            hir::TypeId::Enum(e) => context.gen_enum(e),
            hir::TypeId::Opaque(o) => context.gen_opaque(o),
            hir::TypeId::Struct(s) => context.gen_struct::<hir::Everywhere>(s),
            _ => unreachable!("not covered by these tests"),
        };
        assert!(errors.take_all().is_empty(), "generation reported errors");
        code
    }

    #[test]
    fn test_opaque() {
        insta::assert_snapshot!(gen_type(
            quote! {
                #[diplomat::bridge]
                mod ffi {
                    /// An opaque type
                    #[diplomat::opaque_mut]
                    struct MyOpaque(i32);

                    #[diplomat::opaque]
                    struct MyView<'a>(&'a i32);

                    impl MyOpaque {
                        pub fn new(value: i32) -> Box<MyOpaque> {
                            unimplemented!()
                        }

                        pub fn get(&self) -> i32 {
                            unimplemented!()
                        }

                        pub fn set(&mut self, value: i32) {
                            unimplemented!()
                        }

                        /// Skipped: the returned opaque borrows from `self`
                        pub fn view<'a>(&'a self) -> Box<MyView<'a>> {
                            unimplemented!()
                        }
                    }
                }
            },
            "MyOpaque"
        ));
    }

    #[test]
    fn test_borrowed_params() {
        let module = quote! {
                #[diplomat::bridge]
                mod ffi {
                    #[diplomat::opaque_mut]
                    struct Other(i32);

                    #[diplomat::opaque_mut]
                    struct MyOpaque(i32);

                    #[diplomat::opaque]
                    struct MyView<'a>(&'a i32);

                    impl MyOpaque {
                        pub fn strs(
                            a: &DiplomatStr,
                            b: &str,
                            c: &DiplomatStr16,
                        ) -> Box<MyOpaque> {
                            unimplemented!()
                        }

                        pub fn slices(&self, a: &[f64], b: &mut [u8]) {
                            unimplemented!()
                        }

                        pub fn opaques(&mut self, a: &Other, b: &mut Other) -> i32 {
                            unimplemented!()
                        }

                        /// Skipped: the library may keep a `'static` borrow
                        pub fn static_opaque(a: &'static Other) {
                            unimplemented!()
                        }

                        /// Skipped: optional opaques are not supported yet
                        pub fn optional(a: Option<&Other>) {
                            unimplemented!()
                        }

                        /// Skipped: the opaque parameter has lifetimes
                        pub fn view<'a>(v: &MyView<'a>) {
                            unimplemented!()
                        }
                    }

                    impl<'a> MyView<'a> {
                        /// Skipped: the library may keep `s` in `self`
                        pub fn keep(&self, s: &'a DiplomatStr) {
                            unimplemented!()
                        }

                        /// Skipped: `'b` outlives `'a`, so the library may keep `s` in `self`
                        pub fn keep_longer<'b: 'a>(&self, s: &'b DiplomatStr) {
                            unimplemented!()
                        }

                        pub fn read(&self, s: &DiplomatStr) -> i32 {
                            unimplemented!()
                        }

                        /// `'b` may be shorter than `'a`, so `s` cannot be kept in `self`
                        pub fn read_shorter<'b>(&'b self, s: &'b DiplomatStr) -> i32
                        where
                            'a: 'b,
                        {
                            unimplemented!()
                        }
                    }
                }
        };
        insta::assert_snapshot!(gen_type(module.clone(), "MyOpaque"));
        insta::assert_snapshot!("borrowed_params_view", gen_type(module, "MyView"));
    }

    #[test]
    fn test_result() {
        insta::assert_snapshot!(gen_type(
            quote! {
                #[diplomat::bridge]
                mod ffi {
                    #[diplomat::opaque]
                    struct MyError(i32);

                    #[diplomat::opaque]
                    struct MyView<'a>(&'a i32);

                    enum ErrorKind {
                        A,
                        B,
                    }

                    #[diplomat::opaque]
                    struct MyOpaque(i32);

                    impl MyOpaque {
                        pub fn try_new(value: i32) -> Result<Box<MyOpaque>, ()> {
                            unimplemented!()
                        }

                        pub fn check(&self) -> Result<(), ErrorKind> {
                            unimplemented!()
                        }

                        pub fn get(&self) -> Result<i32, Box<MyError>> {
                            unimplemented!()
                        }

                        /// Skipped: the returned opaque borrows from `self`
                        pub fn view<'a>(&'a self) -> Result<Box<MyView<'a>>, ()> {
                            unimplemented!()
                        }

                        /// Skipped: optional returns are not supported yet
                        pub fn maybe(&self) -> Option<i32> {
                            unimplemented!()
                        }
                    }
                }
            },
            "MyOpaque"
        ));
    }

    #[test]
    fn test_borrowed_returns() {
        let module = quote! {
            #[diplomat::bridge]
            mod ffi {
                #[diplomat::opaque]
                struct Bar(i32);

                #[diplomat::opaque]
                struct MyView<'a>(&'a i32);

                #[diplomat::opaque_mut]
                struct Foo(Bar);

                impl Foo {
                    pub fn bar<'a>(&'a self) -> &'a Bar {
                        unimplemented!()
                    }

                    pub fn bar_mut<'a>(&'a mut self) -> &'a mut Bar {
                        unimplemented!()
                    }

                    pub fn try_bar<'a>(&'a self) -> Result<&'a Bar, ()> {
                        unimplemented!()
                    }

                    pub fn pick<'a, 'b: 'a>(&'a self, other: &'b Bar, s: &str) -> &'a Bar {
                        unimplemented!()
                    }

                    pub fn global() -> &'static Bar {
                        unimplemented!()
                    }
                }

                impl<'a> MyView<'a> {
                    /// Skipped: `MyView` has lifetimes
                    pub fn bar<'b>(&'b self) -> &'b Bar {
                        unimplemented!()
                    }
                }
            }
        };
        insta::assert_snapshot!(gen_type(module.clone(), "Foo"));
        insta::assert_snapshot!("borrowed_returns_view", gen_type(module, "MyView"));
    }

    #[test]
    fn test_enum() {
        insta::assert_snapshot!(gen_type(
            quote! {
                #[diplomat::bridge]
                mod ffi {
                    /// An enum
                    enum MyEnum {
                        /// The first variant
                        A,
                        B = 5,
                    }

                    impl MyEnum {
                        pub fn is_a(self) -> bool {
                            unimplemented!()
                        }
                    }
                }
            },
            "MyEnum"
        ));
    }

    #[test]
    fn test_struct() {
        insta::assert_snapshot!(gen_type(
            quote! {
                #[diplomat::bridge]
                mod ffi {
                    enum MyEnum {
                        A,
                    }

                    /// A struct
                    struct MyStruct {
                        /// An integer
                        a: u8,
                        b: f64,
                        c: MyEnum,
                    }

                    impl MyStruct {
                        pub fn new(a: u8) -> MyStruct {
                            unimplemented!()
                        }

                        pub fn sum(self) -> f64 {
                            unimplemented!()
                        }
                    }
                }
            },
            "MyStruct"
        ));
    }
}
