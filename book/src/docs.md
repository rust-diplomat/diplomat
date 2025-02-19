# Documentation


Some Diplomat backends support `--docs`, which will generate additional documentation from your Markdown doc comments

```shell
$ diplomat-tool cpp cpp/ --docs cpp-docs/
```

The C++ and JS backends generate Sphinx docs. If using TypeScript, the definition files will automatically come with `tsdoc`-compatible doc comments.


A limited amount of intra-doc-links are supported: it is possible to link to custom _types_ (but not methods or variants) using ``[`FooBar`]`` syntax, like Rust.

Furthermore, you can use `#[diplomat::rust_link(path::to::rust::type, Struct)]` to autogenerate links for to published docs, which typically show up as a "For more information see \<link\>" at the bottom of the docs for the given item. Since Diplomat cannot do resolution on other crates, it relies on the `rust_link` annotation to provide the kind of Rust item or doc page being linked to. An additional `compact` parameter can be passed in case you wish to provide multiple `rust_link`s that are to be collapsed into a single "For more information see 1, 2, 3" line.

Put together, this might look something like the following:

```rust
#[diplomat::bridge]
mod ffi {
    use my_thingy::MyThingy;

    /// A Thingy
    #[diplomat::rust_link(my_thingy::MyThingy, Struct)]
    #[diplomat::opaque]
    pub struct Thingy(MyThingy);

    #[diplomat::enum_convert(my_thingy::SpeedSetting)]
    #[diplomat::rust_link(my_thingy::SpeedSetting, Enum)]
    pub enum SpeedSetting {
        Fast, Medium, Slow
    }

    #[diplomat::enum_convert(my_thingy::ThingyStatus)]
    #[diplomat::rust_link(my_thingy::ThingyStatus, Enum)]
    pub enum ThingyStatus {
        Good,
        Bad
    }

    impl Thingy {
        /// Make a [`MyThingy`]!
        #[diplomat::rust_link(my_thingy::MyThingy::new, FnInStruct)]
        pub fn create(speed: SpeedSetting) -> Box<Thingy> {
            Box::new(Thingy(Thingy::new(speed.into())))
        }

        /// Get the status
        #[diplomat::rust_link(my_thingy::MyThingy::get_status, FnInStruct)]
        pub fn get_status(&self) -> ThingyStatus {
            self.0.get_status().into()
        }
    }
}
```

The full list of item kinds recognized by `rust_link` is:

 - `Struct`
 - `StructField`
 - `Enum`
 - `EnumVariant`
 - `EnumVariantField`
 - `Trait`
 - `FnInStruct`
 - `FnInEnum`
 - `FnInTrait`
 - `DefaultFnInTrait`
 - `Fn`
 - `Mod`
 - `Constant`
 - `AssociatedConstantInEnum`
 - `AssociatedConstantInTrait`
 - `AssociatedConstantInStruct`
 - `Macro`
 - `AssociatedTypeInEnum`
 - `AssociatedTypeInTrait`
 - `AssociatedTypeInStruct`
 - `Typedef`
