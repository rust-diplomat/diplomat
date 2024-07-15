use std::borrow::Cow;

use diplomat_core::hir::{
    self, borrowing_param::StructBorrowInfo, LifetimeEnv, OpaqueOwner, PrimitiveType, ReturnType,
    SelfType, StructPathLike, SuccessType, TyPosition, Type,
};
use std::fmt::Write;

use super::TypeGenerationContext;

/// Part of JSGenerationContext that handles conversions between C and JS.
/// This is a partial implementation so I don't have really long files.

fn is_contiguous_enum(ty: &hir::EnumDef) -> bool {
    ty.variants
        .iter()
        .enumerate()
        .all(|(i, v)| i as isize == v.discriminant)
}

/// Context about a struct being borrowed when doing js-to-c conversions
/// Borrowed from dart implementation.
pub(super) struct StructBorrowContext<'tcx> {
    /// Is this in a method or struct?
    ///
    /// Methods generate things like `[aEdges, bEdges]`
    /// whereas structs do `[...aAppendArray, ...bAppendArray]`
    pub is_method: bool,
    pub use_env: &'tcx LifetimeEnv,
    pub param_info: StructBorrowInfo<'tcx>,
}

impl<'jsctx, 'tcx> TypeGenerationContext<'jsctx, 'tcx> {
    // #region C to JS
    /// Given a type from Rust, convert it into something Typescript will understand.
    /// We use this to double-check our Javascript work as well.
    pub(super) fn gen_js_type_str<P: hir::TyPosition>(&mut self, ty: &Type<P>) -> Cow<'tcx, str> {
        match *ty {
            Type::Primitive(primitive) => self
                .js_ctx
                .formatter
                .fmt_primitive_as_ffi(primitive, true)
                .into(),
            Type::Opaque(ref op) => {
                let opaque_id = op.tcx_id.into();
                let type_name = self.js_ctx.formatter.fmt_type_name(opaque_id);

                // Add to the import list:
                self.imports
                    .insert(self.js_ctx.formatter.fmt_import_statement(
                        &type_name,
                        self.typescript,
                        "./".into(),
                    ));

                if self.js_ctx.tcx.resolve_type(opaque_id).attrs().disable {
                    self.js_ctx
                        .errors
                        .push_error(format!("Found usage of disabled type {type_name}"))
                }

                if op.is_optional() {
                    self.js_ctx.formatter.fmt_nullable(&type_name).into()
                } else {
                    type_name
                }
            }
            Type::Struct(ref st) => {
                let id = st.id();
                let type_name = self.js_ctx.formatter.fmt_type_name(id);

                // Add to the import list:
                self.imports
                    .insert(self.js_ctx.formatter.fmt_import_statement(
                        &type_name,
                        self.typescript,
                        "./".into(),
                    ));

                if self.js_ctx.tcx.resolve_type(id).attrs().disable {
                    self.js_ctx
                        .errors
                        .push_error(format!("Found usage of disabled type {type_name}"))
                }
                type_name
            }
            Type::Enum(ref enumerator) => {
                let enum_id = enumerator.tcx_id.into();
                let type_name = self.js_ctx.formatter.fmt_type_name(enum_id);

                // Add to the import list:
                self.imports
                    .insert(self.js_ctx.formatter.fmt_import_statement(
                        &type_name,
                        self.typescript,
                        "./".into(),
                    ));

                if self.js_ctx.tcx.resolve_type(enum_id).attrs().disable {
                    self.js_ctx
                        .errors
                        .push_error(format!("Using disabled type {type_name}"))
                }
                type_name
            }
            Type::Slice(hir::Slice::Str(..)) => self.js_ctx.formatter.fmt_string().into(),
            Type::Slice(hir::Slice::Primitive(_, p)) => {
                self.js_ctx.formatter.fmt_primitive_list_type(p).into()
            }
            Type::Slice(hir::Slice::Strs(..)) => "Array<String>".into(),
            _ => unreachable!("AST/HIR variant {:?} unknown", ty),
        }
    }

    pub(super) fn gen_success_ty(&mut self, out_ty: &SuccessType) -> Cow<'tcx, str> {
        match out_ty {
            SuccessType::Write => self.js_ctx.formatter.fmt_string().into(),
            SuccessType::OutType(o) => self.gen_js_type_str(o),
            SuccessType::Unit => self.js_ctx.formatter.fmt_void().into(),
            _ => unreachable!(),
        }
    }

    /// Create Javascript to convert Rust types into JS types.
    pub(super) fn gen_c_to_js_for_type<P: hir::TyPosition>(
        &self,
        ty: &Type<P>,
        variable_name: Cow<'tcx, str>,
        lifetime_environment: &LifetimeEnv,
    ) -> Cow<'tcx, str> {
        match *ty {
            Type::Primitive(..) => variable_name,
            Type::Opaque(ref op) => {
                let type_id = op.tcx_id.into();
                let type_name = self.js_ctx.formatter.fmt_type_name(type_id);

                let mut edges = if let Some(lt) = op.owner.lifetime() {
                    match lt {
                        hir::MaybeStatic::NonStatic(lt) => self
                            .js_ctx
                            .formatter
                            .fmt_lifetime_edge_array(lt, lifetime_environment)
                            .into_owned(),
                        _ => panic!("'static not implemented for JS2 backend"),
                    }
                } else {
                    "[]".into()
                };

                for lt in op.lifetimes.lifetimes() {
                    match lt {
                        hir::MaybeStatic::NonStatic(lt) => write!(
                            edges,
                            ", {}",
                            self.js_ctx
                                .formatter
                                .fmt_lifetime_edge_array(lt, lifetime_environment)
                        )
                        .unwrap(),
                        _ => panic!("'static not implemented for JS2 backend"),
                    }
                }

                if op.is_optional() {
                    format!("(({variable_name} == 0) ? undefined : new {type_name}({variable_name}, {edges}))").into()
                } else {
                    format!("new {type_name}({variable_name}, {edges})").into()
                }
            }
            Type::Struct(ref st) => {
                let id = st.id();
                let type_name = self.js_ctx.formatter.fmt_type_name(id);
                let mut edges = String::new();
                for lt in st.lifetimes().lifetimes() {
                    match lt {
                        hir::MaybeStatic::NonStatic(lt) => {
                            write!(edges, ", {}Edges", lifetime_environment.fmt_lifetime(lt))
                                .unwrap()
                        }
                        _ => panic!("'static not implemented for JS2 backend"),
                    }
                }

                let type_def = self.js_ctx.tcx.resolve_type(id);
                match type_def {
                    hir::TypeDef::Struct(..) => {
                        format!("new {type_name}()._fromFFI({variable_name}{edges})").into()
                    }
                    hir::TypeDef::OutStruct(..) => {
                        format!("new {type_name}({variable_name}{edges})").into()
                    }
                    _ => unreachable!("Expected struct type def, found {type_def:?}"),
                }
            }
            Type::Enum(ref enum_path) if is_contiguous_enum(enum_path.resolve(self.js_ctx.tcx)) => {
                let id = enum_path.tcx_id.into();
                let type_name = self.js_ctx.formatter.fmt_type_name(id);
                format!("{type_name}[Array.from({type_name}.values.keys())[{variable_name}]]")
                    .into()
            }
            Type::Enum(ref enum_path) => {
                let id = enum_path.tcx_id.into();
                let type_name = self.js_ctx.formatter.fmt_type_name(id);
                // Based on Dart specifics, but if storing too many things in memory isn't an issue we could just make a reverse-lookup map in the enum template.
                format!("(() => {{for (let i of {type_name}.values) {{ if(i[1] === {variable_name}) return {type_name}[i[0]]; }} return null;}})();").into()
            }
            Type::Slice(slice) => {
                // Slices are always returned to us by way of pointers, so we take advantage of a helper function:
                match slice {
					hir::Slice::Primitive(_, primitive_type) => {
						format!(r#"diplomatRuntime.DiplomatBuf.sliceFromPtr(wasm, {variable_name}, "{}")"#, self.js_ctx.formatter.fmt_primitive_list_view(primitive_type)).into()
					},
					hir::Slice::Str(_, encoding) => {
						format!(r#"diplomatRuntime.DiplomatBuf.stringFromPtr(wasm.memory.buffer, {variable_name}, "string{}")"#, 
						match encoding {
							hir::StringEncoding::Utf8 | hir::StringEncoding::UnvalidatedUtf8 => 8,
							hir::StringEncoding::UnvalidatedUtf16 => 16,
							_ => unreachable!("Unknown string_encoding {encoding:?} found")
						}).into()
					},
					hir::Slice::Strs(encoding) => {
						// Old JS backend didn't support this.
						// We basically iterate through and read each string into the array. 
						// TODO: Need a test for this.
						format!(r#"diplomatRuntime.DiplomatBuf.stringsFromPtr(wasm, {variable_name}, "string{}")"#,
						match encoding {
							hir::StringEncoding::Utf8 | hir::StringEncoding::UnvalidatedUtf8 => 8,
							hir::StringEncoding::UnvalidatedUtf16 => 16,
							_ => unreachable!("Unknown string_encoding {encoding:?} found")
						}).into()
					},
					_ => unreachable!("Unknown slice {slice:?} found"),
				}
            }
            _ => unreachable!("AST/HIR variant {:?} unknown.", ty),
        }
    }

    pub(super) fn gen_c_to_js_deref_for_type<P: hir::TyPosition>(
        &self,
        ty: &Type<P>,
        variable_name: Cow<'tcx, str>,
        offset: usize,
    ) -> Cow<'tcx, str> {
        let o = if offset == 0 {
            "".into()
        } else {
            format!(" + {}", offset)
        };
        match *ty {
            Type::Enum(..) => {
                format!("diplomatRuntime.enumDiscriminant(wasm, {variable_name}{o})").into()
            }
            Type::Opaque(..) => format!("diplomatRuntime.ptrRead(wasm, {variable_name}{o})").into(),
            // Structs always assume they're being passed a pointer, so they handle this in their constructors:
            // See NestedBorrowedFields
            Type::Struct(..) | Type::Slice(..) => format!("{variable_name}{o}").into(),
            Type::Primitive(p) => format!(
                "{0}(new {1}(wasm.memory.buffer, {variable_name}{o}, 1))[0]{2}",
                match p {
                    PrimitiveType::Char => "String.fromCharCode(",
                    _ => "",
                },
                self.js_ctx.formatter.fmt_primitive_slice(p),
                match p {
                    PrimitiveType::Bool => " == 1",
                    PrimitiveType::Char => ")",
                    _ => "",
                }
            )
            .into(),
            _ => unreachable!("Unknown AST/HIR variant {:?}", ty),
        }
    }

    // #region Return Types

    /// Give us a Typescript return type from [`ReturnType`]
    pub(super) fn gen_js_return_type_str(&mut self, return_type: &ReturnType) -> Cow<'tcx, str> {
        match *return_type {
            // -> () or a -> Result<(), Error>.
            ReturnType::Infallible(SuccessType::Unit)
            | ReturnType::Fallible(SuccessType::Unit, Some(_)) => {
                self.js_ctx.formatter.fmt_void().into()
            }

            // Something we can write to? We just treat it as a string.
            ReturnType::Infallible(SuccessType::Write)
            | ReturnType::Fallible(SuccessType::Write, Some(_)) => {
                self.js_ctx.formatter.fmt_string().into()
            }

            // Anything we get returned that is not a [`SuccessType::Write`].
            ReturnType::Infallible(SuccessType::OutType(ref o))
            | ReturnType::Fallible(SuccessType::OutType(ref o), Some(_)) => self.gen_js_type_str(o),

            // Nullable string (no error on return).
            ReturnType::Fallible(SuccessType::Write, None)
            | ReturnType::Nullable(SuccessType::Write) => self
                .js_ctx
                .formatter
                .fmt_nullable(self.js_ctx.formatter.fmt_string())
                .into(),

            // Something like Option<()>. Basically, did we run successfully?
            ReturnType::Fallible(SuccessType::Unit, None)
            | ReturnType::Nullable(SuccessType::Unit) => self
                .js_ctx
                .formatter
                .fmt_primitive_as_ffi(hir::PrimitiveType::Bool, true)
                .into(),

            // A nullable out type. Something like `MyStruct?` in Typescript.
            ReturnType::Fallible(SuccessType::OutType(ref o), None)
            | ReturnType::Nullable(SuccessType::OutType(ref o)) => self
                .js_ctx
                .formatter
                .fmt_nullable(&self.gen_js_type_str(o))
                .into(),

            _ => unreachable!("AST/HIR variant {:?} unknown.", return_type),
        }
    }

    /// Give us pure JS for returning types.
    /// This basically handles the conversions from whatever the WASM gives us to a JS-friendly type.
    /// We access [`super::MethodInfo`] to handle allocation and cleanup.
    pub(super) fn gen_c_to_js_for_return_type(
        &mut self,
        method_info: &mut super::MethodInfo,
        lifetime_environment: &LifetimeEnv,
    ) -> Option<Cow<'tcx, str>> {
        let return_type = &method_info.method.unwrap().output;

        // Conditions for allocating a diplomat buffer:
        // 1. Function returns an Option<> or Result<>.
        // 2. Infallible function returns a slice.
        // 3. Infallible function returns a struct.
        match *return_type {
            // -> ()
            ReturnType::Infallible(SuccessType::Unit) => None,

            ReturnType::Infallible(SuccessType::Write) => {
                method_info
                    .alloc_expressions
                    .push("const write = wasm.diplomat_buffer_write_create(0);".into());
                method_info.param_conversions.push("write".into());
                method_info
                    .cleanup_expressions
                    .push("wasm.diplomat_buffer_write_destroy(write);".into());
                Some("return diplomatRuntime.readString8(wasm, wasm.diplomat_buffer_write_get_bytes(write), wasm.diplomat_buffer_write_len(write));".into())
            }

            // Any out that is not a [`SuccessType::Write`].
            ReturnType::Infallible(SuccessType::OutType(ref o)) => {
                let mut result = "result";
                match o {
                    Type::Struct(_) | Type::Slice(_) => {
                        let layout = crate::layout_hir::type_size_alignment(o, self.js_ctx.tcx);
                        let size = layout.size();
                        let align = layout.align();

                        method_info.alloc_expressions.push(
							format!("const diplomat_receive_buffer = wasm.diplomat_alloc({size}, {align});")
							.into()
						);
                        // This is the first thing in param converison order:
                        method_info
                            .param_conversions
                            .insert(0, "diplomat_receive_buffer".into());
                        method_info.cleanup_expressions.push(
                            format!(
                                "wasm.diplomat_free(diplomat_receive_buffer, {size}, {align});"
                            )
                            .into(),
                        );
                        result = "diplomat_receive_buffer";
                    }
                    _ => (),
                }
                Some(
                    format!(
                        "return {};",
                        self.gen_c_to_js_for_type(o, result.into(), lifetime_environment)
                    )
                    .into(),
                )
            }

            // Result<(), ()> or Option<()>
            ReturnType::Fallible(SuccessType::Unit, None)
            | ReturnType::Nullable(SuccessType::Unit) => Some("return result == 1;".into()),

            // Result<Write, ()> or Option<Write>.
            ReturnType::Fallible(SuccessType::Write, None)
            | ReturnType::Nullable(SuccessType::Write) => {
                method_info
                    .alloc_expressions
                    .push("const write = wasm.diplomat_buffer_write_create(0);".into());
                method_info.param_conversions.push("write".into());
                method_info
                    .cleanup_expressions
                    .push("wasm.diplomat_buffer_write_destroy(write);".into());
                Some("if (!(result == 1)) {\n     throw new diplomatRuntime.FFIError(null);}\n    return diplomatRuntime.readString8(wasm, wasm.diplomat_buffer_write_get_bytes(write), wasm.diplomat_buffer_write_len(write));".into())
            }

            // Result<Type, Error> or Option<Type>
            ReturnType::Fallible(ref ok, _) | ReturnType::Nullable(ref ok) => {
                let layout = match ok {
                    SuccessType::Unit => crate::layout_hir::unit_size_alignment(),
                    SuccessType::OutType(ref o) => {
                        crate::layout_hir::type_size_alignment(o, self.js_ctx.tcx)
                    }
                    SuccessType::Write => match return_type {
                        ReturnType::Fallible(_, ref err) if err.is_some() => {
                            crate::layout_hir::type_size_alignment(
                                &err.clone().unwrap(),
                                self.js_ctx.tcx,
                            )
                        }
                        ReturnType::Fallible(_, None) | ReturnType::Nullable(_) => {
                            crate::layout_hir::unit_size_alignment()
                        }
                        _ => unreachable!("AST/HIR variant {:?} unknown.", return_type),
                    },
                    _ => unreachable!("AST/HIR variant {:?} unknown.", return_type),
                };
                // Add size for checking whether or not we're a pass/fail result. And we make sure to see if our error type is bigger, so if we need to add extra width based on that:
                let size = std::cmp::max(
                    layout.size(),
                    match return_type {
                        // We already account for an error in the Write match up above:
                        ReturnType::Fallible(_, e) if e.is_some() => {
                            crate::layout_hir::type_size_alignment(
                                &e.clone().unwrap(),
                                self.js_ctx.tcx,
                            )
                            .size()
                        }
                        _ => 0,
                    },
                ) + 1;
                let align = layout.align();

                method_info.alloc_expressions.push(
                    format!(
                        "const diplomat_receive_buffer = wasm.diplomat_alloc({}, {});",
                        size, align
                    )
                    .into(),
                );
                method_info
                    .param_conversions
                    .insert(0, "diplomat_receive_buffer".into());
                method_info.cleanup_expressions.push(
                    format!(
                        "wasm.diplomat_free(diplomat_receive_buffer, {}, {});",
                        size, align
                    )
                    .into(),
                );

                let err_check = format!("if (!diplomatRuntime.resultFlag(wasm, diplomat_receive_buffer, {})) {{\n    {};\n}}\n",
				size - 1,
				match return_type {
					ReturnType::Fallible(_, Some(e)) => {
						// Because we don't add Result<_, Error> types to imports, we do that here:
						if !self.typescript {
							let type_name = self.js_ctx.formatter.fmt_type_name(e.id().unwrap());
							self.imports.insert(self.js_ctx.formatter.fmt_import_statement(&type_name, false, "./".into()));
						}

						let receive_deref = self.gen_c_to_js_deref_for_type(e, "diplomat_receive_buffer".into(), 0);
						format!("throw new diplomatRuntime.FFIError({})", self.gen_c_to_js_for_type(e, receive_deref, lifetime_environment))
					},
                    // We don't want an error, we just want a null value that we can process.
                    ReturnType::Nullable(_) => "return null".into(),
                    // Otherwise we just error out with the Unit error:
					_ => "throw new diplomatRuntime.FFIError(null)".into(),
				});

                Some(match ok {
					SuccessType::Unit => err_check,
					SuccessType::Write => {
						// Pretty sure you can't have a Result<Write, Err> and also return something else.
						// So this is our ideal output:
						/*
						const write = alloc(0);
						const diplomat_receive_buffer = diplomat.alloc(error_size, error_align);
						wasm.c_func(write);
						if (diplomat.resultFlag(wasm, diplomat_receive_buffer, error_size - 1)) {
							return write;
						} else {
							throw Error();
						}
						*/
						// TODO: This could probably be its own diplomatRuntime function, instead of lots of little wasm calls.
						method_info.alloc_expressions.push("const write = wasm.diplomat_buffer_write_create(0);".into());
						method_info.param_conversions.push("write".into());
						method_info.cleanup_expressions.push("wasm.diplomat_buffer_write_destroy(write);".into());
						format!("{err_check}return diplomatRuntime.readString8(wasm, wasm.diplomat_buffer_write_get_bytes(write), wasm.diplomat_buffer_write_len(write));")
					},
					SuccessType::OutType(ref o) => {
						let ptr_deref = self.gen_c_to_js_deref_for_type(o, "diplomat_receive_buffer".into(), 0);
						format!("{err_check}return {};", 
						self.gen_c_to_js_for_type(o, ptr_deref, lifetime_environment))
					},
					_ => unreachable!("AST/HIR variant {:?} unknown.", return_type)
				}.into())
            }

            _ => unreachable!("AST/HIR variant {:?} unknown", return_type),
        }
    }
    // #endregion

    // #endregion

    // #region JS to C

    pub(super) fn gen_js_to_c_self(&self, ty: &SelfType) -> Cow<'static, str> {
        match *ty {
            SelfType::Enum(..) | SelfType::Opaque(..) => "this.ffiValue".into(),
            // The way Rust generates WebAssembly, each function that requires a self struct require us to pass in each parameter into the function.
            // So we call a function in JS that lets us do this.
            // We use spread syntax to avoid a complicated array setup.
            SelfType::Struct(..) => "...this._intoFFI()".into(),
            _ => unreachable!("Unknown AST/HIR variant {:?}", ty),
        }
    }

    pub(super) fn gen_js_to_c_for_type<P: TyPosition>(
        &self,
        ty: &Type<P>,
        js_name: Cow<'tcx, str>,
        struct_borrow_info: Option<&StructBorrowContext<'tcx>>,
    ) -> Cow<'tcx, str> {
        match *ty {
            Type::Primitive(p) => match p {
                PrimitiveType::Char => {
                    format!("diplomatRuntime.extractCodePoint({js_name}, '{js_name}')").into()
                }
                _ => js_name.clone(),
            },
            Type::Opaque(ref op) if op.is_optional() => format!("{js_name}.ffiValue ?? 0").into(),
            Type::Enum(..) | Type::Opaque(..) => format!("{js_name}.ffiValue").into(),
            Type::Struct(..) => self.gen_js_to_c_for_struct_type(js_name, struct_borrow_info),
            Type::Slice(hir::Slice::Str(_, encoding) | hir::Slice::Strs(encoding)) => {
                match encoding {
                    hir::StringEncoding::UnvalidatedUtf8 | hir::StringEncoding::Utf8 => {
                        format!("diplomatRuntime.DiplomatBuf.str8(wasm, {js_name})").into()
                    }
                    _ => format!("diplomatRuntime.DiplomatBuf.str16(wasm, {js_name})").into(),
                }
            }
            Type::Slice(hir::Slice::Primitive(_, p)) => format!(
                r#"diplomatRuntime.DiplomatBuf.slice(wasm, {js_name}, "{}")"#,
                self.js_ctx.formatter.fmt_primitive_list_view(p)
            )
            .into(),
            _ => unreachable!("Unknown AST/HIR variant {ty:?}"),
        }
    }

    pub(super) fn gen_js_to_c_for_struct_type(
        &self,
        js_name: Cow<'tcx, str>,
        struct_borrow_info: Option<&StructBorrowContext<'tcx>>,
    ) -> Cow<'tcx, str> {
        let mut params = String::new();
        if let Some(info) = struct_borrow_info {
            for (def_lt, use_lts) in &info.param_info.borrowed_struct_lifetime_map {
                write!(
                    &mut params,
                    "{}AppendArray: [",
                    info.param_info.env.fmt_lifetime(def_lt)
                )
                .unwrap();
                let mut maybe_comma = "";
                for use_lt in use_lts.iter() {
                    // Generate stuff like `, aEdges` or for struct fields, `, ...aAppendArray`
                    let lt = info.use_env.fmt_lifetime(use_lt);
                    if info.is_method {
                        write!(&mut params, "{maybe_comma}{lt}Edges",).unwrap();
                    } else {
                        write!(&mut params, "{maybe_comma}...{lt}AppendArray",).unwrap();
                    }
                    maybe_comma = ", ";
                }
                write!(&mut params, "],").unwrap();
            }
        }
        format!("...{js_name}._intoFFI(slice_cleanup_callbacks, {{{params}}})").into()
    }
    // #endregion
}
