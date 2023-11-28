#[diplomat::bridge]
mod ffi {
    use diplomat_runtime::DiplomatWriteable;
    use std::fmt::Write as _;

    #[diplomat::opaque]
    struct MyString(String);

    impl MyString {
        pub fn new(v: &DiplomatStr) -> Box<MyString> {
            Box::new(Self(String::from_utf8(v.to_owned()).unwrap()))
        }

        pub fn new_unsafe(v: &str) -> Box<MyString> {
            Box::new(Self(v.to_string()))
        }

        pub fn set_str(&mut self, new_str: &DiplomatStr) {
            self.0 = String::from_utf8(new_str.to_owned()).unwrap();
        }

        pub fn get_str(&self, writeable: &mut DiplomatWriteable) {
            let _ = write!(writeable, "{}", self.0);
            writeable.flush();
        }
    }

    #[diplomat::opaque]
    struct Float64Vec(Vec<f64>);

    impl Float64Vec {
        pub fn new(v: &[f64]) -> Box<Float64Vec> {
            Box::new(Self(v.to_vec()))
        }

        pub fn fill_slice(&self, v: &mut [f64]) {
            v.copy_from_slice(&self.0)
        }

        pub fn set_value(&mut self, new_slice: &[f64]) {
            self.0 = new_slice.to_vec();
        }
    }
}
