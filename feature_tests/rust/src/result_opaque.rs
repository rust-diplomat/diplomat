#[repr(C)]
pub struct ResultOpaque {
    // Lives in the library's memory, so it is only ever reached through `&`, `&mut`
    // or `DiplomatBox`. Not `Send`, `Sync` or `Unpin`: the library does not say
    // whether that would be safe.
    _private: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

impl ResultOpaque {
    pub fn new(i: i32) -> Result<crate::DiplomatBox<crate::ResultOpaque>, crate::ErrorEnum> {
        // Safety: `Box` returns are never null, and the caller owns them.
        unsafe { ResultOpaque_new(i) }
            .into_result()
            .map(|ok| unsafe { crate::DiplomatBox::from_raw(ok) })
    }

    pub fn new_failing_foo() -> Result<crate::DiplomatBox<crate::ResultOpaque>, crate::ErrorEnum> {
        // Safety: `Box` returns are never null, and the caller owns them.
        unsafe { ResultOpaque_new_failing_foo() }
            .into_result()
            .map(|ok| unsafe { crate::DiplomatBox::from_raw(ok) })
    }

    pub fn new_failing_bar() -> Result<crate::DiplomatBox<crate::ResultOpaque>, crate::ErrorEnum> {
        // Safety: `Box` returns are never null, and the caller owns them.
        unsafe { ResultOpaque_new_failing_bar() }
            .into_result()
            .map(|ok| unsafe { crate::DiplomatBox::from_raw(ok) })
    }

    pub fn new_failing_unit() -> Result<crate::DiplomatBox<crate::ResultOpaque>, ()> {
        // Safety: `Box` returns are never null, and the caller owns them.
        unsafe { ResultOpaque_new_failing_unit() }
            .into_result()
            .map(|ok| unsafe { crate::DiplomatBox::from_raw(ok) })
    }

    pub fn new_failing_struct(i: i32) -> Result<crate::DiplomatBox<crate::ResultOpaque>, crate::ErrorStruct> {
        // Safety: `Box` returns are never null, and the caller owns them.
        unsafe { ResultOpaque_new_failing_struct(i) }
            .into_result()
            .map(|ok| unsafe { crate::DiplomatBox::from_raw(ok) })
    }

    pub fn new_in_err(i: i32) -> Result<(), crate::DiplomatBox<crate::ResultOpaque>> {
        // Safety: `Box` returns are never null, and the caller owns them.
        unsafe { ResultOpaque_new_in_err(i) }
            .into_result()
            .map_err(|err| unsafe { crate::DiplomatBox::from_raw(err) })
    }

    pub fn new_int(i: i32) -> Result<i32, ()> {
        unsafe { ResultOpaque_new_int(i) }
            .into_result()
    }

    pub fn new_in_enum_err(i: i32) -> Result<crate::ErrorEnum, crate::DiplomatBox<crate::ResultOpaque>> {
        // Safety: `Box` returns are never null, and the caller owns them.
        unsafe { ResultOpaque_new_in_enum_err(i) }
            .into_result()
            .map_err(|err| unsafe { crate::DiplomatBox::from_raw(err) })
    }

    pub fn give_self<'a>(&'a self) -> Result<(), &'a crate::ResultOpaque> {
        // Safety: Borrowed returns are never null, and live as long as the signature says.
        unsafe { ResultOpaque_give_self(self) }
            .into_result()
            .map_err(|err| unsafe { &*err })
    }

    /// When we take &str, the return type becomes a Result
    /// Test that this interacts gracefully with returning a reference type
    pub fn takes_str<'a, 'anon_0>(&'a mut self, _v: &'anon_0 str) -> &'a mut crate::ResultOpaque {
        // Safety: Borrowed returns are never null, and live as long as the signature says.
        unsafe { &mut *ResultOpaque_takes_str(self, crate::DiplomatSlice::new(_v.as_bytes())) }
    }

    pub fn assert_integer(&self, i: i32) {
        unsafe { ResultOpaque_assert_integer(self, i) }
    }
}

extern "C" {
    fn ResultOpaque_new(i: i32) -> crate::DiplomatResult<*mut crate::ResultOpaque, crate::ErrorEnum>;
    fn ResultOpaque_new_failing_foo() -> crate::DiplomatResult<*mut crate::ResultOpaque, crate::ErrorEnum>;
    fn ResultOpaque_new_failing_bar() -> crate::DiplomatResult<*mut crate::ResultOpaque, crate::ErrorEnum>;
    fn ResultOpaque_new_failing_unit() -> crate::DiplomatResult<*mut crate::ResultOpaque, ()>;
    fn ResultOpaque_new_failing_struct(i: i32) -> crate::DiplomatResult<*mut crate::ResultOpaque, crate::ErrorStruct>;
    fn ResultOpaque_new_in_err(i: i32) -> crate::DiplomatResult<(), *mut crate::ResultOpaque>;
    fn ResultOpaque_new_int(i: i32) -> crate::DiplomatResult<i32, ()>;
    fn ResultOpaque_new_in_enum_err(i: i32) -> crate::DiplomatResult<crate::ErrorEnum, *mut crate::ResultOpaque>;
    fn ResultOpaque_give_self(this: *const ResultOpaque) -> crate::DiplomatResult<(), *const crate::ResultOpaque>;
    fn ResultOpaque_takes_str(this: *mut ResultOpaque, _v: crate::DiplomatSlice<u8>) -> *mut crate::ResultOpaque;
    fn ResultOpaque_assert_integer(this: *const ResultOpaque, i: i32);
}

// Safety: `ResultOpaque_destroy` frees a `Box<ResultOpaque>` returned by the library.
unsafe impl crate::DiplomatOpaque for ResultOpaque {
    unsafe fn destroy(ptr: *mut Self) {
        ResultOpaque_destroy(ptr)
    }
}

extern "C" {
    fn ResultOpaque_destroy(this: *mut ResultOpaque);
}