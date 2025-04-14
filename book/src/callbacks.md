# Callbacks

Diplomat has limited, experimental support for exposing and working with callbacks. See [tracking issue](https://github.com/rust-diplomat/diplomat/issues/146)
Currently these are only supported in the C++, kotlin, and nanobind backends.

Functions taking callbacks as parameters take the form

```rust
#[diplomat::bridge]
impl MyType{
    pub fn acceptsCallback(&self, impl Fn())
}
```
or
```rust
impl MyType{
    pub fn acceptsCallback(&self, impl FnMut())
}
```

The callback's parameters & return types may be limited depending on the target language for now. Additional traits are not supported, however `+ 'static` may be given, which allows for moving the trait object into a `Box<dyn Fn()>`.

Callback may accept references & mutable references to opaque types as parameters, however lifetime parsing for these is not yet supported. Users should assume that it is not safe to store any such parameter outside the callback.