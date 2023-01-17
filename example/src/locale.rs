#[diplomat::bridge]
pub mod ffi {
    use icu::locid::Locale;
    use std::str::FromStr;

    #[diplomat::opaque]
    /// An ICU4X Locale, capable of representing strings like `"en-US"`.
    #[diplomat::rust_link(icu::locid::Locale, Struct)]
    pub struct ICU4XLocale(pub Locale);

    impl ICU4XLocale {
        /// Construct an [`ICU4XLocale`] from a locale identifier represented as a string.
        pub fn new(name: &str) -> Box<ICU4XLocale> {
            Box::new(ICU4XLocale(Locale::from_str(name).unwrap()))
        }

        /// Construct an [`ICU4XLocale`] from a locale identifier represented as bytes.
        pub fn new_from_bytes(bytes: &[u8]) -> Box<ICU4XLocale> {
            Box::new(ICU4XLocale(Locale::try_from_bytes(bytes).unwrap()))
        }
    }
}
