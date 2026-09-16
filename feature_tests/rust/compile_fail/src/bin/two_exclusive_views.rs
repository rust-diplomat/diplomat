use diplomat_rust_backend_generated::Counter;

fn main() {
    let mut counter = Counter::new();
    let first = counter.view_mut();
    let second = counter.view_mut();
    drop((first, second));
}
