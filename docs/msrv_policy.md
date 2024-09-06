# Minimum Supported Rust Versions policy

The diplomat libraries that end up as dependencies of diplomat-using crates (`diplomat`, `diplomat_core`, `diplomat-runtime`) attempt to maintain a reasonable Minimum Supported Rust Version policy for its users.

At the moment, the primary user of Diplomat is [ICU4X](https://github.com/unicode-org/icu4x), and as such Diplomat matches ICU4X's [MSRV polic
y], with a tweak:

 - Diplomat releases must have an MSRV that is 4 or more releases before the current stable Rust (the version that Rust will be when Diplomat is released)
 - Diplomat maintainers may upgrade MSRV to something that will be 6 or more releases before current stable Rust without needing justification.
 - Upgrading MSRV to something between 4 and 6 releases before current stable Rust requires justification listing the concrete benefit Diplomat gets from this change.
 - When a Diplomat MSRV update on Diplomat `main` will affect MSRV on ICU4X `main`, ICU4X must sign off on the MSRV update, either by policy, or by discussion when needed.
 
When it comes to dependencies, Diplomat's MSRV implies that _some_ combination of dependency versions satisfies that MSRV. This combination can usually be found in Diplomat's lockfile, which is tested by CI.


The tool library, `diplomat-tool`, does not maintain any MSRV.


Diplomat is open to tweaking this policy to support the needs of other downstream users.


 [MSRV policy]: https://github.com/unicode-org/icu4x/blob/main/documents/process/rust_versions.md