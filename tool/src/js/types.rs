use std::collections::HashMap;

use diplomat_core::ast;

#[derive(PartialEq, Eq, Debug)]
pub enum ReturnTypeForm {
    /// A struct recursively containing no scalar fields.
    Empty,

    /// A single scalar or a struct recursively containing only a single scalar.
    Scalar,

    /// A struct recursively containing multiple scalar fields.
    Complex,
}

/// Determines what return form the given return type will be translated to
/// in the WASM ABI.
///
/// See https://github.com/WebAssembly/tool-conventions/blob/master/BasicCABI.md#function-signatures.
pub fn return_type_form(
    typ: &ast::TypeName,
    in_path: &ast::Path,
    env: &HashMap<ast::Path, HashMap<String, ast::ModSymbol>>,
) -> ReturnTypeForm {
    match typ {
        ast::TypeName::Named(_) => match typ.resolve(in_path, env) {
            ast::CustomType::Struct(strct) => {
                let all_field_forms: Vec<ReturnTypeForm> = strct
                    .fields
                    .iter()
                    .map(|f| return_type_form(&f.1, in_path, env))
                    .collect();

                let scalar_count = all_field_forms
                    .iter()
                    .filter(|v| v == &&ReturnTypeForm::Scalar)
                    .count();
                let complex_count = all_field_forms
                    .iter()
                    .filter(|v| v == &&ReturnTypeForm::Complex)
                    .count();

                if scalar_count == 0 && complex_count == 0 {
                    ReturnTypeForm::Empty
                } else if scalar_count == 1 && complex_count == 0 {
                    ReturnTypeForm::Scalar
                } else {
                    ReturnTypeForm::Complex
                }
            }

            ast::CustomType::Opaque(_) => ReturnTypeForm::Scalar,
            ast::CustomType::Enum(_) => ReturnTypeForm::Scalar,
        },

        ast::TypeName::Result(ok, err) => {
            let ok_form = return_type_form(ok, in_path, env);
            let err_form = return_type_form(err, in_path, env);

            if ok_form == ReturnTypeForm::Empty && err_form == ReturnTypeForm::Empty {
                ReturnTypeForm::Scalar
            } else {
                ReturnTypeForm::Complex
            }
        }

        ast::TypeName::Option(underlying) => return_type_form(underlying, in_path, env),

        ast::TypeName::Unit => ReturnTypeForm::Empty,

        ast::TypeName::Box(_) => ReturnTypeForm::Scalar,

        ast::TypeName::Reference(_, _) => ReturnTypeForm::Scalar,

        ast::TypeName::StrReference => ReturnTypeForm::Scalar,

        ast::TypeName::Primitive(_) => ReturnTypeForm::Scalar,

        ast::TypeName::Writeable => panic!("Cannot return writeable"),
    }
}
