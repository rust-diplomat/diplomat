// ───────────────────────────────────────────────────────────────────────
// WIP — .NET callback support is incomplete.
//
// This module sketches out the C#-side `DiplomatCallback_*` wire struct
// + delegate plumbing for `impl Fn` / `impl FnMut` bridge parameters,
// but the implementation is unfinished. The backend's
// `attr_support.callbacks` flag is currently set to `false`, so HIR
// validation rejects any bridge that uses callbacks before this code
// path is reached. The struct definition + template (callback.cs.jinja)
// are in tree so the day we flip the flag, the wiring is already in
// place. Until then this file compiles but its output isn't exercised.
//
// Outstanding work to make this complete:
//  - Lifetime tracking for captured GCHandles on the C# side.
//  - Slice / opaque / struct callback parameter types (currently only
//    primitives are lowered — see `lower_callback_param_type`).
//  - Result / Option callback return types.
//  - Escaping vs sync callback distinction.
//  - End-to-end smoke test against feature_tests/src/callbacks.rs.
// ───────────────────────────────────────────────────────────────────────

use askama::Template;

use crate::dotnet::gen::method::{DotnetReturnType, MethodInputContext};

#[derive(Template)]
#[template(path = "dotnet/callback.cs.jinja", escape = "none")]
pub struct DotnetCallback {
    pub namespace: String,
    pub name: String,
    pub return_type: DotnetReturnType,
    pub args: String,
    pub callback_args: String,
    pub callback_arg_names: String,
    pub idiomatic_type: String,
}

impl DotnetCallback {
    pub(super) fn new(
        namespace: String,
        input_context: &MethodInputContext<'_>,
        return_type: DotnetReturnType,
        args: String,
        callback_args: String,
        callback_arg_names: String,
        idiomatic_type: String,
    ) -> Self {
        let method_context = input_context.method();
        let method_abi_name = method_context.method_abi_name();
        let param_name = if input_context.param_ident().is_empty() {
            format!("arg{}", input_context.param_index())
        } else {
            input_context.param_ident().to_string()
        };
        let name = format!("DiplomatCallback_{method_abi_name}_{param_name}");

        Self {
            namespace,
            name,
            return_type,
            args,
            callback_args,
            callback_arg_names,
            idiomatic_type,
        }
    }

    pub(super) fn run_delegate_name(&self) -> String {
        format!("{}_Run", self.name)
    }
}
