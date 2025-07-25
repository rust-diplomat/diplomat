# Macros

Diplomat has support for generating bindings with macro_rules!, with quite a few caveats:

1. Any `macro_rules` definitions that you wish Diplomat to evaluate must use the `#[diplomat::macro_rules]` attribute.
2. Macros can only contain a single arm.
    - The arm can only contain a sequence of comma separated matchers. I.e., `$matcher:expr, $matcher:ident, ...` etc.
    - Note that the current implementation of the matcher parser pays no mind to fragment specifiers of the matcher. More robust parsing is TBD.
3. Macros can exist only in `#[diplomat::bridge] mod ... { ... }` blocks or `impl` blocks.
4. Macros exist solely in the file you've defined them in. They must also be defined first.
5. Macros do not use interpolation matchers, nor do they allow recursive usage of other `#[diplomat::macro_rules]` macros.
