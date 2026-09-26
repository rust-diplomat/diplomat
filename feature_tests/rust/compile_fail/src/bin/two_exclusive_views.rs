//! Two exclusive views of the same opaque cannot be live at once.
use diplomat_rust_backend_generated::ResultOpaque;

fn main() {
    let mut opaque = ResultOpaque::new(1).expect("1 is accepted");
    let first = opaque.takes_str("a");
    let second = opaque.takes_str("b");
    drop((first, second));
}
