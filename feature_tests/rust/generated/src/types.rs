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

/// A plain `repr(C)` value struct. Marked `abi_compatible` so it can appear in
/// slices; both sides already share the layout, so `&[Point]` is a native
/// `DiplomatSlice<Point>` rather than an intermediate buffer.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Snapshot {
    pub value: u32,
    pub active: bool,
    pub mode: Mode,
}
