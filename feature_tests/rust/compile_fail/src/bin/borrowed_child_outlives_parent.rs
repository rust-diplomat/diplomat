use diplomat_rust_backend_generated::{ChildRef, Counter};

fn escapes<'a>() -> ChildRef<'a> {
    let parent = Counter::new();
    parent.child()
}

fn main() {
    let _ = escapes();
}
