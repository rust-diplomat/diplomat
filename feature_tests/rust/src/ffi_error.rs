#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FFIError {
    FFI = 0,
    User = 1,
}
