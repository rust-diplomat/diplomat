#[diplomat::bridge]
mod ffi {
    use std::str::FromStr;

    use icu::{
        decimal::{
            options::{FixedDecimalFormatOptions, GroupingStrategy, SignDisplay},
            FixedDecimalFormat,
        },
        locid::Locale,
    };
    use icu_provider::serde::SerdeDeDataProvider;
    use writeable::Writeable;

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
    /// An ICU4X Fixed Decimal Format object, capable of formatting a [`crate::fixed_decimal::ffi::ICU4XFixedDecimal`] as a string.
    /// See [the Rust docs](https://unicode-org.github.io/icu4x-docs/doc/icu/decimal/struct.FixedDecimalFormat.html) for more information.
    pub struct ICU4XFixedDecimalFormat(pub FixedDecimalFormat<'static, 'static>);

    pub struct ICU4XFixedDecimalFormatResult {
        /// The [`ICU4XFixedDecimalFormat`], exists if creation was successful.
        pub fdf: Option<Box<ICU4XFixedDecimalFormat>>,
        /// Whether creating the [`ICU4XFixedDecimalFormat`] was successful.
        pub success: bool,
    }

    pub struct ICU4XFixedDecimalFormatOptions {
        pub grouping_strategy: u8,
        pub sign_display: u8,
    }

    impl ICU4XFixedDecimalFormatOptions {
        pub fn default() -> ICU4XFixedDecimalFormatOptions {
            ICU4XFixedDecimalFormatOptions {
                grouping_strategy: 0,
                sign_display: 0,
            }
        }
    }

    impl ICU4XFixedDecimalFormat {
        /// Creates a new [`ICU4XFixedDecimalFormat`] from locale data. See [the Rust docs](https://unicode-org.github.io/icu4x-docs/doc/icu/decimal/struct.FixedDecimalFormat.html#method.try_new) for more information.
        fn try_new(
            locale: &ICU4XLocale,
            provider: &ICU4XDataProvider,
            options: ICU4XFixedDecimalFormatOptions,
        ) -> ICU4XFixedDecimalFormatResult {
            let langid = locale.0.as_ref().clone();
            let provider = provider.0.as_ref();

            if let Result::Ok(fdf) = FixedDecimalFormat::try_new(
                langid,
                provider,
                FixedDecimalFormatOptions {
                    grouping_strategy: match options.grouping_strategy {
                        0 => GroupingStrategy::Auto,
                        1 => GroupingStrategy::Never,
                        2 => GroupingStrategy::Always,
                        3 => GroupingStrategy::Min2,
                        _ => panic!(),
                    },
                    sign_display: match options.sign_display {
                        0 => SignDisplay::Auto,
                        1 => SignDisplay::Never,
                        2 => SignDisplay::Always,
                        3 => SignDisplay::ExceptZero,
                        4 => SignDisplay::Negative,
                        _ => panic!(),
                    },
                },
            ) {
                ICU4XFixedDecimalFormatResult {
                    fdf: Some(Box::new(ICU4XFixedDecimalFormat(fdf))),
                    success: true,
                }
            } else {
                ICU4XFixedDecimalFormatResult {
                    fdf: None,
                    success: false,
                }
            }
        }

        /// Formats a [`crate::fixed_decimal::ffi::ICU4XFixedDecimal`] to a string. See [the Rust docs](https://unicode-org.github.io/icu4x-docs/doc/icu/decimal/struct.FixedDecimalFormat.html#method.format) for more information.
        fn format_write(
            &self,
            value: &crate::fixed_decimal::ffi::ICU4XFixedDecimal,
            write: &mut diplomat_runtime::DiplomatWriteable,
        ) {
            self.0.format(&value.0).write_to(write).unwrap();
            write.flush();
        }
    }
}
