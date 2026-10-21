//! Rust code generation, one module per kind of type:
//!
//! * [`enums`], [`structs`] and [`opaque`] render a single type definition.
//! * [`method`] lowers the methods all three share into their safe wrapper and
//!   `extern "C"` declaration.
//! * This file holds [`ItemGenContext`] and the type lowering the others share.

use super::formatter::RustFormatter;
use crate::ErrorStore;
use diplomat_core::hir::{PrimitiveType, StructPathLike, TyPosition, Type, TypeContext, TypeId};

mod enums;
mod method;
mod opaque;
mod structs;

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
