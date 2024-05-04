use diplomat_core::hir::{OpaqueDef, TypeContext, TypeId};

use crate::c2::CFormatter;

fn gen_opaque_def(ctx: &TypeContext, type_id: TypeId, opaque_def: &OpaqueDef) -> String {
    let c_formatter = CFormatter::new(ctx);
    opaque_def
        .methods
        .iter()
        .map(|method| c_formatter.fmt_method_name(type_id, method))
        .collect::<Vec<_>>()
        .join("\n")
}

#[cfg(test)]
mod test {
    use diplomat_core::{
        ast::{self},
        hir::{self, TypeDef},
    };
    use quote::quote;

    #[test]
    fn test_opaque_gen() {
        let tokens = quote! {
            #[diplomat::bridge]
            mod ffi {

                #[diplomat::opaque]
                struct OpaqueStruct;

                impl OpaqueStruct {
                    pub fn add_two(i: i32) -> i32 {
                        i + 2
                    }
                }

            }
        };
        let item = syn::parse2::<syn::File>(tokens).expect("failed to parse item ");

        let diplomat_file = ast::File::from(&item);
        let env = diplomat_file.all_types();
        let attr_validator = hir::BasicAttributeValidator::new("my_backend_test");

        let context = match hir::TypeContext::from_ast(&env, attr_validator) {
            Ok(context) => context,
            Err(e) => {
                for (_cx, err) in e {
                    eprintln!("Lowering error: {}", err);
                }
                panic!("Failed to create context")
            }
        };

        let (type_id, opaque_def) = match context
            .all_types()
            .next()
            .expect("Failed to generate first opaque def")
        {
            (type_id, TypeDef::Opaque(opaque_def)) => (type_id, opaque_def),
            _ => panic!("Failed to find opaque type from AST"),
        };

        let generated = super::gen_opaque_def(&context, type_id, opaque_def);

        insta::assert_snapshot!(generated)
    }
}
