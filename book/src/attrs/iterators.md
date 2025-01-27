# Iterators and iterables

(Supported by `js`, `dart`, `kotlin`, queried with `supports = iterators` and `supports = iterables`)

Some languages support a first-class notion of "iterator", a type that can be looped over in a `for` loop, stepping through its values typically until they are exhausted.

Furthermore, some languages support a first-class notion of an "iterable": types like collections that can be asked to _produce_ an iterator over their values which can be stepped through. Typically `for` loops will also accept iterables, and looping through an iterable involves producing and then looping through the associated iterator.

The distinction here is: iterators are mutated by iteration and often one-time use, iterables are not mutated and can be iterated over multiple times. Put in Rust terms, `Iterator` types are iterators, and `IntoIterator` types are iterables.

Diplomat supports marking types as such by using `iterator` and `iterable` on a specific _method_ of the type.

## Marking a type as an iterator

To mark a type as an iterator, it should have a signature of `fn next(&mut self) -> Option<...>`. The method name may be anything, and it may take `self` by immutable reference instead, if needed (this is useful for situations where aliasing safety is needed, see [#225](https://github.com/rust-diplomat/diplomat/issues/225) ).

```rust
#[diplomat::bridge]
mod ffi {
    #[diplomat::opaque]
    struct MyIterator<'a>(std::slice::Iter<'a, u8>);

    impl<'a> MyIterator<'a> {
        #[diplomat::attr(auto, iterator)]
        pub fn next(&mut self) -> Option<u8> {
            self.0.next().copied()
        }
    }
}
```

## Marking a type as an iterable


Marking a type as an iterable requires annotating it with `iterable`, and it just needs to be a method that takes `self` and returns a type that has a method marked as `iterator`.

```rust
#[diplomat::bridge]
mod ffi {
    #[diplomat::opaque]
    struct MyVector(Vec<u8>);

    impl MyVector {
        #[diplomat::attr(auto, iterable)]
        pub fn iter<'a> (&'a self) -> Box<MyIterator<'a>> {
            Box::new(MyIterator(self.0.iter()))
        }
    }
}
```