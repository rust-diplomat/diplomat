//! A borrowed slice cannot outlive the opaque that owns the buffer.
use diplomat_rust_backend_generated::MyString;

fn main() {
    let values = {
        let message = MyString::new(b"hello");
        message.borrow()
    };
    drop(values);
}
