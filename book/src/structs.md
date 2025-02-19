# Structs and enums

Diplomat allows for exposing basic structs and enums over FFI. Typically these should be used as inputs and outputs to other methods, rather than having methods of their own, however it is possible to give them methods which capture `self` by-value.

Structs are most commonly found when making an options type for a method, or when doing multiple return values.


```rust
#[diplomat::bridge]
mod ffi {
    use my_thingy::MyThingy;

    // just exists so we can get methods
    #[diplomat::opaque]
    pub struct Thingy(MyThingy);

    pub struct ThingySettings {
        pub a: bool,
        pub b: u8,
        pub speed: SpeedSetting,
    }

    #[diplomat::enum_convert(my_thingy::SpeedSetting)]
    pub enum SpeedSetting {
        Fast, Medium, Slow
    }

    #[diplomat::enum_convert(my_thingy::ThingyStatus)]
    pub enum ThingyStatus {
        Good,
        Bad
    }

    impl Thingy {
        pub fn create(settings: ThingySettings) -> Box<Thingy> {
            // Convert our FFI type to whatever internal settings type was needed
            let settings = my_thingy::ThingySettings {
                a: settings.a,
                b: settings.b,
                speed: settings.speed.into()
            };
            Box::new(Thingy::new(settings))
        }

        pub fn get_status(&self) -> ThingyStatus {
            self.0.get_status().into()
        }
    }
}
```

Enums exposed via Diplomat must be simple C-like enums. They can have explicit discriminants. Structs may only contain fields which are themselves [allowed types](./types.md).

In C++ the structs are translated to simple structs and the enums become simple enum classes. In JS the structs become objects with fields, and the enums are exposed as strings that get converted at the boundary.

# `diplomat::enum_convert`

Diplomat can autogenerate `Into` impls to an enum from your library using `#[diplomat::enum_convert]`:

```rust
#[diplomat::bridge]
mod ffi {
    // ...

    #[diplomat::enum_convert(my_thingy::SpeedSetting)]
    enum SpeedSetting {
        Fast, Medium, Slow
    }

    // ...
}
```

In case the enum is `#[non_exhaustive]`, you may need to supply a `needs_wildcard` argument, like so: `#[diplomat::enum_convert(my_library::SpeedSetting, needs_wildcard)]`.

# Structs containing boxes

By default, structs cannot contain output-only types like `Box<T>`. This can be opted in to by using `#[diplomat::out]`, which will have the additional effect of making the struct an output-only type.


```rust
mod ffi {
    use my_thingy::MyThingy;

    #[diplomat::opaque]
    pub struct Thingy(=MyThingy);

    #[diplomat::out]
    pub struct ThingyAndExtraStuff {
        pub thingy: Box<Thingy>,
        pub stuff: u32
    }

    impl Thingy {
        pub fn create() -> ThingyAndExtraStuff {
            let thingy = Box::new(Thingy(MyThingy::new()));
            let stuff = 42;
            ThingyAndExtraStuff {
                thingy, stuff
            }
        }
    }

}
```