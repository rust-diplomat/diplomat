//! Store all the types contained in the HIR.

use super::lowering::ItemAndInfo;
use super::ty_position::StructPathLike;
use super::{
    AttributeContext, AttributeValidator, Attrs, EnumDef, LoweringContext, LoweringError,
    MaybeStatic, OpaqueDef, OutStructDef, StructDef, TypeDef,
};
use crate::ast::attrs::AttrInheritContext;
#[allow(unused_imports)] // use in docs links
use crate::hir;
use crate::{ast, Env};
use core::fmt::Display;
use smallvec::SmallVec;
use std::collections::HashMap;
use std::ops::Index;

/// A context type owning all types exposed to Diplomat.
#[derive(Debug)]
pub struct TypeContext {
    out_structs: Vec<OutStructDef>,
    structs: Vec<StructDef>,
    opaques: Vec<OpaqueDef>,
    enums: Vec<EnumDef>,
}

/// Key used to index into a [`TypeContext`] representing a struct.
#[derive(Copy, Clone, Debug)]
pub struct StructId(usize);

/// Key used to index into a [`TypeContext`] representing an out struct.
#[derive(Copy, Clone, Debug)]
pub struct OutStructId(usize);

/// Key used to index into a [`TypeContext`] representing a opaque.
#[derive(Copy, Clone, Debug)]
pub struct OpaqueId(usize);

/// Key used to index into a [`TypeContext`] representing an enum.
#[derive(Copy, Clone, Debug)]
pub struct EnumId(usize);

#[derive(Copy, Clone, Debug)]
#[non_exhaustive]
pub enum TypeId {
    Struct(StructId),
    OutStruct(OutStructId),
    Opaque(OpaqueId),
    Enum(EnumId),
}

impl TypeContext {
    pub fn all_types<'tcx>(&'tcx self) -> impl Iterator<Item = (TypeId, TypeDef<'tcx>)> {
        self.structs
            .iter()
            .enumerate()
            .map(|(i, ty)| (TypeId::Struct(StructId(i)), TypeDef::Struct(ty)))
            .chain(
                self.out_structs
                    .iter()
                    .enumerate()
                    .map(|(i, ty)| (TypeId::OutStruct(OutStructId(i)), TypeDef::OutStruct(ty))),
            )
            .chain(
                self.opaques
                    .iter()
                    .enumerate()
                    .map(|(i, ty)| (TypeId::Opaque(OpaqueId(i)), TypeDef::Opaque(ty))),
            )
            .chain(
                self.enums
                    .iter()
                    .enumerate()
                    .map(|(i, ty)| (TypeId::Enum(EnumId(i)), TypeDef::Enum(ty))),
            )
    }

    pub fn out_structs(&self) -> &[OutStructDef] {
        &self.out_structs
    }

    pub fn structs(&self) -> &[StructDef] {
        &self.structs
    }

    pub fn opaques(&self) -> &[OpaqueDef] {
        &self.opaques
    }

    pub fn enums(&self) -> &[EnumDef] {
        &self.enums
    }

    pub fn resolve_type<'tcx>(&'tcx self, id: TypeId) -> TypeDef<'tcx> {
        match id {
            TypeId::Struct(i) => TypeDef::Struct(self.resolve_struct(i)),
            TypeId::OutStruct(i) => TypeDef::OutStruct(self.resolve_out_struct(i)),
            TypeId::Opaque(i) => TypeDef::Opaque(self.resolve_opaque(i)),
            TypeId::Enum(i) => TypeDef::Enum(self.resolve_enum(i)),
        }
    }

    pub(crate) fn resolve_out_struct(&self, id: OutStructId) -> &OutStructDef {
        self.out_structs.index(id.0)
    }

    pub(crate) fn resolve_struct(&self, id: StructId) -> &StructDef {
        self.structs.index(id.0)
    }

    pub(crate) fn resolve_opaque(&self, id: OpaqueId) -> &OpaqueDef {
        self.opaques.index(id.0)
    }

    pub(crate) fn resolve_enum(&self, id: EnumId) -> &EnumDef {
        self.enums.index(id.0)
    }

    /// Lower the AST to the HIR while simultaneously performing validation.
    pub fn from_ast(
        env: &Env,
        attr_validator: impl AttributeValidator + 'static,
    ) -> Result<Self, Vec<LoweringError>> {
        let mut ast_out_structs = SmallVec::<[_; 16]>::new();
        let mut ast_structs = SmallVec::<[_; 16]>::new();
        let mut ast_opaques = SmallVec::<[_; 16]>::new();
        let mut ast_enums = SmallVec::<[_; 16]>::new();

        let mut errors = Vec::with_capacity(0);

        for (path, mod_env) in env.iter_modules() {
            let mod_attrs = Attrs::from_ast(
                &mod_env.attrs,
                &attr_validator,
                AttributeContext::Module,
                &Default::default(),
                &mut errors,
            );
            let ty_attrs = mod_attrs.for_inheritance(AttrInheritContext::Type);
            let method_attrs =
                mod_attrs.for_inheritance(AttrInheritContext::MethodOrImplFromModule);

            for sym in mod_env.items() {
                if let ast::ModSymbol::CustomType(custom_type) = sym {
                    match custom_type {
                        ast::CustomType::Struct(strct) => {
                            let item = ItemAndInfo {
                                item: strct,
                                in_path: path,
                                ty_parent_attrs: ty_attrs.clone(),
                                method_parent_attrs: method_attrs.clone(),
                            };
                            if strct.output_only {
                                ast_out_structs.push(item);
                            } else {
                                ast_structs.push(item);
                            }
                        }
                        ast::CustomType::Opaque(opaque) => {
                            let item = ItemAndInfo {
                                item: opaque,
                                in_path: path,
                                ty_parent_attrs: ty_attrs.clone(),
                                method_parent_attrs: method_attrs.clone(),
                            };
                            ast_opaques.push(item)
                        }
                        ast::CustomType::Enum(enm) => {
                            let item = ItemAndInfo {
                                item: enm,
                                in_path: path,
                                ty_parent_attrs: ty_attrs.clone(),
                                method_parent_attrs: method_attrs.clone(),
                            };
                            ast_enums.push(item)
                        }
                    }
                }
            }
        }

        let lookup_id = LookupId::new(
            &ast_out_structs[..],
            &ast_structs[..],
            &ast_opaques[..],
            &ast_enums[..],
        );
        let attr_validator = Box::new(attr_validator);

        let mut ctx = LoweringContext {
            lookup_id,
            env,
            errors: &mut errors,
            attr_validator,
        };

        let out_structs = ctx.lower_all_out_structs(ast_out_structs.into_iter());
        let structs = ctx.lower_all_structs(ast_structs.into_iter());
        let opaques = ctx.lower_all_opaques(ast_opaques.into_iter());
        let enums = ctx.lower_all_enums(ast_enums.into_iter());

        match (out_structs, structs, opaques, enums) {
            (Ok(out_structs), Ok(structs), Ok(opaques), Ok(enums)) => {
                let res = Self {
                    out_structs,
                    structs,
                    opaques,
                    enums,
                };
                assert!(
                    errors.is_empty(),
                    "All lowering succeeded but still found error messages: {errors:?}"
                );
                res.validate(&mut errors);
                if !errors.is_empty() {
                    return Err(errors);
                }
                Ok(res)
            }
            _ => {
                assert!(!errors.is_empty(), "Lowering failed without error messages");
                Err(errors)
            }
        }
    }

    /// Run validation phase
    ///
    /// Currently validates that methods are not inheriting any transitive bounds from parameters
    ///    Todo: Automatically insert these bounds during HIR construction in a second phase
    fn validate(&self, errors: &mut Vec<LoweringError>) {
        // Lifetime validity check
        for (_id, ty) in self.all_types() {
            for method in ty.methods() {
                for param in &method.params {
                    self.validate_ty_in_method(errors, &ty, &param.name, &param.ty, method);
                }

                method.output.with_contained_types(|out_ty| {
                    self.validate_ty_in_method(errors, &ty, "return type", out_ty, method)
                })
            }
        }
    }

    /// Ensure that a given method's input our output type does not implicitly introduce bounds that are not
    /// already specified on the method
    fn validate_ty_in_method<P: hir::TyPosition>(
        &self,
        errors: &mut Vec<LoweringError>,
        ty: &hir::TypeDef,
        param_name: impl Display,
        param_ty: &hir::Type<P>,
        method: &hir::Method,
    ) {
        let linked = match &param_ty {
            hir::Type::Opaque(p) => p.link_lifetimes(self),
            hir::Type::Struct(p) => p.link_lifetimes(self),
            _ => return,
        };

        for (use_lt, def_lt) in linked.lifetimes_all() {
            let MaybeStatic::NonStatic(use_lt) = use_lt else {
                continue;
            };
            let Some(use_bounds) = &method.lifetime_env.get_bounds(use_lt) else {
                continue;
            };
            let use_longer_lifetimes = &use_bounds.longer;
            let anchor;
            let def_longer_lifetimes = if let Some(def_lt) = def_lt {
                let Some(def_bounds) = &linked.def_env().get_bounds(def_lt) else {
                    continue;
                };
                &def_bounds.longer
            } else {
                anchor = linked.def_env().all_lifetimes().collect();
                &anchor
            };

            for def_longer in def_longer_lifetimes {
                let MaybeStatic::NonStatic(corresponding_use) = linked.def_to_use(*def_longer)
                else {
                    continue;
                };

                // In the case of stuff like <'a, 'a> passed to Foo<'x, 'y: 'x> the bound
                // is trivially fulfilled
                if corresponding_use == use_lt {
                    continue;
                }

                if !use_longer_lifetimes.contains(&corresponding_use) {
                    let ty_name = &ty.name();
                    let method_name = &method.name;
                    let use_name = method.lifetime_env.fmt_lifetime(use_lt);
                    let use_longer_name = method.lifetime_env.fmt_lifetime(corresponding_use);
                    let def_cause = if let Some(def_lt) = def_lt {
                        let def_name = linked.def_env().fmt_lifetime(def_lt);
                        let def_longer_name = linked.def_env().fmt_lifetime(def_longer);
                        format!("comes from source type's '{def_longer_name}: '{def_name}")
                    } else {
                        // This case is technically already handled in the lifetime lowerer, we're being careful
                        "comes from &-ref's lifetime in parameter".into()
                    };
                    errors.push(LoweringError::Other(format!("{ty_name}::{method_name} should explicitly include this \
                                        lifetime bound from param {param_name}: '{use_longer_name}: '{use_name} ({def_cause})")))
                }
            }
        }
    }
}

/// Struct that just wraps the mapping from AST custom types to their IDs that
/// will show up in the final [`TypeContext`].
///
/// The entire point of this type is to reduce the number of arguments in helper
/// functions which need to look up IDs for structs. It does nothing fancy and
/// is only ever used when constructing a [`TypeContext`].
pub(super) struct LookupId<'ast> {
    out_struct_map: HashMap<&'ast ast::Struct, OutStructId>,
    struct_map: HashMap<&'ast ast::Struct, StructId>,
    opaque_map: HashMap<&'ast ast::OpaqueStruct, OpaqueId>,
    enum_map: HashMap<&'ast ast::Enum, EnumId>,
}

impl<'ast> LookupId<'ast> {
    /// Returns a new [`LookupId`].
    fn new(
        out_structs: &[ItemAndInfo<'ast, ast::Struct>],
        structs: &[ItemAndInfo<'ast, ast::Struct>],
        opaques: &[ItemAndInfo<'ast, ast::OpaqueStruct>],
        enums: &[ItemAndInfo<'ast, ast::Enum>],
    ) -> Self {
        Self {
            out_struct_map: out_structs
                .iter()
                .enumerate()
                .map(|(index, item)| (item.item, OutStructId(index)))
                .collect(),
            struct_map: structs
                .iter()
                .enumerate()
                .map(|(index, item)| (item.item, StructId(index)))
                .collect(),
            opaque_map: opaques
                .iter()
                .enumerate()
                .map(|(index, item)| (item.item, OpaqueId(index)))
                .collect(),
            enum_map: enums
                .iter()
                .enumerate()
                .map(|(index, item)| (item.item, EnumId(index)))
                .collect(),
        }
    }

    pub(super) fn resolve_out_struct(&self, strct: &ast::Struct) -> Option<OutStructId> {
        self.out_struct_map.get(strct).copied()
    }

    pub(super) fn resolve_struct(&self, strct: &ast::Struct) -> Option<StructId> {
        self.struct_map.get(strct).copied()
    }

    pub(super) fn resolve_opaque(&self, opaque: &ast::OpaqueStruct) -> Option<OpaqueId> {
        self.opaque_map.get(opaque).copied()
    }

    pub(super) fn resolve_enum(&self, enm: &ast::Enum) -> Option<EnumId> {
        self.enum_map.get(enm).copied()
    }
}

impl From<StructId> for TypeId {
    fn from(x: StructId) -> Self {
        TypeId::Struct(x)
    }
}

impl From<OutStructId> for TypeId {
    fn from(x: OutStructId) -> Self {
        TypeId::OutStruct(x)
    }
}

impl From<OpaqueId> for TypeId {
    fn from(x: OpaqueId) -> Self {
        TypeId::Opaque(x)
    }
}

impl From<EnumId> for TypeId {
    fn from(x: EnumId) -> Self {
        TypeId::Enum(x)
    }
}

#[cfg(test)]
mod tests {
    use crate::hir;
    use std::fmt::Write;

    macro_rules! uitest_validity {
        ($($file:tt)*) => {
            let parsed: syn::File = syn::parse_quote! { $($file)* };
            let custom_types = crate::ast::File::from(&parsed);
            let env = custom_types.all_types();

            let errors = custom_types.check_validity(&env);

            let mut output = String::new();
            for error in errors {
                writeln!(&mut output, "AST ERROR: {error}").unwrap();
            }

            let attr_validator = hir::BasicAttributeValidator::new("tests");
            match hir::TypeContext::from_ast(&env, attr_validator) {
                Ok(_context) => (),
                Err(e) => {
                    for err in e {
                        writeln!(&mut output, "Lowering error: {err}").unwrap();
                    }
                }
            };
            insta::with_settings!({}, {
                insta::assert_display_snapshot!(output)
            });
        }
    }

    #[test]
    fn test_required_implied_bounds() {
        uitest_validity! {
            #[diplomat::bridge]
            mod ffi {
                #[diplomat::opaque]
                struct Foo<'a, 'b: 'a, 'c: 'b> (&'a u8, &'b u8, &'c u8);

                #[diplomat::opaque]
                struct Opaque;


                #[diplomat::opaque]
                struct OneLifetime<'a>(&'a u8);

                impl Opaque {
                    pub fn use_foo<'x, 'y, 'z>(&self, foo: &Foo<'x, 'y, 'z>) {}
                    pub fn return_foo<'x, 'y, 'z>(&'x self) -> Box<Foo<'x, 'y, 'z>> {}
                    pub fn return_result_foo<'x, 'y, 'z>(&'x self) -> Result<Box<Foo<'x, 'y, 'z>>, ()> {}
                    // This doesn't actually error since the lowerer inserts the implicit bound
                    pub fn implied_ref_bound<'a, 'b>(&self, one_lt: &'a OneLifetime<'b>) {}
                }
            }
        }
    }
}
