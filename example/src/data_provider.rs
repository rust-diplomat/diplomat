#[diplomat::bridge]
#[diplomat::abi_rename = "icu4x_{0}_mv1"]
#[diplomat::attr(auto, namespace = "icu4x")]
pub mod ffi {
    use icu_provider::AnyProvider;

    #[diplomat::opaque]
    /// An  data provider, capable of loading  data keys from some source.
    #[diplomat::rust_link(icu_provider, Mod)]
    pub struct DataProvider(pub Box<dyn AnyProvider>);

    impl DataProvider {
        #[diplomat::rust_link(icu_testdata::get_static_provider, Fn)]
        #[diplomat::demo(default_constructor)]
        #[diplomat::attr(auto, named_constructor = "static")]
        pub fn new_static() -> Box<DataProvider> {
            #[allow(deprecated)]
            let provider = icu_testdata::any();
            Box::new(DataProvider(Box::new(provider)))
        }

        #[allow(clippy::result_unit_err)]
        /// This exists as a regression test for https://github.com/rust-diplomat/diplomat/issues/155
        pub fn returns_result() -> Result<(), ()> {
            Ok(())
        }
    }
}
