#![forbid(unsafe_code)]

//! Opaque type-level lifetimes cannot be erased from an input capability.
use diplomat_rust_backend_generated::TypeLifetimeOpaque;

fn main() {
    let long_lived = String::from("long");
    let mut owner = TypeLifetimeOpaque::new(long_lived.as_bytes());

    {
        let short_lived = String::from("short");
        let other = TypeLifetimeOpaque::new(short_lived.as_bytes());

        // The provider requires `other: &TypeLifetimeOpaque<'owner>`. A
        // `TypeLifetimeOpaque<'short>` must be rejected at this call site.
        owner.accept_same_lifetime(&other);
    }

    // Keep the owner live after the shorter source scope. If the call above
    // were accepted, the provider could leave a dangling `&'owner` inside it.
    let _ = owner.get();
}
