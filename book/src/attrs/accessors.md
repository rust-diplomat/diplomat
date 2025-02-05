# Getters and setters

(Supported by `dart`, `js`, queried with `supports = accessors`)


## Getters

A method that returns a value (and takes no argument) can be marked as a getter. It may be fallible.


```rust
#[diplomat::bridge]
mod ffi {
    #[diplomat::opaque]
    struct Foo {
        a: u8
    };

    impl Foo {
        #[diplomat::attr(auto, getter = "a")]
        pub fn get_a(&self) -> u8 {
            self.0
        }
    }
}

```

If the name is not provided, the method name is used[^1].

In languages that support accessors, instead of there being a `Foo::get_a()` method, people can use field access syntax on `Foo`-typed objects. For example, in JS, `foo.a` will work.


## Setters

A method that accepts a value can be marked as a setter. `self` need not be mutable, and the method may be fallible.


```rust
#[diplomat::bridge]
mod ffi {
    #[diplomat::opaque]
    struct Foo {
        a: u8
    };

    impl Foo {
        #[diplomat::attr(auto, setter = "a")]
        pub fn set_a(&mut self, a: u8)  {
            self.0 = a
        }
    }
}

```

If the name is not provided, the method name is used.

In languages that support accessors, instead of there being a `Foo::set_a()` method, people can use field access syntax on `Foo`-typed objects to set the value. For example, in JS, `foo.a = 1` will work.


 [^1]: A potential future improvement is to use the method name but strip common prefixes like `get_` and `set_`, which would allow a getter and setter for the same field to coexist without requiring them have explicit names.
