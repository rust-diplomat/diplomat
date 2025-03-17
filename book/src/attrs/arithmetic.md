# Arithmetic operator overloads

(Supported by and `cpp`, queried with `supports = arithmetic`)

`+-*/` and their in-place variants can be overloaded in languages that support it by applying an attribute to single-argument functions taking self. The Names are
| Operator | Name       |
|----------|------------|
|    +     | add        |
|    -     | sub        |
|    *     | div        |
|    /     | mul        |
|    +=    | add_assign |
|    +=    | sub_assign |
|    +=    | div_assign |
|    +=    | mul_assign |

```rust
#[diplomat::bridge]
mod ffi {
    #[diplomat::opaque]
    struct OpaqueInt(i32);

    impl OpaqueInt {
        #[diplomat::attr(auto, add)]
        pub fn add(&self, v: i32) -> Box<OpaqueInt> {
            Box::new(Self(self.0 + v))
        }

        #[diplomat::attr(auto, add_assign)]
        pub fn inplace_add(&mut self, v: i32) {
            self.0 += v;
        }
    }
}
```


