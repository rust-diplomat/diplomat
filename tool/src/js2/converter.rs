use std::borrow::Cow;

use diplomat_core::hir::{self, LifetimeEnv, MaybeStatic, OpaqueOwner, ReturnType, StructPathLike, SuccessType, Type};
use std::fmt::{Display, Write};

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
	/// Given a type from Rust, convert it into something Typescript will understand.
	/// We use this to double-check our Javascript work as well.
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
			Type::Struct(ref st) => {
				let id = st.id();
                let type_name = self.formatter.fmt_type_name(id);
                if self.tcx.resolve_type(id).attrs().disable {
                    self.errors
                        .push_error(format!("Found usage of disabled type {type_name}"))
                }
                type_name
			},
            Type::Enum(ref enumerator) => {
                let enum_id = enumerator.tcx_id.into();
                let type_name = self.formatter.fmt_type_name(enum_id);
                if self.tcx.resolve_type(enum_id).attrs().disable {
                    self.errors.push_error(format!("Using disabled type {type_name}"))
                }
                type_name
            },
			Type::Slice(hir::Slice::Str(..)) => self.formatter.fmt_string().into(),
            Type::Slice(hir::Slice::Primitive(_, p)) => {
                self.formatter.fmt_primitive_list_type(p).into()
            }
            Type::Slice(hir::Slice::Strs(..)) => "Array<String>".into(),
            _ => unreachable!("AST/HIR variant {:?} unknown", ty)
        }
    }

	/// Create Javascript to convert Rust types into JS types.
	pub(super) fn gen_c_to_js_for_type<P: hir::TyPosition>(&self, ty : &Type<P>, variable_name : Cow<'tcx, str>, lifetime_environment : &LifetimeEnv) -> Cow<'tcx, str> {
		match *ty {
			Type::Primitive(..) => variable_name,
			Type::Opaque(ref op) => {
				let type_id = op.tcx_id.into();
				let type_name = self.formatter.fmt_type_name(type_id);

				let mut edges = if let Some(lt) = op.owner.lifetime() {
					match lt {
						hir::MaybeStatic::NonStatic(lt) => self.formatter
						.fmt_lifetime_edge_array(lt, lifetime_environment)
						.into_owned(),
						_ => todo!()
					}
				} else {
					"[]".into()
				};

				for lt in op.lifetimes.lifetimes() {
					match lt {
						hir::MaybeStatic::NonStatic(lt) => write!(edges, ", {}", self.formatter.fmt_lifetime_edge_array(lt, lifetime_environment)).unwrap(),
						_ => todo!(),
					}
				}

				// TODO: Owned? Check JS
				if op.is_optional() {
					format!("({variable_name} === 0) ? undefined : new {type_name}({variable_name}, {edges});").into()
				} else {
					format!("new {type_name}({variable_name}, {edges})").into()
				}
			},
			Type::Struct(ref st) => {
				let id = st.id();
				let type_name = self.formatter.fmt_type_name(id);
				let mut edges = String::new();
				for lt in st.lifetimes().lifetimes() {
					match lt {
						hir::MaybeStatic::NonStatic(lt) => write!(edges, ", {}Edges", lifetime_environment.fmt_lifetime(lt)).unwrap(),
						_ => todo!()
					}
				}
				// TODO:
				format!("{type_name} // TODO").into()
			},
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
			Type::Slice(slice) => {
				if let Some(lt) = slice.lifetime() {
					match lt {
						MaybeStatic::NonStatic(lifetime) => {
							// TODO:
							format!("{variable_name}({}Edges) // TODO", lifetime_environment.fmt_lifetime(lifetime)).into()
						},
						_ => todo!()
					}
				} else {
					// TODO:
					format!("{variable_name} // TODO").into()
				}
			},
			_ => unreachable!("AST/HIR variant {:?} unknown.", ty)
		}
	}

	// #region Return Types

	/// Give us a Typescript return type from [`ReturnType`]
    pub(super) fn gen_js_return_type_str(&self, return_type : &ReturnType) -> Cow<'tcx, str> {
        match *return_type {
            // -> () or a -> Result<(), Error>.
            ReturnType::Infallible(SuccessType::Unit)
			| ReturnType::Fallible(SuccessType::Unit, Some(_))
			=> self.formatter.fmt_void().into(),

			// Something we can write to? We just treat it as a string.
			ReturnType::Infallible(SuccessType::Writeable)
			| ReturnType::Fallible(SuccessType::Writeable, Some(_))
			=> self.formatter.fmt_string().into(),

            // Anything we get returned that is not a [`SuccessType::Writeable`].
            ReturnType::Infallible(SuccessType::OutType(ref o))
			| ReturnType::Fallible(SuccessType::OutType(ref o), Some(_))
			=> self.gen_js_type_str(o),

			// Nullable string (no error on return).
			ReturnType::Fallible(SuccessType::Writeable, None)
			| ReturnType::Nullable(SuccessType::Writeable)
			=> self.formatter.fmt_nullable(self.formatter.fmt_string()).into(),

			// Something like Option<()>. Basically, did we run successfully?
			ReturnType::Fallible(SuccessType::Unit, None)
			| ReturnType::Nullable(SuccessType::Unit)
			=> self.formatter.fmt_primitive_as_ffi(hir::PrimitiveType::Bool, true).into(),

			// A nullable out type. Something like `MyStruct?` in Typescript.
			ReturnType::Fallible(SuccessType::OutType(ref o), None)
			| ReturnType::Nullable(SuccessType::OutType(ref o))
			=> self.formatter.fmt_nullable(&self.gen_js_type_str(o)).into(),

			_ => unreachable!("AST/HIR variant {:?} unknown.", return_type),
        }
    }

	/// Give us pure JS for returning types.
	/// This basically handles the conversions from whatever the WASM gives us to a JS-friendly type.
	pub(super) fn gen_c_to_js_for_return_type(&self, return_type : &ReturnType, lifetime_environment : &LifetimeEnv) -> Option<Cow<'tcx, str>> {
		match *return_type {
			// -> ()
			ReturnType::Infallible(SuccessType::Unit) => None,
			
			ReturnType::Infallible(SuccessType::Writeable) => Some("return writeable;".into()),

			// Any out that is not a [`SuccessType::Writeable`].
			ReturnType::Infallible(SuccessType::OutType(ref o)) => Some(
				format!("return {};", self.gen_c_to_js_for_type(o, "result".into(), lifetime_environment))
				.into()
			),

			// Result<(), ()> or Option<()>
			// TODO: See js/api/OptionOpaque.mjs.
			ReturnType::Fallible(SuccessType::Unit, None)
			| ReturnType::Nullable(SuccessType::Unit)
			=> Some("return result.isOk;".into()),

			// Result<Type, Error> or Option<Type>
			// TODO: See js/api/OptionOpaque.mjs.
			ReturnType::Fallible(ref ok, _) | ReturnType::Nullable(ref ok)  => {
				let err_check = format!("if (!result.isOk) {{\n    {}\n}}\n",
				match return_type {
					ReturnType::Fallible(_, Some(e)) => format!("throw {}",
					self.gen_c_to_js_for_type(e, "result.union.error".into(), lifetime_environment)),
					_ => "return null".into(),
				});

				Some(match ok {
					SuccessType::Unit => err_check,
					SuccessType::Writeable => format!("{err_check} return writeable;"),
					SuccessType::OutType(ref o) => format!("{err_check} return {};", 
					self.gen_c_to_js_for_type(o, "result.union.ok".into(), lifetime_environment)),
					_ => unreachable!("AST/HIR variant {:?} unknown.", return_type)
				}.into())
			},

			_ => unreachable!("AST/HIR variant {:?} unknown", return_type)
		}
	}
	// #endregion
}