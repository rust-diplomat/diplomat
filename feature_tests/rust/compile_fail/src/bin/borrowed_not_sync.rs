use diplomat_rust_backend_generated::CounterRefMut;

fn assert_sync<T: Sync>() {}

fn main() {
    assert_sync::<CounterRefMut<'static>>();
}
