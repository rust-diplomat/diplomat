# Custom Extra Code
Currently supported by `cpp` and `nanobind`.

For developers who have a greater understanding of how Diplomat backends function, you are able to mark `enum` and `struct` definitions to insert custom extra binding code:

```rs
#[diplomat::bridge]
mod ffi {
    #[diplomat::attr(cpp, custom_extra_code(source="void extraFunc();", location="def_block"))]
    pub struct SomeType {}
}
```

You can currently specify three locations:
- `def_block`
    - Extra code inserted into the class *definition* of a type.
    - Supported by C++/Nanobind.
- `impl_block`
    - Extra code inserted into the class *implementation* of a type.
    - Supported by C++/Nanobind.
- `init_block`
    - Extra code inserted into any initialization code for a type.
    - Supported by nanobind.

You can also either specify `source="SOURCE CODE"` or `file=/path/to/file` for where to grab the code from. For files, you can customize the directory Diplomat searches from with the `custom_extra_code_location` config.