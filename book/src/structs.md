# Structs and enums

Diplomat allows for exposing basic structs and enums over FFI. Typically these should be used as inputs and outputs to other methods, rather than having methods of their own, however


```rust
#[diplomat::bridge]
mod ffi {
    // just exists so we can get methods
    #[diplomat::opaque]
    struct Thingy;

    struct ThingySettings {
        a: bool,
        b: u8,
        speed: SpeedSetting,
    }

    enum SpeedSetting {
        Fast, Medium, Slow
    }

    enum ThingyStatus {
        Good,
        Bad
    }

    impl Thingy {
        pub fn create(settings: ThingySettings) -> Box<Thingy> {
            // in real code this would construct a Thingy based on settings
            Thingy
        }

        pub fn get_status(&self) -> ThingyStatus {
            // in real code this would call some methods on `self`
            ThingyStatus::Good
        }
    }
}
```

Enums exposed via Diplomat must be simple C-like enums. Structs may only contain fields which are [allowed](./types.md).

In C++ the structs are translated to simple structs and the enums become simple enum classes. In JS the structs become classes with getters, and the enums are exposed as strings that get converted at the boundary.
