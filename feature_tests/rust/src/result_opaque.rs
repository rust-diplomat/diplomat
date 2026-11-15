/// The type behind `ResultOpaque` pointers at the ABI level.
#[repr(C)]
pub(crate) struct ResultOpaqueRaw {
    _private: [u8; 0],
}

pub struct ResultOpaque(pub(crate) core::ptr::NonNull<ResultOpaqueRaw>);

impl ResultOpaque {
    pub fn new(i: i32) -> Result<crate::ResultOpaque, crate::ErrorEnum> {
        // Safety: `Box` returns are never null, and the wrapper takes ownership.
        unsafe { ResultOpaque_new(i) }
            .into_result()
            .map(|ok| unsafe { crate::ResultOpaque(core::ptr::NonNull::new_unchecked(ok)) })
    }

    pub fn new_failing_foo() -> Result<crate::ResultOpaque, crate::ErrorEnum> {
        // Safety: `Box` returns are never null, and the wrapper takes ownership.
        unsafe { ResultOpaque_new_failing_foo() }
            .into_result()
            .map(|ok| unsafe { crate::ResultOpaque(core::ptr::NonNull::new_unchecked(ok)) })
    }

    pub fn new_failing_bar() -> Result<crate::ResultOpaque, crate::ErrorEnum> {
        // Safety: `Box` returns are never null, and the wrapper takes ownership.
        unsafe { ResultOpaque_new_failing_bar() }
            .into_result()
            .map(|ok| unsafe { crate::ResultOpaque(core::ptr::NonNull::new_unchecked(ok)) })
    }

    pub fn new_failing_unit() -> Result<crate::ResultOpaque, ()> {
        // Safety: `Box` returns are never null, and the wrapper takes ownership.
        unsafe { ResultOpaque_new_failing_unit() }
            .into_result()
            .map(|ok| unsafe { crate::ResultOpaque(core::ptr::NonNull::new_unchecked(ok)) })
    }

    pub fn new_failing_struct(i: i32) -> Result<crate::ResultOpaque, crate::ErrorStruct> {
        // Safety: `Box` returns are never null, and the wrapper takes ownership.
        unsafe { ResultOpaque_new_failing_struct(i) }
            .into_result()
            .map(|ok| unsafe { crate::ResultOpaque(core::ptr::NonNull::new_unchecked(ok)) })
    }

    pub fn new_in_err(i: i32) -> Result<(), crate::ResultOpaque> {
        // Safety: `Box` returns are never null, and the wrapper takes ownership.
        unsafe { ResultOpaque_new_in_err(i) }
            .into_result()
            .map_err(|err| unsafe { crate::ResultOpaque(core::ptr::NonNull::new_unchecked(err)) })
    }

    pub fn new_int(i: i32) -> Result<i32, ()> {
        unsafe { ResultOpaque_new_int(i) }
            .into_result()
    }

    pub fn new_in_enum_err(i: i32) -> Result<crate::ErrorEnum, crate::ResultOpaque> {
        // Safety: `Box` returns are never null, and the wrapper takes ownership.
        unsafe { ResultOpaque_new_in_enum_err(i) }
            .into_result()
            .map_err(|err| unsafe { crate::ResultOpaque(core::ptr::NonNull::new_unchecked(err)) })
    }

    pub fn assert_integer(&self, i: i32) {
        unsafe { ResultOpaque_assert_integer(self.0.as_ptr(), i) }
    }
}

extern "C" {
    fn ResultOpaque_new(i: i32) -> crate::DiplomatResult<*mut crate::result_opaque::ResultOpaqueRaw, crate::ErrorEnum>;
    fn ResultOpaque_new_failing_foo() -> crate::DiplomatResult<*mut crate::result_opaque::ResultOpaqueRaw, crate::ErrorEnum>;
    fn ResultOpaque_new_failing_bar() -> crate::DiplomatResult<*mut crate::result_opaque::ResultOpaqueRaw, crate::ErrorEnum>;
    fn ResultOpaque_new_failing_unit() -> crate::DiplomatResult<*mut crate::result_opaque::ResultOpaqueRaw, ()>;
    fn ResultOpaque_new_failing_struct(i: i32) -> crate::DiplomatResult<*mut crate::result_opaque::ResultOpaqueRaw, crate::ErrorStruct>;
    fn ResultOpaque_new_in_err(i: i32) -> crate::DiplomatResult<(), *mut crate::result_opaque::ResultOpaqueRaw>;
    fn ResultOpaque_new_int(i: i32) -> crate::DiplomatResult<i32, ()>;
    fn ResultOpaque_new_in_enum_err(i: i32) -> crate::DiplomatResult<crate::ErrorEnum, *mut crate::result_opaque::ResultOpaqueRaw>;
    fn ResultOpaque_assert_integer(this: *const ResultOpaqueRaw, i: i32);
}

impl Drop for ResultOpaque {
    fn drop(&mut self) {
        unsafe { ResultOpaque_destroy(self.0.as_ptr()) }
    }
}

extern "C" {
    fn ResultOpaque_destroy(this: *mut ResultOpaqueRaw);
}
