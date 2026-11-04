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
    self, Lifetime, MaybeOwn, MaybeStatic, Mutability, PrimitiveType, ReturnType, SelfType, Slice,
    StringEncoding, StructPathLike, SuccessType, TyPosition, Type, TypeContext, TypeId,
};
use std::borrow::Cow;
use std::cell::Cell;
use std::collections::BTreeSet;

/// The context used for generating a particular type
pub(super) struct ItemGenContext<'cx, 'tcx> {
    pub tcx: &'tcx TypeContext,
    pub formatter: &'cx RustFormatter<'tcx>,
    pub errors: &'cx ErrorStore<'tcx, String>,
    pub used_slices: &'cx UsedSlices,
}

/// Which of the slice types defined in `lib.rs` the generated methods use
#[derive(Default)]
pub(super) struct UsedSlices {
    pub slice: Cell<bool>,
    pub slice_mut: Cell<bool>,
}

impl UsedSlices {
    fn mark(&self, kind: SliceKind) {
        match kind {
            SliceKind::Slice => self.slice.set(true),
            SliceKind::SliceMut => self.slice_mut.set(true),
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

    /// The type that opaque pointers point to at the ABI level
    fn fmt_opaque_raw(&self, id: TypeId) -> String {
        format!(
            "crate::{}::{}Raw",
            self.formatter.fmt_module_name(id),
            self.formatter.fmt_type_name(id)
        )
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

        if let Some(param_self) = &method.param_self {
            match &param_self.ty {
                SelfType::Opaque(path) => {
                    let (receiver, ptr) = match path.owner.mutability {
                        Mutability::Mutable => ("&mut self", "*mut"),
                        Mutability::Immutable => ("&self", "*const"),
                    };
                    params.push(receiver.to_string());
                    let raw = format!("{}Raw", self.formatter.fmt_type_name(self_id));
                    abi_params.push(format!("this: {ptr} {raw}"));
                    args.push("self.0.as_ptr()".to_string());
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
        let mut slices = vec![];
        for param in &method.params {
            let param = self.gen_param(param, &storable)?;
            params.push(param.decl);
            abi_params.push(param.abi_decl);
            args.push(param.arg);
            slices.extend(param.slice);
        }

        let (ret, abi_ret, opaque_ret) = match &method.output {
            ReturnType::Infallible(SuccessType::Unit) => (None, None, None),
            // The wrapper has no lifetime, so it cannot hold on to a borrow
            ReturnType::Infallible(SuccessType::OutType(Type::Opaque(path)))
                if path.is_owned()
                    && !path.is_optional()
                    && path.lifetimes.lifetimes().len() == 0 =>
            {
                let id = path.tcx_id.into();
                let wrapper = self.fmt_type_path(id);
                (
                    Some(wrapper.clone()),
                    Some(format!("*mut {}", self.fmt_opaque_raw(id))),
                    Some(wrapper),
                )
            }
            ReturnType::Infallible(SuccessType::OutType(ty)) => {
                let ty = self.gen_value_type(ty)?;
                (Some(ty.clone()), Some(ty), None)
            }
            _ => return None,
        };

        // Only record the slice types once the method is known to be generated
        for kind in slices {
            self.used_slices.mark(kind);
        }

        Some(MethodInfo {
            docs: self.formatter.fmt_docs(&method.docs, "    "),
            name: method.name.to_string(),
            abi_name: method.abi_name.to_string(),
            params,
            abi_params,
            args,
            ret,
            abi_ret,
            opaque_ret,
        })
    }

    /// Lowers a parameter of the safe method. Borrowed parameters are only supported
    /// when the borrow cannot outlive the call, i.e. when its lifetime is not in
    /// `storable` (see [`storable_lifetimes`]) and is not `'static`.
    fn gen_param(&self, param: &hir::Param, storable: &BTreeSet<Lifetime>) -> Option<ParamInfo> {
        let call_only = |lifetime: &MaybeStatic<Lifetime>| match lifetime {
            MaybeStatic::Static => false,
            MaybeStatic::NonStatic(lifetime) => !storable.contains(lifetime),
        };
        let name = &param.name;
        let (ty, abi_ty, arg, slice) = match &param.ty {
            Type::Opaque(path)
                if !path.is_optional()
                    && path.lifetimes.lifetimes().len() == 0
                    && call_only(&path.owner.lifetime) =>
            {
                let id = path.tcx_id.into();
                let (reference, ptr) = match path.owner.mutability {
                    Mutability::Mutable => ("&mut ", "*mut"),
                    Mutability::Immutable => ("&", "*const"),
                };
                (
                    format!("{reference}{}", self.fmt_type_path(id)),
                    format!("{ptr} {}", self.fmt_opaque_raw(id)),
                    format!("{name}.0.as_ptr()"),
                    None,
                )
            }
            Type::Slice(Slice::Str(Some(lifetime), encoding)) if call_only(lifetime) => {
                let (ty, elem, bytes) = match encoding {
                    StringEncoding::Utf8 => ("&str", "u8", ".as_bytes()"),
                    StringEncoding::UnvalidatedUtf8 => ("&[u8]", "u8", ""),
                    StringEncoding::UnvalidatedUtf16 => ("&[u16]", "u16", ""),
                    _ => return None,
                };
                (
                    ty.to_string(),
                    format!("crate::DiplomatSlice<{elem}>"),
                    format!("crate::DiplomatSlice::new({name}{bytes})"),
                    Some(SliceKind::Slice),
                )
            }
            Type::Slice(Slice::Primitive(MaybeOwn::Borrow(borrow), prim))
                if call_only(&borrow.lifetime) =>
            {
                let elem = Self::gen_primitive(prim)?;
                let (reference, kind) = match borrow.mutability {
                    Mutability::Mutable => ("&mut ", SliceKind::SliceMut),
                    Mutability::Immutable => ("&", SliceKind::Slice),
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
            slice,
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

/// A slice type defined in `lib.rs`
#[derive(Clone, Copy)]
enum SliceKind {
    Slice,
    SliceMut,
}

impl SliceKind {
    fn type_name(self) -> &'static str {
        match self {
            SliceKind::Slice => "DiplomatSlice",
            SliceKind::SliceMut => "DiplomatSliceMut",
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
    /// The slice type in `lib.rs` that the argument is passed as, if any
    slice: Option<SliceKind>,
}

/// A method that can be called through the safe wrapper.
///
/// The safe signature and the `extern` declaration differ in how `self`, opaques and
/// slices are spelled; value types cross the ABI as-is.
struct MethodInfo {
    docs: String,
    name: String,
    abi_name: String,
    /// The parameters of the safe method, including `self`
    params: Vec<String>,
    /// The parameters of the `extern "C"` declaration
    abi_params: Vec<String>,
    /// The arguments passed to the `extern "C"` function
    args: Vec<String>,
    /// The return type of the safe method
    ret: Option<String>,
    /// The return type of the `extern "C"` declaration
    abi_ret: Option<String>,
    /// For a `Box<Opaque>` return, the wrapper type that takes ownership of the pointer
    opaque_ret: Option<String>,
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
    use super::{ItemGenContext, UsedSlices};
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
        let used_slices = UsedSlices::default();
        let context = ItemGenContext {
            tcx: &tcx,
            formatter: &formatter,
            errors: &errors,
            used_slices: &used_slices,
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
