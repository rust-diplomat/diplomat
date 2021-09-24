#[diplomat::bridge]
pub mod ffi {
    use icu::locid::Locale;
    use std::str::FromStr;

    #[diplomat::opaque]
    /// An ICU4X Locale, capable of representing strings like `"en-US"`.
    /// See [the Rust docs](https://unicode-org.github.io/icu4x-docs/doc/icu/locid/struct.Locale.html) for more information.
    pub struct ICU4XLocale(pub Locale);

    impl ICU4XLocale {
        /// Construct an [`ICU4XLocale`] from a locale identifier represented as a string.
        pub fn new(name: &str) -> Box<ICU4XLocale> {
            Box::new(ICU4XLocale(Locale::from_str(name).unwrap()))
        }

        /// Construct an [`ICU4XLocale`] from a locale identifier represented as bytes.
        pub fn new_from_bytes(bytes: &[u8]) -> Box<ICU4XLocale> {
            Box::new(ICU4XLocale(Locale::from_bytes(bytes).unwrap()))
        }
    }
}
