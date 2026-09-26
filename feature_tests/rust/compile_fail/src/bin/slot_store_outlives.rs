#![forbid(unsafe_code)]

//! `Slot::store` must keep the opaque's lifetime on the slice it stores.
use diplomat_rust_backend_generated::Slot;

fn main() {
    let mut slot: Slot<'static> = Slot::new(b"initial");
    {
        let temporary = vec![42_u8; 8];
        slot.store(&temporary);
    }
    let dangling: &'static [u8] = slot.get();
    let _ = dangling[0];
}
