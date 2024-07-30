# Indexing

(Supported by `dart`, queried with `supports = indexing`. Intended to be supported in `cpp`)


`[]` can be overloaded in languages that support it by applying an `indexer` attribute to a method that takes an integer argument and returns an `Option<u8>`.

```rust
#[diplomat::bridge]
mod ffi {
    #[diplomat::opaque]
    struct Foo(Vec<u8>);

    impl Foo {
        #[diplomat::attr(auto, indexer)]
        pub fn get(&self, idx: usize) -> Option<u8> {
            self.0.get(idx).copied()
        }
    }
}
```


