/// The type behind `DataProvider` pointers at the ABI level.
#[repr(C)]
pub(crate) struct DataProviderRaw {
    _private: [u8; 0],
}

/// An  data provider, capable of loading  data keys from some source.
///
/// See the [Rust documentation for `icu_provider`](https://docs.rs/icu_provider/latest/icu_provider/index.html) for more information.
pub struct DataProvider(pub(crate) core::ptr::NonNull<DataProviderRaw>);

impl DataProvider {
    /// See the [Rust documentation for `get_static_provider`](https://docs.rs/icu_testdata/latest/icu_testdata/fn.get_static_provider.html) for more information.
    pub fn new_static() -> crate::DataProvider {
        // Safety: `Box` returns are never null, and the wrapper takes ownership.
        unsafe {
            crate::DataProvider(core::ptr::NonNull::new_unchecked(
                icu4x_DataProvider_new_static_mv1(),
            ))
        }
    }
}

impl Drop for DataProvider {
    fn drop(&mut self) {
        unsafe { icu4x_DataProvider_destroy_mv1(self.0.as_ptr()) }
    }
}

extern "C" {
    fn icu4x_DataProvider_new_static_mv1() -> *mut crate::data_provider::DataProviderRaw;
    fn icu4x_DataProvider_destroy_mv1(this: *mut DataProviderRaw);
}
