use diplomat_rust_backend_generated::Numbers;

fn main() {
    let mut numbers = Numbers::new(&[1, 2, 3]);
    let shared = numbers.values();
    let exclusive = numbers.values_mut();
    drop((shared, exclusive));
}
