/// An  data provider, capable of loading  data keys from some source.
///
/// See the [Rust documentation for `icu_provider`](https://docs.rs/icu_provider/latest/icu_provider/index.html) for more information.
#[repr(C)]
pub struct DataProvider {
    // Lives in the library's memory, so it is only ever reached through `&`, `&mut`
    // or `DiplomatBox`. Not `Send`, `Sync` or `Unpin`: the library does not say
    // whether that would be safe.
    _private: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

impl DataProvider {
    /// See the [Rust documentation for `get_static_provider`](https://docs.rs/icu_testdata/latest/icu_testdata/fn.get_static_provider.html) for more information.
    pub fn new_static() -> crate::DiplomatBox<crate::DataProvider> {
        // Safety: `Box` returns are never null, and the caller owns them.
        unsafe { crate::DiplomatBox::from_raw(icu4x_DataProvider_new_static_mv1()) }
    }

    /// This exists as a regression test for https://github.com/rust-diplomat/diplomat/issues/155
    pub fn returns_result() -> Result<(), ()> {
        unsafe { icu4x_DataProvider_returns_result_mv1() }
            .into_result()
    }
}

extern "C" {
    fn icu4x_DataProvider_new_static_mv1() -> *mut crate::DataProvider;
    fn icu4x_DataProvider_returns_result_mv1() -> crate::DiplomatResult<(), ()>;
}

// Safety: `icu4x_DataProvider_destroy_mv1` frees a `Box<DataProvider>` returned by the library.
unsafe impl crate::DiplomatOpaque for DataProvider {
    unsafe fn destroy(ptr: *mut Self) {
        icu4x_DataProvider_destroy_mv1(ptr)
    }
}

extern "C" {
    fn icu4x_DataProvider_destroy_mv1(this: *mut DataProvider);
}