# Diplomat Design doc

# Problem statement

A common task in the world of Rust FFI is taking a self-contained Rust project and making it possible to call from other languages. This is typically done through a C API, requiring clients to write their own wrappers.
Sometimes it is ideal if the team can maintain their own wrappers, and it would be ideal if these wrappers were auto generated over the same stable-ish C API.

# Background

There are many many tools for making Rust FFI pleasant to work with. Some of them exist to make it easier for Rust to call C++ code ([bindgen](https://github.com/rust-lang/rust-bindgen)), some exist to make it easier for C/C++ to call Rust ([cbindgen](https://github.com/eqrion/cbindgen), [gir](https://github.com/gtk-rs/gir/) for GTK), and some exist to make bidirectional integration better ([cxx](https://github.com/dtolnay/cxx), [autocxx](https://github.com/google/autocxx)).

Such tools prioritize a single pair of languages, so the state of the art for generating bindings for multiple languages is to use cbindgen and then manually write wrappers for all the other languages. This is fine, but it means that you have to often write manual target-language FFI code, which can be somewhat fraught. Notably, it's possible for the bindings to go out of sync from the actual C API (or to be written with bugs), causing strange segfaults or link errors. This is the reason we use tools like cbindgen in the first place, so it's somewhat unsatisfying that we can only solve the problem at the C-Rust layer and not the C-otherlanguage layer.

I noticed this gap while performing my [FFI Investigation](https://docs.google.com/document/d/1Y1mNFAGbGNvK_I64dd0fRWOxx9xqi12dXeLivnxRWvA/edit?resourcekey=0-l9QvvqXW7cC-TrfLWt7nZw#heading=h.mwn95cq0jjht), and while ICU4X's needs are satisfied with manual bindings, such a tool would be useful for us and the broader community.

## Requirements

-   Users should be able to write one or multiple cxx-style tagged bridge blocks of code to define their public FFI API (<span style="color:red">**required**</span>)
-   It should be easy for the user to structure their bridge block in a way that controls what the generated language side looks like (<span style="color:orange">**preferred**</span>)
-   The bindings should all go through an underlying C layer (<span style="color:red">**required**</span>)
-   The underlying C layer should be stable unless the tagged block is changed (<span style="color:orange">**preferred**</span>)
-   It should be easy to write a "language target" for the tool as a plugin (<span style="color:red">**required**</span>)
-   It should be possible for users to write plugin-specific tags/blocks on the Rust side to signal language-specific semantics to the plugin (<span style="color:orange">**preferred**</span>)
-   It would be nice if it were possible to run this tool on multiple interdependent crates and produce modular bindings (<span style="color:#729468">**optional**</span>)
-   It would be nice if it were possible to export types that are outside of a bridge block, cbindgen-style (<span style="color:#729468">**optional**</span>)
-   The bindings should be able to autogenerate the appropriate conversions to/from idiomatic types on either side (<span style="color:orange">**preferred**</span>)
-   Should support WebAssembly bindings as well as C FFI (<span style="color:orange">**preferred**</span>)

## Design

The design is heavily based on [cxx](https://github.com/dtolnay/cxx), and can almost be thought of as "cxx, but if the target language were pluggable". I'm going to reuse the "bridge" terminology from cxx for the sake of familiarity, but ultimately it could be called something else.

This design is super rough; consider it to be an example of a general direction.

### Invocation

Diplomat will provide a proc macro for generating the C bindings, as well as a `diplomat` tool that can produce an API description. For a first pass it should be sufficient to provide libraries so that plugins can be written as binaries that consume the API description.


### Bridge blocks

Essentially the user can define blocks like the following:

```rust=
// The Rust-side PluralRules.
struct PluralRules {
 // ...
}
// ...

#[diplomat::bridge]
mod ffi {
    struct PluralError {
       // snip
    }
    enum PluralCategory {
       // snip
    }
    // #[repr(C)] automatically added
    #[opaque]
    struct PluralRules(super::PluralRules);

    impl PluralRules {
       fn new(...) -> Result<Box<Self>, Error> {
            Box::new(Self(super::PluralRules::new(...)?)) };
       }
       fn select(&self, ...) -> PluralCategory {
           self.0.select(...)
       }
   }
}
```

This will internally generate something like:

```rust
mod ffi {
    // same code as above for PluralError, PluralCategory, PluralRules,
    // and `impl PluralRules`

    #[repr(C)]
    struct PluralNewResult {
      // something sensible to represent Result<PluralRules, PluralError>
    }

    // generate From implementation for PluralNewResult

    extern "C" {
       type PluralRulesOpaque;
    }

    #[no_mangle]
    extern "C" fn PluralRules_new(...) -> PluralRulesResult {
        // potentially convert arguments
        PluralRules::new(...).into();
    }
    #[no_mangle]
    extern "C" fn PluralRules_select(pr: *const PluralRules, ...)
        -> PluralCategory {
        // potentially convert arguments
        pr.select(...);
    }
}
```

(We may have to assume `Box<T>` is layout-compatible with `*mut T` for this. If that’s not possible, we may need users to use our own variant of `Box<T>` for this pattern)


### Generated C Code

The bridge block above can also be fed to the tool to generate C header code that looks like the following:

```c
struct PluralRulesRust;

struct PluralRules {
   PluralRulesRust* inner;
}

enum PluralCategory {..};

struct PluralRulesResult {
   // ...
};

PluralRulesResult PluralRules_new(...);
PluralCategory PluralRules_select(PluralRules* pr);
void PluralRules_destroy(...);
```

It would be nice if certain enums could be bridged easily without having to duplicate their definitions, but this can be done via additional features (e.g. a proc macro you apply to the enum itself).

This C code is not intended to be called directly. It will be _stable_, however it may not be _idiomatic_, and someone desiring idiomatic C will likely need to write a C plugin.

_Ideally_ we only need to generate C headers, not object files, since those would complicate the compilation model.

### Plugin interface

A language target plugin (say, Java) would be fed information of the form:

-   There is a PluralRules type with given layout (contents are opaque unless all fields are public). Layout can potentially be lazily calculated.
-   There is a PluralCategory type with a given layout and contents
-   There is a PluralError type with given layout
-   It has one constructor with a given signature. It can be called via PluralRules_new()
-   It has one method select() with a given signature, that can be called via PluralRules_select()
-   It has a destructor that can be called with PluralRules_destroy

It is then allowed to generate whatever it likes based off of this.

For a first pass, this API will likely be handled by the `diplomat` tool generating a JSON structure with this information that other "plugin" tools can consume, and having `diplomat` provide libraries for working with these objects.


### Generated Java example

A Java plugin could potentially generate the following API:

```java
public class PluralRules {
    public PluralRules(...) throws PluralError {
       // call PluralRules_new(), convert any types
    }

    public void select(...) {
       // call PluralRules_select(), convert any types
    }

    // could also generate a finalizer, AutoCloseable, or something else
    public void destroy(..) {
       // call PluralRules_destroy()
    }
}
```

It may be desirable to tweak some things about the generated code, for example we may want `select()` to be `protected` for some reason, or we may want to load in additional methods from a file. The plugin could register interest in additional attributes, for example we could support something like:

```rust
#[diplomat::bridge]
mod ffi {
    // snip
    #[diplomat_java::extra_methods(./PluralRulesExtra.java)]
    impl PluralRules {
        #[diplomat_java::protected]
        fn select(...) -> PluralCategory {
          // snip
       }
    }
```

Individual plugins can basically do whatever they want here. They’re free to use their vision of idiomaticness, or expose many different knobs, or whatever else.


### Builtin types

It is worth being able to generate things for as many builtins as possible. For example, we can generate `FooSlice` for `&[Foo]`, `FFIStr` for `&str`, etc. The C API is not expected to be ergonomic, as long as the plugins can use it to do the appropriate conversions.
We can ideally generate these in separate headers that will be compatible cross-project.

### Bridge blocks are self-contained

It’s important that the bridge blocks be self-contained. To perform code generation, the tool must only know the contents of the bridge blocks in a crate, and nothing else. Unknown external types are treated as opaque.

To aid this it might be necessary for the annotations to generate some layout-checking tests to ensure that an external “type” is not actually a trait object in disguise.

It is okay for bridge blocks to refer to types from each other. We might even make it possible for `diplomat`-using crates to be layered such that the icu_datetime crate can assume the existence of the FFI generated by its dependency, icu_numberformat. This can be done by having some json that gets generated. and passed around. However, `diplomat` will never look outside of bridge blocks.

A small issue here is paths: if we’re not looking outside of bridge blocks it might be tricky to figure out what the paths are if we still want bridge blocks to be able to reference each other. We can make an exception for understanding the module structure of the program, or we can declare that paths to things that share the same name are the same type, or we can require that imports annotate whether they are opaque or internal.

The reason behind all this is that doing full dependency analysis properly is hard – you need to properly understand your dependency tree and parse everything and understand all the paths and imports.. 

Furthermore, it adds a ton of nonlocality to the problem, we want this property for the tool because we really want this property for the FFI layer too: one should not have to look past the bridge blocks to understand the FFI layer. This means that any given part of the FFI layer is completely defined in one place, and it’s impossible to break the FFI layer by changing code in the rest of the project. This is definitely a property ICU4X wants, to the point that we are happy to have a divergence between FFI enums and Rust enums since some Rust enums can be marked `#[non_exhaustive]` and we can evolve their FFI counterparts at a different cadence if necessary

### Differentiating between "object types" and "bag of stuff"

In [ICU4X: FFI solutions](https://docs.google.com/document/d/1Y1mNFAGbGNvK_I64dd0fRWOxx9xqi12dXeLivnxRWvA/edit?resourcekey=0-l9QvvqXW7cC-TrfLWt7nZw#heading=h.mwn95cq0jjht) I distinguish between "object types" (opaque, have semantics) and "bag of stuff" (public fields, usually `Copy`, usually for option bag types). We probably want to do something similar here, perhaps changing the codegen based on whether or not the type has private fields.

### Integrating with an existing tool

It may be possible to do this within the framework of cbindgen or cxx. I have not attempted to figure out the feasibility of this, and would be glad if we could have such functionality without having to write Yet Another Tool.
