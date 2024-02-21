#[diplomat::bridge]
pub mod ffi {
    use diplomat_runtime::DiplomatWriteable;
    use fixed_decimal::FixedDecimal;
    use writeable::Writeable;

    #[diplomat::opaque]
    #[diplomat::rust_link(fixed_decimal::FixedDecimal, Struct)]
    pub struct ICU4XFixedDecimal(pub FixedDecimal);

    impl ICU4XFixedDecimal {
        /// Construct an [`ICU4XFixedDecimal`] from an integer.
        pub fn new(v: i32) -> Box<ICU4XFixedDecimal> {
            Box::new(ICU4XFixedDecimal(FixedDecimal::from(v)))
        }

        /// Multiply the [`ICU4XFixedDecimal`] by a given power of ten.
        #[diplomat::rust_link(fixed_decimal::FixedDecimal::multiply_pow10, FnInStruct)]
        pub fn multiply_pow10(&mut self, power: i16) {
            self.0.multiply_pow10(power);
        }

        /// Format the [`ICU4XFixedDecimal`] as a string.
        #[diplomat::rust_link(fixed_decimal::FixedDecimal::write_to, FnInStruct)]
        #[allow(clippy::result_unit_err)]
        #[diplomat::attr(dart, rename = "toStringFallible")]
        pub fn to_string(&self, to: &mut DiplomatWriteable) -> Result<(), ()> {
            self.0.write_to(to).map_err(|_| ())
        }
    }
}
