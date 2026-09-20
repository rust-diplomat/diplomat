//! A borrowed view cannot outlive the owner it was taken from.
use diplomat_rust_backend_generated::{OpaqueMutexedString, OpaqueMutexedStringRef};

fn escapes<'a>() -> OpaqueMutexedStringRef<'a> {
    let owner = OpaqueMutexedString::from_usize(1);
    owner.borrow()
}

fn main() {
    let _ = escapes();
}
