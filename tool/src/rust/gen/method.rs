use super::ItemGenContext;
use diplomat_core::hir::{self, Mutability, ReturnType, SelfType, SuccessType, Type, TypeId};

/// A method that can be called through the safe wrapper.
///
/// Parameters cross the ABI as-is, so the safe signature and the `extern` declaration
/// only differ in how `self` and opaque returns are spelled.
pub(super) struct MethodInfo {
    pub(super) docs: String,
    pub(super) name: String,
    pub(super) abi_name: String,
    /// The parameters of the safe method, including `self`
    pub(super) params: Vec<String>,
    /// The parameters of the `extern "C"` declaration
    pub(super) abi_params: Vec<String>,
    /// The arguments passed to the `extern "C"` function
    pub(super) args: Vec<String>,
    /// The return type of the safe method
    pub(super) ret: Option<String>,
    /// The return type of the `extern "C"` declaration
    pub(super) abi_ret: Option<String>,
    /// For a `Box<Opaque>` return, the wrapper type that takes ownership of the pointer
    pub(super) opaque_ret: Option<String>,
}

impl ItemGenContext<'_, '_> {
    pub(super) fn gen_methods(&self, self_id: TypeId, methods: &[hir::Method]) -> Vec<MethodInfo> {
        methods
            .iter()
            .filter(|m| !m.attrs.disable)
            .filter_map(|m| self.gen_method(self_id, m))
            .collect()
    }

    fn gen_method(&self, self_id: TypeId, method: &hir::Method) -> Option<MethodInfo> {
        let mut params = vec![];
        let mut abi_params = vec![];
        let mut args = vec![];

        if let Some(param_self) = &method.param_self {
            match &param_self.ty {
                SelfType::Opaque(path) => {
                    let (receiver, ptr) = match path.owner.mutability {
                        Mutability::Mutable => ("&mut self", "*mut"),
                        Mutability::Immutable => ("&self", "*const"),
                    };
                    params.push(receiver.to_string());
                    let raw = format!("{}Raw", self.formatter.fmt_type_name(self_id));
                    abi_params.push(format!("this: {ptr} {raw}"));
                    args.push("self.0.as_ptr()".to_string());
                }
                SelfType::Struct(_) | SelfType::Enum(_) => {
                    params.push("self".to_string());
                    abi_params.push(format!("this: {}", self.fmt_type_path(self_id)));
                    args.push("self".to_string());
                }
                _ => return None,
            }
        }

        for param in &method.params {
            let ty = self.gen_value_type(&param.ty)?;
            params.push(format!("{}: {ty}", param.name));
            abi_params.push(format!("{}: {ty}", param.name));
            args.push(param.name.to_string());
        }

        let (ret, abi_ret, opaque_ret) = match &method.output {
            ReturnType::Infallible(SuccessType::Unit) => (None, None, None),
            // The wrapper has no lifetime, so it cannot hold on to a borrow
            ReturnType::Infallible(SuccessType::OutType(Type::Opaque(path)))
                if path.is_owned()
                    && !path.is_optional()
                    && path.lifetimes.lifetimes().len() == 0 =>
            {
                let id = path.tcx_id.into();
                let wrapper = self.fmt_type_path(id);
                (
                    Some(wrapper.clone()),
                    Some(format!("*mut {}", self.fmt_opaque_raw(id))),
                    Some(wrapper),
                )
            }
            ReturnType::Infallible(SuccessType::OutType(ty)) => {
                let ty = self.gen_value_type(ty)?;
                (Some(ty.clone()), Some(ty), None)
            }
            _ => return None,
        };

        Some(MethodInfo {
            docs: self.formatter.fmt_docs(&method.docs, "    "),
            name: method.name.to_string(),
            abi_name: method.abi_name.to_string(),
            params,
            abi_params,
            args,
            ret,
            abi_ret,
            opaque_ret,
        })
    }
}
