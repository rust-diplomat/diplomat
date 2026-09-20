//! Generated enums and value structs.

use core::marker::PhantomData;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mode {
    Idle = 0,
    Active = 1,
}

/// A custom error payload.
///
/// `#[diplomat::attr(auto, error)]` is what declares a type usable as a `Result`
/// error, and the generated backend enforces it: an unmarked enum is refused with a
/// diagnostic rather than quietly accepted.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ValueError {
    TooLarge = 0,
}

/// Value struct with borrowed slice fields, returned by `SliceView::fields`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FieldView<'a> {
    pub bytes: &'a [u8],
    pub count: u32,
    pub(crate) _lifetimes: PhantomData<fn(&'a ()) -> &'a ()>,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Snapshot {
    pub value: u32,
    pub active: bool,
    pub mode: Mode,
}
