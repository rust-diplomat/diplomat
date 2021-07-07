#[diplomat::bridge]
mod ffi {
    use fixed_decimal::FixedDecimal;

    #[diplomat::opaque]
    pub struct ICU4XFixedDecimal(pub FixedDecimal);

    impl ICU4XFixedDecimal {
        fn new(v: i32) -> Box<ICU4XFixedDecimal> {
            Box::new(ICU4XFixedDecimal(FixedDecimal::from(v)))
        }

        fn multiply_pow10(&mut self, power: i16) {
            self.0.multiply_pow10(power).unwrap();
        }

        fn digit_at(&self, magnitude: i16) -> u8 {
            self.0.digit_at(magnitude)
        }
    }
}
