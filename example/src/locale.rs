#[diplomat::bridge]
#[diplomat::abi_rename = "icu4x_{0}_mv1"]
#[diplomat::attr(auto, namespace = "icu4x")]
pub mod ffi {
    #[diplomat::opaque]
    /// An  Locale, capable of representing strings like `"en-US"`.
    #[diplomat::rust_link(icu::locid::Locale, Struct)]
    pub struct Locale(pub icu::locid::Locale);

    impl Locale {
        /// Construct an [`Locale`] from a locale identifier represented as a string.
        #[diplomat::attr(auto, constructor)]
        pub fn new(name: &DiplomatStr) -> Box<Locale> {
            Box::new(Locale(icu::locid::Locale::try_from_bytes(name).unwrap()))
        }
    }
}
