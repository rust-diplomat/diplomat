//! Holding a shared view of provider-owned memory blocks an exclusive borrow.
use diplomat_rust_backend_generated::Float64Vec;

fn main() {
    let mut numbers = Float64Vec::new(&[1.0, 2.0]);
    let shared = numbers.borrow();
    numbers.set_value(&[3.0]);
    drop(shared);
}
