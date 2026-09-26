//! A borrowed view is deliberately not `Sync`.
use diplomat_rust_backend_generated::OpaqueMutexedStringRefMut;

fn assert_sync<T: Sync>() {}

fn main() {
    assert_sync::<OpaqueMutexedStringRefMut<'static>>();
}
