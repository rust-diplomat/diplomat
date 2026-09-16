use diplomat_rust_backend_generated::CounterRef;

fn assert_send<T: Send>() {}

fn main() {
    assert_send::<CounterRef<'static>>();
}
