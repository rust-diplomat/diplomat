//! Holding an exclusive view blocks any further borrow of its source.
use diplomat_rust_backend_generated::ResultOpaque;

fn main() {
    let mut opaque = ResultOpaque::new(1).expect("1 is accepted");
    let exclusive = opaque.takes_str("a");
    let shared = opaque.assert_integer(1);
    drop((exclusive, shared));
}
