/// A macro to test that a cpp file matches the output from diplomat. It checks the output
/// against an [insta] snapshot.
///
/// # Usage
/// ```
/// test_file! {
///     #[diplomat::bridge]
///     mod ffi {
///         enum MyEnum {
///             A, B, C
///         }
///     }
/// }
/// ```
macro_rules! test_file {
    ($($file:tt)*) => {
        let parsed: syn::File = syn::parse_quote! { $($file)* };
        let custom_types = diplomat_core::ast::File::from(&parsed);
        let env = custom_types.all_types();

        let mut out_texts = std::collections::HashMap::new();

        crate::cpp::gen_bindings(&env, &None, &Default::default(), &mut out_texts).unwrap();

        out_texts.retain(|k, _| !k.ends_with(".h"));
        out_texts.remove("diplomat_runtime.hpp");

        for out in out_texts.keys() {
            insta::with_settings!({ snapshot_suffix => out.clone() }, {
                insta::assert_display_snapshot!(out_texts.get(out).unwrap())
            });
        }
    }
}

/// A macro to test that a cpp file matches the output from diplomat. It checks the output
/// against an [insta] snapshot.
///
/// # Usage
/// ```
/// test_file! {
///     #[diplomat::bridge]
///     mod ffi {
///         enum MyEnum {
///             A, B, C
///         }
///     }
/// }
/// ```
macro_rules! test_file_using_library_config {
    ($($file:tt)*) => {
        let parsed: syn::File = syn::parse_quote! { $($file)* };
        let custom_types = diplomat_core::ast::File::from(&parsed);
        let env = custom_types.all_types();

        let mut out_texts = std::collections::HashMap::new();

        use std::path::PathBuf;
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("src/cpp/configs/mfbt.toml");
        crate::cpp::gen_bindings(&env, &Some(path), &Default::default(), &mut out_texts).unwrap();

        out_texts.retain(|k, _| !k.ends_with(".h"));
        out_texts.remove("diplomat_runtime.hpp");

        for out in out_texts.keys() {
            insta::with_settings!({ snapshot_suffix => out.clone() }, {
                insta::assert_display_snapshot!(out_texts.get(out).unwrap())
            });
        }
    }
}
