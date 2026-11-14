#[repr(C)]
pub struct Float64Vec {
    // Lives in the library's memory, so it is only ever reached through `&`, `&mut`
    // or `DiplomatBox`. Not `Send`, `Sync` or `Unpin`: the library does not say
    // whether that would be safe.
    _private: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

impl Float64Vec {
    pub fn new_bool(v: &[bool]) -> crate::DiplomatBox<crate::Float64Vec> {
        // Safety: `Box` returns are never null, and the caller owns them.
        unsafe { crate::DiplomatBox::from_raw(Float64Vec_new_bool(crate::DiplomatSlice::new(v))) }
    }

    pub fn new_i16(v: &[i16]) -> crate::DiplomatBox<crate::Float64Vec> {
        // Safety: `Box` returns are never null, and the caller owns them.
        unsafe { crate::DiplomatBox::from_raw(Float64Vec_new_i16(crate::DiplomatSlice::new(v))) }
    }

    pub fn new_u16(v: &[u16]) -> crate::DiplomatBox<crate::Float64Vec> {
        // Safety: `Box` returns are never null, and the caller owns them.
        unsafe { crate::DiplomatBox::from_raw(Float64Vec_new_u16(crate::DiplomatSlice::new(v))) }
    }

    pub fn new_isize(v: &[isize]) -> crate::DiplomatBox<crate::Float64Vec> {
        // Safety: `Box` returns are never null, and the caller owns them.
        unsafe { crate::DiplomatBox::from_raw(Float64Vec_new_isize(crate::DiplomatSlice::new(v))) }
    }

    pub fn new_usize(v: &[usize]) -> crate::DiplomatBox<crate::Float64Vec> {
        // Safety: `Box` returns are never null, and the caller owns them.
        unsafe { crate::DiplomatBox::from_raw(Float64Vec_new_usize(crate::DiplomatSlice::new(v))) }
    }

    pub fn new_f64_be_bytes(v: &[u8]) -> crate::DiplomatBox<crate::Float64Vec> {
        // Safety: `Box` returns are never null, and the caller owns them.
        unsafe { crate::DiplomatBox::from_raw(Float64Vec_new_f64_be_bytes(crate::DiplomatSlice::new(v))) }
    }

    pub fn fill_slice(&self, v: &mut [f64]) {
        unsafe { Float64Vec_fill_slice(self, crate::DiplomatSliceMut::new(v)) }
    }

    pub fn set_value(&mut self, new_slice: &[f64]) {
        unsafe { Float64Vec_set_value(self, crate::DiplomatSlice::new(new_slice)) }
    }
}

extern "C" {
    fn Float64Vec_new_bool(v: crate::DiplomatSlice<bool>) -> *mut crate::Float64Vec;
    fn Float64Vec_new_i16(v: crate::DiplomatSlice<i16>) -> *mut crate::Float64Vec;
    fn Float64Vec_new_u16(v: crate::DiplomatSlice<u16>) -> *mut crate::Float64Vec;
    fn Float64Vec_new_isize(v: crate::DiplomatSlice<isize>) -> *mut crate::Float64Vec;
    fn Float64Vec_new_usize(v: crate::DiplomatSlice<usize>) -> *mut crate::Float64Vec;
    fn Float64Vec_new_f64_be_bytes(v: crate::DiplomatSlice<u8>) -> *mut crate::Float64Vec;
    fn Float64Vec_fill_slice(this: *const Float64Vec, v: crate::DiplomatSliceMut<f64>);
    fn Float64Vec_set_value(this: *mut Float64Vec, new_slice: crate::DiplomatSlice<f64>);
}

// Safety: `Float64Vec_destroy` frees a `Box<Float64Vec>` returned by the library.
unsafe impl crate::DiplomatOpaque for Float64Vec {
    unsafe fn destroy(ptr: *mut Self) {
        Float64Vec_destroy(ptr)
    }
}

extern "C" {
    fn Float64Vec_destroy(this: *mut Float64Vec);
}