use diplomat_rust_backend_generated::Counter;

fn main() {
    let counter = Counter::new();
    let mut shared = counter.view();
    shared.increment();
}
