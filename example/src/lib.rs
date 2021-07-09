#[diplomat::bridge]
mod ffi {
    use std::str::FromStr;

    use fixed_decimal::FixedDecimal;
    use icu::locid::Locale;
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
}
