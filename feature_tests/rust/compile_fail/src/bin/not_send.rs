//! Expected failure: owner handles remain not Send in the initial MVP.

use diplomat_rust_backend_generated::Counter;

fn assert_send<T: Send>() {}

fn main() {
    assert_send::<Counter>();
}
