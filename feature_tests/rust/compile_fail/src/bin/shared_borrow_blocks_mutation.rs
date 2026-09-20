//! A live shared borrow of provider memory blocks an exclusive one.
use diplomat_rust_backend_generated::MyString;

fn main() {
    let mut message = MyString::new(b"before");
    let shared = message.borrow();
    message.set_str(b"after");
    drop(shared);
}
