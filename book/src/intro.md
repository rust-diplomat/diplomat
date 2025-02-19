# Introduction

[Diplomat] is a framework and tool for generating bindings to Rust libraries from an extensible set of languages.

Diplomat is for _unidirectional_ bindings: it's for when foreign code wishes to call into a Rust library, but not vice versa. If you're looking for bidirectional bindings, tools like [cxx](https://github.com/dtolnay/cxx) are a good bet.

Diplomat is a proc macro paired with a tool. The proc macro is capable of generating an `extern "C"` binding layer around tagged Rust code, while the tool is able to generate corresponding C, C++, JS, or `<insert language here>` that philosophically matches the API on the Rust side. This means that methods in Rust map to "methods" in the target language, `Result` in Rust map to tagged unions in C++ and exceptions in Javascript, etc. These all work through the generated C API, however they preserve higher level API features which C cannot express.

## A note on the design

You can read the full design doc [here](https://github.com/rust-diplomat/diplomat/blob/main/docs/design_doc.md). 

Diplomat does not do cross-crate global analysis, it restricts its view to specially tagged modules, and only generates bindings based on information found in those modules. This means that changing some struct in some dependency will not magically affect your generated C++/JS/etc APIs; all such change can only come from deliberate change to these tagged modules. This also means that Diplomat can cleanly define a subset of Rust used for declaring the exported API without impacting the flavor of Rust used in dependencies. One can imagine `#[diplomat::bridge]` blocks to almost be a DSL for bridging between your Rust APIs and a more general API shape that can be translated cleanly across languages.

Diplomat is designed such that it should not be a large amount of effort to [write new language targets for Diplomat](developer.html).


## Backends supported


Diplomat currently supports the following backends:

 - C
 - C++
 - JavaScript/TypeScript (using WASM)
   - [demo_gen](./demo_gen/intro.md)
 - Dart
 - Kotlin (using JNA)
 
There is work in progress for a [Java backend] (using Panama). We used to have a .NET backend but it was removed in a refactor, it may get added again.

We're happy to fix bugs or add configurability in the current backends if their produced output does not match what you need in your language. Details on how to write new backends is documented [later in this book](developer.html): you can do so as a third party library depending on `diplomat_core`, but we are also happy to accept these into Diplomat with the understanding that we'll only do minimal work to keep them working over time.

## Setup

To install the `diplomat` CLI tool, run

```shell
$ cargo install diplomat-tool
```

Let's say this installs `diplomat-tool 0.8.0`

You can then add `diplomat` as a dependency to your project like so:

```toml
diplomat = "0.8.0"
diplomat-runtime = "0.8.0"
```

It is recommended to create a separate crate for the FFI interface. Diplomat will only read the contents of specially tagged modules so it is possible to mix Diplomat code with normal Rust code, but it is prefereable to minimize this since proc macros can make debugging hard.


 [Diplomat]: https://github.com/rust-diplomat/diplomat
 [Java backend]: https://github.com/rust-diplomat/diplomat/issues/144