use diplomat_rust_backend_generated::Numbers;

fn main() {
    let values = {
        let numbers = Numbers::new(&[1, 2, 3]);
        numbers.values()
    };
    drop(values);
}
