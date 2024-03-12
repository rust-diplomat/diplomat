use super::TypeName;

#[cfg_attr(feature = "displaydoc", derive(displaydoc::Display))]
#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum ValidityError {
    /// A return type contains elided lifetimes.
    #[cfg_attr(
        feature = "displaydoc",
        displaydoc("A return type contains elided lifetimes, which aren't yet supported: {sub_type} in {full_type}")
    )]
    LifetimeElisionInReturn {
        full_type: TypeName,
        sub_type: TypeName,
    },
}

#[cfg(test)]
mod tests {
    use std::fmt::Write;

    macro_rules! uitest_validity {
        ($($file:tt)*) => {
            let parsed: syn::File = syn::parse_quote! { $($file)* };
            let custom_types = crate::ast::File::from(&parsed);
            let env = custom_types.all_types();

            let errors = custom_types.check_validity(&env);

            let mut output = String::new();
            for error in errors {
                write!(&mut output, "{}\n", error).unwrap();
            }
            insta::with_settings!({}, {
                insta::assert_display_snapshot!(output)
            });
        }
    }

    #[test]
    fn test_lifetime_in_return() {
        uitest_validity! {
            #[diplomat::bridge]
            mod ffi {
                #[diplomat::opaque]
                struct Opaque;

                struct Foo<'a> {
                    x: &'a Opaque,
                }

                impl Opaque {
                    pub fn returns_self(&self) -> &Self {}
                    pub fn returns_foo(&self) -> Foo {}
                }
            }
        };
    }
}
