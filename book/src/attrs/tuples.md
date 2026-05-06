# Tuples
({{get_supports("tuples")}})

In languages with tuples, Diplomat can treat structures as being tuples on the boundary between Rust and the backend when marked as such:

```rs
#[diplomat::attr(auto, tuple)]
pub struct SomeStruct {
    x: f32,
    y: f32
}

impl SomeStruct {
    pub fn some_fn() -> SomeStruct;
}
```

In C++, `SomeStruct` will be returned as

```cpp
std::tuple<float, float>
```

Tuples do not support non-static methods, since past the boundary a tuple is not the struct it was generated from.

See https://github.com/rust-diplomat/diplomat/issues/1145 for the tracking issue for Tuples.