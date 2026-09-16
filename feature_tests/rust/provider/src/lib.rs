//! Owner-side cdylib for the Safe Rust backend fixture.
//!
//! The generated consumer is a separate Cargo package and has no dependency
//! on this crate. All allocation and destruction stay in this owner.

#![allow(clippy::needless_lifetimes)]

use std::sync::atomic::{AtomicUsize, Ordering};

static COUNTER_DROPS: AtomicUsize = AtomicUsize::new(0);
static NEXT_ID: AtomicUsize = AtomicUsize::new(1);

#[diplomat::bridge]
pub mod ffi {
    use super::{Ordering, COUNTER_DROPS, NEXT_ID};
    use diplomat_runtime::{DiplomatStr, DiplomatStr16, DiplomatStrSlice};

    pub enum Mode {
        Idle = 0,
        Active = 1,
    }

    pub struct Snapshot {
        pub value: u32,
        pub active: bool,
        pub mode: Mode,
    }

    /// Owner-side mutable value. The generated Rust API is expected to expose
    /// Counter, CounterRef<'a>, and CounterRefMut<'a>.
    #[diplomat::opaque_mut]
    pub struct Counter {
        value: u32,
        identity: usize,
        child: Child,
    }

    /// A distinct owner-side opaque returned only as a borrow from Counter.
    #[diplomat::opaque_mut]
    pub struct Child {
        value: u32,
        owner_identity: usize,
    }

    impl Counter {
        pub fn new() -> Box<Self> {
            let identity = NEXT_ID.fetch_add(1, Ordering::SeqCst);
            Box::new(Self {
                value: 0,
                identity,
                child: Child {
                    value: 0,
                    owner_identity: identity,
                },
            })
        }

        pub fn maybe_new(present: bool) -> Option<Box<Self>> {
            present.then(Self::new)
        }

        pub fn get(&self) -> u32 {
            self.value
        }

        pub fn identity(&self) -> usize {
            self.identity
        }

        pub fn increment(&mut self) {
            self.value += 1;
            self.child.value = self.value;
        }

        pub fn add(&mut self, amount: Option<u32>) -> Option<u32> {
            self.value += amount?;
            self.child.value = self.value;
            Some(self.value)
        }

        pub fn snapshot(&self) -> Snapshot {
            Snapshot {
                value: self.value,
                active: self.value != 0,
                mode: if self.value == 0 {
                    Mode::Idle
                } else {
                    Mode::Active
                },
            }
        }

        pub fn maybe_snapshot(&self, present: bool) -> Option<Snapshot> {
            present.then(|| self.snapshot())
        }

        pub fn view<'a>(&'a self) -> &'a Self {
            self
        }

        pub fn view_mut<'a>(&'a mut self) -> &'a mut Self {
            self
        }

        pub fn same_identity(&self, other: &Self) -> bool {
            self.identity == other.identity
        }

        pub fn copy_value_from(&mut self, other: &Self) -> u32 {
            self.value = other.value;
            self.child.value = self.value;
            self.value
        }

        pub fn exchange_values(&mut self, other: &mut Self) {
            core::mem::swap(&mut self.value, &mut other.value);
            self.child.value = self.value;
            other.child.value = other.value;
        }

        pub fn child<'a>(&'a self) -> &'a Child {
            &self.child
        }

        pub fn child_short<'short, 'long: 'short>(&'long self) -> &'short Child {
            &self.child
        }

        pub fn child_mut<'a>(&'a mut self) -> &'a mut Child {
            &mut self.child
        }

        pub fn maybe_child<'a>(&'a self, present: bool) -> Option<&'a Child> {
            present.then_some(&self.child)
        }

        pub fn reset_drop_count() {
            COUNTER_DROPS.store(0, Ordering::SeqCst);
        }

        pub fn drop_count() -> usize {
            COUNTER_DROPS.load(Ordering::SeqCst)
        }
    }

    impl Child {
        pub fn get(&self) -> u32 {
            self.value
        }

        pub fn set(&mut self, value: u32) {
            self.value = value;
        }

        pub fn owner_identity(&self) -> usize {
            self.owner_identity
        }
    }

    /// Opaque with a primitive slice, exercising borrowed slice inputs/outputs.
    #[diplomat::opaque_mut]
    pub struct Numbers {
        values: Vec<u32>,
    }

    impl Numbers {
        pub fn new(values: &[u32]) -> Box<Self> {
            Box::new(Self {
                values: values.to_vec(),
            })
        }

        pub fn values<'a>(&'a self) -> &'a [u32] {
            &self.values
        }

        pub fn values_mut<'a>(&'a mut self) -> &'a mut [u32] {
            &mut self.values
        }

        pub fn sum(&self) -> u32 {
            self.values.iter().copied().sum()
        }

        pub fn total<'a>(fields: FieldView<'a>) -> u32 {
            fields.count + fields.bytes.iter().map(|&b| b as u32).sum::<u32>()
        }

        pub fn fill(&self, out: &mut [u32]) {
            out.copy_from_slice(&self.values);
        }
    }

    /// Opaque owning a string, exercising `DiplomatStr`/`&str` borrows.
    #[diplomat::opaque_mut]
    pub struct Message(String);

    impl Message {
        pub fn new(v: &DiplomatStr) -> Box<Self> {
            Box::new(Self(String::from_utf8(v.to_vec()).unwrap()))
        }

        pub fn bytes<'a>(&'a self) -> DiplomatStrSlice<'a> {
            self.0.as_bytes().into()
        }

        pub fn text<'a>(&'a self) -> &'a str {
            &self.0
        }
    }

    /// Owned opaque whose type-level lifetime is tied to the caller's buffer.
    #[diplomat::opaque]
    pub struct SliceView<'a>(&'a [u8]);

    impl<'a> SliceView<'a> {
        pub fn wrap(data: &'a [u8]) -> Box<Self> {
            Box::new(Self(data))
        }

        pub fn len(&self) -> u32 {
            self.0.len() as u32
        }

        pub fn is_empty(&self) -> bool {
            self.0.is_empty()
        }

        pub fn get(&self, index: u32) -> u8 {
            self.0.get(index as usize).copied().unwrap_or(0)
        }

        pub fn sum(&self) -> u32 {
            self.0
                .iter()
                .fold(0u32, |acc, &b| acc.wrapping_add(b as u32))
        }

        pub fn data<'b>(&'b self) -> &'b [u8] {
            self.0
        }
    }

    /// Value struct with borrowed slice fields, returned by `SliceView::fields`.
    pub struct FieldView<'a> {
        bytes: DiplomatStrSlice<'a>,
        count: u32,
    }

    impl<'a> SliceView<'a> {
        pub fn fields(&self) -> FieldView<'a> {
            FieldView {
                bytes: self.0.into(),
                count: self.0.len() as u32,
            }
        }
    }

    /// Owned `Box<[u8]>` returns: the provider allocates and hands ownership across.
    #[diplomat::opaque]
    pub struct Bytes;

    impl Bytes {
        pub fn make(len: u32) -> Box<[u8]> {
            (0..len).map(|i| (i % 256) as u8).collect()
        }

        pub fn join<'a>(a: &'a [u8], b: &'a [u8]) -> Box<[u8]> {
            let mut out = Vec::with_capacity(a.len() + b.len());
            out.extend_from_slice(a);
            out.extend_from_slice(b);
            out.into_boxed_slice()
        }
    }

    // ---- capability-gated shapes -------------------------------------------------
    //
    // Every API below sits behind a `BackendAttrSupport` flag this backend claims.
    // They exist so the claims are exercised by the fixture rather than only
    // declared: if a flag is claimed but not honoured, the gated API is missing from
    // the generated package and the consumer tests stop compiling.

    impl Counter {
        /// An explicit named-constructor name becomes the generated function name.
        #[diplomat::attr(auto, named_constructor = "with_value")]
        pub fn new_named(value: u32) -> Box<Self> {
            let mut counter = Self::new();
            counter.value = value;
            counter.child.value = value;
            counter
        }
    }

    impl Message {
        #[diplomat::cfg(supports = utf8_strings)]
        pub fn utf8_len(&self) -> u32 {
            self.0.len() as u32
        }
    }

    /// UTF-16 strings in both directions, gated on `utf16_strings`.
    #[diplomat::opaque]
    pub struct WideMessage(Vec<u16>);

    impl WideMessage {
        #[diplomat::cfg(supports = utf16_strings)]
        pub fn new(v: &DiplomatStr16) -> Box<Self> {
            Box::new(Self(v.to_vec()))
        }

        #[diplomat::cfg(supports = utf16_strings)]
        pub fn units<'a>(&'a self) -> &'a DiplomatStr16 {
            &self.0
        }
    }

    impl Numbers {
        /// Borrowed slice pointing at caller-owned memory, gated on `memory_sharing`.
        #[diplomat::cfg(supports = memory_sharing)]
        pub fn from_slice(values: &[u32]) -> Box<Self> {
            Box::new(Self {
                values: values.to_vec(),
            })
        }
    }

    /// Float scalars and a float slice. The constructor is the exact shape the shared
    /// corpus gates on `memory_sharing` (`feature_tests/src/slices.rs`:
    /// `pub fn new(v: &[f64])`), so it holds that shape against this backend's ABI.
    #[diplomat::opaque_mut]
    pub struct Float64Vec(Vec<f64>);

    impl Float64Vec {
        #[diplomat::cfg(supports = memory_sharing)]
        pub fn new(values: &[f64]) -> Box<Self> {
            Box::new(Self(values.to_vec()))
        }

        pub fn sum(&self) -> f64 {
            self.0.iter().sum()
        }

        pub fn get(&self, index: u32) -> f64 {
            self.0.get(index as usize).copied().unwrap_or(f64::NAN)
        }

        pub fn scale_in_place(&mut self, factor: f64) {
            for value in &mut self.0 {
                *value *= factor;
            }
        }
    }

    impl Drop for Counter {
        fn drop(&mut self) {
            COUNTER_DROPS.fetch_add(1, Ordering::SeqCst);
        }
    }
}
