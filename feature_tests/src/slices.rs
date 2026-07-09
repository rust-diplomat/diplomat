#[diplomat::bridge]
pub mod ffi {
    use diplomat_runtime::{DiplomatStr, DiplomatStr16Slice, DiplomatStrSlice, DiplomatWrite};
    use std::fmt::Write as _;

    #[diplomat::opaque_mut]
    #[derive(Debug)]
    pub struct MyString(String);

    impl MyString {
        #[diplomat::attr(auto, constructor)]
        pub fn new(#[diplomat::attr(auto, default_value = 'T')] v: &DiplomatStr) -> Box<MyString> {
            Box::new(Self(String::from_utf8(v.to_owned()).unwrap()))
        }

        #[diplomat::attr(auto, named_constructor = "unsafe")]
        pub fn new_unsafe(v: &str) -> Box<MyString> {
            Box::new(Self(v.to_string()))
        }

        #[diplomat::cfg(supports=owned_slices)]
        pub fn new_owned(v: Box<DiplomatStr>) -> Box<MyString> {
            Box::new(Self(String::from_utf8(v.into()).unwrap()))
        }

        #[diplomat::attr(dotnet, disable)]
        pub fn new_from_first(v: &[DiplomatStrSlice]) -> Box<MyString> {
            Box::new(Self(core::str::from_utf8(v[0].into()).unwrap().into()))
        }

        #[diplomat::attr(dotnet, disable)]
        pub fn new_from_utf16(v: &[DiplomatStr16Slice]) -> Box<MyString> {
            let first: &[u16] = v[0].into();
            Box::new(Self(String::from_utf16(first).unwrap()))
        }

        #[diplomat::attr(auto, setter = "str")]
        pub fn set_str(&mut self, new_str: &DiplomatStr) {
            self.0 = String::from_utf8(new_str.to_owned()).unwrap();
        }

        #[diplomat::attr(auto, getter = "str")]
        pub fn get_str(&self, write: &mut DiplomatWrite) {
            let _infallible = write!(write, "{}", self.0);
        }

        #[diplomat::attr(dotnet, disable)]
        pub fn get_static_str() -> &'static str {
            "hello"
        }

        pub fn string_transform(foo: &str, write: &mut DiplomatWrite) {
            let _ = foo;
            let _ = write;
        }

        #[diplomat::attr(dotnet, disable)]
        pub fn borrow<'a>(&'a self) -> DiplomatStrSlice<'a> {
            AsRef::<[u8]>::as_ref(&self.0).into()
        }

        #[diplomat::cfg(supports=opaque_slices)]
        pub fn slice_of_opaques(sl: &[&MyString], w: &mut DiplomatWrite) {
            let st: String = sl.iter().map(|o| o.0.clone()).collect();
            write!(w, "{}", st).expect("Could not write string.");
        }

        #[diplomat::cfg(supports=opaque_slices)]
        pub fn optional_slice_of_opaques(sl: &[Option<&MyString>], w: &mut DiplomatWrite) {
            for op in sl {
                write!(w, "{:?} ", op).expect("Could not write");
            }
        }

        #[diplomat::cfg(supports=opaque_slices)]
        pub fn other_opaque_type(other: &[&Float64Vec], w: &mut DiplomatWrite) {
            for v in other {
                write!(w, "{:?}", v).expect("Could not write");
            }
        }
    }

    #[diplomat::opaque_mut]
    #[derive(Debug)]
    pub struct Float64Vec(Vec<f64>);

    impl Float64Vec {
        #[diplomat::cfg(supports = memory_sharing)]
        pub fn new(v: &[f64]) -> Box<Float64Vec> {
            Box::new(Self(v.to_vec()))
        }

        #[diplomat::attr(auto, named_constructor = "bool")]
        #[diplomat::attr(dotnet, disable)]
        pub fn new_bool(v: &[bool]) -> Box<Float64Vec> {
            Box::new(Self(v.iter().map(|&x| x as u8 as f64).collect()))
        }

        #[diplomat::attr(auto, named_constructor = "i16")]
        #[diplomat::attr(dotnet, disable)]
        pub fn new_i16(v: &[i16]) -> Box<Float64Vec> {
            Box::new(Self(v.iter().map(|&x| x as f64).collect()))
        }

        #[diplomat::attr(auto, named_constructor = "u16")]
        #[diplomat::attr(dotnet, disable)]
        pub fn new_u16(v: &[u16]) -> Box<Float64Vec> {
            Box::new(Self(v.iter().map(|&x| x as f64).collect()))
        }

        #[diplomat::attr(auto, named_constructor = "isize")]
        #[diplomat::attr(dotnet, disable)]
        pub fn new_isize(v: &[isize]) -> Box<Float64Vec> {
            Box::new(Self(v.iter().map(|&x| x as f64).collect()))
        }

        #[diplomat::attr(auto, named_constructor = "usize")]
        #[diplomat::attr(dotnet, disable)]
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

        #[diplomat::attr(any(supports = memory_sharing, not(supports = owned_slices)), disable)]
        #[diplomat::attr(auto, constructor)]
        pub fn new_from_owned(v: Box<[f64]>) -> Box<Float64Vec> {
            Box::new(Self(v.into()))
        }

        #[diplomat::attr(auto, getter = "asSlice")]
        #[diplomat::attr(dotnet, disable)]
        pub fn as_slice<'a>(&'a self) -> &'a [f64] {
            &self.0
        }

        #[diplomat::cfg(supports=mutable_slices)]
        #[diplomat::attr(dotnet, disable)]
        pub fn fill_slice(&self, v: &mut [f64]) {
            v.copy_from_slice(&self.0)
        }

        #[diplomat::attr(dotnet, disable)]
        pub fn set_value(&mut self, new_slice: &[f64]) {
            self.0 = new_slice.to_vec();
        }

        #[diplomat::attr(auto, stringifier)]
        pub fn to_string(&self, w: &mut DiplomatWrite) {
            let _infallible = write!(w, "{:?}", self.0);
        }

        #[allow(clippy::needless_lifetimes)]
        #[diplomat::attr(dotnet, disable)]
        pub fn borrow<'a>(&'a self) -> &'a [f64] {
            &self.0
        }

        #[diplomat::attr(auto, indexer)]
        pub fn get(&self, i: usize) -> Option<f64> {
            self.0.get(i).copied()
        }
    }

    // Owned opaque returns borrowing a `&[u8]` param: on .NET the param becomes
    // ReadOnlyMemory<u8> pinned for the returned view's lifetime (PR #1201).
    #[diplomat::opaque]
    #[diplomat::attr(not(dotnet), disable)]
    pub struct OpaqueSliceView<'a>(&'a [u8]);

    #[diplomat::opaque]
    #[diplomat::attr(not(dotnet), disable)]
    pub struct SliceParseError;

    impl<'a> OpaqueSliceView<'a> {
        pub fn parse(data: &'a [u8]) -> Result<Box<OpaqueSliceView<'a>>, Box<SliceParseError>> {
            if data.is_empty() {
                Err(Box::new(SliceParseError))
            } else {
                Ok(Box::new(OpaqueSliceView(data)))
            }
        }

        // Errs on a NON-empty buffer (leading zero byte), so the .NET side
        // pins a real GCHandle and then must dispose it on the throw path.
        pub fn parse_strict(
            data: &'a [u8],
        ) -> Result<Box<OpaqueSliceView<'a>>, Box<SliceParseError>> {
            if !data.is_empty() && data[0] == 0 {
                Err(Box::new(SliceParseError))
            } else {
                Ok(Box::new(OpaqueSliceView(data)))
            }
        }

        pub fn wrap(data: &'a [u8]) -> Box<OpaqueSliceView<'a>> {
            Box::new(OpaqueSliceView(data))
        }

        pub fn length(&self) -> u32 {
            self.0.len() as u32
        }

        pub fn get(&self, index: u32) -> u8 {
            self.0.get(index as usize).copied().unwrap_or(0)
        }

        pub fn sum(&self) -> u32 {
            self.0
                .iter()
                .fold(0u32, |acc, &b| acc.wrapping_add(b as u32))
        }
    }

    // Owned `Box<[u8]>` return: on .NET this lowers to a zero-copy `RustVec`
    // (`System.Buffers.MemoryManager<byte>`) wrapping the raw `(ptr, len)`
    // pair directly, rather than copying into a managed `byte[]`.
    #[diplomat::opaque]
    #[diplomat::cfg(supports=owned_slice_returns)]
    pub struct OwnedSliceReturn;

    impl OwnedSliceReturn {
        /// Returns an owned `Box<[u8]>` of `len` bytes, each set to `(i % 256) as u8`.
        /// `len == 0` exercises the empty-buffer case; a large `len` exercises the
        /// GC memory-pressure path.
        pub fn make_bytes(len: u32) -> Box<[u8]> {
            (0..len).map(|i| (i % 256) as u8).collect()
        }
    }

    // For testing throwing IndexError:
    #[diplomat::opaque]
    #[diplomat::cfg(nanobind)]
    struct Float64VecError(Vec<f64>);

    impl Float64VecError {
        #[diplomat::cfg(supports = memory_sharing)]
        pub fn new(v: &[f64]) -> Box<Float64VecError> {
            Box::new(Self(v.to_vec()))
        }

        #[diplomat::attr(auto, indexer)]
        pub fn get(&self, i: usize) -> Result<f64, ()> {
            if let Some(i) = self.0.get(i) {
                Ok(*i)
            } else {
                Err(())
            }
        }
    }
}
