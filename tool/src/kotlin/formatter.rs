use crate::c2::CFormatter;
use diplomat_core::hir::{self, TypeContext, TypeId};
use heck::ToLowerCamelCase;
use std::borrow::Cow;

/// This type mediates all formatting
///
/// of C types and methods.
pub(super) struct KotlinFormatter<'tcx> {
    c: CFormatter<'tcx>,
    strip_prefix: Option<String>,
}

const INVALID_METHOD_NAMES: &[&str] = &["new", "static", "default", "private", "internal"];
const DISALLOWED_CORE_TYPES: &[&str] = &["Object", "String"];

impl<'tcx> KotlinFormatter<'tcx> {
    pub fn new(tcx: &'tcx TypeContext, strip_prefix: Option<String>) -> Self {
        Self {
            c: CFormatter::new(tcx),
            strip_prefix,
        }
    }

    pub fn fmt_void(&self) -> &'static str {
        "Unit"
    }

    pub fn fmt_string(&self) -> &'static str {
        "DiplomatStr"
    }

    pub fn fmt_c_method_name<'a>(&self, ty: TypeId, method: &'a hir::Method) -> Cow<'a, str> {
        self.c.fmt_method_name(ty, method).into()
    }

    pub fn fmt_primitive_as_ffi(&self, prim: hir::PrimitiveType) -> &'static str {
        use diplomat_core::hir::{FloatType, IntType, PrimitiveType};
        match prim {
            PrimitiveType::Bool => "Boolean",
            PrimitiveType::Char => "Char",
            PrimitiveType::Int(IntType::I8) => "Byte",
            PrimitiveType::Int(IntType::I16) => "Short",
            PrimitiveType::Int(IntType::I32) => "Int",
            PrimitiveType::Int(IntType::I64) => "Long",
            PrimitiveType::Int(IntType::U8) => "UByte",
            PrimitiveType::Int(IntType::U16) => "UShort",
            PrimitiveType::Int(IntType::U32) => "UInt",
            PrimitiveType::Int(IntType::U64) => "ULong",
            PrimitiveType::Byte => "Byte",
            PrimitiveType::IntSize(_) => "Long", // this feels wrong
            PrimitiveType::Float(FloatType::F32) => "Float",
            PrimitiveType::Float(FloatType::F64) => "Double",
            PrimitiveType::Int128(_) => panic!("i128 not supported in Dart"),
        }
    }

    pub fn fmt_method_name<'a>(&self, method: &'a hir::Method) -> Cow<'a, str> {
        // TODO(#60): handle other keywords

        // TODO: we should give attrs.rename() control over the camelcasing
        let name = method.name.as_str().to_lower_camel_case();
        let name = method.attrs.rename.apply(name.into());
        if INVALID_METHOD_NAMES.contains(&&*name) {
            format!("{name}_").into()
        } else {
            name
        }
    }

    pub fn fmt_param_name<'a>(&self, ident: &'a str) -> Cow<'a, str> {
        ident.to_lower_camel_case().into()
    }

    pub fn fmt_type_name(&self, id: TypeId) -> Cow<'tcx, str> {
        let resolved = self.c.tcx().resolve_type(id);

        let candidate: Cow<str> = if let Some(strip_prefix) = self.strip_prefix.as_ref() {
            resolved
                .name()
                .as_str()
                .strip_prefix(strip_prefix)
                .unwrap_or(resolved.name().as_str())
                .into()
        } else {
            resolved.name().as_str().into()
        };

        if DISALLOWED_CORE_TYPES.contains(&&*candidate) {
            panic!("{candidate:?} is not a valid Kotlin type name. Please rename.");
        }

        resolved.attrs().rename.apply(candidate)
    }

    pub fn fmt_nullable(&self, ident: &str) -> String {
        format!("{ident}?")
    }
}

#[cfg(test)]
pub mod test {
    use std::borrow::Cow;

    use super::KotlinFormatter;
    use diplomat_core::{
        ast::{self},
        hir::{self, TypeContext},
    };
    use proc_macro2::TokenStream;

    use quote::quote;

    pub fn new_tcx(tk_stream: TokenStream) -> TypeContext {
        let item = syn::parse2::<syn::File>(tk_stream).expect("failed to parse item ");

        let diplomat_file = ast::File::from(&item);

        let env = diplomat_file.all_types();
        let mut attr_validator = hir::BasicAttributeValidator::new("kotlin_test");
        attr_validator.support.renaming = true;
        attr_validator.support.disabling = true;

        match hir::TypeContext::from_ast(&env, attr_validator) {
            Ok(context) => context,
            Err(e) => {
                for err in e {
                    eprintln!("Lowering error: {}", err);
                }
                panic!()
            }
        }
    }

    #[test]
    fn test_type_name() {
        let tk_stream = quote! {
            #[diplomat::bridge]
            mod ffi {
                #[diplomat::opaque]
                struct MyOpaqueStruct {
                    a: SomeExternalType
                }

                #[diplomat::opaque]
                struct StringWrapper(String);

                impl MyOpaqueStruct {
                    pub fn new() -> Box<MyOpaqueStruct> {
                        unimplemented!();
                    }

                    pub fn get_byte(&self) -> u8 {
                        unimplemented!()
                    }

                    pub fn get_string_wrapper(&self) -> Box<StringWrapper> {
                        unimplemented!()
                    }

                }

            }
        };
        let tcx = new_tcx(tk_stream);
        let formatter = KotlinFormatter::new(&tcx, None);
        let opaques = tcx.opaques();
        assert!(!opaques.is_empty());
        let mut all_types = tcx.all_types();
        let (ty_id, _) = all_types.next().expect("Failed to get next type");

        assert_eq!(Cow::from("MyOpaqueStruct"), formatter.fmt_type_name(ty_id));

        let (ty_id, _) = all_types.next().expect("Failed to get next type");

        assert_eq!(Cow::from("StringWrapper"), formatter.fmt_type_name(ty_id));
    }
}