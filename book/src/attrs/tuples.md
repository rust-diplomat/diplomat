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

See https://github.com/rust-diplomat/diplomat/issues/1145 for the tracking issue for Tuples.