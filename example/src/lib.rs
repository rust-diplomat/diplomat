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
    pub struct ICU4XFixedDecimal(pub FixedDecimal);

    impl ICU4XFixedDecimal {
        fn new(v: i32) -> Box<ICU4XFixedDecimal> {
            Box::new(ICU4XFixedDecimal(FixedDecimal::from(v)))
        }

        fn multiply_pow10(&mut self, power: i16) {
            self.0.multiply_pow10(power).unwrap();
        }

        fn negate(&mut self) {
            self.0.negate()
        }

        fn to_string(&self, to: &mut diplomat_runtime::DiplomatWriteable) {
            self.0.write_to(to).unwrap();
        }
    }

    #[diplomat::opaque]
    pub struct ICU4XLocale(pub Locale);

    impl ICU4XLocale {
        fn new(name: &str) -> Box<ICU4XLocale> {
            Box::new(ICU4XLocale(Locale::from_str(name).unwrap()))
        }
    }

    #[diplomat::opaque]
    pub struct ICU4XDataProvider(Box<dyn SerdeDeDataProvider>);

    impl ICU4XDataProvider {
        fn new_static() -> Box<ICU4XDataProvider> {
            let provider = icu_testdata::get_static_provider();
            Box::new(ICU4XDataProvider(Box::new(provider)))
        }
    }

    #[diplomat::opaque]
    pub struct ICU4XFixedDecimalFormat(pub FixedDecimalFormat<'static, 'static>);

    impl ICU4XFixedDecimalFormat {
        fn new(locale: &ICU4XLocale, provider: &ICU4XDataProvider) -> Box<ICU4XFixedDecimalFormat> {
            let langid = locale.0.as_ref().clone();
            let provider = provider.0.as_ref();
            Box::new(ICU4XFixedDecimalFormat(
                FixedDecimalFormat::try_new(langid, provider, FixedDecimalFormatOptions::default())
                    .unwrap(),
            ))
        }

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
