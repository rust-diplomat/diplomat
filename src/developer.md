# How to Set up a Minimal Backend
How to create a backend is quite language dependent, and what is easier in one
may be harder in another. Below we will start setting up a simple backend. We'll 
show you how to set up a test in diplomat so you can start generating code quickly.
Then we give you a template for a simple dynamic library that you can then link 
to your host language. Finally we provide a suggested checklist for your backend.
It is not automatically generated so when in doubt look at diplomat's [HIR](https://docs.rs/diplomat_core/latest/diplomat_core/hir/index.html)

- [ ] **project structure**: You will need need to test your generated code so you should
first set up a host language project. It should have all dependencies to be able to interface 
with native code (or WASM).

## Setting up Basic Code Generation in a Test

Your backend should iterate over all [`TypeDefs`](https://docs.rs/diplomat_core/latest/diplomat_core/hir/enum.TypeDef.html)
and generate the required code for these. To do that we start with an 
[`ast::File`](https://docs.rs/diplomat_core/latest/diplomat_core/ast/struct.File.html), which can 
then be parsed into a [`Env`](https://docs.rs/diplomat_core/latest/diplomat_core/struct.Env.html)
using the 
[`all_types`](https://docs.rs/diplomat_core/latest/diplomat_core/ast/struct.File.html#method.all_types) 
method. Then we can create the 
[`TypeContext`](https://docs.rs/diplomat_core/latest/diplomat_core/hir/struct.TypeContext.html) 
which is generated using the 
[`from_ast`]( https://docs.rs/diplomat_core/latest/diplomat_core/hir/struct.TypeContext.html#method.from_ast)
method. You will also need an 
[`AttributeValidator`](https://docs.rs/diplomat_core/latest/diplomat_core/hir/trait.AttributeValidator.html), 
but should probably start with the simple 
[`BasicAttributeValidator`](https://docs.rs/diplomat_core/latest/diplomat_core/hir/struct.BasicAttributeValidator.html).



We will now build an example by way of a test. A good starting point is to create a test for
generating a simple opaque struct without any methods. Your backend should go in the tool crate: 
create a module `tool/src/{backend}/mod.rs`  (make sure you add a line `pub mod backend;` to 
`tool/src/lib.rs`). Add the following to it

```rs
use diplomat_core::hir::{OpaqueDef, TypeContext, TypeId};

fn gen_opaque_def(ctx: &TypeContext, type_id: TypeId, opaque_path: &OpaqueDef) -> String {
    "We'll get to it".into()
}

#[cfg(test)]
mod test {
    use diplomat_core::{
        ast::{self},
        hir::{self, TypeDef},
    };
    use quote::quote;

    #[test]
    fn test_opaque_gen() {
        let tokens = quote! {
            #[diplomat::bridge]
            mod ffi {

                #[diplomat::opaque]
                struct OpaqueStruct;

            }
        };
        let item = syn::parse2::<syn::File>(tokens).expect("failed to parse item ");

        let diplomat_file = ast::File::from(&item);
        let env = diplomat_file.all_types();
        let attr_validator = hir::BasicAttributeValidator::new("my_backend_test");

        let context = match hir::TypeContext::from_ast(&env, attr_validator) {
            Ok(context) => context,
            Err(e) => {
                for (_cx, err) in e {
                    eprintln!("Lowering error: {}", err);
                }
                panic!("Failed to create context")
            }
        };

        let (type_id, opaque_def) = match context
            .all_types()
            .next()
            .expect("Failed to generate first opaque def")
        {
            (type_id, TypeDef::Opaque(opaque_def)) => (type_id, opaque_def),
            _ => panic!("Failed to find opaque type from AST"),
        };

        let generated = super::gen_opaque_def(&context, type_id, opaque_def);

        insta::assert_snapshot!(generated)
    }
}
```

You can now run 
```sh
cargo test -p diplomat-tool -- backend::test --nocapture
```
You should also have a generated snapshot `diplomat_tool__backend__test__opaque_gen.snap.new` 
which you can use to pick up your generated code.

## How to Generate the Library
Now to actually test native methods you will need to create some kind of library, be it static, dynamic, or
even WASM. In the following we will be creating a dynamically linked library.

You should set up a separate rust project next to your diplomat fork e.g. `mybackendtest`
```sh
cargo new --lib mybackendtest
```

with the following Cargo.toml
```toml
[package]
name = "mybackendtest"
version = "0.1.0"
edition = "2021"

[lib]
crate_type = ["cdylib"]
name = "mybackendtest"

[dependencies]
diplomat = {path = "../diplomat/macro"}
diplomat-runtime = {path = "../diplomat/runtime"}
```
Because you are using path dependencies, it is important that your library project be in 
the same directory as your fork of diplomat

Copy the following into your lib.rs
```rs
#[diplomat::bridge]
mod ffi {

    #[diplomat::opaque]
    struct OpaqueStruct;

    impl OpaqueStruct {
        pub fn add_two(i: i32) -> i32 {
            i + 2
        }
    }
}

```
Note it is very important that the method be marked `pub` otherwise diplomat will ignore it.
Now you can run 
```sh
cargo build
```
to create a debug artifact in `target/debug/libmybackendtest.dylib`

## Getting Access to your Native Method

Now we can add code that will iterate over all of the methods of the opaque struct.
First, copy the impl block for `OpaqueStruct` into the test code underneath the `OpaqueStruct`.
Next, update your the code for `gen_opaque_def` to the following which will generate the native 
symbol for your new impl method:
```rust
use crate::c2::CFormatter;

fn gen_opaque_def(ctx: &TypeContext, type_id: TypeId, opaque_path: &OpaqueDef) -> String {
    let c_formatter = CFormatter::new(ctx);

    opaque_def
        .methods
        .iter()
        .map(|method| c_formatter.fmt_method_name(type_id, method))
        .collect::<Vec<_>>()
        .join("\n")
}
```
Now your snapshot should have the following contents
```
---
source: tool/src/backend/mod.rs
assertion_line: 67
expression: generated
---
OpaqueStruct_add_two
```
where `OpaqueStruct_add_two` is the native symbol for your method. It has a simple signature `i32 -> i32`,
so now you have a dynamic library and a symbol to load from it that you can start building. Now it is up
to you to figure how to integrate these into your host language project skeleton.


## Minimal Backend
You should now work on building a minimal backend that can generate opaque type definitions
with methods that only accept and return [**primitive types**](https://docs.rs/diplomat_core/latest/diplomat_core/hir/enum.PrimitiveType.html).

You will need to update `tool/src/lib.rs` to add handling for your backend.

Once you have the basics of a backend you can add attribute handling. The best way to do this is to check the existing backends
e.g. [dart](https://github.com/rust-diplomat/diplomat/blob/b3a8702f6736dbd6e667638ca0025b8f8cd1509f/tool/src/lib.rs#L95)(Note: git permalink may be out of date).
The most important is to ignore disabled types and methods, as then you can take advantage of diplomat's feature tests
and start building progressively.

## Feature Tests
Diplomat already includes feature tests that you can disable with `#[diplomat::attrs(disable, {backend})]`.
where `{backend}` refers to your new backend. As you add functionality to your backend you 
can progressively enable the types and methods for your backend. This way you can iterate with 
working examples. These are called via [cargo-make](https://sagiegurari.github.io/cargo-make/)
e.g
```sh
cargo make gen-dart-feature
```
You can look at `Makefile.toml` to see how tasks are defined. Most of the generative tasks make use of this
[duckscript function](https://github.com/rust-diplomat/diplomat/blob/b3a8702f6736dbd6e667638ca0025b8f8cd1509f/support/functions.ds#L1)
([Duckscript](https://sagiegurari.github.io/duckscript/) is a simple scripting language)

## Backend Checklist

- [ ] [**primitive types**](https://docs.rs/diplomat_core/latest/diplomat_core/hir/enum.PrimitiveType.html): This will be the most basic piece of the backend, and you will want
to implement them early in order to test your ability to correctly call methods. 
- [ ] [**opaque types**](): 
  - [ ] basic definiton
  - [ ] return a boxed opaque. This needs to be cleaned in managed languages.
  You can use the autogenerated `{OpaqueName}_destroy({OpaqueName}*)` native method to clean up 
  the memory of the associated opaque.
  - [ ] as self parameter
  - [ ] as another parameter
- [ ] [**structs**](https://docs.rs/diplomat_core/0.7.0/diplomat_core/hir/struct.StructDef.html)
- [ ] [**enums**](https://docs.rs/diplomat_core/0.7.0/diplomat_core/hir/struct.EnumDef.html)
- [ ] [**writeable**](https://docs.rs/diplomat_core/0.7.0/diplomat_core/hir/enum.SuccessType.html#variant.Writeable)
- [ ] [**slices**](https://docs.rs/diplomat_core/0.7.0/diplomat_core/hir/enum.Slice.html)
  - [ ] primitive slices
  - [ ] str slices
  - [ ] owned slices
  - [ ] slices of strings
  - [ ] strings
- [ ] borrows. This is probably one of the trickiest things, as you need to ensure that managed objects don't get
cleaned up if something depends on them.
  - [ ] borrows of parameters
  - [ ] in struct fields
- [ ] nullables, i.e. returning option types.
- [ ] fallibles, i.e. returning result types. The resulting native type will be a discriminated union.
