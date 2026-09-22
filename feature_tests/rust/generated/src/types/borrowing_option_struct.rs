use core::marker::PhantomData;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BorrowingOptionStruct<'a> {
    pub a: Option<&'a [u8]>,
    pub(crate) _lifetimes: PhantomData<*mut &'a ()>,
}
