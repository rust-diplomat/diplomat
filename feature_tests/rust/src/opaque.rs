/// The type behind `Opaque` pointers at the ABI level.
#[repr(C)]
pub(crate) struct OpaqueRaw {
    _private: [u8; 0],
}

pub struct Opaque(pub(crate) core::ptr::NonNull<OpaqueRaw>);

impl Opaque {
    pub fn new() -> crate::Opaque {
        // Safety: `Box` returns are never null, and the wrapper takes ownership.
        unsafe {
            crate::Opaque(core::ptr::NonNull::new_unchecked(
                Opaque_new(),
            ))
        }
    }

    pub fn from_str(input: &str) -> crate::Opaque {
        // Safety: `Box` returns are never null, and the wrapper takes ownership.
        unsafe {
            crate::Opaque(core::ptr::NonNull::new_unchecked(
                Opaque_from_str(crate::DiplomatSlice::new(input.as_bytes())),
            ))
        }
    }

    pub fn returns_usize() -> usize {
        unsafe { Opaque_returns_usize() }
    }

    pub fn returns_imported() -> crate::ImportedStruct {
        unsafe { Opaque_returns_imported() }
    }
}

extern "C" {
    fn Opaque_new() -> *mut crate::opaque::OpaqueRaw;
    fn Opaque_from_str(input: crate::DiplomatSlice<u8>) -> *mut crate::opaque::OpaqueRaw;
    fn Opaque_returns_usize() -> usize;
    fn Opaque_returns_imported() -> crate::ImportedStruct;
}

impl Drop for Opaque {
    fn drop(&mut self) {
        unsafe { Opaque_destroy(self.0.as_ptr()) }
    }
}

extern "C" {
    fn Opaque_destroy(this: *mut OpaqueRaw);
}
