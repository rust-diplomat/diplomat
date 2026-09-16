use diplomat_rust_backend_generated::Counter;

fn assert_sync<T: Sync>() {}

fn main() {
    assert_sync::<Counter>();
}
