/// The type behind `Float64Vec` pointers at the ABI level.
#[repr(C)]
pub(crate) struct Float64VecRaw {
    _private: [u8; 0],
}

pub struct Float64Vec(pub(crate) core::ptr::NonNull<Float64VecRaw>);

impl Float64Vec {
    pub fn new_bool(v: &[bool]) -> crate::Float64Vec {
        // Safety: `Box` returns are never null, and the wrapper takes ownership.
        unsafe {
            crate::Float64Vec(core::ptr::NonNull::new_unchecked(
                Float64Vec_new_bool(crate::DiplomatSlice::new(v)),
            ))
        }
    }

    pub fn new_i16(v: &[i16]) -> crate::Float64Vec {
        // Safety: `Box` returns are never null, and the wrapper takes ownership.
        unsafe {
            crate::Float64Vec(core::ptr::NonNull::new_unchecked(
                Float64Vec_new_i16(crate::DiplomatSlice::new(v)),
            ))
        }
    }

    pub fn new_u16(v: &[u16]) -> crate::Float64Vec {
        // Safety: `Box` returns are never null, and the wrapper takes ownership.
        unsafe {
            crate::Float64Vec(core::ptr::NonNull::new_unchecked(
                Float64Vec_new_u16(crate::DiplomatSlice::new(v)),
            ))
        }
    }

    pub fn new_isize(v: &[isize]) -> crate::Float64Vec {
        // Safety: `Box` returns are never null, and the wrapper takes ownership.
        unsafe {
            crate::Float64Vec(core::ptr::NonNull::new_unchecked(
                Float64Vec_new_isize(crate::DiplomatSlice::new(v)),
            ))
        }
    }

    pub fn new_usize(v: &[usize]) -> crate::Float64Vec {
        // Safety: `Box` returns are never null, and the wrapper takes ownership.
        unsafe {
            crate::Float64Vec(core::ptr::NonNull::new_unchecked(
                Float64Vec_new_usize(crate::DiplomatSlice::new(v)),
            ))
        }
    }

    pub fn new_f64_be_bytes(v: &[u8]) -> crate::Float64Vec {
        // Safety: `Box` returns are never null, and the wrapper takes ownership.
        unsafe {
            crate::Float64Vec(core::ptr::NonNull::new_unchecked(
                Float64Vec_new_f64_be_bytes(crate::DiplomatSlice::new(v)),
            ))
        }
    }

    pub fn fill_slice(&self, v: &mut [f64]) {
        unsafe { Float64Vec_fill_slice(self.0.as_ptr(), crate::DiplomatSliceMut::new(v)) }
    }

    pub fn set_value(&mut self, new_slice: &[f64]) {
        unsafe { Float64Vec_set_value(self.0.as_ptr(), crate::DiplomatSlice::new(new_slice)) }
    }
}

extern "C" {
    fn Float64Vec_new_bool(v: crate::DiplomatSlice<bool>) -> *mut crate::float64_vec::Float64VecRaw;
    fn Float64Vec_new_i16(v: crate::DiplomatSlice<i16>) -> *mut crate::float64_vec::Float64VecRaw;
    fn Float64Vec_new_u16(v: crate::DiplomatSlice<u16>) -> *mut crate::float64_vec::Float64VecRaw;
    fn Float64Vec_new_isize(v: crate::DiplomatSlice<isize>) -> *mut crate::float64_vec::Float64VecRaw;
    fn Float64Vec_new_usize(v: crate::DiplomatSlice<usize>) -> *mut crate::float64_vec::Float64VecRaw;
    fn Float64Vec_new_f64_be_bytes(v: crate::DiplomatSlice<u8>) -> *mut crate::float64_vec::Float64VecRaw;
    fn Float64Vec_fill_slice(this: *const Float64VecRaw, v: crate::DiplomatSliceMut<f64>);
    fn Float64Vec_set_value(this: *mut Float64VecRaw, new_slice: crate::DiplomatSlice<f64>);
}

impl Drop for Float64Vec {
    fn drop(&mut self) {
        unsafe { Float64Vec_destroy(self.0.as_ptr()) }
    }
}

extern "C" {
    fn Float64Vec_destroy(this: *mut Float64VecRaw);
}
