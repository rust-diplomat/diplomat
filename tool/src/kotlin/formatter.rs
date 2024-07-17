use diplomat_core::hir::{
    self,
    borrowing_param::{LifetimeEdge, LifetimeEdgeKind},
    FloatType, IntSizeType, IntType, LifetimeEnv, MaybeStatic, PrimitiveType, Slice,
    StringEncoding, StructPathLike, TyPosition, Type, TypeContext, TypeId,
};
use heck::ToLowerCamelCase;
use std::{borrow::Cow, iter::once};

/// This type mediates all formatting
pub(super) struct KotlinFormatter<'tcx> {
    tcx: &'tcx TypeContext,
    strip_prefix: Option<String>,
}

const INVALID_METHOD_NAMES: &[&str] = &[
    "new", "static", "default", "private", "internal", "toString",
];
const DISALLOWED_CORE_TYPES: &[&str] = &["Object", "String"];

impl<'tcx> KotlinFormatter<'tcx> {
    pub fn new(tcx: &'tcx TypeContext, strip_prefix: Option<String>) -> Self {
        Self { tcx, strip_prefix }
    }

    pub fn fmt_void(&self) -> &'static str {
        "Unit"
    }

    pub fn fmt_primitive_to_native_conversion(&self, name: &str, prim: PrimitiveType) -> String {
        match prim {
            PrimitiveType::Bool => format!("{name}.toByte()"),
            PrimitiveType::Int(IntType::U8) => format!("{name}.toByte()"),
            PrimitiveType::Int(IntType::U16) => format!("{name}.toShort()"),
            PrimitiveType::Int(IntType::U32) => format!("{name}.toInt()"),
            PrimitiveType::Int(IntType::U64) => format!("{name}.toLong()"),
            PrimitiveType::IntSize(IntSizeType::Usize) => format!("{name}.toLong()"),
            PrimitiveType::Int128(_) => panic!("128 bit ints not supported"),
            _ => name.into(),
        }
    }

    pub fn fmt_string(&self) -> &'static str {
        "String"
    }

    pub fn fmt_primitive_slice(&self, ty: PrimitiveType) -> String {
        format!("{}Array", self.fmt_primitive_as_kt(ty))
    }

    pub fn fmt_str_slices(&self) -> &'static str {
        "Array<String>"
    }

    pub fn fmt_primitive_as_ffi(&self, prim: PrimitiveType) -> &'static str {
        match prim {
            PrimitiveType::Bool => "Byte",
            PrimitiveType::Char => "Int",
            PrimitiveType::Int(IntType::I8) => "Byte",
            PrimitiveType::Int(IntType::I16) => "Short",
            PrimitiveType::Int(IntType::I32) => "Int",
            PrimitiveType::Int(IntType::I64) => "Long",
            PrimitiveType::Int(IntType::U8) => "Byte",
            PrimitiveType::Int(IntType::U16) => "Short",
            PrimitiveType::Int(IntType::U32) => "Int",
            PrimitiveType::Int(IntType::U64) => "Long",
            PrimitiveType::Byte => "Byte",
            PrimitiveType::IntSize(IntSizeType::Isize) => "Long",
            PrimitiveType::IntSize(IntSizeType::Usize) => "Long",
            PrimitiveType::Float(FloatType::F32) => "Float",
            PrimitiveType::Float(FloatType::F64) => "Double",
            PrimitiveType::Int128(_) => panic!("i128 not supported in Kotlin"),
        }
    }

    pub fn fmt_primitive_as_kt(&self, prim: PrimitiveType) -> &'static str {
        match prim {
            PrimitiveType::Bool => "Boolean",
            PrimitiveType::Char => "Int",
            PrimitiveType::Int(IntType::I8) => "Byte",
            PrimitiveType::Int(IntType::I16) => "Short",
            PrimitiveType::Int(IntType::I32) => "Int",
            PrimitiveType::Int(IntType::I64) => "Long",
            PrimitiveType::Int(IntType::U8) => "UByte",
            PrimitiveType::Int(IntType::U16) => "UShort",
            PrimitiveType::Int(IntType::U32) => "UInt",
            PrimitiveType::Int(IntType::U64) => "ULong",
            PrimitiveType::Byte => "Byte",
            PrimitiveType::IntSize(IntSizeType::Isize) => "Long",
            PrimitiveType::IntSize(IntSizeType::Usize) => "ULong",
            PrimitiveType::Float(FloatType::F32) => "Float",
            PrimitiveType::Float(FloatType::F64) => "Double",
            PrimitiveType::Int128(_) => panic!("i128 not supported in Kotlin"),
        }
    }

    pub fn fmt_method_name<'a>(&self, method: &'a hir::Method) -> Cow<'a, str> {
        // TODO(#60): handle other keywords

        let name = method.name.as_str().to_lower_camel_case();
        let name = method.attrs.rename.apply(name.into());
        if INVALID_METHOD_NAMES.contains(&&*name) {
            format!("{name}_").into()
        } else {
            name
        }
    }

    pub fn fmt_param_name<'a>(&self, ident: &'a str) -> Cow<'tcx, str> {
        ident.to_lower_camel_case().into()
    }

    pub fn fmt_borrow<'a>(&self, edge: &LifetimeEdge<'a>) -> Cow<'a, str> {
        let LifetimeEdge {
            param_name,
            kind: ty,
            ..
        } = edge;
        let param_name = self.fmt_param_name(param_name).to_string();
        match ty {
            LifetimeEdgeKind::OpaqueParam => format!("listOf({param_name})").into(),
            LifetimeEdgeKind::SliceParam => format!("listOf({param_name}Mem)").into(),
            LifetimeEdgeKind::StructLifetime(lt_env, lt) => {
                let lt = lt_env.fmt_lifetime(lt);
                format!("{param_name}.{lt}Edges").into()
            }
            _ => panic!("unsupported lifetime kind"),
        }
    }

    pub fn fmt_field_name<'a>(&'a self, ident: &'a str) -> Cow<'tcx, str> {
        self.fmt_param_name(ident)
    }

    pub fn fmt_primitive_default(&self, prim: PrimitiveType) -> &'static str {
        match prim {
            PrimitiveType::Float(FloatType::F32) => "0.0F",
            PrimitiveType::Float(FloatType::F64) => "0.0",
            _ => "0",
        }
    }
    pub fn fmt_field_default<'a, P: TyPosition>(&'a self, ty: &'a Type<P>) -> Cow<'tcx, str> {
        match ty {
            Type::Primitive(prim) => self.fmt_primitive_default(*prim).into(),
            Type::Opaque(op) => if op.is_optional() {
                "null"
            } else {
                "Pointer(0)" // Aren't these the same thing?
            }
            .into(),
            Type::Struct(s) => {
                let field_type_name: &str = self.tcx.resolve_type(s.id()).name().as_ref();
                format!("{field_type_name}Native()").into()
            }
            Type::Enum(enum_def) => {
                let field_type_name: &str = self.tcx.resolve_enum(enum_def.tcx_id).name.as_ref();
                format!("{field_type_name}.default().toNative()").into()
            }
            Type::Slice(_) => "Slice()".into(),
            ty => unreachable!("reached struct field that can't be handled: {ty:?}"),
        }
    }

    pub fn fmt_unsized_conversion(&self, prim: PrimitiveType, optional: bool) -> Cow<str> {
        let optional_conversion = if optional { "?" } else { "" };
        match prim {
            PrimitiveType::Bool => format!("{optional_conversion} > 0").into(),
            PrimitiveType::Int(IntType::U8) => format!("{optional_conversion}.toUByte()").into(),
            PrimitiveType::Int(IntType::U16) => format!("{optional_conversion}.toUShort()").into(),
            PrimitiveType::Int(IntType::U32) => format!("{optional_conversion}.toUInt()").into(),
            PrimitiveType::Int(IntType::U64) => format!("{optional_conversion}.toULong()").into(),
            PrimitiveType::IntSize(IntSizeType::Usize) => {
                format!("{optional_conversion}.toULong()").into()
            }
            PrimitiveType::Int128(_) => panic!("Int128 not supported"),
            _ => "".into(),
        }
    }

    pub fn fmt_struct_field_native_to_kt<'a, P: TyPosition>(
        &'a self,
        field_name: &'a str,
        lifetime_env: &'a LifetimeEnv,
        ty: &'a Type<P>,
    ) -> Cow<'tcx, str> {
        match ty {
            Type::Primitive(prim) => {
                let maybe_unsized_conversion = self.fmt_unsized_conversion(*prim, false);
                format!("nativeStruct.{field_name}{maybe_unsized_conversion}").into()
            }
            Type::Opaque(opaque) => {
                let lt_list: String =
                    once("listOf()".to_string()) // we only support owned opaque types, so the self edges
                                     // should be empty
                        .chain(opaque.lifetimes.lifetimes().filter_map(|maybe_static_lt| match maybe_static_lt{
                            MaybeStatic::Static => None,
                            MaybeStatic::NonStatic(lt) => {
                                let lts = lifetime_env
                                    .all_longer_lifetimes(lt)
                                    .map(|longer_lt|  {
                                         let longer_lt = lifetime_env.fmt_lifetime(longer_lt);
                                         format!("{longer_lt}Edges")
                                    })
                                    .collect::<Vec<_>>();
                                 Some(if lts.is_empty() {
                                    "listOf()".into()
                                } else {
                                    lts.join("+")
                                })
                            }
                        }))
                        .collect::<Vec<_>>()
                        .join(", ");
                let ty_name =
                    self.fmt_type_name(ty.id().expect("Failed to get type id for opaque"));
                if opaque.is_optional() {
                    format!(
                        r#"if (nativeStruct.{field_name} == null) {{
        null
    }} else {{
        {ty_name}(nativeStruct.{field_name}!!, {lt_list})
    }}"#
                    )
                } else {
                    format!("{ty_name}(nativeStruct.{field_name}, {lt_list})")
                }
                .into()
            }
            Type::Struct(strct) => {
                let ty_name =
                    self.fmt_type_name(ty.id().expect("Failed to get type id for opaque"));
                let lt_list: String = strct
                    .lifetimes()
                    .lifetimes()
                    .filter_map(|maybe_static_lt| match maybe_static_lt {
                        MaybeStatic::Static => None,
                        MaybeStatic::NonStatic(lt) => {
                            let lt_name = lifetime_env.fmt_lifetime(lt);
                            Some(format!("{lt_name}Edges"))
                        }
                    })
                    .fold(String::new(), |accum, new| format!("{accum}, {new}"));
                format!("{ty_name}(nativeStruct.{field_name}{lt_list})").into()
            }
            Type::Enum(enum_path) => {
                let field_type_name: &str = self.tcx.resolve_enum(enum_path.tcx_id).name.as_ref();
                format!("{field_type_name}.fromNative(nativeStruct.{field_name})").into()
            }
            Type::Slice(Slice::Primitive(_, prim)) => format!(
                "PrimitiveArrayTools.get{}Array(nativeStruct.{field_name})",
                self.fmt_primitive_as_kt(*prim)
            )
            .into(),
            Type::Slice(Slice::Str(_, StringEncoding::UnvalidatedUtf16)) => {
                format!("PrimitiveArrayTools.getUtf16(nativeStruct.{field_name})").into()
            }
            Type::Slice(Slice::Str(_, _)) => {
                format!("PrimitiveArrayTools.getUtf8(nativeStruct.{field_name})").into()
            }
            Type::Slice(Slice::Strs(StringEncoding::UnvalidatedUtf16)) => {
                format!("PrimitiveArrayTools.getUt16s(nativeStruct.{field_name})").into()
            }
            Type::Slice(Slice::Strs(_)) => {
                format!("PrimitiveArrayTools.getUt16s(nativeStruct.{field_name})").into()
            }
            _ => todo!(),
        }
    }

    pub fn fmt_struct_field_type_kt<'a, P: TyPosition>(
        &'a self,
        ty: &'a Type<P>,
    ) -> Cow<'tcx, str> {
        match ty {
            Type::Primitive(prim) => self.fmt_primitive_as_kt(*prim).into(),
            Type::Opaque(op) => {
                // todo: optional
                let optional = if op.is_optional() { "?" } else { "" };
                format!(
                    "{}{optional}",
                    self.fmt_type_name(ty.id().expect("Failed to get type id for opaque"))
                )
                .into()
            }
            Type::Struct(_) => {
                self.fmt_type_name(ty.id().expect("Failed to get type id for struct"))
            }
            Type::Enum(_) => self.fmt_type_name(ty.id().expect("Failed to get type id for enum")),
            Type::Slice(Slice::Primitive(_, prim)) => {
                format!("{}Array", self.fmt_primitive_as_kt(*prim)).into()
            }
            Type::Slice(Slice::Str(_, _)) => "String".into(),
            Type::Slice(Slice::Strs(_)) => "List<String>".into(),
            _ => todo!(),
        }
    }

    pub fn fmt_primitive_type_native(&self, prim: PrimitiveType) -> &'static str {
        match prim {
            PrimitiveType::Bool => "Byte",
            PrimitiveType::Int(IntType::U8) => "Byte",
            PrimitiveType::Int(IntType::U16) => "Short",
            PrimitiveType::Int(IntType::U32) => "Int",
            PrimitiveType::Int(IntType::U64) => "Long",
            PrimitiveType::IntSize(_) => "Long",
            prim => self.fmt_primitive_as_ffi(prim),
        }
    }

    pub fn fmt_struct_field_type_native<'a, P: TyPosition>(
        &'a self,
        ty: &'a Type<P>,
    ) -> Cow<'tcx, str> {
        match ty {
            Type::Primitive(prim) => self.fmt_primitive_type_native(*prim).into(),
            Type::Opaque(op) => {
                let optional = if op.is_optional() { "?" } else { "" };
                format!("Pointer{optional}").into()
            }
            Type::Struct(s) => {
                format!("{}Native", self.tcx.resolve_type(s.id()).name().as_str()).into()
            }
            Type::Enum(_) => "Int".into(),
            Type::Slice(_) => "Slice".into(),
            ty => unreachable!("reached struct field that can't be handled: {ty:?}"),
        }
    }

    pub fn fmt_type_name(&self, id: TypeId) -> Cow<'tcx, str> {
        let resolved = self.tcx.resolve_type(id);

        let candidate: Cow<str> = if let Some(strip_prefix) = self.strip_prefix.as_ref() {
            resolved
                .name()
                .as_str()
                .strip_prefix(strip_prefix)
                .unwrap_or(resolved.name().as_str())
                .into()
        } else {
            resolved.name().as_str().into()
        };

        if DISALLOWED_CORE_TYPES.contains(&&*candidate) {
            panic!("{candidate:?} is not a valid Kotlin type name. Please rename.");
        }

        resolved.attrs().rename.apply(candidate)
    }

    pub fn fmt_nullable(&self, ident: &str) -> String {
        format!("{ident}?")
    }
}

#[cfg(test)]
pub mod test {
    use std::borrow::Cow;

    use super::KotlinFormatter;
    use diplomat_core::{
        ast::{self},
        hir::{self, TypeContext},
    };
    use proc_macro2::TokenStream;

    use quote::quote;

    pub fn new_tcx(tk_stream: TokenStream) -> TypeContext {
        let item = syn::parse2::<syn::File>(tk_stream).expect("failed to parse item ");

        let diplomat_file = ast::File::from(&item);

        let env = diplomat_file.all_types();
        let mut attr_validator = hir::BasicAttributeValidator::new("kotlin_test");
        attr_validator.support.renaming = true;
        attr_validator.support.disabling = true;

        match hir::TypeContext::from_ast(&env, attr_validator) {
            Ok(context) => context,
            Err(e) => {
                for (_cx, err) in e {
                    eprintln!("Lowering error: {}", err);
                }
                panic!("Failed to create context")
            }
        }
    }

    #[test]
    fn test_type_name() {
        let tk_stream = quote! {
            #[diplomat::bridge]
            mod ffi {
                #[diplomat::opaque]
                struct MyOpaqueStruct {
                    a: SomeExternalType
                }

                #[diplomat::opaque]
                struct StringWrapper(String);

                impl MyOpaqueStruct {
                    pub fn new() -> Box<MyOpaqueStruct> {
                        unimplemented!();
                    }

                    pub fn get_byte(&self) -> u8 {
                        unimplemented!()
                    }

                    pub fn get_string_wrapper(&self) -> Box<StringWrapper> {
                        unimplemented!()
                    }

                }

            }
        };
        let tcx = new_tcx(tk_stream);
        let formatter = KotlinFormatter::new(&tcx, None);
        let opaques = tcx.opaques();
        assert!(!opaques.is_empty());
        let mut all_types = tcx.all_types();
        let (ty_id, _) = all_types.next().expect("Failed to get next type");

        assert_eq!(Cow::from("MyOpaqueStruct"), formatter.fmt_type_name(ty_id));

        let (ty_id, _) = all_types.next().expect("Failed to get next type");

        assert_eq!(Cow::from("StringWrapper"), formatter.fmt_type_name(ty_id));
    }
}
