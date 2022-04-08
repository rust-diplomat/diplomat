#[diplomat::bridge]
pub mod ffi {
    use icu_provider::serde::SerdeDeDataProvider;

    #[diplomat::opaque]
    /// An ICU4X data provider, capable of loading ICU4X data keys from some source.
    #[diplomat::rust_link(icu_provider, Mod)]
    pub struct ICU4XDataProvider(pub Box<dyn SerdeDeDataProvider>);

    impl ICU4XDataProvider {
        #[diplomat::rust_link(icu_testdata::get_static_provider, Fn)]
        pub fn new_static() -> Box<ICU4XDataProvider> {
            let provider = icu_testdata::get_static_provider();
            Box::new(ICU4XDataProvider(Box::new(provider)))
        }
    }
}
