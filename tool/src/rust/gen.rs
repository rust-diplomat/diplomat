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
    self, Mutability, PrimitiveType, ReturnType, SelfType, StructPathLike, SuccessType, TyPosition,
    Type, TypeContext, TypeId,
};
use std::borrow::Cow;

/// The context used for generating a particular type
pub(super) struct ItemGenContext<'cx, 'tcx> {
    pub tcx: &'tcx TypeContext,
    pub formatter: &'cx RustFormatter<'tcx>,
    pub errors: &'cx ErrorStore<'tcx, String>,
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

        for param in &method.params {
            let ty = self.gen_value_type(&param.ty)?;
            params.push(format!("{}: {ty}", param.name));
            abi_params.push(format!("{}: {ty}", param.name));
            args.push(param.name.to_string());
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
}

/// A method that can be called through the safe wrapper.
///
/// Parameters cross the ABI as-is, so the safe signature and the `extern` declaration
/// only differ in how `self` and opaque returns are spelled.
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
    use super::ItemGenContext;
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
        let context = ItemGenContext {
            tcx: &tcx,
            formatter: &formatter,
            errors: &errors,
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

                        /// Skipped: strings are not supported yet
                        pub fn from_str(s: &DiplomatStr) -> Box<MyOpaque> {
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
