# Opaque Types

In the vast majority of cases, we'd like to expose Rust types over FFI "opaquely", that is, the FFI code does not know anything about the contents of these types, rather it wants to do things with the type.

By default, Diplomat will not let you expose fields of types other than the [allowed types](./types.md) over FFI. The following code will trigger a resolution error when running `diplomat-tool`:

```rust
#[diplomat::bridge]
mod ffi {
    pub struct MyFFIType {
        pub a: i32,
        pub b: Vec<String>, // or "SomeTypeDefinedElsewhere"
    }
    
    impl MyFFIType {
        pub fn create() -> MyFFIType {
            todo!()
        }
    }
}
```

Of course, if Diplomat is to be able to usefully expose Rust APIs without requiring everything be defined within Diplomat's bridge blocks, there has to be some way to include them in this the API.

For this in Diplomat we declare _opaque types_, which can only exist behind pointers. Such types can contain whatever they want, but they can never be passed over the stack through FFI, and the other side cannot peek into them in ways other than calling explicitly defined methods.

For example, say we have the following type:

```rust
struct Person {
    name: String,
    age: u8,
}

impl Person {
    pub fn new(name: String, age: u8) -> Self {
        Self {
            name, age
        }
    }

    pub fn get_age(&self) -> u8 {
        self.age
    }

    pub fn dump(&self) {
        println!("Person {} of age {}", self.name, self.age);
    }
}
```

To expose it over FFI, we'd do something like:

```rust
#[diplomat::bridge]
mod ffi {
    // import this from wherever, does not need
    // to be the same crate
    use super::Person as RustPerson;

    #[diplomat::opaque]
    pub struct Person(RustPerson);

    impl Person {
        pub fn create(s: &str) -> Box<MyCollection> {
            Box::new(Person(RustPerson::new(s.into())))
        }

        pub fn get_age(&self) -> u8 {
            self.age
        }

        pub fn dump(&self) {
            self.0.dump()
        }
    }
}
```

This will generate code exposing `create()`, `get_age()`, and `dump()` over FFI, as well as glue to ensure the destructor is called. However this will not expose any way to get at the `RustPerson`.

For example, the generated C++ looks something like

```cpp
class Person {
 public:
  static std::unique_ptr<Person> create(const std::string_view s, int8_t age);
  int8_t get_age();
  void dump();
  // snip
 private:
};
```

When exposing your library over FFI, most of the main types will probably end up being "opaque".

# Boxes are return-only

`Box<T>` can only be returned, not accepted as a parameter. This is because in garbage collected languages it is not possible to know if we are the unique owner when converting back to Rust. There are some techniques we could use to add such functionality, see [#317](https://github.com/rust-diplomat/diplomat/issues/317)

# Mutation

There are some [soundness concerns](https://github.com/rust-diplomat/diplomat/issues/225) around mutable types over FFI. To help with checking that, Diplomat requires explicit opt in for opaque types to be mutated, use `#[diplomat::opaque_mut]` if you wish to mutate an opaque type.

Currently the full set of checks is not implemented yet. The general idea is that any type that can be mutated over FFI should not also hand out references to stuff within itself (copies are fine).

If you need that property to implement borrowing iterators, consider using a `Cell` to store iterator state.
