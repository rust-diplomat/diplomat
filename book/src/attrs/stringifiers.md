# Stringifiers

({{get_supports("stringifiers")}}. Intended to be supported in `js`.)

Some languages have a designated way to provide a method for converting a type to a string.

The `stringifier`  attribute can be applied to such a method:

```rust
#[diplomat::bridge]
mod ffi {
    #[diplomat::opaque]
    struct Foo;

    impl Foo {
        #[diplomat::attr(auto, stringifier)]
        pub fn dump(&self, out: &mut DiplomatWrite) {
            ...
        }
    }
}
```


In Dart, this will generate a `toString()` method.
