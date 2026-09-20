#[diplomat::bridge]
pub mod ffi {
    use std::sync::atomic::Ordering;

    use crate::{COUNTER_DROPS, FAILURE_DROPS, NEXT_ID};
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

        /// `Option` over a primitive, in the parameter and in the return, gated on
        /// `option`. Lowering is what checks the flag, and it only consults it for
        /// `Option<struct/enum/primitive>` — a nullable owned opaque is a different
        /// lowering path that this flag does not govern.
        #[diplomat::cfg(supports = option)]
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

        /// `Option<Struct>`, the struct arm of the same `option` lowering check.
        #[diplomat::cfg(supports = option)]
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

        /// `&mut [T]` in the return position, gated on `mutable_slices`.
        #[diplomat::cfg(supports = mutable_slices)]
        pub fn values_mut<'a>(&'a mut self) -> &'a mut [u32] {
            &mut self.values
        }

        pub fn sum(&self) -> u32 {
            self.values.iter().copied().sum()
        }

        pub fn total<'a>(fields: FieldView<'a>) -> u32 {
            fields.count + fields.bytes.iter().map(|&b| b as u32).sum::<u32>()
        }

        /// `&mut [T]` in the parameter position, gated on the same flag.
        #[diplomat::cfg(supports = mutable_slices)]
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
        /// An owned byte-slice return: ownership of the provider's allocation crosses
        /// the ABI, which is what `owned_byte_slice_returns` promises.
        #[diplomat::cfg(supports = owned_byte_slice_returns)]
        pub fn make(len: u32) -> Box<[u8]> {
            (0..len).map(|i| (i % 256) as u8).collect()
        }

        /// The same flag with borrowed-slice parameters alongside the owned return.
        #[diplomat::cfg(supports = owned_byte_slice_returns)]
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
        ///
        /// The `cfg` guard is load-bearing rather than redundant: `attr(auto, ...)` only
        /// checks support through `BackendAttrSupport::check_string`, whose keys are the
        /// flag names (`named_constructors`), while the attribute path it is called with
        /// is singular (`named_constructor`). That lookup misses, so the `auto` gate is
        /// skipped and this attribute would be applied even with the flag off. Guarding
        /// the method explicitly is what makes `named_constructors` load-bearing here.
        #[diplomat::cfg(supports = named_constructors)]
        #[diplomat::attr(auto, named_constructor = "with_value")]
        pub fn new_named(value: u32) -> Box<Self> {
            let mut counter = Self::new();
            counter.value = value;
            counter.child.value = value;
            counter
        }

        /// A plain `#[diplomat::attr(auto, constructor)]`, gated on `constructors`.
        /// Rust lowers this to an ordinary associated function, so the flag's promise is
        /// only that such a method is accepted and emitted at all.
        #[diplomat::cfg(supports = constructors)]
        #[diplomat::attr(auto, constructor)]
        pub fn from_value(value: u32) -> Box<Self> {
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

        /// A `'static` slice parameter, gated on `static_slices`: the borrow carries no
        /// caller lifetime, so the generated signature must not introduce one.
        #[diplomat::cfg(supports = static_slices)]
        pub fn from_static(values: &'static [u32]) -> Box<Self> {
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

    /// A custom error payload.
    ///
    /// `#[diplomat::attr(auto, error)]` is what declares a type usable as a `Result`
    /// error, and the generated backend enforces it: an unmarked enum is refused with a
    /// diagnostic rather than quietly accepted.
    #[diplomat::attr(auto, error)]
    pub enum ValueError {
        TooLarge = 0,
    }

    /// An error that owns a native allocation.
    ///
    /// A `Result`'s error payload crosses the ABI as a raw pointer, so the generated
    /// wrapper is its only owner: a `?` that discards the error still has to run this
    /// destructor.
    #[diplomat::opaque]
    pub struct AllocationFailure {
        code: u32,
    }

    impl AllocationFailure {
        pub fn code(&self) -> u32 {
            self.code
        }

        pub fn reset_drop_count() {
            FAILURE_DROPS.store(0, Ordering::SeqCst);
        }

        pub fn drop_count() -> usize {
            FAILURE_DROPS.load(Ordering::SeqCst)
        }
    }

    impl Drop for AllocationFailure {
        fn drop(&mut self) {
            FAILURE_DROPS.fetch_add(1, Ordering::SeqCst);
        }
    }

    impl Counter {
        /// A fallible call whose error is a custom enum, gated on `custom_errors`.
        #[diplomat::cfg(supports = custom_errors)]
        pub fn try_from_value(value: u32) -> Result<Box<Self>, ValueError> {
            if value > 100 {
                return Err(ValueError::TooLarge);
            }
            let mut counter = Self::new();
            counter.value = value;
            counter.child.value = value;
            Ok(counter)
        }

        /// A fallible call whose error owns a native allocation, gated on the same flag.
        #[diplomat::cfg(supports = custom_errors)]
        pub fn take(&mut self, n: u32) -> Result<u32, Box<AllocationFailure>> {
            if n > self.value {
                return Err(Box::new(AllocationFailure { code: n }));
            }
            self.value -= n;
            self.child.value = self.value;
            Ok(n)
        }
    }

    impl Drop for Counter {
        fn drop(&mut self) {
            COUNTER_DROPS.fetch_add(1, Ordering::SeqCst);
        }
    }
}
