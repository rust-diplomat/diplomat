//! The shared-argument capability is sealed: a downstream crate cannot implement
//! it, so it cannot forge a handle the generated code would trust.
use diplomat_rust_backend_generated::{OpaqueMutexedString, OpaqueMutexedStringSharedArg};

struct Forged;

impl OpaqueMutexedStringSharedArg for Forged {}

fn main() {
    let _ = core::mem::size_of::<Forged>();
    let _ = core::mem::size_of::<OpaqueMutexedString>();
}
