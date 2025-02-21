#[diplomat::bridge]
mod ffi {
    use diplomat_runtime::{DiplomatStr, DiplomatStrSlice, DiplomatWrite};
    use std::fmt::Write as _;

    #[diplomat::opaque]
    struct MyString(String);

    impl MyString {
        #[diplomat::attr(auto, constructor)]
        pub fn new(v: &DiplomatStr) -> Box<MyString> {
            Box::new(Self(String::from_utf8(v.to_owned()).unwrap()))
        }

        #[diplomat::attr(auto, named_constructor = "unsafe")]
        pub fn new_unsafe(v: &str) -> Box<MyString> {
            Box::new(Self(v.to_string()))
        }

        pub fn new_owned(v: Box<DiplomatStr>) -> Box<MyString> {
            Box::new(Self(String::from_utf8(v.into()).unwrap()))
        }

        pub fn new_from_first(v: &[DiplomatStrSlice]) -> Box<MyString> {
            Box::new(Self(core::str::from_utf8(v[0].into()).unwrap().into()))
        }

        #[diplomat::attr(auto, setter = "str")]
        pub fn set_str(&mut self, new_str: &DiplomatStr) {
            self.0 = String::from_utf8(new_str.to_owned()).unwrap();
        }

        #[diplomat::attr(auto, getter = "str")]
        pub fn get_str(&self, write: &mut DiplomatWrite) {
            let _infallible = write!(write, "{}", self.0);
        }

        pub fn get_static_str() -> &'static str {
            "hello"
        }

        pub fn string_transform(foo: &str, write: &mut DiplomatWrite) {
            let _ = foo;
            let _ = write;
        }

        pub fn borrow<'a>(&'a self) -> DiplomatStrSlice<'a> {
            AsRef::<[u8]>::as_ref(&self.0).into()
        }
    }

    #[diplomat::opaque]
    struct Float64Vec(Vec<f64>);

    impl Float64Vec {
        #[diplomat::attr(not(supports = memory_sharing), disable)]
        pub fn new(v: &[f64]) -> Box<Float64Vec> {
            Box::new(Self(v.to_vec()))
        }

        #[diplomat::attr(auto, named_constructor = "bool")]
        pub fn new_bool(v: &[bool]) -> Box<Float64Vec> {
            Box::new(Self(v.iter().map(|&x| x as u8 as f64).collect()))
        }

        #[diplomat::attr(auto, named_constructor = "i16")]
        pub fn new_i16(v: &[i16]) -> Box<Float64Vec> {
            Box::new(Self(v.iter().map(|&x| x as f64).collect()))
        }

        #[diplomat::attr(auto, named_constructor = "u16")]
        pub fn new_u16(v: &[u16]) -> Box<Float64Vec> {
            Box::new(Self(v.iter().map(|&x| x as f64).collect()))
        }

        #[diplomat::attr(auto, named_constructor = "isize")]
        pub fn new_isize(v: &[isize]) -> Box<Float64Vec> {
            Box::new(Self(v.iter().map(|&x| x as f64).collect()))
        }

        #[diplomat::attr(auto, named_constructor = "usize")]
        pub fn new_usize(v: &[usize]) -> Box<Float64Vec> {
            Box::new(Self(v.iter().map(|&x| x as f64).collect()))
        }

        #[diplomat::attr(auto, named_constructor = "f64BeBytes")]
        pub fn new_f64_be_bytes(v: &[DiplomatByte]) -> Box<Float64Vec> {
            Box::new(Self(
                v.chunks_exact(8)
                    .map(|b| f64::from_be_bytes([b[0], b[1], b[2], b[3], b[4], b[5], b[6], b[7]]))
                    .collect(),
            ))
        }

        #[diplomat::attr(supports = memory_sharing, disable)]
        #[diplomat::attr(auto, constructor)]
        pub fn new_from_owned(v: Box<[f64]>) -> Box<Float64Vec> {
            Box::new(Self(v.into()))
        }

        #[diplomat::attr(auto, getter = "asSlice")]
        pub fn as_slice<'a>(&'a self) -> &'a [f64] {
            &self.0
        }

        pub fn fill_slice(&self, v: &mut [f64]) {
            v.copy_from_slice(&self.0)
        }

        pub fn set_value(&mut self, new_slice: &[f64]) {
            self.0 = new_slice.to_vec();
        }

        #[diplomat::attr(auto, stringifier)]
        pub fn to_string(&self, w: &mut DiplomatWrite) {
            let _infallible = write!(w, "{:?}", self.0);
        }

        #[allow(clippy::needless_lifetimes)]
        pub fn borrow<'a>(&'a self) -> &'a [f64] {
            &self.0
        }

        #[diplomat::attr(auto, indexer)]
        pub fn get(&self, i: usize) -> Option<f64> {
            self.0.get(i).copied()
        }
    }
}
