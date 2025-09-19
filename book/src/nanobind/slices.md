# Slices

## Mutable Slices and Copying on the Boundary
Nanobind supports taking mutable slices:

```
#[diplomat::bridge]
mod ffi {
    #[diplomat::attr(auto, abi_compatible)]
    pub struct Foo {
        x: i32,
        y : i32
    }

    impl Foo {
        pub fn takes_mut_slice(sl : &mut [Foo]) {
            for s in sl.iter() {
                s.x += 1;
            }
        }
    }
}
```

However, if you attempt to mutate the slice in Python:

```python
f = [somelib.Foo(x=10,y=10)]
print([foo.x for foo in f])
somelib.Foo.takes_mut_slice(f)
print([foo.x for foo in f])
```

You will get an error:

```
[10]
TypeError: takes_mut_slice(): incompatible function arguments. The following argument types are supported:
    1. mutable_slice(s: collections.abc.Sequence[somelib.somelib.Foo]) -> None
Invoked with types: list
```

Diplomat's bindings will automatically do conversion for immutable slice types, by copying to a list type that Nanobind understands. However, if you wish to pass over a reference to a given list, Diplomat will automatically generate a `TSlice` type (i.e., `somelib.FooSlice`):

```python
f = somelib.FooSlice([somelib.Foo(x=10, y=10)])
print([foo.x for foo in f])
somelib.Foo.takes_mut_slice(f)
print([foo.x for foo in f])
```

Which prints the correct result:

```
[10]
[11]
```

### Explanation

Converting Rust types to and from Python is not straightforward. Every `list` object in Python is a [sequence of `PyObject` types](https://docs.python.org/3/c-api/list.html) in C. For passing information to and from Rust, this makes straightforward conversion extremely difficult. Instead, nanobind will copy Python types into C++ memory layouts it understands.

This is why we the `somelib.FooSlice` type exists. In nanobind terminology, this is a [bound object](https://nanobind.readthedocs.io/en/latest/exchanging.html#option-2-bindings), or a class that exists in Python that allows us to easily grab its memory and manipulate in C++. Any `list` you pass into a parameter that takes `&[Foo]` as an input type will copy the contents of the `list` upon conversion into C/C++ into `somelib.FooSlice`.