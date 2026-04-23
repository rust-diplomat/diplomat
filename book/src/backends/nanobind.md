# Nanobind Backend

## Type Conversion
The Nanobind backend is backed by the [C++ backend](./cpp.md#type-conversion) (through [Nanobind](https://nanobind.readthedocs.io/en/latest/)), so a lot of type conversions are handled through Nanobind's discretion, with some exceptions.

### Primitives
| Rust Type |   Python Type   |
|-----------|------------|
|    u8     |   int  |
|    u16    |  int  |
|    u32    |  int  |
|    u64    |  int  |
|    u128   | unsupported|
|    i8     |    int  |
|    i16    |    int |
|    i32    |    int |
|    i64    |    int |
|    i128   | unsupported|
|   bool    |   bool     |
|   char    |   str |
|   isize   |   int |
|   usize   |   int   |
|    f32    |    float   |
|    f64    |    float  |

### Struct Types
|    Diplomat Type                       |       Python Type      |
|----------------------------------------|-------------------|
|  `#[diplomat::opaque] pub struct Type` | `class Type`    |
|           `pub struct Type`            | `class Type`|
|           `pub enum Type`              | `class Type`       |

#### Opaques
These are Python classes bound to C++ through Nanobind.

#### Structs
These are Python classes, bound similarly to opaques; however, each field is defined as properties on the class.

Note that Nanobind treats structures in Python as classes that hold mutable references to an underlying C++ type. For this reason, Nanobind treats every struct as if it is a [mutable reference when generating C++ bindings](./cpp.md#mut-struct).
However, if you still wish to access these structs mutably within your own method bindings, you will still need to tag them with [`#[diplomat::attr(*, mut_struct_ref)]`](../attrs/references.md).

#### Enums
These are also bound as classes, but with an inner bound `enum.Enum` class. The variants can be accessed through the parent `Type.*`, but the inner enum type can be accessed through `Type.Type`. 

### Options
Any `Option<T>` type in Python is represented as a `T | None` value.

### Results
All `Result<T, E>` functions return `T`, and throw an `Exception` with `Exception.args` set to contain `E` converted into Python:

```py
try:
    somelib.FailingFunction()
except Exception as e:
    error_type = e.args[0]
```

### Slices
|    Diplomat Type                       |       Python Type      |
|----------------------------------------|-------------------|
|           `&[Primitive]`               |`List[Primitive]` or `ndarray((N,), dtype=Primitive)`|
|`&str` or `&DiplomatStr` or `DiplomatStrSlice` or `DiplomatUtf8StrSlice`|`str`|
|`&DiplomatStr16` or `DiplomatStr16Slice`|Unsupported|
|`&[&str] or &[DiplomatStrSlice]` or `&[DiplomatUtf8StrSlice]`|`str`|
|`&[DiplomatStr16Slice]`|Unsupported|

Slices in the Nanobind backend are `List` types. Generally, lists are copied on the C++->Python boundary, except for [numpy types](#numpy) and [specialized slice types](#slices-copying-on-the-boundary).

#### NumPy
If you have `ndarray` support through `NumPy`, and the inner slice type is supported by `dlpack`, Diplomat will return an `ndarray` type. These are passed by reference, and are not copied over the C++->Python boundary. 

### DiplomatWrite
The Nanobind backend uses the [default C++ implementation for DiplomatWrite](./cpp.md#diplomatwrite), and so returns a `str` type.

### Callbacks
Implemented as any ordinary Python function (lambda or `def` will work).

## Debugging
Nanobind `.pyd` files can be stepped through using any debugger. As long as you've built the `.pyd` file with debugging symbols, you can attach to any running Python process that has the `.pyd` imported. Here are the steps:

1. Launch a Python process and import the library.
2. Get the PID of the Python process (`import os; print(os.getpid())`)
3. Attach the debugger to the process (for LLDB, this is `lldb -p PID`).
4. Add breakpoints as you would normally.

Alternately, you can use [`breakpoint()`](https://docs.python.org/3/library/functions.html#breakpoint) right before the code you wish to debug, and follow from step 2 above.

## Slices Copying on the Boundary
Nanobind supports taking slices:

```
#[diplomat::bridge]
mod ffi {
    #[diplomat::attr(auto, abi_compatible)]
    pub struct Foo {
        x: i32,
        y : i32
    }

    impl Foo {
        pub fn takes_slice(sl : &[Foo]) {
            for s in sl.iter() {
                println!("{}", s.x);
            }
        }
    }
}
```

However, note that this is a *copy* of the slice. Diplomat's bindings will automatically do conversion for immutable slice types, by copying to a list type that Nanobind understands. However, if you wish to pass over a reference to a given list, Diplomat will automatically generate a `TSlice` type (i.e., `somelib.FooSlice`):

```python
f = somelib.FooSlice([somelib.Foo(x=10, y=10)])
somelib.Foo.takes_slice(f)
```
Which will copy the slice's memory.

### Explanation

Converting Rust types to and from Python is not straightforward. Every `list` object in Python is a [sequence of `PyObject` types](https://docs.python.org/3/c-api/list.html) in C. For passing information to and from Rust, this makes straightforward conversion extremely difficult. Instead, nanobind will copy Python types into C++ memory layouts it understands.

This is why we the `somelib.FooSlice` type exists. In nanobind terminology, this is a [bound object](https://nanobind.readthedocs.io/en/latest/exchanging.html#option-2-bindings), or a class that exists in Python that allows us to easily grab its memory and manipulate in C++. Any `list` you pass into a parameter that takes `&[Foo]` as an input type will copy the contents of the `list` upon conversion into C/C++ into `somelib.FooSlice`.

{{supports("nanobind")}}