//! A borrowed view is deliberately not `Send`.
use diplomat_rust_backend_generated::OpaqueMutexedStringRef;

fn assert_send<T: Send>() {}

fn main() {
    assert_send::<OpaqueMutexedStringRef<'static>>();
}
