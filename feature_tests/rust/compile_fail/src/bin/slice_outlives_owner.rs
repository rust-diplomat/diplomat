//! Same shape over a different element type: `&[f64]` out of a `Float64Vec`.
use diplomat_rust_backend_generated::Float64Vec;

fn main() {
    let values = {
        let numbers = Float64Vec::new(&[1.0, 2.0, 3.0]);
        numbers.as_slice()
    };
    drop(values);
}
