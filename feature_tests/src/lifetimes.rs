#[diplomat::bridge]
pub mod ffi {
    use std::fmt::Write;

    use diplomat_runtime::DiplomatStr16;

    #[diplomat::opaque]
    pub struct Foo<'a>(&'a DiplomatStr);

    #[diplomat::opaque]
    #[diplomat::transparent_convert]
    pub struct Bar<'b, 'a: 'b>(&'b Foo<'a>);

    #[diplomat::attr(dotnet, disable)]
    pub struct BorrowedFields<'a> {
        a: DiplomatStr16Slice<'a>,
        b: DiplomatStrSlice<'a>,
        c: DiplomatUtf8StrSlice<'a>,
    }

    #[diplomat::attr(dotnet, disable)]
    pub struct BorrowedFieldsWithBounds<'a, 'b: 'a, 'c: 'b> {
        field_a: DiplomatStr16Slice<'a>,
        field_b: DiplomatStrSlice<'b>,
        field_c: DiplomatUtf8StrSlice<'c>,
    }

    #[diplomat::attr(dotnet, disable)]
    pub struct BorrowedFieldsReturning<'a> {
        bytes: DiplomatStrSlice<'a>,
    }
    impl<'a> Foo<'a> {
        #[diplomat::attr(auto, constructor)]
        #[diplomat::attr(dotnet, disable)]
        pub fn new(x: &'a DiplomatStr) -> Box<Self> {
            Box::new(Foo(x))
        }

        #[diplomat::attr(auto, getter = "bar")]
        pub fn get_bar<'b>(&'b self) -> Box<Bar<'b, 'a>> {
            Box::new(Bar(self))
        }

        #[diplomat::attr(auto, named_constructor = "static")]
        #[diplomat::cfg(supports = static_slices)]
        pub fn new_static(x: &'static DiplomatStr) -> Box<Self> {
            Box::new(Foo(x))
        }

        #[diplomat::attr(dotnet, disable)]
        pub fn as_returning(&self) -> BorrowedFieldsReturning<'a> {
            BorrowedFieldsReturning {
                bytes: self.0.into(),
            }
        }

        #[diplomat::attr(auto, named_constructor)]
        #[diplomat::attr(dotnet, disable)]
        pub fn extract_from_fields(fields: BorrowedFields<'a>) -> Box<Self> {
            Box::new(Foo(fields.b.into()))
        }

        // Don't yet support borrowing from slices
        #[diplomat::attr(auto, named_constructor)]
        #[diplomat::attr(dotnet, disable)]
        /// Test that the extraction logic correctly pins the right fields
        pub fn extract_from_bounds<'x, 'y: 'x + 'a, 'z: 'x + 'y>(
            bounds: BorrowedFieldsWithBounds<'x, 'y, 'z>,
            another_string: &'a DiplomatStr,
        ) -> Box<Self> {
            if bounds.field_b.is_empty() {
                Box::new(Self(another_string))
            } else {
                Box::new(Self(bounds.field_b.into()))
            }
        }
    }

    impl<'x> BorrowedFields<'x> {
        pub fn from_bar_and_strings(
            bar: &'x Bar<'x, 'x>,
            dstr16: &'x DiplomatStr16,
            utf8_str: &'x str,
        ) -> Self {
            BorrowedFields {
                a: dstr16.into(),
                b: bar.0 .0.into(),
                c: utf8_str.into(),
            }
        }
    }

    impl<'x, 'y: 'x, 'z: 'y> BorrowedFieldsWithBounds<'x, 'y, 'z> {
        pub fn from_foo_and_strings(
            foo: &'x Foo<'y>,
            dstr16_x: &'x DiplomatStr16,
            utf8_str_z: &'z str,
        ) -> Self {
            BorrowedFieldsWithBounds {
                field_a: dstr16_x.into(),
                field_b: foo.0.into(),
                field_c: utf8_str_z.into(),
            }
        }
    }

    #[diplomat::attr(dotnet, disable)]
    pub struct NestedBorrowedFields<'x, 'y: 'x, 'z> {
        fields: BorrowedFields<'x>,
        bounds: BorrowedFieldsWithBounds<'x, 'y, 'y>,
        bounds2: BorrowedFieldsWithBounds<'z, 'z, 'z>,
    }

    impl<'x, 'y: 'x, 'z> NestedBorrowedFields<'x, 'y, 'z> {
        pub fn from_bar_and_foo_and_strings(
            bar: &'x Bar<'x, 'y>,
            foo: &'z Foo<'z>,
            dstr16_x: &'x DiplomatStr16,
            dstr16_z: &'z DiplomatStr16,
            utf8_str_y: &'y str,
            utf8_str_z: &'z str,
        ) -> Self {
            let fields = BorrowedFields::from_bar_and_strings(bar, dstr16_x, utf8_str_y);
            let bounds =
                BorrowedFieldsWithBounds::from_foo_and_strings(bar.0, dstr16_x, utf8_str_y);
            let bounds2 = BorrowedFieldsWithBounds::from_foo_and_strings(foo, dstr16_z, utf8_str_z);
            Self {
                fields,
                bounds,
                bounds2,
            }
        }
    }

    // FIXME(#191): This test breaks the C++ codegen
    impl<'b, 'a: 'b> Bar<'b, 'a> {
        #[diplomat::attr(auto, getter)]
        pub fn foo(&'b self) -> &'b Foo<'a> {
            self.0
        }
    }

    #[derive(Copy, Clone)]
    #[diplomat::opaque]
    pub struct One<'a>(super::One<'a>);

    #[derive(Copy, Clone)]
    #[diplomat::opaque]
    pub struct Two<'a, 'b>(super::Two<'a, 'b>);

    impl<'o> One<'o> {
        // Holds: [hold]
        #[allow(clippy::extra_unused_lifetimes)]
        #[diplomat::attr(auto, named_constructor)]
        pub fn transitivity<'a, 'b: 'a, 'c: 'b, 'd: 'c, 'e: 'd, 'x>(
            hold: &'x One<'e>,
            nohold: &One<'x>,
        ) -> Box<One<'a>> {
            let _ = (hold, nohold);
            unimplemented!()
        }

        // Holds: [hold]
        #[allow(clippy::extra_unused_lifetimes)]
        #[diplomat::attr(auto, named_constructor)]
        pub fn cycle<'a: 'b, 'b: 'c, 'c: 'a, 'x>(
            hold: &Two<'x, 'b>,
            nohold: &'x One<'x>,
        ) -> Box<One<'a>> {
            let _ = (hold, nohold);
            unimplemented!()
        }

        // Holds: [a, b, c, d]
        #[diplomat::attr(auto, named_constructor)]
        pub fn many_dependents<'a, 'b: 'a, 'c: 'a, 'd: 'b + 'x, 'x, 'y>(
            a: &'x One<'a>,
            b: &'b One<'a>,
            c: &Two<'x, 'c>,
            d: &'x Two<'d, 'y>,
            nohold: &'x Two<'x, 'y>,
        ) -> Box<One<'a>> {
            let _ = (a, b, c, d, nohold);
            unimplemented!()
        }

        // Holds: [hold]
        #[diplomat::attr(auto, named_constructor)]
        pub fn return_outlives_param<'short, 'long: 'short>(
            hold: &Two<'long, 'short>,
            nohold: &'short One<'short>,
        ) -> Box<One<'long>> {
            let _ = (hold, nohold);
            unimplemented!()
        }

        // Holds: [top, left, right, bottom]
        #[diplomat::attr(auto, named_constructor)]
        pub fn diamond_top<'top, 'left: 'top, 'right: 'top, 'bottom: 'left + 'right>(
            top: &One<'top>,
            left: &One<'left>,
            right: &One<'right>,
            bottom: &One<'bottom>,
        ) -> Box<One<'top>> {
            Box::new(match 0 {
                0 => *bottom,
                1 => *left,
                2 => *right,
                _ => *top,
            })
        }

        // Holds: [left, bottom]
        #[diplomat::attr(auto, named_constructor)]
        pub fn diamond_left<'top, 'left: 'top, 'right: 'top, 'bottom: 'left + 'right>(
            top: &One<'top>,
            left: &One<'left>,
            right: &One<'right>,
            bottom: &One<'bottom>,
        ) -> Box<One<'left>> {
            let _ = (top, right);
            Box::new(match 0 {
                0 => *bottom,
                _ => *left,
            })
        }

        // Holds: [right, bottom]
        #[diplomat::attr(auto, named_constructor)]
        pub fn diamond_right<'top, 'left: 'top, 'right: 'top, 'bottom: 'left + 'right>(
            top: &One<'top>,
            left: &One<'left>,
            right: &One<'right>,
            bottom: &One<'bottom>,
        ) -> Box<One<'right>> {
            let _ = (top, left);
            Box::new(match 0 {
                0 => *bottom,
                _ => *right,
            })
        }

        // Holds: [bottom]
        #[diplomat::attr(auto, named_constructor)]
        pub fn diamond_bottom<'top, 'left: 'top, 'right: 'top, 'bottom: 'left + 'right>(
            top: &One<'top>,
            left: &One<'left>,
            right: &One<'right>,
            bottom: &One<'bottom>,
        ) -> Box<One<'bottom>> {
            let _ = (top, left, right);
            Box::new(*bottom)
        }

        // Holds: [a, b, c, d]
        #[diplomat::attr(auto, named_constructor)]
        pub fn diamond_and_nested_types<'a, 'b: 'a, 'c: 'b, 'd: 'b + 'c, 'x, 'y>(
            a: &One<'a>,
            b: &'y One<'b>,
            c: &One<'c>,
            d: &One<'d>,
            nohold: &One<'x>,
        ) -> Box<One<'a>> {
            let _ = nohold;
            Box::new(match 0 {
                0 => *a,
                1 => *b,
                2 => *c,
                _ => *d,
            })
        }

        // Holds: [implicit_hold, explicit_hold]
        #[allow(clippy::extra_unused_lifetimes)]
        #[diplomat::attr(auto, named_constructor)]
        pub fn implicit_bounds<'a, 'b: 'a, 'c: 'b, 'd: 'c, 'x, 'y>(
            explicit_hold: &'d One<'x>, // implies that 'x: 'd
            implicit_hold: &One<'x>,
            nohold: &One<'y>,
        ) -> Box<One<'a>> {
            let _ = nohold;
            Box::new(match 0 {
                0 => *explicit_hold,
                _ => *implicit_hold,
            })
        }

        // Holds: [a, b, c]
        #[allow(clippy::needless_lifetimes)]
        #[diplomat::attr(auto, named_constructor)]
        pub fn implicit_bounds_deep<'a, 'b, 'c, 'd, 'x>(
            explicit_: &'a One<'b>,
            implicit_1: &'b One<'c>,
            implicit_2: &'c One<'d>,
            nohold: &'x One<'x>,
        ) -> Box<One<'a>> {
            let _ = nohold;
            Box::new(match 0 {
                0 => *explicit_,
                1 => *implicit_1,
                _ => *implicit_2,
            })
        }
    }

    // Test a common iterator pattern for exposing vectors of native elements
    // The Vec type stores the underlying type, and when it returns accessors, it does so
    // via transparent_convert and non-owning references. Iterators, iterables, and getters
    // are all handled via attributes, which may have slightly different codepaths.
    #[diplomat::opaque]
    #[diplomat::attr(dotnet, manually_disposable)]
    #[diplomat::transparent_convert]
    #[diplomat::attr(demo_gen, disable)]
    pub struct OpaqueThin(pub crate::lifetimes::Internal);

    impl OpaqueThin {
        #[diplomat::attr(auto, getter)]
        pub fn a(&self) -> i32 {
            self.0.a
        }
        #[diplomat::attr(auto, getter)]
        pub fn b(&self) -> f32 {
            self.0.b
        }

        #[diplomat::attr(auto, getter)]
        pub fn c(&self, w: &mut DiplomatWrite) {
            w.write_str(&self.0.c).unwrap();
        }
    }

    #[diplomat::opaque_mut]
    pub struct OpaqueThinIter<'a>(pub std::slice::Iter<'a, crate::lifetimes::Internal>);

    impl<'a> OpaqueThinIter<'a> {
        #[diplomat::attr(auto, iterator)]
        pub fn next(&'a mut self) -> Option<&'a OpaqueThin> {
            self.0.next().map(OpaqueThin::transparent_convert)
        }
    }

    #[diplomat::opaque_mut]
    #[diplomat::attr(dotnet, manually_disposable)]
    pub struct OpaqueThinVec(std::vec::Vec<crate::lifetimes::Internal>);

    impl OpaqueThinVec {
        #[diplomat::attr(auto, constructor)]
        #[diplomat::attr(dotnet, disable)]
        pub fn create(a: &[i32], b: &[f32], c: &DiplomatStr) -> Box<Self> {
            assert!(a.len() == b.len(), "arrays must be of equal size");
            Box::new(Self(
                a.iter()
                    .zip(b.iter())
                    .map(|(a, b)| crate::lifetimes::Internal {
                        a: *a,
                        b: *b,
                        c: String::from_utf8(c.to_vec()).unwrap(),
                    })
                    .collect(),
            ))
        }

        // The .NET backend disables the slice-based `create`, so the dotnet
        // borrowed-return tests need a constructor they can call from C# to get
        // a real owner to borrow `First()`/`Get()` out of.
        #[diplomat::attr(not(dotnet), disable)]
        pub fn create_single(a: i32, b: f32, c: &DiplomatStr) -> Box<Self> {
            Box::new(Self(vec![crate::lifetimes::Internal {
                a,
                b,
                c: String::from_utf8(c.to_vec()).unwrap(),
            }]))
        }

        // dotnet-only: the borrowed-return aliasing test replaces the owner's
        // heap-backed `String` here and reads it back through `first` to prove
        // the borrow isn't a copy. There is no matching getter, so this is also
        // the one accessor here that has to render as a write-only property.
        #[diplomat::attr(not(dotnet), disable)]
        #[diplomat::attr(auto, setter = "first_c")]
        pub fn set_first_c(&mut self, value: &DiplomatStr) {
            if let Some(first) = self.0.first_mut() {
                first.c = String::from_utf8(value.to_vec()).unwrap();
            }
        }

        #[diplomat::attr(auto, iterable)]
        #[allow(clippy::should_implement_trait)]
        pub fn iter<'a>(&'a self) -> Box<OpaqueThinIter<'a>> {
            Box::new(OpaqueThinIter(self.0.iter()))
        }

        #[diplomat::attr(nanobind, rename = "__len__")]
        #[allow(clippy::len_without_is_empty)]
        pub fn len(&self) -> usize {
            self.0.len()
        }

        #[diplomat::attr(auto, indexer)]
        pub fn get<'a>(&'a self, idx: usize) -> Option<&'a OpaqueThin> {
            self.0.get(idx).map(OpaqueThin::transparent_convert)
        }

        #[diplomat::attr(auto, getter)]
        #[diplomat::attr(dart, rename = "firstelement")]
        pub fn first<'a>(&'a self) -> Option<&'a OpaqueThin> {
            self.0.get(0).map(OpaqueThin::transparent_convert)
        }

        #[diplomat::attr(not(dotnet), disable)]
        pub fn try_first<'a>(&'a self, fail: bool) -> Result<&'a OpaqueThin, ()> {
            if fail {
                Err(())
            } else {
                self.0
                    .first()
                    .map(OpaqueThin::transparent_convert)
                    .ok_or(())
            }
        }

        #[diplomat::attr(not(dotnet), disable)]
        pub fn try_get<'a>(&'a self, idx: usize, fail: bool) -> Result<Option<&'a OpaqueThin>, ()> {
            if fail {
                Err(())
            } else {
                Ok(self.0.get(idx).map(OpaqueThin::transparent_convert))
            }
        }

        #[diplomat::attr(not(dotnet), disable)]
        pub fn try_iter<'a>(&'a self, fail: bool) -> Result<Box<OpaqueThinIter<'a>>, ()> {
            if fail {
                Err(())
            } else {
                Ok(Box::new(OpaqueThinIter(self.0.iter())))
            }
        }

        #[diplomat::attr(not(dotnet), disable)]
        pub fn optional_iter<'a>(&'a self, some: bool) -> Option<Box<OpaqueThinIter<'a>>> {
            if some {
                Some(Box::new(OpaqueThinIter(self.0.iter())))
            } else {
                None
            }
        }

        // Ok is owned (no edges), so any keep-alive edges ride on the thrown
        // exception and its inner error rather than on a success wrapper.
        #[diplomat::attr(not(dotnet), disable)]
        pub fn try_borrow<'a>(&'a self, fail: bool) -> Result<i32, Box<BorrowingError<'a>>> {
            if fail {
                Err(Box::new(BorrowingError(self)))
            } else {
                Ok(i32::try_from(self.0.len()).unwrap())
            }
        }
    }

    // A borrowing opaque error: a non-owning reference into the Vec it came
    // from, so a caught exception must root that owner or reads back through
    // the borrow would dangle.
    #[diplomat::attr(not(dotnet), disable)]
    #[diplomat::opaque]
    pub struct BorrowingError<'a>(&'a OpaqueThinVec);

    impl<'a> BorrowingError<'a> {
        // A real non-owning view into the owner's storage rather than a
        // copied-out value, so reads go through the live borrow into the owner.
        pub fn owner_first<'b>(&'b self) -> Option<&'b OpaqueThin> {
            let owner = self.0;
            owner.0.first().map(OpaqueThin::transparent_convert)
        }
    }

    // GC-race probe for the GC.KeepAlive fix: `drops_during_spin` sleeps without
    // touching `self`, then reports drops during the call — >= 1 means the
    // receiver was finalized mid-call (the UAF). dotnet-only to avoid churning
    // other backends.
    #[diplomat::attr(not(dotnet), disable)]
    #[diplomat::opaque]
    pub struct GcRaceProbe(u64);

    impl GcRaceProbe {
        pub fn create() -> Box<Self> {
            Box::new(GcRaceProbe(0))
        }

        pub fn drops_during_spin(&self, millis: u64) -> u64 {
            let before = super::PROBE_DROPS.load(super::Ordering::SeqCst);
            std::thread::sleep(std::time::Duration::from_millis(millis));
            super::PROBE_DROPS.load(super::Ordering::SeqCst) - before
        }
    }

    // Dedicated drop probes for dotnet opt-in IDisposable behavior:
    // one unmarked opaque (finalizer-only default), and one opt-in opaque.
    #[diplomat::attr(not(dotnet), disable)]
    #[diplomat::opaque]
    pub struct DefaultDropProbe;

    impl DefaultDropProbe {
        pub fn create() -> Box<Self> {
            Box::new(Self)
        }

        pub fn reset_drop_count() {
            super::DEFAULT_DROP_PROBE_DROPS.store(0, super::Ordering::SeqCst);
        }

        pub fn drop_count() -> u64 {
            super::DEFAULT_DROP_PROBE_DROPS.load(super::Ordering::SeqCst)
        }
    }

    #[diplomat::attr(not(dotnet), disable)]
    #[diplomat::attr(dotnet, manually_disposable)]
    #[diplomat::opaque]
    pub struct DisposableDropProbe;

    impl DisposableDropProbe {
        pub fn create() -> Box<Self> {
            Box::new(Self)
        }

        /// Exists so C# tests can observe that Dispose() invalidates the wrapper.
        pub fn is_alive(&self) -> bool {
            true
        }

        pub fn reset_drop_count() {
            super::DISPOSABLE_DROP_PROBE_DROPS.store(0, super::Ordering::SeqCst);
        }

        pub fn drop_count() -> u64 {
            super::DISPOSABLE_DROP_PROBE_DROPS.load(super::Ordering::SeqCst)
        }
    }

    // ── .NET non-atomic borrow-dependency RC fixtures ──────────────────────
    //
    // These exercise the .NET-only reference-counted borrow-dependency
    // mechanism directly (see `tool/templates/dotnet/RustHandle.cs.jinja`):
    // an IDisposable opt-in "source", a borrowed (non-owning) "view" of it,
    // an owned-but-borrowing "dependent" that has its own Rust destructor
    // while holding a reference into the source (a direct RC edge), a second
    // layer of transitive dependency (a dependent of a dependent — only the
    // *direct* edge at each layer is ever recorded by the generator; the
    // full chain is only reachable by each layer's own recursive Release()),
    // and a finalizer-only (non-opt-in) parent/child pair for the same
    // destruction-ordering invariant exercised via the finalizer path
    // instead of explicit `Dispose()`.
    //
    // A shared logical clock plus a per-type "drop sequence" cell let tests
    // assert *relative* destruction order deterministically (dependent
    // destroyed strictly before its source) instead of depending on GC
    // timing. `reset_drop_stats()`/`drop_count()`/`drop_seq()` follow the
    // existing `DefaultDropProbe`/`DisposableDropProbe` static-method
    // convention above.

    #[diplomat::attr(not(dotnet), disable)]
    #[diplomat::attr(dotnet, manually_disposable)]
    #[diplomat::opaque]
    pub struct RcSource(u64);

    impl RcSource {
        pub fn create(id: u64) -> Box<Self> {
            Box::new(Self(id))
        }

        pub fn id(&self) -> u64 {
            self.0
        }

        /// A borrowed (non-owning) view of `self`: the RC dependency case
        /// with no Rust destructor of its own — releasing it only ever
        /// decrements `self`'s refcount.
        pub fn view<'b>(&'b self) -> &'b Self {
            self
        }

        /// An owned wrapper with its own Rust destructor that also borrows
        /// `self`'s lifetime — the RC "destroy self, then release the
        /// dependency" (owned-borrowing) case.
        pub fn make_dependent<'b>(&'b self) -> Box<RcDependent<'b>> {
            Box::new(RcDependent(self, self.0))
        }

        pub fn reset_drop_stats() {
            super::RC_SOURCE_DROPS.store(0, super::Ordering::SeqCst);
            super::RC_SOURCE_DROP_SEQ.store(0, super::Ordering::SeqCst);
        }

        pub fn drop_count() -> u64 {
            super::RC_SOURCE_DROPS.load(super::Ordering::SeqCst)
        }

        pub fn drop_seq() -> u64 {
            super::RC_SOURCE_DROP_SEQ.load(super::Ordering::SeqCst)
        }
    }

    #[diplomat::attr(not(dotnet), disable)]
    #[diplomat::attr(dotnet, manually_disposable)]
    #[diplomat::opaque]
    pub struct RcDependent<'a>(&'a RcSource, u64);

    impl<'a> RcDependent<'a> {
        pub fn id(&self) -> u64 {
            self.1
        }

        pub fn source_id(&self) -> u64 {
            self.0 .0
        }

        /// A second, transitive layer: `RcDependent2` borrows `self`
        /// (`RcDependent`), not `RcSource` directly.
        pub fn make_dependent2<'b>(&'b self) -> Box<RcDependent2<'b, 'a>> {
            Box::new(RcDependent2(self, self.1))
        }

        pub fn reset_drop_stats() {
            super::RC_DEPENDENT_DROPS.store(0, super::Ordering::SeqCst);
            super::RC_DEPENDENT_DROP_SEQ.store(0, super::Ordering::SeqCst);
        }

        pub fn drop_count() -> u64 {
            super::RC_DEPENDENT_DROPS.load(super::Ordering::SeqCst)
        }

        pub fn drop_seq() -> u64 {
            super::RC_DEPENDENT_DROP_SEQ.load(super::Ordering::SeqCst)
        }
    }

    #[diplomat::attr(not(dotnet), disable)]
    #[diplomat::attr(dotnet, manually_disposable)]
    #[diplomat::opaque]
    pub struct RcDependent2<'b, 'a: 'b>(&'b RcDependent<'a>, u64);

    impl<'b, 'a: 'b> RcDependent2<'b, 'a> {
        pub fn id(&self) -> u64 {
            self.1
        }

        pub fn reset_drop_stats() {
            super::RC_DEPENDENT2_DROPS.store(0, super::Ordering::SeqCst);
            super::RC_DEPENDENT2_DROP_SEQ.store(0, super::Ordering::SeqCst);
        }

        pub fn drop_count() -> u64 {
            super::RC_DEPENDENT2_DROPS.load(super::Ordering::SeqCst)
        }

        pub fn drop_seq() -> u64 {
            super::RC_DEPENDENT2_DROP_SEQ.load(super::Ordering::SeqCst)
        }
    }

    // Finalizer-only (default, non-opt-in) parent/child pair exercising the
    // same destruction-ordering invariant without explicit `Dispose()`.
    #[diplomat::attr(not(dotnet), disable)]
    #[diplomat::opaque]
    pub struct RcFinalizerSource(u64);

    impl RcFinalizerSource {
        pub fn create(id: u64) -> Box<Self> {
            Box::new(Self(id))
        }

        pub fn id(&self) -> u64 {
            self.0
        }

        pub fn make_dependent<'b>(&'b self) -> Box<RcFinalizerDependent<'b>> {
            Box::new(RcFinalizerDependent(self, self.0))
        }

        pub fn reset_drop_stats() {
            super::RC_FINALIZER_SOURCE_DROPS.store(0, super::Ordering::SeqCst);
            super::RC_FINALIZER_SOURCE_DROP_SEQ.store(0, super::Ordering::SeqCst);
        }

        pub fn drop_count() -> u64 {
            super::RC_FINALIZER_SOURCE_DROPS.load(super::Ordering::SeqCst)
        }

        pub fn drop_seq() -> u64 {
            super::RC_FINALIZER_SOURCE_DROP_SEQ.load(super::Ordering::SeqCst)
        }
    }

    #[diplomat::attr(not(dotnet), disable)]
    #[diplomat::opaque]
    pub struct RcFinalizerDependent<'a>(&'a RcFinalizerSource, u64);

    impl<'a> RcFinalizerDependent<'a> {
        pub fn id(&self) -> u64 {
            self.1
        }

        pub fn reset_drop_stats() {
            super::RC_FINALIZER_DEPENDENT_DROPS.store(0, super::Ordering::SeqCst);
            super::RC_FINALIZER_DEPENDENT_DROP_SEQ.store(0, super::Ordering::SeqCst);
        }

        pub fn drop_count() -> u64 {
            super::RC_FINALIZER_DEPENDENT_DROPS.load(super::Ordering::SeqCst)
        }

        pub fn drop_seq() -> u64 {
            super::RC_FINALIZER_DEPENDENT_DROP_SEQ.load(super::Ordering::SeqCst)
        }
    }

    // ── Pin-lifetime regression fixture ─────────────────────────────────────
    //
    // Combines the two keep-alive mechanisms that must compose correctly: an
    // owned opaque that borrows its OWN pinned input buffer (like
    // `OpaqueSliceView` in slices.rs) that is ALSO the source of an RC
    // borrow-dependent (like `RcSource`/`RcDependent` above). This is exactly
    // the shape that exposed the pin-lifetime bug: disposing the source while
    // a dependent still holds an RC reference must defer BOTH the source's
    // Rust destructor AND the release of the source's own pinned input —
    // never unpinning while the (deferred) destructor might still read it.
    // `Drop` reads the borrowed slice and records a checksum, so a
    // moved/corrupted buffer is directly observable from C#.
    #[diplomat::attr(not(dotnet), disable)]
    #[diplomat::attr(dotnet, manually_disposable)]
    #[diplomat::opaque]
    pub struct PinnedRcSource<'a>(pub(crate) &'a [u8]);

    impl<'a> PinnedRcSource<'a> {
        pub fn create(data: &'a [u8]) -> Box<Self> {
            Box::new(Self(data))
        }

        /// An owned wrapper with its own Rust destructor that also borrows
        /// `self`'s lifetime, exactly like `RcSource::make_dependent` — but
        /// `self` here ALSO owns a pinned input buffer of its own.
        pub fn make_dependent<'b>(&'b self) -> Box<PinnedRcDependent<'b>> {
            Box::new(PinnedRcDependent(self))
        }

        pub fn reset_drop_stats() {
            super::PINNED_RC_SOURCE_DROPS.store(0, super::Ordering::SeqCst);
            super::PINNED_RC_SOURCE_DROP_SEQ.store(0, super::Ordering::SeqCst);
            super::PINNED_RC_SOURCE_DROP_CHECKSUM.store(0, super::Ordering::SeqCst);
        }

        pub fn drop_count() -> u64 {
            super::PINNED_RC_SOURCE_DROPS.load(super::Ordering::SeqCst)
        }

        pub fn drop_seq() -> u64 {
            super::PINNED_RC_SOURCE_DROP_SEQ.load(super::Ordering::SeqCst)
        }

        /// Checksum of the borrowed slice, computed INSIDE `Drop` (see
        /// below) — proves the pinned buffer was still valid/unmoved at the
        /// moment the native destructor actually ran, however long that was
        /// deferred by an outstanding RC dependent.
        pub fn drop_checksum() -> u64 {
            super::PINNED_RC_SOURCE_DROP_CHECKSUM.load(super::Ordering::SeqCst)
        }
    }

    #[diplomat::attr(not(dotnet), disable)]
    #[diplomat::attr(dotnet, manually_disposable)]
    #[diplomat::opaque]
    pub struct PinnedRcDependent<'a>(&'a PinnedRcSource<'a>);

    impl<'a> PinnedRcDependent<'a> {
        pub fn reset_drop_stats() {
            super::PINNED_RC_DEPENDENT_DROPS.store(0, super::Ordering::SeqCst);
            super::PINNED_RC_DEPENDENT_DROP_SEQ.store(0, super::Ordering::SeqCst);
        }

        pub fn drop_count() -> u64 {
            super::PINNED_RC_DEPENDENT_DROPS.load(super::Ordering::SeqCst)
        }

        pub fn drop_seq() -> u64 {
            super::PINNED_RC_DEPENDENT_DROP_SEQ.load(super::Ordering::SeqCst)
        }
    }
}

// Bumped by GcRaceProbe's Drop. Outside the bridge so the macro doesn't see it.
pub(crate) static PROBE_DROPS: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
pub(crate) static DEFAULT_DROP_PROBE_DROPS: std::sync::atomic::AtomicU64 =
    std::sync::atomic::AtomicU64::new(0);
pub(crate) static DISPOSABLE_DROP_PROBE_DROPS: std::sync::atomic::AtomicU64 =
    std::sync::atomic::AtomicU64::new(0);
pub(crate) use std::sync::atomic::Ordering;

// Shared logical clock + per-type "when did I drop" cells for the RC
// borrow-dependency fixtures below: lets tests assert *relative* destruction
// order deterministically (e.g. dependent-before-source) instead of
// depending on GC timing. 0 means "not dropped yet"; the clock starts at 1
// so a real sequence number is always non-zero and distinguishable.
pub(crate) static RC_CLOCK: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(1);

pub(crate) static RC_SOURCE_DROPS: std::sync::atomic::AtomicU64 =
    std::sync::atomic::AtomicU64::new(0);
pub(crate) static RC_SOURCE_DROP_SEQ: std::sync::atomic::AtomicU64 =
    std::sync::atomic::AtomicU64::new(0);
pub(crate) static RC_DEPENDENT_DROPS: std::sync::atomic::AtomicU64 =
    std::sync::atomic::AtomicU64::new(0);
pub(crate) static RC_DEPENDENT_DROP_SEQ: std::sync::atomic::AtomicU64 =
    std::sync::atomic::AtomicU64::new(0);
pub(crate) static RC_DEPENDENT2_DROPS: std::sync::atomic::AtomicU64 =
    std::sync::atomic::AtomicU64::new(0);
pub(crate) static RC_DEPENDENT2_DROP_SEQ: std::sync::atomic::AtomicU64 =
    std::sync::atomic::AtomicU64::new(0);

pub(crate) static RC_FINALIZER_SOURCE_DROPS: std::sync::atomic::AtomicU64 =
    std::sync::atomic::AtomicU64::new(0);
pub(crate) static RC_FINALIZER_SOURCE_DROP_SEQ: std::sync::atomic::AtomicU64 =
    std::sync::atomic::AtomicU64::new(0);
pub(crate) static RC_FINALIZER_DEPENDENT_DROPS: std::sync::atomic::AtomicU64 =
    std::sync::atomic::AtomicU64::new(0);
pub(crate) static RC_FINALIZER_DEPENDENT_DROP_SEQ: std::sync::atomic::AtomicU64 =
    std::sync::atomic::AtomicU64::new(0);

pub(crate) static PINNED_RC_SOURCE_DROPS: std::sync::atomic::AtomicU64 =
    std::sync::atomic::AtomicU64::new(0);
pub(crate) static PINNED_RC_SOURCE_DROP_SEQ: std::sync::atomic::AtomicU64 =
    std::sync::atomic::AtomicU64::new(0);
pub(crate) static PINNED_RC_SOURCE_DROP_CHECKSUM: std::sync::atomic::AtomicU64 =
    std::sync::atomic::AtomicU64::new(0);
pub(crate) static PINNED_RC_DEPENDENT_DROPS: std::sync::atomic::AtomicU64 =
    std::sync::atomic::AtomicU64::new(0);
pub(crate) static PINNED_RC_DEPENDENT_DROP_SEQ: std::sync::atomic::AtomicU64 =
    std::sync::atomic::AtomicU64::new(0);

impl Drop for ffi::GcRaceProbe {
    fn drop(&mut self) {
        PROBE_DROPS.fetch_add(1, Ordering::SeqCst);
    }
}

impl Drop for ffi::DefaultDropProbe {
    fn drop(&mut self) {
        DEFAULT_DROP_PROBE_DROPS.fetch_add(1, Ordering::SeqCst);
    }
}

impl Drop for ffi::DisposableDropProbe {
    fn drop(&mut self) {
        DISPOSABLE_DROP_PROBE_DROPS.fetch_add(1, Ordering::SeqCst);
    }
}

impl Drop for ffi::RcSource {
    fn drop(&mut self) {
        RC_SOURCE_DROPS.fetch_add(1, Ordering::SeqCst);
        RC_SOURCE_DROP_SEQ.store(RC_CLOCK.fetch_add(1, Ordering::SeqCst), Ordering::SeqCst);
    }
}

impl Drop for ffi::RcDependent<'_> {
    fn drop(&mut self) {
        RC_DEPENDENT_DROPS.fetch_add(1, Ordering::SeqCst);
        RC_DEPENDENT_DROP_SEQ.store(RC_CLOCK.fetch_add(1, Ordering::SeqCst), Ordering::SeqCst);
    }
}

impl Drop for ffi::RcDependent2<'_, '_> {
    fn drop(&mut self) {
        RC_DEPENDENT2_DROPS.fetch_add(1, Ordering::SeqCst);
        RC_DEPENDENT2_DROP_SEQ.store(RC_CLOCK.fetch_add(1, Ordering::SeqCst), Ordering::SeqCst);
    }
}

impl Drop for ffi::RcFinalizerSource {
    fn drop(&mut self) {
        RC_FINALIZER_SOURCE_DROPS.fetch_add(1, Ordering::SeqCst);
        RC_FINALIZER_SOURCE_DROP_SEQ
            .store(RC_CLOCK.fetch_add(1, Ordering::SeqCst), Ordering::SeqCst);
    }
}

impl Drop for ffi::RcFinalizerDependent<'_> {
    fn drop(&mut self) {
        RC_FINALIZER_DEPENDENT_DROPS.fetch_add(1, Ordering::SeqCst);
        RC_FINALIZER_DEPENDENT_DROP_SEQ
            .store(RC_CLOCK.fetch_add(1, Ordering::SeqCst), Ordering::SeqCst);
    }
}

impl Drop for ffi::PinnedRcSource<'_> {
    fn drop(&mut self) {
        // Reads the borrowed pinned slice DURING the destructor — exactly
        // what would observe moved/freed memory if the .NET wrapper had
        // already unpinned the buffer before this destructor actually ran
        // (the bug: unpinning right after `Release()` regardless of whether
        // that call's refcount decrement was the one that ran this
        // destructor, or merely deferred it to a still-outstanding
        // dependent).
        let checksum: u64 = self.0.iter().map(|&b| b as u64).sum();
        PINNED_RC_SOURCE_DROP_CHECKSUM.store(checksum, Ordering::SeqCst);
        PINNED_RC_SOURCE_DROPS.fetch_add(1, Ordering::SeqCst);
        PINNED_RC_SOURCE_DROP_SEQ.store(RC_CLOCK.fetch_add(1, Ordering::SeqCst), Ordering::SeqCst);
    }
}

impl Drop for ffi::PinnedRcDependent<'_> {
    fn drop(&mut self) {
        PINNED_RC_DEPENDENT_DROPS.fetch_add(1, Ordering::SeqCst);
        PINNED_RC_DEPENDENT_DROP_SEQ
            .store(RC_CLOCK.fetch_add(1, Ordering::SeqCst), Ordering::SeqCst);
    }
}

#[derive(Copy, Clone)]
pub struct One<'a>(&'a ());

#[derive(Copy, Clone)]
pub struct Two<'a, 'b>(&'a (), &'b ());

pub struct Internal {
    a: i32,
    b: f32,
    c: String,
}
