//! An owning opaque handle is deliberately not `Send`.
use diplomat_rust_backend_generated::Opaque;

fn assert_send<T: Send>() {}

fn main() {
    assert_send::<Opaque>();
}
