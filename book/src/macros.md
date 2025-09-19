# Macros

Diplomat has support for generating bindings with macro_rules!, with quite a few caveats:

1. Any `macro_rules` definitions that you wish Diplomat to evaluate must use the `#[diplomat::macro_rules]` attribute.
2. Macros can only contain a single arm. The arm can contain any syntax as defined , with three exceptions:
    - The `pat` and `stmt` MacroFragSpecs are currently forbidden.
    - The `$(MacroMatch+) MacroRepSep? MacroRepOp` pattern is currently forbidden.
3. Macros can exist only in `#[diplomat::bridge] mod ... { ... }` blocks or `impl` blocks.
4. Macros exist solely in the file you've defined them in. They must also be defined first.
5. Macros do not use interpolation matchers, nor do they allow recursive usage of other `#[diplomat::macro_rules]` macros.
