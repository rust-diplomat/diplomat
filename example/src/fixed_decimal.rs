#[diplomat::bridge]
#[diplomat::abi_rename = "icu4x_{0}_mv1"]
#[diplomat::attr(auto, namespace = "icu4x")]
pub mod ffi {
    use diplomat_runtime::DiplomatWrite;
    use writeable::Writeable;

    #[diplomat::opaque]
    #[diplomat::rust_link(fixed_decimal::FixedDecimal, Struct)]
    // Link to where other custom functions for this class can be found.
    // Make sure any .mjs file export defaults an object that matches the `RenderTerminus.terminus` object in content.
    #[diplomat::demo(custom_func = "../demo_gen/custom_func/a.mjs")]
    pub struct FixedDecimal(pub fixed_decimal::FixedDecimal);

    impl FixedDecimal {
        /// Construct an [`FixedDecimal`] from an integer.
        #[diplomat::attr(auto, constructor)]
        pub fn new(
            #[diplomat::demo(input(label = "ICU4XFixedDecimal Value", default_value = 1000))]
            v: i32,
        ) -> Box<FixedDecimal> {
            Box::new(FixedDecimal(fixed_decimal::FixedDecimal::from(v)))
        }

        /// Multiply the [`FixedDecimal`] by a given power of ten.
        #[diplomat::rust_link(fixed_decimal::FixedDecimal::multiply_pow10, FnInStruct)]
        pub fn multiply_pow10(&mut self, power: i16) {
            self.0.multiply_pow10(power);
        }

        /// Format the [`FixedDecimal`] as a string.
        #[diplomat::rust_link(fixed_decimal::FixedDecimal::write_to, FnInStruct)]
        #[allow(clippy::result_unit_err)]
        #[diplomat::attr(dart, rename = "toStringFallible")]
        pub fn to_string(&self, to: &mut DiplomatWrite) -> Result<(), ()> {
            self.0.write_to(to).map_err(|_| ())
        }
    }
}
