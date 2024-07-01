#[diplomat::bridge]
pub mod ffi {
    use icu::locid::Locale;

    #[diplomat::opaque]
    /// An ICU4X Locale, capable of representing strings like `"en-US"`.
    #[diplomat::rust_link(icu::locid::Locale, Struct)]
    #[diplomat::demo(input(label = "Locale"))]
    pub struct ICU4XLocale(pub Locale);

    impl ICU4XLocale {
        /// Construct an [`ICU4XLocale`] from a locale identifier represented as a string.
        #[diplomat::attr(supports = constructors, constructor)]
        pub fn new(name: &DiplomatStr) -> Box<ICU4XLocale> {
            Box::new(ICU4XLocale(Locale::try_from_bytes(name).unwrap()))
        }
    }
}
