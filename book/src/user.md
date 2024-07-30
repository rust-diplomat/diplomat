# User Guide

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
