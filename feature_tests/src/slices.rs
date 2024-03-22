#[diplomat::bridge]
#[diplomat::attr(kotlin, disable)]
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

        pub fn new_bool(v: &[bool]) -> Box<Float64Vec> {
            Box::new(Self(v.iter().map(|&x| x as u8 as f64).collect()))
        }

        pub fn new_i16(v: &[i16]) -> Box<Float64Vec> {
            Box::new(Self(v.iter().map(|&x| x as f64).collect()))
        }

        pub fn new_u16(v: &[u16]) -> Box<Float64Vec> {
            Box::new(Self(v.iter().map(|&x| x as f64).collect()))
        }

        pub fn new_isize(v: &[isize]) -> Box<Float64Vec> {
            Box::new(Self(v.iter().map(|&x| x as f64).collect()))
        }

        pub fn new_usize(v: &[usize]) -> Box<Float64Vec> {
            Box::new(Self(v.iter().map(|&x| x as f64).collect()))
        }

        pub fn new_f64_be_bytes(v: &[DiplomatByte]) -> Box<Float64Vec> {
            Box::new(Self(
                v.chunks_exact(8)
                    .map(|b| f64::from_be_bytes([b[0], b[1], b[2], b[3], b[4], b[5], b[6], b[7]]))
                    .collect(),
            ))
        }

        pub fn fill_slice(&self, v: &mut [f64]) {
            v.copy_from_slice(&self.0)
        }

        pub fn set_value(&mut self, new_slice: &[f64]) {
            self.0 = new_slice.to_vec();
        }

        pub fn to_string(&self, w: &mut DiplomatWriteable) {
            write!(w, "{:?}", self.0).unwrap();
        }
    }
}
