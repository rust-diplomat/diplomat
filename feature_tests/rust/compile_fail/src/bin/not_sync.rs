//! An owning opaque handle is deliberately not `Sync`.
use diplomat_rust_backend_generated::Opaque;

fn assert_sync<T: Sync>() {}

fn main() {
    assert_sync::<Opaque>();
}
