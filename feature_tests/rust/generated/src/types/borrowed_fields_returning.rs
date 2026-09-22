use core::marker::PhantomData;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BorrowedFieldsReturning<'a> {
    pub bytes: &'a [u8],
    pub(crate) _lifetimes: PhantomData<*mut &'a ()>,
}
