macro_rules! test_file {
    ($($file:tt)*) => {
        let parsed: syn::File = syn::parse_quote! { $($file)* };
        let custom_types = diplomat_core::ast::File::from(&parsed);
        let env = custom_types.all_types();

        let mut out_texts = std::collections::HashMap::new();

        crate::js::gen_bindings(&env, &mut out_texts, None).unwrap();

        let mut out_docs = std::collections::HashMap::new();
        crate::js::docs::gen_docs(&env, &mut out_docs, &Default::default()).unwrap();

        out_texts.remove("diplomat-runtime.mjs");
        out_texts.remove("diplomat-runtime.d.ts");
        out_texts.remove("diplomat-wasm.mjs");

        for out in out_texts.keys() {
            insta::with_settings!({ snapshot_suffix => out.clone() }, {
                insta::assert_display_snapshot!(out_texts.get(out).unwrap())
            });
        }

        for out in out_docs.keys() {
            insta::with_settings!({ snapshot_suffix => out.clone() }, {
                insta::assert_display_snapshot!(out_docs.get(out).unwrap())
            });
        }
    }
}
