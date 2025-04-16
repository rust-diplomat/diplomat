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

Some backends may support callbacks accepting \[mutable\] references to opaque types as parameters, but lifetime parsing for these is not yet supported by any backend. Only anonymous lifetimes are allowed as a result, meaning that no such parameter may be guaranteed to live longer than a single invocation of the callback.