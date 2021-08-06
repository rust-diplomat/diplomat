#[diplomat::bridge]
pub mod ffi {
    use diplomat_runtime::{DiplomatResult, DiplomatWriteable};
    use fixed_decimal::FixedDecimal;
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
        fn to_string(&self, to: &mut DiplomatWriteable) -> DiplomatResult<(), ()> {
            self.0.write_to(to).map_err(|_| ()).into()
        }
    }
}
