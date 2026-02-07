# Traits

Backends that support traits (with `#[diplomat::cfg(supports=traits)]`) can be passed in as input-only parameters:

```rs
#[diplomat::bridge]
mod ffi {
    pub trait SomeTrait {
        fn some_function(&self, value : u32) -> i64;
        fn some_other_function()
    }

    #[diplomat::opaque]
    pub struct SomeStruct();
    impl SomeStruct {
        pub fn use_trait(t : impl SomeTrait) -> i64 {
            t.some_function(20)
        }
    }
}
```

Traits are treated internally like holders of [callbacks](./callbacks.md), and so support all the same types that callbacks do.