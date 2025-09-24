//! This module contains functions for formatting types

use diplomat_core::hir::{
    self, DocsTypeReferenceSyntax, DocsUrlGenerator, MaybeOwn, StringEncoding, SymbolId, TraitId,
    TyPosition, TypeContext, TypeId,
};
use std::borrow::Cow;
use std::sync::LazyLock;

/// This type mediates all formatting
///
/// All identifiers from the HIR should go through here before being formatted
/// into the output: This makes it easy to handle reserved words or add rename support
///
/// If you find yourself needing an identifier formatted in a context not yet available here, please add a new method
///
/// This type may be used by other backends attempting to figure out the names
/// of C types and methods.
pub struct CFormatter<'tcx> {
    tcx: &'tcx TypeContext,
    is_for_cpp: bool,
    docs_url_gen: &'tcx DocsUrlGenerator,

    lib_name: Option<String>,
}

pub(crate) const CAPI_NAMESPACE: &str = "capi";

impl<'tcx> CFormatter<'tcx> {
    pub fn new(
        tcx: &'tcx TypeContext,
        is_for_cpp: bool,
        config: &crate::Config,
        docs_url_gen: &'tcx DocsUrlGenerator,
    ) -> Self {
        Self {
            tcx,
            is_for_cpp,
            lib_name: config.shared_config.lib_name.clone(),
            docs_url_gen,
        }
    }
    pub fn tcx(&self) -> &'tcx TypeContext {
        self.tcx
    }

    /// Resolve and format a named type for use in code (without the namespace)
    pub fn fmt_type_name(&self, id: TypeId) -> Cow<'tcx, str> {
        let resolved = self.tcx.resolve_type(id);
        self.fmt_type_name_resolved(resolved)
    }

    pub fn fmt_type_name_resolved(&self, resolved: hir::TypeDef<'tcx>) -> Cow<'tcx, str> {
        let name: Cow<_> = resolved.name().as_str().into();
        let attrs = resolved.attrs();

        // Only apply renames in cpp mode, in pure C mode you'd want the
        // method names to match the type names.
        // Potential future improvement: Use alias attributes in pure C mode.
        if self.is_for_cpp {
            attrs.rename.apply(name)
        } else {
            name
        }
    }

    pub fn fmt_trait_name(&self, id: TraitId) -> Cow<'tcx, str> {
        let resolved = self.tcx.resolve_trait(id);
        self.fmt_trait_name_resolved(resolved)
    }

    pub fn fmt_trait_name_resolved(&self, resolved: &'tcx hir::TraitDef) -> Cow<'tcx, str> {
        let name: Cow<_> = resolved.name.as_str().into();
        let attrs = &resolved.attrs;

        // Only apply renames in cpp mode, in pure C mode you'd want the
        // method names to match the type names.
        // Potential future improvement: Use alias attributes in pure C mode.
        if self.is_for_cpp {
            attrs.rename.apply(name)
        } else {
            name
        }
    }

    /// Given a type found inside a DiplomatOption<T>, provide the name of the corresponding option type
    ///
    /// ty_name may or may not have namespacing done to it, you can use the result of `fmt_type_name`, `fmt_type_name_maybe_namespaced`,
    /// or something more complex here.
    pub fn fmt_optional_type_name<P: TyPosition>(
        &self,
        ty: &hir::Type<P>,
        ty_name: &str,
    ) -> String {
        match ty {
            hir::Type::Primitive(prim) => self.diplomat_namespace(format!("Option{}", self.fmt_primitive_name_for_derived_type(*prim)).into()).into(),
            hir::Type::Struct(..) | hir::Type::Enum(..) => format!("{ty_name}_option"),
            hir::Type::Slice(hir::Slice::Strs(encoding)) => {
                self.diplomat_namespace(
                match encoding {
                    StringEncoding::UnvalidatedUtf8 => "OptionStringsView".into(),
                    StringEncoding::UnvalidatedUtf16 => "OptionStrings16View".into(),
                    _ => unimplemented!("Utf8 StringEncoding unsupported")
                    }
                ).to_string()
            },
            hir::Type::Slice(hir::Slice::Str(_lifetime, encoding )) => {
                self.diplomat_namespace(
                match encoding {
                    StringEncoding::UnvalidatedUtf16 => "OptionString16View".into(),
                    _ => "OptionStringView".into(),
                    }
                ).to_string()
            }
            hir::Type::Slice(hir::Slice::Primitive(borrow, prim)) => {
                let prim = self.fmt_primitive_name_for_derived_type(*prim);
                let mtb = match borrow {
                    MaybeOwn::Borrow(borrow) if borrow.mutability.is_immutable() => "",
                    _ => "Mut",
                };
                self.diplomat_namespace(format!("Option{prim}View{mtb}").into()).to_string()
            }
            _ => unreachable!("Called fmt_optional_type_name with type {ty_name}, which is not allowed inside an Option")
        }
    }

    /// Resolve and format a named type for use in code (with a namespace, if needed by C++)
    pub fn fmt_type_name_maybe_namespaced(&self, id: SymbolId) -> Cow<'tcx, str> {
        let (name, attrs) = match id {
            SymbolId::TypeId(id) => {
                let resolved = self.tcx.resolve_type(id);
                let name: Cow<_> = resolved.name().as_str().into();
                let attrs = resolved.attrs();
                (name, attrs)
            }
            SymbolId::TraitId(id) => {
                let resolved = self.tcx.resolve_trait(id);
                let name: Cow<_> = resolved.name.as_str().into();
                let attrs = &resolved.attrs;
                (name, attrs)
            }
            _ => panic!("Unexpected symbol ID type"),
        };
        // Only apply renames in cpp mode, in pure C mode you'd want the
        // method names to match the type names.
        // Potential future improvement: Use alias attributes in pure C mode.
        let name = if self.is_for_cpp {
            attrs.rename.apply(name)
        } else {
            name
        };
        self.diplomat_namespace_for_custom_type(name, attrs.namespace.as_deref())
    }

    /// Resolve and format the name of a type for use in header names: decl version
    //
    /// Enums can't be forward-declared in C, but we do want enums to have methods,
    /// which may require additional #includes leading to potential cycles.
    /// To handle this, we make a separate header file called Foo_decl.h, that contains
    /// *just* the enum. It is included from Foo.h, and external users should not be importing
    /// it directly. (We can potentially add a #define guard that makes this actually private, if needed)
    pub fn fmt_decl_header_path(&self, id: SymbolId) -> String {
        let type_name = match id {
            SymbolId::TypeId(id) => self.fmt_type_name(id),
            SymbolId::TraitId(id) => self.fmt_trait_name(id),
            _ => panic!("Unexpected symbol ID type"),
        };
        format!("{type_name}.d.h")
    }
    /// Resolve and format the name of a type for use in header names: impl version
    pub fn fmt_impl_header_path(&self, id: SymbolId) -> String {
        let type_name = match id {
            SymbolId::TypeId(id) => self.fmt_type_name(id),
            SymbolId::TraitId(id) => self.fmt_trait_name(id),
            _ => panic!("Unexpected symbol ID type"),
        };
        format!("{type_name}.h")
    }

    /// Format an enum variant.
    pub fn fmt_enum_variant(
        &self,
        type_name: &str,
        variant: &'tcx hir::EnumVariant,
    ) -> Cow<'tcx, str> {
        let variant_name = variant.name.as_str();
        format!("{type_name}_{variant_name}").into()
    }
    /// Format a field name or parameter name
    // might need splitting in the future if we decide to support renames here
    pub fn fmt_param_name<'a>(&self, ident: &'a str) -> Cow<'a, str> {
        ident.into()
    }

    pub fn fmt_ptr<'a>(&self, ident: &'a str, mutability: hir::Mutability) -> Cow<'a, str> {
        // TODO: Where is the right place to put `const` here?
        if mutability.is_mutable() {
            format!("{ident}*").into()
        } else {
            format!("const {ident}*").into()
        }
    }

    /// Get the primitive type as a C type
    pub fn fmt_primitive_as_c(&self, prim: hir::PrimitiveType) -> Cow<'static, str> {
        use diplomat_core::hir::{FloatType, IntSizeType, IntType, PrimitiveType};
        let s = match prim {
            PrimitiveType::Bool => "bool",

            PrimitiveType::Char => "char32_t",
            PrimitiveType::Int(IntType::I8) | PrimitiveType::Ordering => "int8_t",
            PrimitiveType::Int(IntType::U8) | PrimitiveType::Byte => "uint8_t",
            PrimitiveType::Int(IntType::I16) => "int16_t",
            PrimitiveType::Int(IntType::U16) => "uint16_t",
            PrimitiveType::Int(IntType::I32) => "int32_t",
            PrimitiveType::Int(IntType::U32) => "uint32_t",
            PrimitiveType::Int(IntType::I64) => "int64_t",
            PrimitiveType::Int(IntType::U64) => "uint64_t",
            PrimitiveType::Int128(_) => panic!("i128 not supported in C"),
            PrimitiveType::IntSize(IntSizeType::Isize) => "intptr_t",
            PrimitiveType::IntSize(IntSizeType::Usize) => "size_t",
            PrimitiveType::Float(FloatType::F32) => "float",
            PrimitiveType::Float(FloatType::F64) => "double",
        };
        s.into()
    }

    /// Get the primitive name as used in a "derived" type (like slices and options)
    pub fn fmt_primitive_name_for_derived_type(&self, prim: hir::PrimitiveType) -> &'static str {
        use diplomat_core::hir::{FloatType, IntSizeType, IntType, PrimitiveType};
        match prim {
            PrimitiveType::Bool => "Bool",
            PrimitiveType::Char => "Char",
            PrimitiveType::Int(IntType::I8) | PrimitiveType::Ordering => "I8",
            PrimitiveType::Int(IntType::U8) | PrimitiveType::Byte => "U8",
            PrimitiveType::Int(IntType::I16) => "I16",
            PrimitiveType::Int(IntType::U16) => "U16",
            PrimitiveType::Int(IntType::I32) => "I32",
            PrimitiveType::Int(IntType::U32) => "U32",
            PrimitiveType::Int(IntType::I64) => "I64",
            PrimitiveType::Int(IntType::U64) => "U64",
            PrimitiveType::Int128(_) => panic!("i128 not supported in C"),
            PrimitiveType::IntSize(IntSizeType::Isize) => "Isize",
            PrimitiveType::IntSize(IntSizeType::Usize) => "Usize",
            PrimitiveType::Float(FloatType::F32) => "F32",
            PrimitiveType::Float(FloatType::F64) => "F64",
        }
    }
    /// Get the primitive type as a C type
    pub fn fmt_primitive_slice_name(
        &self,
        borrow: MaybeOwn,
        prim: hir::PrimitiveType,
    ) -> Cow<'tcx, str> {
        let prim = self.fmt_primitive_name_for_derived_type(prim);
        let mtb = match borrow {
            MaybeOwn::Borrow(borrow) if borrow.mutability.is_immutable() => "",
            _ => "Mut",
        };
        self.diplomat_namespace(format!("Diplomat{prim}View{mtb}").into())
    }

    pub fn fmt_struct_slice_name<P: TyPosition>(
        &self,
        borrow: MaybeOwn,
        st_ty: &P::StructPath,
    ) -> Cow<'tcx, str> {
        let st_id = hir::StructPathLike::id(st_ty);
        let st_name = self.fmt_type_name(st_id);

        let def = self.tcx.resolve_type(st_id);

        let mtb = match borrow {
            MaybeOwn::Borrow(borrow) if borrow.mutability.is_immutable() => "",
            _ => "Mut",
        };

        let ty = format!("Diplomat{st_name}View{mtb}");

        self.diplomat_namespace_for_custom_type(ty.into(), def.attrs().namespace.as_deref())
    }

    pub(crate) fn fmt_write_name(&self) -> Cow<'tcx, str> {
        self.diplomat_namespace("DiplomatWrite".into())
    }

    pub(crate) fn fmt_str_view_name(&self, encoding: StringEncoding) -> Cow<'tcx, str> {
        self.diplomat_namespace(
            match encoding {
                hir::StringEncoding::UnvalidatedUtf16 => "DiplomatString16View",
                _ => "DiplomatStringView",
            }
            .into(),
        )
    }

    pub(crate) fn fmt_strs_view_name(&self, encoding: StringEncoding) -> Cow<'tcx, str> {
        self.diplomat_namespace(
            match encoding {
                hir::StringEncoding::UnvalidatedUtf16 => "DiplomatStrings16View",
                _ => "DiplomatStringsView",
            }
            .into(),
        )
    }

    pub(crate) fn fmt_docs(&self, docs: &hir::Docs) -> String {
        docs.to_markdown(DocsTypeReferenceSyntax::AtLink, self.docs_url_gen)
            .trim()
            .to_string()
    }

    pub(crate) fn fmt_identifier<'a>(&self, name: Cow<'a, str>) -> Cow<'a, str> {
        // Source: https://en.cppreference.com/w/c/keyword
        #[rustfmt::skip]
        static C_KEYWORDS: LazyLock<std::collections::HashSet<&str>> =
            LazyLock::new(|| [
                "alignas", "alignof", "auto", "bool", "break", "case", "char", "const", "constexpr", "continue",
                "default", "do", "double", "else", "enum", "extern", "false", "float", "for", "goto", "if", "inline",
                "int", "long", "nullptr", "register", "restrict", "return", "short", "signed", "sizeof", "static",
                "static_assert", "struct", "switch", "thread_local", "true", "typedef", "typeof", "typeof_unqual",
                "union", "unsigned", "void", "volatile", "while", "_Alignas", "alignas", "_Alignof", "alignof", 
                "_Atomic", "_BitInt", "_Bool", "bool", "_Complex", "complex", "_Decimal128", "_Decimal32",
                "_Decimal64", "_Generic", "_Imaginary", "imaginary", "_Noreturn", "noreturn", "_Static_assert",
                "static_assert", "_Thread_local", "thread_local"].into());

        static CPP_KEYWORDS: LazyLock<std::collections::HashSet<&str>> = LazyLock::new(|| {
            let mut v = C_KEYWORDS.clone();
            // Source: https://en.cppreference.com/w/cpp/keyword
            #[rustfmt::skip]
            v.extend(
                [
                    "alignas", "alignof", "and", "and_eq", "asm", "atomic_cancel", "atomic_commit", "atomic_noexcept",
                    "auto", "bitand", "bitor", "bool", "break", "case", "catch", "char", "char8_t", "char16_t",
                    "char32_t", "class", "compl", "concept", "const", "consteval", "constexpr", "constinit", "const_cast",
                    "continue", "contract_assert", "co_await", "co_return", "co_yield", "decltype", "default", "delete",
                    "do", "double", "dynamic_cast", "else", "enum", "explicit", "export", "extern", "false", "float", 
                    "for", "friend", "goto", "if", "inline", "int", "long", "mutable", "namespace", "new", "noexcept",
                    "not", "not_eq", "nullptr", "operator", "or", "or_eq", "private", "protected", "public", "reflexpr",
                    "register", "reinterpret_cast", "requires", "return", "short", "signed", "sizeof", "static", 
                    "static_assert", "static_cast", "struct", "switch", "synchronized", "template", "this", 
                    "thread_local", "throw", "true", "try", "typedef", "typeid", "typename", "union", "unsigned", 
                    "using", "virtual", "void", "volatile", "wchar_t", "while", "xor", "xor_eq",                 ]
                .iter(),
            );
            v
        });

        let lang_keywords = {
            if self.is_for_cpp {
                &CPP_KEYWORDS
            } else {
                &C_KEYWORDS
            }
        };

        if lang_keywords.contains(name.as_ref()) {
            format!("{name}_").into()
        } else {
            name
        }
    }

    /// Custom types in the capi namespace end up in either diplomat::capi::foo
    /// or somens::capi::foo (Diplomat avoids polluting the global namespace with a `capi` namespace)
    pub fn diplomat_namespace_for_custom_type(
        &self,
        ty: Cow<'tcx, str>,
        ns: Option<&'_ str>,
    ) -> Cow<'tcx, str> {
        if self.is_for_cpp {
            if let Some(lib_name) = &self.lib_name {
                if let Some(ns) = ns {
                    format!("{lib_name}::{ns}::{CAPI_NAMESPACE}::{ty}").into()
                } else {
                    format!("{lib_name}::{CAPI_NAMESPACE}::{ty}").into()
                }
            } else {
                let root = ns.unwrap_or("diplomat");
                format!("{root}::{CAPI_NAMESPACE}::{ty}").into()
            }
        } else {
            ty
        }
    }

    /// For types from diplomat_runtime.h
    fn diplomat_namespace(&self, ty: Cow<'tcx, str>) -> Cow<'tcx, str> {
        if self.is_for_cpp {
            if let Some(lib_name) = &self.lib_name {
                format!("{lib_name}::diplomat::{CAPI_NAMESPACE}::{ty}").into()
            } else {
                format!("diplomat::{CAPI_NAMESPACE}::{ty}").into()
            }
        } else {
            ty
        }
    }
}
