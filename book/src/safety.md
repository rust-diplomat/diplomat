# Notes on Diplomat and safety

Overall, Diplomat is intended to provide safe FFI. However, there are some caveats around the current state of Diplomat's safety.


## The safety of the bridge crate when called from Rust

Diplomat's `extern "C"` functions are generated in a "bridge crate" that contains "bridge modules" tagged with `#[diplomat::bridge]`. This generated code is not designed with Rust as a primary client, rather, it is designed to be used from C.

As such, it may be possible to trigger unsoundness by directly calling these `extern "C"` APIs from Rust. Work shoring this up is welcome.


## Aliasing safety

Primary issue: [#225](https://github.com/rust-diplomat/diplomat/issues/225)

In Rust, it is undefined behavior to have any additional references (`&` or `&mut`) to some data whilst there is an active `&mut T` reference to it.

Diplomat currently lets you do the following:

```rust
#[diplomat::bridge]
mod ffi {

    #[diplomat::opaque]
    struct Foo(u8);

    #[diplomat::opaque]
    struct Bar(Foo);


    impl Bar {
        pub fn change_foo_number(&mut self, num: u8) {
            self.0.0 = num;
        }
        pub fn get_foo(&self) -> &Foo {
            &self.0
        }
    }
}

```

Calling `change_foo_number()` while a reference from `get_foo()` is active could be UB, or cause thread safety issues in multithreaded code.

Diplomat has [plans](https://github.com/rust-diplomat/diplomat/issues/225) on how to fix this, but until this is fixed, it can be avoided by doing the following:

 - Limit the number of mutable methods you expose (use `RefCell` where needed)
 - If you wish to expose a mutable method, ensure that:
     - The type cannot ever be obtained as an `&T` or `&mut T` from some other type
     - The type does not have any methods that produce an `&U` or `&mut U`, where `U` is an opaque type or slice that borrows from things that the method intends to mutate.
     

As long as you're careful, this should not be an issue. ICU4X tends to not need much in the way of mutation and is so far safe from this.

## Thread safety

Primary issue: [#533](https://github.com/rust-diplomat/diplomat/issues/533)


So far ICU4X has mostly been targeting languages where thread safety is locked down by thread isolation (Dart, JS), or where thread safety is a matter of documentation convention (C++), so it has not yet needed to think deeply about this. However, other users of Diplomat may.


Specific backends may choose their own route as to how they apply and enforce thread safety. Diplomat may over time add some annotation system to help with this. Limiting mutation and using `RwLock`/`Mutex` can help here.



