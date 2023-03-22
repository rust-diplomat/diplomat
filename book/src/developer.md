# Backend Developer Guide


This is yet to be fleshed out. In general, if trying to write a backend, please use the [Diplomat HIR] ("higher level IR"). This is similar to a syntax tree but is far easier to work with, with paths being pre-resolved and a bunch of invalid states being unrepresentable.


It's obtained from a [`TypeContext`], which is itself constructed from an [`Env`], from an [`ast::File`], which can be constructed from a `syn` module covering the entire crate.


Currently Diplomat has `c2` and `cpp2` backends that use the HIR. The other backends still use the AST, but we hope to move off of that. We recommend you look at the `c2` and `cpp2` backends of `diplomat-tool` to understand how to implement your own backend.


You can write a new backend as a standalone library, or as a module under `tool`. The Diplomat team is happy to accept new modules but may not necessarily commit to keeping them working when Diplomat changes. We promise to notify you if such a module breaks, and will always try to fix things when it's a minor change.



 [Diplomat HIR]: https://docs.rs/diplomat_core/latest/diplomat_core/hir/index.html
 [`TypeContext`]: https://docs.rs/diplomat_core/latest/diplomat_core/hir/struct.TypeContext.html
 [`Env`]: https://docs.rs/diplomat_core/latest/diplomat_core/struct.Env.html
 [`ast::File`]: https://docs.rs/diplomat_core/latest/diplomat_core/ast/struct.File.html