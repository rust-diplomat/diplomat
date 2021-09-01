# Opaque Types

In the vast majority of cases, we'd like to expose Rust types over FFI "opaquely", that is, the FFI code does not know anything about the contents of these types.

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
struct MyCollection {
    name: String,
    items: Vec<String>,
}

impl MyCollection {
    fn new(name: String) -> Self {
        Self {
            name, items: vec![]
        }
    }

    fn push(&mut self, s: String) {
        self.items.push(s)
    }

    fn dump(&self) {
        println!("Collection {} with items {:?}", self.name, self.items);
    }
}
```

To expose it over FFI, we'd do something like:

```rust
#[diplomat::bridge]
mod ffi {
    // import this from wherever, does not need
    // to be the same crate
    use super::MyCollection as RustCollection;

    #[diplomat::opaque]
    struct MyCollection(RustCollection);

    impl MyCollection {
        pub fn create(s: &str) -> Box<Self> {
            Box::new(MyCollection(RustCollection::new(s.into())))
        }

        pub fn push(&mut self, s: &str) {
            self.0.push(s.into())
        }

        pub fn dump(&self) {
            self.0.dump()
        }
    }
}
```

This will generate code exposing `create()`, `push()`, and `dump()` over FFI, as well as glue to ensure the destructor is called. However this will not expose any way to get at the `RustCollection`.

For example, the generated C++ looks something like

```cpp
class MyCollection {
 public:
  static MyCollection create(const std::string_view s);
  void push(const std::string_view s);
  void dump();
  // snip
 private:
  // just a pointer with a custom destructor
  std::unique_ptr<capi::MyCollection, MyCollectionDeleter> inner;
};
```

