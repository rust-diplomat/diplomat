#[diplomat::bridge]
pub mod ffi {
    use icu_provider::serde::SerdeDeDataProvider;

    #[diplomat::opaque]
    /// An ICU4X data provider, capable of loading ICU4X data keys from some source.
    /// See [the Rust docs](https://unicode-org.github.io/icu4x-docs/doc/icu_provider/prelude/trait.DataProvider.html) for more information.
    pub struct ICU4XDataProvider(pub Box<dyn SerdeDeDataProvider>);

    impl ICU4XDataProvider {
        /// Construct a [StaticDataProvider](https://unicode-org.github.io/icu4x-docs/doc/icu_testdata/fn.get_static_provider.html).
        fn new_static() -> Box<ICU4XDataProvider> {
            let provider = icu_testdata::get_static_provider();
            Box::new(ICU4XDataProvider(Box::new(provider)))
        }
    }
}
