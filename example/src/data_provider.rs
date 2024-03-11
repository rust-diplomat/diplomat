#[diplomat::bridge]
pub mod ffi {
    use icu_provider::AnyProvider;

    #[diplomat::opaque]
    /// An ICU4X data provider, capable of loading ICU4X data keys from some source.
    #[diplomat::rust_link(icu_provider, Mod)]
    pub struct ICU4XDataProvider(pub Box<dyn AnyProvider>);

    impl ICU4XDataProvider {
        #[diplomat::rust_link(icu_testdata::get_static_provider, Fn)]
        #[diplomat::attr(supports = named_constructors, named_constructor)]
        pub fn new_static() -> Box<ICU4XDataProvider> {
            let provider = icu_testdata::any();
            Box::new(ICU4XDataProvider(Box::new(provider)))
        }

        #[allow(clippy::result_unit_err)]
        /// This exists as a regression test for https://github.com/rust-diplomat/diplomat/issues/155
        pub fn returns_result() -> Result<(), ()> {
            Ok(())
        }
    }
}
