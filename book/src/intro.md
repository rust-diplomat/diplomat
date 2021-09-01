# Introduction

[Diplomat] is a framework and tool for generating bindings to Rust libraries from an extensible set of languages.

Diplomat is for _unidirectional_ bindings: it's for when foreign code wishes to call into a Rust library, but not vice versa. If you're looking for bidirectional bindings, tools like [cxx](https://github.com/dtolnay/cxx) are a good bet.

Diplomat is a proc macro paired with a tool. The proc macro is capable of generating an `extern "C"` binding layer around tagged Rust code, while the tool is able to generate corresponding C, C++, JS, or `<insert language here>` that philosophically matches the API on the Rust side. This means that methods in Rust map to "methods" in the target language, `Result` in Rust map to tagged unions in C++ and exceptions in Javascript, etc. These all work through the generated C API, however they preserve higher level API features which C cannot express.

## A note on the design

You can read the full design doc [here](https://github.com/rust-diplomat/diplomat/blob/main/docs/design_doc.md). 

Diplomat does not do cross-crate global analysis, it restricts its view to specially tagged modules, and only generates bindings based on information found in those modules. This means that changing some struct in some dependency will not magically affect your generated C++/JS/etc APIs; all such change can only come from deliberate change to these tagged modules. This also means that Diplomat can cleanly define a subset of Rust used for declaring the exported API without impacting the flavor of Rust used in dependencies. One can imagine `#[diplomat::bridge]` blocks to almost be a DSL for bridging between your Rust APIs and a more general API shape that can be translated cleanly across languages.

Diplomat is designed such that it should not be a large amount of effort to write new language targets for Diplomat.


## Setup

To install the `diplomat` CLI tool, run

```shell
$ cargo install diplomat-tool
```

Note that right now Diplomat is still subject to change and it might be better to install from a git revision for the latest features.


You can then add `diplomat` as a dependency to your project like so:

```toml
diplomat = "0.2.0"
diplomat-runtime = "0.2.0"
```

It is recommended to create a separate crate for the FFI interface. Diplomat will only read the contents of specially tagged modules so it is possible to mix Diplomat code with normal Rust code, but it is prefereable to minimize this since proc macros can make debugging hard.


 [Diplomat]: https://github.com/rust-diplomat/diplomat