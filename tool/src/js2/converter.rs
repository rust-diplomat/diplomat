use std::borrow::Cow;

use diplomat_core::hir::{self, LifetimeEnv, ReturnType, SuccessType, Type};

use super::JSGenerationContext;

/// Part of JSGenerationContext that handles conversions between C and JS.
/// This is a partial implementation so I don't have really long files.

fn is_contiguous_enum(ty: &hir::EnumDef) -> bool {
    ty.variants
        .iter()
        .enumerate()
        .all(|(i, v)| i as isize == v.discriminant)
}

impl<'tcx> JSGenerationContext<'tcx> {
	/// Given a type from Rust, convert it into something Javascript will understand.
    pub(super) fn gen_js_type_str<P: hir::TyPosition>(&self, ty: &Type<P>) -> Cow<'tcx, str> {
        match *ty {
            Type::Primitive(primitive) => {
                self.formatter.fmt_primitive_as_ffi(primitive, true).into()
            },
			Type::Opaque(ref op) => {
				let opaque_id = op.tcx_id.into();
				let type_name = self.formatter.fmt_type_name(opaque_id);

				if self.tcx.resolve_type(opaque_id).attrs().disable {
					self.errors
                        .push_error(format!("Found usage of disabled type {type_name}"))
				}

				let ret = if op.is_optional() {
					self.formatter.fmt_nullable(&type_name).into()
				} else {
					type_name
				};

				ret.to_owned().into()
			},
            Type::Enum(ref enumerator) => {
                let enum_id = enumerator.tcx_id.into();
                let type_name = self.formatter.fmt_type_name(enum_id);
                if self.tcx.resolve_type(enum_id).attrs().disable {
                    self.errors.push_error(format!("Using disabled type {type_name}"))
                }
                type_name
            }, 
            _ => todo!("Type {:?} not supported", ty)
        }
    }

	pub(super) fn gen_c_to_js_for_type<P: hir::TyPosition>(&self, ty : &Type<P>, variable_name : Cow<'tcx, str>, lifetime_environment : &LifetimeEnv) -> Cow<'tcx, str> {
		match *ty {
			Type::Primitive(..) => variable_name,
			Type::Enum(ref enum_path) if is_contiguous_enum(enum_path.resolve(self.tcx)) => {
				let id = enum_path.tcx_id.into();
				let type_name = self.formatter.fmt_type_name(id);
				format!("{type_name}[Array.from({type_name}.values.keys())[{variable_name}]]").into()
			},
			Type::Enum(ref enum_path) => {
				let id = enum_path.tcx_id.into();
				let type_name = self.formatter.fmt_type_name(id);
				// Based on Dart specifics, but if storing too many things in memory isn't an issue we could just make a reverse-lookup map in the enum template.
				format!("(() => {{for (let i of {type_name}.values) {{ if(i[1] === {variable_name}) return i[0]; }} return null;}})();").into()
			},
			_ => todo!("{:?} is not yet implemented.", ty)
		}
	}

	// #region Return Types
    pub(super) fn gen_js_return_type_str(&self, return_type : &ReturnType) -> Cow<'tcx, str> {
        match *return_type {
            // -> () or a -> Result<(), Error>.
            ReturnType::Infallible(SuccessType::Unit) | ReturnType::Fallible(SuccessType::Unit, Some(_)) => self.formatter.fmt_void().into(),
            // Any out that is not a [`SuccessType::Writeable`].
            // TODO:
            ReturnType::Infallible(SuccessType::OutType(ref o)) => self.gen_js_type_str(o),
            _ => todo!("Return type {:?} not supported", return_type)
        }
    }

	pub(super) fn gen_c_to_js_for_return_type(&self, return_type : &ReturnType, lifetime_environment : &LifetimeEnv) -> Option<Cow<'tcx, str>> {
		match *return_type {
			// -> ()
			ReturnType::Infallible(SuccessType::Unit) => None,
			// Any out that is not a [`SuccessType::Writeable`].
			ReturnType::Infallible(SuccessType::OutType(ref o)) => Some(
				format!("return {};", self.gen_c_to_js_for_type(o, "result".into(), lifetime_environment))
				.into()
			),
			_ => todo!("Return type {:?} not supported", return_type)
		}
	}
	// #endregion
}