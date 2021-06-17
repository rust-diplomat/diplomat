#[diplomat::bridge]
mod ffi {
    use fixed_decimal::FixedDecimal;

    pub struct ICU4XFixedDecimal(pub FixedDecimal);

    impl ICU4XFixedDecimal {
        fn new(v: i32) -> Box<ICU4XFixedDecimal> {
            Box::new(ICU4XFixedDecimal(FixedDecimal::from(v)))
        }

        fn multiply_pow10(&mut self, power: i16) {
            self.0.multiply_pow10(power).unwrap();
        }
    }
}

fn main() {
    let mut decimal = ffi::ICU4XFixedDecimal_new(123);
    ffi::ICU4XFixedDecimal_multiply_pow10(&mut decimal, -1);
}
