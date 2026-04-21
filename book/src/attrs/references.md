# References

## &Struct

({{get_supports("struct_refs")}})

Structs can be borrowed immutably as parameters to functions, but nowhere else.

## &mut Struct

({{get_supports("mut_struct_refs")}})

Structs can be borrowed mutably as parameters to functions, but nowhere else. Mutable structs must be marked with `#[diplomat::attr(*, mut_struct_ref)]`, as not every backend will represent its structure identically to how it is stored in the C ABI. Therefore, most backends will:

1. Clone the structure into a C-ABI friendly struct
2. Call the mutable method on the C-ABI friendly struct
3. Clone the C-ABI friendly struct back into the native backend type

To assist in making mutable structs possible, some backends will codegen different structure field types from normal behavior. See the [C++ backend's docs for an example of why this might be](../backends/cpp.md).