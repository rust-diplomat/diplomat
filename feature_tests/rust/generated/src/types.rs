//! Generated enums and value structs.

use core::marker::PhantomData;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mode {
    Idle = 0,
    Active = 1,
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
