# Default Values
{{get_supports("defaults")}}

Some backends support marking parameters with default values:

```rs
impl Test {
    pub fn test(a : i32, b : i64,
    #[diplomat::attr(supports=defaults, default_value=100)]
    c : i64) {}
}
```

In C++ for instance, this function will now be defined with a default value:

```c++
void Test::test(int32_t a, int64_t b, int64_t c = 100);
```
## Supported Default Value Types
Diplomat currently supports reading default values in the following formats:
- Characters
- 64-bit integers
- 64-bit floats
- Booleans

You can set a default value to be any of these, regardless of the type of the parameter itself (although the compiler may complain).