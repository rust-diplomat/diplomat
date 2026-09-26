#![forbid(unsafe_code)]

//! Store a short-lived opaque into one whose lifetime is a longer caller buffer,
//! then return that borrow after the short buffer is gone. Rejected.
use diplomat_rust_backend_generated::TypeLifetimeOpaque;

fn kept<'a>(buf: &'a [u8]) -> &'a [u8] {
    let mut owner = TypeLifetimeOpaque::new(buf);
    {
        let short_lived = String::from("short");
        let other = TypeLifetimeOpaque::new(short_lived.as_bytes());
        owner.accept_same_lifetime(&other);
    }
    owner.get()
}

fn main() {
    let long_lived = String::from("long");
    let bytes = kept(long_lived.as_bytes());
    assert_eq!(bytes, b"short");
}
