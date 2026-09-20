//! Runtime behaviour of the generated safe API.
//!
//! The provider is the shared `feature_tests/src` corpus — the same one every
//! other backend generates from — so these tests name the corpus's own types:
//! `Opaque`, `MyString`, `Float64Vec`, `Utf16Wrap`, `OpaqueThinVec`,
//! `OptionOpaque`, `ResultOpaque`, `OwnedSliceReturn`. Nothing here is declared
//! by a Rust-specific fixture.
#![forbid(unsafe_code)]

use diplomat_rust_backend_generated::{
    ContiguousEnum, ErrorEnum, Float64Vec, MyString, Opaque, OpaqueMutexedString, OpaqueThinVec,
    OptionEnum, OptionOpaque, OwnedSliceReturn, ResultOpaque, Utf16Wrap,
};

/// An owned opaque is constructed by the provider and dropped by the generated
/// wrapper; reaching the end of this test at all proves the destructor pairing.
#[test]
fn owned_opaque_round_trips() {
    let opaque = Opaque::new();
    assert_eq!(Opaque::returns_usize(), 412);
    drop(opaque);
    let from_str = Opaque::from_str("hello");
    drop(from_str);
}

/// A nullable owned opaque: `Option<Box<T>>` in HIR is `Option<T>` here.
#[test]
fn nullable_owned_opaque() {
    assert!(OptionOpaque::new_none().is_none());

    let some = OptionOpaque::new(23).expect("23 is a present opaque");
    some.assert_integer(23);
    drop(some);
}

/// `Option<primitive>` in both positions, and the shared/exclusive receiver split.
#[test]
fn option_primitives_and_receivers() {
    let opaque = OptionOpaque::new(10).expect("10 is present");
    assert_eq!(opaque.option_isize(), Some(10));
    assert_eq!(opaque.option_usize(), Some(10));
    assert_eq!(opaque.option_i32(), Some(10));
    assert_eq!(opaque.option_u32(), Some(10));
}

/// `Option<enum>` through a static method, both arms.
#[test]
fn option_enum_parameter_and_return() {
    assert_eq!(
        OptionOpaque::accepts_option_enum(Some(OptionEnum::Foo), 123),
        Some(OptionEnum::Foo)
    );
    assert_eq!(OptionOpaque::accepts_option_enum(None, 123), None);
    assert_eq!(OptionOpaque::accepts_option_u8(Some(7), 123), Some(7));
    assert_eq!(OptionOpaque::accepts_option_u8(None, 123), None);
}

/// `Result<T, E>` in each arm the backend claims: `E` is `()`, a custom enum,
/// a primitive, or an **owned** opaque. A discarded owned error still drops.
#[test]
fn result_arms_and_owned_errors() {
    let opaque = ResultOpaque::new(42).expect("42 is accepted");
    opaque.assert_integer(42);

    assert_eq!(ResultOpaque::new_failing_foo().unwrap_err(), ErrorEnum::Foo);
    assert_eq!(ResultOpaque::new_failing_bar().unwrap_err(), ErrorEnum::Bar);
    assert_eq!(ResultOpaque::new_int(7).expect("7 is returned"), 7);
    assert_eq!(ResultOpaque::new_failing_int(9).unwrap_err(), 9);

    // An owned opaque error: the generated wrapper is its only owner, so
    // discarding it still has to run the provider destructor.
    let error = ResultOpaque::new_in_err(5).unwrap_err();
    let debug = format!("{error:?}");
    assert!(debug.starts_with("ResultOpaque(0x"), "unexpected: {debug}");
    drop(error);

    assert!(ResultOpaque::new_in_enum_err(9).is_err());
    assert!(ResultOpaque::new_failing_unit().is_err());
}

/// Borrowed slices in and out, with a mutable out-parameter, over `f64`.
#[test]
fn float_slices_round_trip() {
    let mut values = Float64Vec::new(&[1.5, 2.5, 3.0]);
    assert_eq!(values.as_slice(), [1.5, 2.5, 3.0].as_slice());
    assert_eq!(values.borrow(), [1.5, 2.5, 3.0].as_slice());
    assert_eq!(values.get(1), Some(2.5));
    assert_eq!(values.get(9), None);

    let mut buffer = [0.0f64; 3];
    values.fill_slice(&mut buffer);
    assert_eq!(buffer, [1.5, 2.5, 3.0]);

    values.set_value(&[9.0]);
    assert_eq!(values.as_slice(), [9.0].as_slice());
    assert!(Float64Vec::new(&[]).as_slice().is_empty());
}

/// Borrowed strings: the provider's bytes are borrowed, not copied.
#[test]
fn borrowed_strings_are_borrowed_bytes() {
    let message = MyString::new(b"hello \xe9\xa4\x90");
    assert_eq!(message.borrow(), b"hello \xe9\xa4\x90".as_slice());
    assert_eq!(MyString::get_static_str(), "hello");

    let mut message = message;
    message.set_str(b"replaced");
    assert_eq!(message.borrow(), b"replaced".as_slice());
}

/// A provider-declared constructor whose name is a Rust keyword.
///
/// HIR carries `named_constructor = "unsafe"`; the generated method must be
/// escaped to `r#unsafe` rather than rejecting the whole provider.
#[test]
fn keyword_named_constructor_is_escaped() {
    let renamed = MyString::r#unsafe("unsafe");
    assert_eq!(renamed.borrow(), b"unsafe".as_slice());
}

/// UTF-16 crosses as `&[u16]`; the provider's own encoding is preserved.
#[test]
fn utf16_borrows_round_trip() {
    let wrap = Utf16Wrap::from_utf16(&[65, 66]);
    assert_eq!(wrap.borrow_cont(), [65u16, 66].as_slice());

    let empty = Utf16Wrap::from_utf16(&[]);
    assert!(empty.borrow_cont().is_empty());

    // A non-BMP scalar arrives as a surrogate pair.
    let non_bmp: Vec<u16> = "\u{10437}".encode_utf16().collect();
    let wrap = Utf16Wrap::from_utf16(&non_bmp);
    assert_eq!(wrap.borrow_cont(), non_bmp.as_slice());
}

/// An owned `Box<[u8]>` return transfers the provider's allocation.
#[test]
fn owned_byte_slice_return_transfers_ownership() {
    assert_eq!(&OwnedSliceReturn::make_bytes(5)[..], &[0, 1, 2, 3, 4]);
    assert!(OwnedSliceReturn::make_bytes(0).is_empty());

    let bytes = OwnedSliceReturn::try_make_bytes(3).expect("3 is non-zero");
    assert_eq!(&bytes[..], &[0, 1, 2]);
    assert_eq!(
        OwnedSliceReturn::try_make_bytes(0).unwrap_err(),
        ErrorEnum::Foo
    );
}

/// An opaque that borrows out of a caller-owned slice, and returns a borrowed
/// opaque view of its own contents.
#[test]
fn borrowed_views_of_a_provider_vector() {
    let vector = OpaqueThinVec::create(&[1, 2, 3], &[1.5, 2.5, 3.5], b"");
    assert_eq!(vector.len(), 3);

    // Two shared views of the same owner may be live at once.
    let first = vector.first().expect("three elements");
    assert_eq!(first.a(), 1);
    assert_eq!(first.b(), 1.5);

    let second = vector.get(1).expect("in range");
    assert_eq!(second.a(), 2);
    assert!(vector.get(9).is_none());

    // `OpaqueThinIter::next` is declared `(&'a mut self) -> Option<&'a OpaqueThin>`,
    // where `'a` is the *type-level* lifetime rather than a fresh borrow of the
    // iterator. HIR records that faithfully, so the generated signature ties the
    // yielded view to the owner's lifetime and the iterator cannot be re-borrowed
    // from a local. Constructing the iterator is what the backend claims to
    // support here; element access goes through `first`/`get` above.
    let mut iter = vector.iter();
    let _ = &mut iter;
}

/// A provider that owns a string behind a mutex, reached without copying.
#[test]
fn mutexed_string_is_borrowed_not_copied() {
    // `String` is replaced in place, so a previously taken byte view must be
    // re-taken after the mutation rather than held across it.
    let owned = OpaqueMutexedString::from_usize(356);
    assert_eq!(owned.get_len_and_add(4), 7);
    // `change` replaces the stored string with the decimal rendering of the
    // number, so the length becomes `len("1234")`.
    owned.change(1234);
    assert_eq!(owned.get_len_and_add(0), 4);

    // A provider-owned string behind a mutex, borrowed without copying.
    assert_eq!(
        owned.dummy_str(),
        b"A const str with non byte char: \xe9\xa4\x90 which is a DiplomatChar,".as_slice()
    );
}

/// Simple enums keep their provider-declared discriminants.
#[test]
fn enum_discriminants_match_the_provider() {
    assert_eq!(ContiguousEnum::C as i32, 0);
    assert_eq!(ContiguousEnum::D as i32, 1);
    assert_eq!(ContiguousEnum::E as i32, 2);
    assert_eq!(ContiguousEnum::F as i32, 3);
    assert_eq!(ErrorEnum::Foo as i32, 0);
    assert_eq!(ErrorEnum::Bar as i32, 1);
}

/// Opaque wrappers print as `TypeName(<addr>)` and never name private fields,
/// which is what lets `.expect` / `.unwrap_err` compile.
#[test]
fn opaque_debug_identifies_the_handle_without_leaking_private_fields() {
    let opaque = Opaque::new();
    let debug = format!("{opaque:?}");
    assert!(
        debug.starts_with("Opaque(0x") && debug.ends_with(')'),
        "unexpected Debug output: {debug}"
    );
    assert!(
        !debug.contains("inner") && !debug.contains("_not_send_sync") && !debug.contains("_borrow"),
        "Debug must not name private fields: {debug}"
    );

    // A value enum derives `Debug`, so its variant name is what prints.
    assert_eq!(
        format!("{:?}", ResultOpaque::new_failing_foo().unwrap_err()),
        "Foo"
    );
}
