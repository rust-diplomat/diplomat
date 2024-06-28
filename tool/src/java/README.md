* slices appear to be passed as memory segments and lengths and not as slice structs
* structs and opaque are just memsegs which seem to be basically pointers
* some functions on Opaque types don't get generated call methods, only something to create an "invoker"
which must be called separately


## Invoker functions

Only the following methods generate invokers. These are all of the zero argument functions.

```rust
pub fn new_failing_foo() -> Result<Box<ResultOpaque>, ErrorEnum>;
pub fn new_failing_bar() -> Result<Box<ResultOpaque>, ErrorEnum>;
pub fn new_failing_unit() -> Result<Box<ResultOpaque>, ()>;
pub fn new_none() -> Option<Box<OptionOpaque>>;
pub fn returns() -> Option<OptionStruct>;
pub fn new_struct() -> OptionStruct;
pub fn new_struct_nones() -> OptionStruct;
pub fn new() -> MyStruct;
pub fn returns_zst_result() -> Result<(), MyZst>;
pub fn get_a() -> MyEnum;
pub fn new() -> Box<Opaque>;
pub fn returns_usize() -> usize;
pub fn returns_imported() -> ImportedStruct;
pub fn cmp() -> core::cmp::Ordering;
pub fn new() -> Box<AttrOpaque1>;
```
for comparison the method
```rust
pub fn method(&self) -> u8;
```
generates a method
```java
public static byte namespace_AttrOpaque1_method(MemorySegment self) {  
```


