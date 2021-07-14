#[diplomat::bridge]
mod ffi {
    use std::str::FromStr;

    use fixed_decimal::FixedDecimal;
    use icu::{
        decimal::{options::FixedDecimalFormatOptions, FixedDecimalFormat},
        locid::Locale,
    };
    use icu_provider::serde::SerdeDeDataProvider;
    use writeable::Writeable;

    #[diplomat::opaque]
    /// A decimal number. See [the Rust docs](https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/decimal/struct.FixedDecimal.html) for more information.
    pub struct ICU4XFixedDecimal(pub FixedDecimal);

    impl ICU4XFixedDecimal {
        /// Construct an [`ICU4XFixedDecimal`] from an integer.
        fn new(v: i32) -> Box<ICU4XFixedDecimal> {
            Box::new(ICU4XFixedDecimal(FixedDecimal::from(v)))
        }

        /// Multiply the [`ICU4XFixedDecimal`] by a given power of ten.
        /// See [the Rust docs](https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/decimal/struct.FixedDecimal.html#method.multiply_pow10) for more information.
        fn multiply_pow10(&mut self, power: i16) {
            self.0.multiply_pow10(power).unwrap();
        }

        /// Invert the sign of the [`ICU4XFixedDecimal`].
        /// See [the Rust docs](https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/decimal/struct.FixedDecimal.html#method.negate) for more information.
        fn negate(&mut self) {
            self.0.negate()
        }

        /// Format the [`ICU4XFixedDecimal`] as a string.
        /// See [the Rust docs](https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/decimal/struct.FixedDecimal.html#method.write_to) for more information.
        fn to_string(&self, to: &mut diplomat_runtime::DiplomatWriteable) {
            self.0.write_to(to).unwrap();
        }
    }

    #[diplomat::opaque]
    /// An ICU4X Locale, capable of representing strings like `"en-US"`.
    /// See [the Rust docs](https://unicode-org.github.io/icu4x-docs/doc/icu/locid/struct.Locale.html) for more information.
    pub struct ICU4XLocale(pub Locale);

    impl ICU4XLocale {
        /// Construct an [`ICU4XLocale`] from an locale identifier.
        fn new(name: &str) -> Box<ICU4XLocale> {
            Box::new(ICU4XLocale(Locale::from_str(name).unwrap()))
        }
    }

    #[diplomat::opaque]
    /// An ICU4X data provider, capable of loading ICU4X data keys from some source.
    /// See [the Rust docs](https://unicode-org.github.io/icu4x-docs/doc/icu_provider/prelude/trait.DataProvider.html) for more information.
    pub struct ICU4XDataProvider(Box<dyn SerdeDeDataProvider>);

    impl ICU4XDataProvider {
        /// Construct a [StaticDataProvider](https://unicode-org.github.io/icu4x-docs/doc/icu_testdata/fn.get_static_provider.html).
        fn new_static() -> Box<ICU4XDataProvider> {
            let provider = icu_testdata::get_static_provider();
            Box::new(ICU4XDataProvider(Box::new(provider)))
        }
    }

    #[diplomat::opaque]
    /// An ICU4X Fixed Decimal Format object, capable of formatting a [`ICU4XFixedDecimal`] as a string.
    /// See [the Rust docs](https://unicode-org.github.io/icu4x-docs/doc/icu/decimal/struct.FixedDecimalFormat.html) for more information.
    pub struct ICU4XFixedDecimalFormat(pub FixedDecimalFormat<'static, 'static>);

    pub struct ICU4XFixedDecimalFormatResult {
        /// The [`ICU4XFixedDecimalFormat`], valid if creation was successful.
        pub fdf: Box<ICU4XFixedDecimalFormat>,
        /// Whether creating the [`ICU4XFixedDecimalFormat`] was successful.
        pub success: bool,
    }

    impl ICU4XFixedDecimalFormat {
        /// Creates a new [`ICU4XFixedDecimalFormat`] from locale data. See [the Rust docs](https://unicode-org.github.io/icu4x-docs/doc/icu/decimal/struct.FixedDecimalFormat.html#method.try_new) for more information.
        fn try_new(
            locale: &ICU4XLocale,
            provider: &ICU4XDataProvider,
        ) -> ICU4XFixedDecimalFormatResult {
            let langid = locale.0.as_ref().clone();
            let provider = provider.0.as_ref();

            if let Result::Ok(fdf) =
                FixedDecimalFormat::try_new(langid, provider, FixedDecimalFormatOptions::default())
            {
                ICU4XFixedDecimalFormatResult {
                    fdf: Box::new(ICU4XFixedDecimalFormat(fdf)),
                    success: true,
                }
            } else {
                ICU4XFixedDecimalFormatResult {
                    fdf: unsafe { Box::from_raw(std::ptr::null_mut()) },
                    success: false,
                }
            }
        }

        /// Formats a [`ICU4XFixedDecimal`] to a string. See [the Rust docs](https://unicode-org.github.io/icu4x-docs/doc/icu/decimal/struct.FixedDecimalFormat.html#method.format) for more information.
        fn format_write(
            &self,
            value: &ICU4XFixedDecimal,
            write: &mut diplomat_runtime::DiplomatWriteable,
        ) {
            self.0.format(&value.0).write_to(write).unwrap();
            write.flush();
        }
    }
}
