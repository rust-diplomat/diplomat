use super::{
    AttributeContext, AttributeValidator, Attrs, Borrow, BoundedLifetime, EnumDef, EnumPath,
    EnumVariant, IdentBuf, Lifetime, LifetimeEnv, LifetimeLowerer, LookupId, MaybeOwn, Method,
    NonOptional, OpaqueDef, OpaquePath, Optional, OutStructDef, OutStructField, OutStructPath,
    OutType, Param, ParamLifetimeLowerer, ParamSelf, PrimitiveType, ReturnLifetimeLowerer,
    ReturnType, ReturnableStructPath, SelfParamLifetimeLowerer, SelfType, Slice, StructDef,
    StructField, StructPath, SuccessType, Type, TypeDef, TypeId,
};
use crate::ast::attrs::AttrInheritContext;
use crate::{ast, Env};
use core::fmt;
use strck_ident::IntoCk;

/// An error from lowering the AST to the HIR.
#[derive(Debug)]
#[non_exhaustive]
pub enum LoweringError {
    /// The purpose of having this is that translating to the HIR has enormous
    /// potential for really detailed error handling and giving suggestions.
    ///
    /// Unfortunately, working out what the error enum should look like to enable
    /// this is really hard. The plan is that once the lowering code is completely
    /// written, we ctrl+F for `"LoweringError::Other"` in the lowering code, and turn every
    /// instance into an specialized enum variant, generalizing where possible
    /// without losing any information.
    Other(String),
}

impl fmt::Display for LoweringError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::Other(ref s) => s.fmt(f),
        }
    }
}

pub(super) struct LoweringContext<'ast, 'errors> {
    pub lookup_id: LookupId<'ast>,
    pub errors: &'errors mut Vec<LoweringError>,
    pub env: &'ast Env,
    pub attr_validator: Box<dyn AttributeValidator>,
}

/// An item and the info needed to
pub(crate) struct ItemAndInfo<'ast, Ast> {
    pub(crate) item: &'ast Ast,
    pub(crate) in_path: &'ast ast::Path,
    /// Any parent attributes resolved from the module, for a type context
    pub(crate) ty_parent_attrs: Attrs,

    /// Any parent attributes resolved from the module, for a method context
    pub(crate) method_parent_attrs: Attrs,
    pub(crate) id: TypeId,
}

impl<'ast, 'errors> LoweringContext<'ast, 'errors> {
    /// Lowers an [`ast::Ident`]s into an [`hir::IdentBuf`].
    ///
    /// If there are any errors, they're pushed to `errors` and `Err` is returned.
    pub(super) fn lower_ident(
        &mut self,
        ident: &ast::Ident,
        context: &'static str,
    ) -> Result<IdentBuf, ()> {
        match ident.as_str().ck() {
            Ok(name) => Ok(name.to_owned()),
            Err(e) => {
                self.errors.push(LoweringError::Other(format!(
                    "Ident `{ident}` from {context} could not be turned into a Rust ident: {e}"
                )));
                Err(())
            }
        }
    }

    /// Lowers multiple items at once
    fn lower_all<Ast: 'static, Hir>(
        &mut self,
        ast_defs: impl ExactSizeIterator<Item = ItemAndInfo<'ast, Ast>>,
        lower: impl Fn(&mut Self, ItemAndInfo<'ast, Ast>) -> Result<Hir, ()>,
    ) -> Result<Vec<Hir>, ()> {
        let mut hir_types = Ok(Vec::with_capacity(ast_defs.len()));

        for def in ast_defs {
            let hir_type = lower(self, def);

            match (hir_type, &mut hir_types) {
                (Ok(hir_type), Ok(hir_types)) => hir_types.push(hir_type),
                _ => hir_types = Err(()),
            }
        }

        hir_types
    }

    pub(super) fn lower_all_enums(
        &mut self,
        ast_defs: impl ExactSizeIterator<Item = ItemAndInfo<'ast, ast::Enum>>,
    ) -> Result<Vec<EnumDef>, ()> {
        self.lower_all(ast_defs, Self::lower_enum)
    }
    pub(super) fn lower_all_structs(
        &mut self,
        ast_defs: impl ExactSizeIterator<Item = ItemAndInfo<'ast, ast::Struct>>,
    ) -> Result<Vec<StructDef>, ()> {
        self.lower_all(ast_defs, Self::lower_struct)
    }
    pub(super) fn lower_all_out_structs(
        &mut self,
        ast_defs: impl ExactSizeIterator<Item = ItemAndInfo<'ast, ast::Struct>>,
    ) -> Result<Vec<OutStructDef>, ()> {
        self.lower_all(ast_defs, Self::lower_out_struct)
    }
    pub(super) fn lower_all_opaques(
        &mut self,
        ast_defs: impl ExactSizeIterator<Item = ItemAndInfo<'ast, ast::OpaqueStruct>>,
    ) -> Result<Vec<OpaqueDef>, ()> {
        self.lower_all(ast_defs, Self::lower_opaque)
    }

    fn lower_enum(&mut self, item: ItemAndInfo<'ast, ast::Enum>) -> Result<EnumDef, ()> {
        let ast_enum = item.item;
        let name = self.lower_ident(&ast_enum.name, "enum name");
        let attrs =
            self.attr_validator
                .attr_from_ast(&ast_enum.attrs, &item.ty_parent_attrs, self.errors);

        let mut variants = Ok(Vec::with_capacity(ast_enum.variants.len()));
        let variant_parent_attrs = attrs.for_inheritance(AttrInheritContext::Variant);
        for (ident, discriminant, docs, attrs) in ast_enum.variants.iter() {
            let name = self.lower_ident(ident, "enum variant");
            let attrs =
                self.attr_validator
                    .attr_from_ast(attrs, &variant_parent_attrs, self.errors);
            match (name, &mut variants) {
                (Ok(name), Ok(variants)) => {
                    let variant = EnumVariant {
                        docs: docs.clone(),
                        name,
                        discriminant: *discriminant,
                        attrs,
                    };
                    self.attr_validator.validate(
                        &variant.attrs,
                        AttributeContext::EnumVariant(&variant),
                        self.errors,
                    );
                    variants.push(variant);
                }
                _ => variants = Err(()),
            }
        }

        let methods = self.lower_all_methods(
            &ast_enum.methods[..],
            item.in_path,
            &item.method_parent_attrs,
            item.id,
        );

        let def = EnumDef::new(ast_enum.docs.clone(), name?, variants?, methods?, attrs);

        self.attr_validator.validate(
            &def.attrs,
            AttributeContext::Type(TypeDef::from(&def)),
            self.errors,
        );

        Ok(def)
    }

    fn lower_opaque(
        &mut self,
        item: ItemAndInfo<'ast, ast::OpaqueStruct>,
    ) -> Result<OpaqueDef, ()> {
        let ast_opaque = item.item;
        let name = self.lower_ident(&ast_opaque.name, "opaque name");

        let methods = self.lower_all_methods(
            &ast_opaque.methods[..],
            item.in_path,
            &item.method_parent_attrs,
            item.id,
        );
        let attrs = self.attr_validator.attr_from_ast(
            &ast_opaque.attrs,
            &item.ty_parent_attrs,
            self.errors,
        );
        let lifetimes = self.lower_type_lifetime_env(&ast_opaque.lifetimes);

        let def = OpaqueDef::new(ast_opaque.docs.clone(), name?, methods?, attrs, lifetimes?);
        self.attr_validator.validate(
            &def.attrs,
            AttributeContext::Type(TypeDef::from(&def)),
            self.errors,
        );
        Ok(def)
    }

    fn lower_struct(&mut self, item: ItemAndInfo<'ast, ast::Struct>) -> Result<StructDef, ()> {
        let ast_struct = item.item;
        let name = self.lower_ident(&ast_struct.name, "struct name");

        let fields = if ast_struct.fields.is_empty() {
            self.errors.push(LoweringError::Other(format!(
                "struct `{}` is a ZST because it has no fields",
                ast_struct.name
            )));
            Err(())
        } else {
            let mut fields = Ok(Vec::with_capacity(ast_struct.fields.len()));

            for (name, ty, docs) in ast_struct.fields.iter() {
                let name = self.lower_ident(name, "struct field name");
                let ty = self.lower_type(ty, &mut &ast_struct.lifetimes, item.in_path);

                match (name, ty, &mut fields) {
                    (Ok(name), Ok(ty), Ok(fields)) => fields.push(StructField {
                        docs: docs.clone(),
                        name,
                        ty,
                    }),
                    _ => fields = Err(()),
                }
            }

            fields
        };

        let methods = self.lower_all_methods(
            &ast_struct.methods[..],
            item.in_path,
            &item.method_parent_attrs,
            item.id,
        );
        let attrs = self.attr_validator.attr_from_ast(
            &ast_struct.attrs,
            &item.ty_parent_attrs,
            self.errors,
        );
        let lifetimes = self.lower_type_lifetime_env(&ast_struct.lifetimes);

        let def = StructDef::new(
            ast_struct.docs.clone(),
            name?,
            fields?,
            methods?,
            attrs,
            lifetimes?,
        );

        self.attr_validator.validate(
            &def.attrs,
            AttributeContext::Type(TypeDef::from(&def)),
            self.errors,
        );
        Ok(def)
    }

    fn lower_out_struct(
        &mut self,
        item: ItemAndInfo<'ast, ast::Struct>,
    ) -> Result<OutStructDef, ()> {
        let ast_out_struct = item.item;
        let name = self.lower_ident(&ast_out_struct.name, "out-struct name");

        let fields = if ast_out_struct.fields.is_empty() {
            self.errors.push(LoweringError::Other(format!(
                "struct `{}` is a ZST because it has no fields",
                ast_out_struct.name
            )));
            Err(())
        } else {
            let mut fields = Ok(Vec::with_capacity(ast_out_struct.fields.len()));

            for (name, ty, docs) in ast_out_struct.fields.iter() {
                let name = self.lower_ident(name, "out-struct field name");
                let ty = self.lower_out_type(ty, &mut &ast_out_struct.lifetimes, item.in_path);

                match (name, ty, &mut fields) {
                    (Ok(name), Ok(ty), Ok(fields)) => fields.push(OutStructField {
                        docs: docs.clone(),
                        name,
                        ty,
                    }),
                    _ => fields = Err(()),
                }
            }

            fields
        };

        let methods = self.lower_all_methods(
            &ast_out_struct.methods[..],
            item.in_path,
            &item.method_parent_attrs,
            item.id,
        );
        let attrs = self.attr_validator.attr_from_ast(
            &ast_out_struct.attrs,
            &item.ty_parent_attrs,
            self.errors,
        );

        let lifetimes = self.lower_type_lifetime_env(&ast_out_struct.lifetimes);
        let def = OutStructDef::new(
            ast_out_struct.docs.clone(),
            name?,
            fields?,
            methods?,
            attrs,
            lifetimes?,
        );

        self.attr_validator.validate(
            &def.attrs,
            AttributeContext::Type(TypeDef::from(&def)),
            self.errors,
        );
        Ok(def)
    }

    /// Lowers an [`ast::Method`]s an [`hir::Method`].
    ///
    /// If there are any errors, they're pushed to `errors` and `None` is returned.
    fn lower_method(
        &mut self,
        method: &'ast ast::Method,
        in_path: &ast::Path,
        method_parent_attrs: &Attrs,
        self_id: TypeId,
    ) -> Result<Method, ()> {
        let name = self.lower_ident(&method.name, "method name");

        let (ast_params, takes_writeable) = match method.params.split_last() {
            Some((last, remaining)) if last.is_writeable() => (remaining, true),
            _ => (&method.params[..], false),
        };

        let self_param_ltl = SelfParamLifetimeLowerer::new(&method.lifetime_env, self)?;

        let (param_self, param_ltl) = if let Some(self_param) = method.self_param.as_ref() {
            let (param_self, param_ltl) =
                self.lower_self_param(self_param, self_param_ltl, &method.full_path_name, in_path)?;
            (Some(param_self), param_ltl)
        } else {
            (None, SelfParamLifetimeLowerer::no_self_ref(self_param_ltl))
        };

        let (params, return_ltl) = self.lower_many_params(ast_params, param_ltl, in_path)?;

        let (output, lifetime_env) = self.lower_return_type(
            method.return_type.as_ref(),
            takes_writeable,
            return_ltl,
            in_path,
        )?;

        let attrs =
            self.attr_validator
                .attr_from_ast(&method.attrs, method_parent_attrs, self.errors);

        let method = Method {
            docs: method.docs.clone(),
            name: name?,
            lifetime_env,
            param_self,
            params,
            output,
            attrs,
        };

        self.attr_validator.validate(
            &method.attrs,
            AttributeContext::Method(&method, self_id),
            self.errors,
        );

        Ok(method)
    }

    /// Lowers many [`ast::Method`]s into a vector of [`hir::Method`]s.
    ///
    /// If there are any errors, they're pushed to `errors` and `None` is returned.
    fn lower_all_methods(
        &mut self,
        ast_methods: &'ast [ast::Method],
        in_path: &ast::Path,
        method_parent_attrs: &Attrs,
        self_id: TypeId,
    ) -> Result<Vec<Method>, ()> {
        let mut methods = Ok(Vec::with_capacity(ast_methods.len()));

        for method in ast_methods {
            let method = self.lower_method(method, in_path, method_parent_attrs, self_id);
            match (method, &mut methods) {
                (Ok(method), Ok(methods)) => {
                    methods.push(method);
                }
                _ => methods = Err(()),
            }
        }

        methods
    }

    /// Lowers an [`ast::TypeName`]s into a [`hir::Type`].
    ///
    /// If there are any errors, they're pushed to `errors` and `None` is returned.
    fn lower_type(
        &mut self,
        ty: &ast::TypeName,
        ltl: &mut impl LifetimeLowerer,
        in_path: &ast::Path,
    ) -> Result<Type, ()> {
        match ty {
            ast::TypeName::Primitive(prim) => Ok(Type::Primitive(PrimitiveType::from_ast(*prim))),
            ast::TypeName::Named(path) | ast::TypeName::SelfType(path) => {
                match path.resolve(in_path, self.env) {
                    ast::CustomType::Struct(strct) => {
                        if let Some(tcx_id) = self.lookup_id.resolve_struct(strct) {
                            let lifetimes = ltl.lower_generics(&path.lifetimes[..], ty.is_self());

                            Ok(Type::Struct(StructPath::new(lifetimes, tcx_id)))
                        } else if self.lookup_id.resolve_out_struct(strct).is_some() {
                            self.errors.push(LoweringError::Other(format!("found struct in input that is marked with #[diplomat::out]: {ty} in {path}")));
                            Err(())
                        } else {
                            unreachable!("struct `{}` wasn't found in the set of structs or out-structs, this is a bug.", strct.name);
                        }
                    }
                    ast::CustomType::Opaque(_) => {
                        self.errors.push(LoweringError::Other(format!(
                            "Opaque passed by value in input: {path}"
                        )));
                        Err(())
                    }
                    ast::CustomType::Enum(enm) => {
                        let tcx_id = self.lookup_id.resolve_enum(enm).expect(
                            "can't find enum in lookup map, which contains all enums from env",
                        );

                        Ok(Type::Enum(EnumPath::new(tcx_id)))
                    }
                }
            }
            ast::TypeName::Reference(lifetime, mutability, ref_ty) => match ref_ty.as_ref() {
                ast::TypeName::Named(path) | ast::TypeName::SelfType(path) => {
                    match path.resolve(in_path, self.env) {
                        ast::CustomType::Opaque(opaque) => {
                            let borrow = Borrow::new(ltl.lower_lifetime(lifetime), *mutability);
                            let lifetimes =
                                ltl.lower_generics(&path.lifetimes[..], ref_ty.is_self());
                            let tcx_id = self.lookup_id.resolve_opaque(opaque).expect(
                            "can't find opaque in lookup map, which contains all opaques from env",
                        );

                            Ok(Type::Opaque(OpaquePath::new(
                                lifetimes,
                                Optional(false),
                                borrow,
                                tcx_id,
                            )))
                        }
                        _ => {
                            self.errors.push(LoweringError::Other(format!("found &T in input where T is a custom type, but not opaque. T = {ref_ty}")));
                            Err(())
                        }
                    }
                }
                _ => {
                    self.errors.push(LoweringError::Other(format!("found &T in input where T isn't a custom type and therefore not opaque. T = {ref_ty}")));
                    Err(())
                }
            },
            ast::TypeName::Box(box_ty) => {
                self.errors.push(match box_ty.as_ref() {
                ast::TypeName::Named(path) | ast::TypeName::SelfType(path) => {
                    match path.resolve(in_path, self.env) {
                        ast::CustomType::Opaque(_) => LoweringError::Other(format!("found Box<T> in input where T is an opaque, but owned opaques aren't allowed in inputs. try &T instead? T = {path}")),
                        _ => LoweringError::Other(format!("found Box<T> in input where T is a custom type but not opaque. non-opaques can't be behind pointers, and opaques in inputs can't be owned. T = {path}")),
                    }
                }
                _ => LoweringError::Other(format!("found Box<T> in input where T isn't a custom type. T = {box_ty}")),
            });
                Err(())
            }
            ast::TypeName::Option(opt_ty) => {
                match opt_ty.as_ref() {
                    ast::TypeName::Reference(lifetime, mutability, ref_ty) => match ref_ty.as_ref()
                    {
                        ast::TypeName::Named(path) | ast::TypeName::SelfType(path) => match path
                            .resolve(in_path, self.env)
                        {
                            ast::CustomType::Opaque(opaque) => {
                                let borrow = Borrow::new(ltl.lower_lifetime(lifetime), *mutability);
                                let lifetimes =
                                    ltl.lower_generics(&path.lifetimes, ref_ty.is_self());
                                let tcx_id = self.lookup_id.resolve_opaque(opaque).expect(
                                    "can't find opaque in lookup map, which contains all opaques from env",
                                );

                                Ok(Type::Opaque(OpaquePath::new(
                                    lifetimes,
                                    Optional(true),
                                    borrow,
                                    tcx_id,
                                )))
                            }
                            _ => {
                                self.errors.push(LoweringError::Other(format!("found Option<&T> in input where T is a custom type, but it's not opaque. T = {ref_ty}")));
                                Err(())
                            }
                        },
                        _ => {
                            self.errors.push(LoweringError::Other(format!("found Option<&T> in input, but T isn't a custom type and therefore not opaque. T = {ref_ty}")));
                            Err(())
                        }
                    },
                    ast::TypeName::Box(box_ty) => {
                        // we could see whats in the box here too
                        self.errors.push(LoweringError::Other(format!("found Option<Box<T>> in input, but box isn't allowed in inputs. T = {box_ty}")));
                        Err(())
                    }
                    _ => {
                        self.errors.push(LoweringError::Other(format!("found Option<T> in input, where T isn't a reference but Option<T> in inputs requires that T is a reference to an opaque. T = {opt_ty}")));
                        Err(())
                    }
                }
            }
            ast::TypeName::Result(_, _, _) => {
                self.errors.push(LoweringError::Other(
                    "Results can only appear as the top-level return type of methods".into(),
                ));
                Err(())
            }
            ast::TypeName::Writeable => {
                self.errors.push(LoweringError::Other(
                    "Writeables can only appear as the last parameter of a method".into(),
                ));
                Err(())
            }
            ast::TypeName::StrReference(lifetime, encoding) => Ok(Type::Slice(Slice::Str(
                lifetime.as_ref().map(|lt| ltl.lower_lifetime(lt)),
                *encoding,
            ))),
            ast::TypeName::PrimitiveSlice(lm, prim) => Ok(Type::Slice(Slice::Primitive(
                lm.as_ref()
                    .map(|(lt, m)| Borrow::new(ltl.lower_lifetime(lt), *m)),
                PrimitiveType::from_ast(*prim),
            ))),
            ast::TypeName::Unit => {
                self.errors.push(LoweringError::Other("Unit types can only appear as the return value of a method, or as the Ok/Err variants of a returned result".into()));
                Err(())
            }
        }
    }

    /// Lowers an [`ast::TypeName`]s into an [`hir::OutType`].
    ///
    /// If there are any errors, they're pushed to `errors` and `None` is returned.
    fn lower_out_type(
        &mut self,
        ty: &ast::TypeName,
        ltl: &mut impl LifetimeLowerer,
        in_path: &ast::Path,
    ) -> Result<OutType, ()> {
        match ty {
            ast::TypeName::Primitive(prim) => {
                Ok(OutType::Primitive(PrimitiveType::from_ast(*prim)))
            }
            ast::TypeName::Named(path) | ast::TypeName::SelfType(path) => {
                match path.resolve(in_path, self.env) {
                    ast::CustomType::Struct(strct) => {
                        let lifetimes = ltl.lower_generics(&path.lifetimes, ty.is_self());

                        if let Some(tcx_id) = self.lookup_id.resolve_struct(strct) {
                            Ok(OutType::Struct(ReturnableStructPath::Struct(
                                StructPath::new(lifetimes, tcx_id),
                            )))
                        } else if let Some(tcx_id) = self.lookup_id.resolve_out_struct(strct) {
                            Ok(OutType::Struct(ReturnableStructPath::OutStruct(
                                OutStructPath::new(lifetimes, tcx_id),
                            )))
                        } else {
                            unreachable!("struct `{}` wasn't found in the set of structs or out-structs, this is a bug.", strct.name);
                        }
                    }
                    ast::CustomType::Opaque(_) => {
                        self.errors.push(LoweringError::Other(format!(
                            "Opaque passed by value in input: {path}"
                        )));
                        Err(())
                    }
                    ast::CustomType::Enum(enm) => {
                        let tcx_id = self.lookup_id.resolve_enum(enm).expect(
                            "can't find enum in lookup map, which contains all enums from env",
                        );

                        Ok(OutType::Enum(EnumPath::new(tcx_id)))
                    }
                }
            }
            ast::TypeName::Reference(lifetime, mutability, ref_ty) => match ref_ty.as_ref() {
                ast::TypeName::Named(path) | ast::TypeName::SelfType(path) => {
                    match path.resolve(in_path, self.env) {
                        ast::CustomType::Opaque(opaque) => {
                            let borrow = Borrow::new(ltl.lower_lifetime(lifetime), *mutability);
                            let lifetimes = ltl.lower_generics(&path.lifetimes, ref_ty.is_self());
                            let tcx_id = self.lookup_id.resolve_opaque(opaque).expect(
                            "can't find opaque in lookup map, which contains all opaques from env",
                        );

                            Ok(OutType::Opaque(OpaquePath::new(
                                lifetimes,
                                Optional(false),
                                MaybeOwn::Borrow(borrow),
                                tcx_id,
                            )))
                        }
                        _ => {
                            self.errors.push(LoweringError::Other(format!("found &T in output where T is a custom type, but not opaque. T = {ref_ty}")));
                            Err(())
                        }
                    }
                }
                _ => {
                    self.errors.push(LoweringError::Other(format!("found &T in output where T isn't a custom type and therefore not opaque. T = {ref_ty}")));
                    Err(())
                }
            },
            ast::TypeName::Box(box_ty) => match box_ty.as_ref() {
                ast::TypeName::Named(path) | ast::TypeName::SelfType(path) => {
                    match path.resolve(in_path, self.env) {
                        ast::CustomType::Opaque(opaque) => {
                            let lifetimes = ltl.lower_generics(&path.lifetimes, box_ty.is_self());
                            let tcx_id = self.lookup_id.resolve_opaque(opaque).expect(
                            "can't find opaque in lookup map, which contains all opaques from env",
                        );

                            Ok(OutType::Opaque(OpaquePath::new(
                                lifetimes,
                                Optional(false),
                                MaybeOwn::Own,
                                tcx_id,
                            )))
                        }
                        _ => {
                            self.errors.push(LoweringError::Other(format!("found Box<T> in output where T is a custom type but not opaque. non-opaques can't be behind pointers. T = {path}")));
                            Err(())
                        }
                    }
                }
                _ => {
                    self.errors.push(LoweringError::Other(format!(
                        "found Box<T> in output where T isn't a custom type. T = {box_ty}"
                    )));
                    Err(())
                }
            },
            ast::TypeName::Option(opt_ty) => match opt_ty.as_ref() {
                ast::TypeName::Reference(lifetime, mutability, ref_ty) => match ref_ty.as_ref() {
                    ast::TypeName::Named(path) | ast::TypeName::SelfType(path) => {
                        match path.resolve(in_path, self.env) {
                            ast::CustomType::Opaque(opaque) => {
                                let borrow = Borrow::new(ltl.lower_lifetime(lifetime), *mutability);
                                let lifetimes =
                                    ltl.lower_generics(&path.lifetimes, ref_ty.is_self());
                                let tcx_id = self.lookup_id.resolve_opaque(opaque).expect(
                                "can't find opaque in lookup map, which contains all opaques from env",
                            );

                                Ok(OutType::Opaque(OpaquePath::new(
                                    lifetimes,
                                    Optional(true),
                                    MaybeOwn::Borrow(borrow),
                                    tcx_id,
                                )))
                            }
                            _ => {
                                self.errors.push(LoweringError::Other(format!("found Option<&T> where T is a custom type, but it's not opaque. T = {ref_ty}")));
                                Err(())
                            }
                        }
                    }
                    _ => {
                        self.errors.push(LoweringError::Other(format!("found Option<&T>, but T isn't a custom type and therefore not opaque. T = {ref_ty}")));
                        Err(())
                    }
                },
                ast::TypeName::Box(box_ty) => match box_ty.as_ref() {
                    ast::TypeName::Named(path) | ast::TypeName::SelfType(path) => {
                        match path.resolve(in_path, self.env) {
                            ast::CustomType::Opaque(opaque) => {
                                let lifetimes =
                                    ltl.lower_generics(&path.lifetimes, box_ty.is_self());
                                let tcx_id = self.lookup_id.resolve_opaque(opaque).expect(
                            "can't find opaque in lookup map, which contains all opaques from env",
                        );

                                Ok(OutType::Opaque(OpaquePath::new(
                                    lifetimes,
                                    Optional(true),
                                    MaybeOwn::Own,
                                    tcx_id,
                                )))
                            }
                            _ => {
                                self.errors.push(LoweringError::Other(format!("found Option<Box<T>> where T is a custom type, but it's not opaque. T = {box_ty}")));
                                Err(())
                            }
                        }
                    }
                    _ => {
                        self.errors.push(LoweringError::Other(format!("found Option<Box<T>>, but T isn't a custom type and therefore not opaque. T = {box_ty}")));
                        Err(())
                    }
                },
                _ => {
                    self.errors.push(LoweringError::Other(format!("found Option<T>, where T isn't a reference but Option<T> requires that T is a reference to an opaque. T = {opt_ty}")));
                    Err(())
                }
            },
            ast::TypeName::Result(_, _, _) => {
                self.errors.push(LoweringError::Other(
                    "Results can only appear as the top-level return type of methods".into(),
                ));
                Err(())
            }
            ast::TypeName::Writeable => {
                self.errors.push(LoweringError::Other(
                    "Writeables can only appear as the last parameter of a method".into(),
                ));
                Err(())
            }
            ast::TypeName::StrReference(lifetime, encoding) => Ok(OutType::Slice(Slice::Str(
                lifetime.as_ref().map(|l| ltl.lower_lifetime(l)),
                *encoding,
            ))),
            ast::TypeName::PrimitiveSlice(lm, prim) => Ok(OutType::Slice(Slice::Primitive(
                lm.as_ref()
                    .map(|(lt, m)| Borrow::new(ltl.lower_lifetime(lt), *m)),
                PrimitiveType::from_ast(*prim),
            ))),
            ast::TypeName::Unit => {
                self.errors.push(LoweringError::Other("Unit types can only appear as the return value of a method, or as the Ok/Err variants of a returned result".into()));
                Err(())
            }
        }
    }

    /// Lowers an [`ast::SelfParam`] into an [`hir::ParamSelf`].
    ///
    /// If there are any errors, they're pushed to `errors` and `None` is returned.
    fn lower_self_param(
        &mut self,
        self_param: &ast::SelfParam,
        self_param_ltl: SelfParamLifetimeLowerer<'ast>,
        method_full_path: &ast::Ident, // for better error msg
        in_path: &ast::Path,
    ) -> Result<(ParamSelf, ParamLifetimeLowerer<'ast>), ()> {
        match self_param.path_type.resolve(in_path, self.env) {
            ast::CustomType::Struct(strct) => {
                if let Some(tcx_id) = self.lookup_id.resolve_struct(strct) {
                    if self_param.reference.is_some() {
                        self.errors.push(LoweringError::Other(format!("Method `{method_full_path}` takes a reference to a struct as a self parameter, which isn't allowed")));
                        Err(())
                    } else {
                        let mut param_ltl = self_param_ltl.no_self_ref();

                        // Even if we explicitly write out the type of `self` like
                        // `self: Foo<'a>`, the `'a` is still not considered for
                        // elision according to rustc, so is_self=true.
                        let type_lifetimes =
                            param_ltl.lower_generics(&self_param.path_type.lifetimes[..], true);

                        Ok((
                            ParamSelf::new(SelfType::Struct(StructPath::new(
                                type_lifetimes,
                                tcx_id,
                            ))),
                            param_ltl,
                        ))
                    }
                } else if self.lookup_id.resolve_out_struct(strct).is_some() {
                    if let Some((lifetime, _)) = &self_param.reference {
                        self.errors.push(LoweringError::Other(format!("Method `{method_full_path}` takes an out-struct as the self parameter, which isn't allowed. Also, it's behind a reference, `{lifetime}`, but only opaques can be behind references")));
                        Err(())
                    } else {
                        self.errors.push(LoweringError::Other(format!("Method `{method_full_path}` takes an out-struct as the self parameter, which isn't allowed")));
                        Err(())
                    }
                } else {
                    unreachable!(
                    "struct `{}` wasn't found in the set of structs or out-structs, this is a bug.",
                    strct.name
                );
                }
            }
            ast::CustomType::Opaque(opaque) => {
                let tcx_id = self
                    .lookup_id
                    .resolve_opaque(opaque)
                    .expect("opaque is in env");

                if let Some((lifetime, mutability)) = &self_param.reference {
                    let (borrow_lifetime, mut param_ltl) = self_param_ltl.lower_self_ref(lifetime);
                    let borrow = Borrow::new(borrow_lifetime, *mutability);
                    let lifetimes = param_ltl.lower_generics(&self_param.path_type.lifetimes, true);

                    Ok((
                        ParamSelf::new(SelfType::Opaque(OpaquePath::new(
                            lifetimes,
                            NonOptional,
                            borrow,
                            tcx_id,
                        ))),
                        param_ltl,
                    ))
                } else {
                    self.errors.push(LoweringError::Other(format!("Method `{method_full_path}` takes an opaque by value as the self parameter, but opaques as inputs must be behind refs")));
                    Err(())
                }
            }
            ast::CustomType::Enum(enm) => {
                let tcx_id = self.lookup_id.resolve_enum(enm).expect("enum is in env");

                Ok((
                    ParamSelf::new(SelfType::Enum(EnumPath::new(tcx_id))),
                    self_param_ltl.no_self_ref(),
                ))
            }
        }
    }

    /// Lowers an [`ast::Param`] into an [`hir::Param`].
    ///
    /// If there are any errors, they're pushed to `errors` and `None` is returned.
    ///
    /// Note that this expects that if there was a writeable param at the end in
    /// the method, it's not passed into here.
    fn lower_param(
        &mut self,
        param: &ast::Param,
        ltl: &mut impl LifetimeLowerer,
        in_path: &ast::Path,
    ) -> Result<Param, ()> {
        let name = self.lower_ident(&param.name, "param name");
        let ty = self.lower_type(&param.ty, ltl, in_path);

        Ok(Param::new(name?, ty?))
    }

    /// Lowers many [`ast::Param`]s into a vector of [`hir::Param`]s.
    ///
    /// If there are any errors, they're pushed to `errors` and `None` is returned.
    ///
    /// Note that this expects that if there was a writeable param at the end in
    /// the method, `ast_params` was sliced to not include it. This happens in
    /// `self.lower_method`, the caller of this function.
    fn lower_many_params(
        &mut self,
        ast_params: &[ast::Param],
        mut param_ltl: ParamLifetimeLowerer<'ast>,
        in_path: &ast::Path,
    ) -> Result<(Vec<Param>, ReturnLifetimeLowerer<'ast>), ()> {
        let mut params = Ok(Vec::with_capacity(ast_params.len()));

        for param in ast_params {
            let param = self.lower_param(param, &mut param_ltl, in_path);

            match (param, &mut params) {
                (Ok(param), Ok(params)) => {
                    params.push(param);
                }
                _ => params = Err(()),
            }
        }

        Ok((params?, param_ltl.into_return_ltl()))
    }

    /// Lowers the return type of an [`ast::Method`] into a [`hir::ReturnFallability`].
    ///
    /// If there are any errors, they're pushed to `errors` and `None` is returned.
    fn lower_return_type(
        &mut self,
        return_type: Option<&ast::TypeName>,
        takes_writeable: bool,
        mut return_ltl: ReturnLifetimeLowerer<'_>,
        in_path: &ast::Path,
    ) -> Result<(ReturnType, LifetimeEnv), ()> {
        let writeable_or_unit = if takes_writeable {
            SuccessType::Writeable
        } else {
            SuccessType::Unit
        };
        match return_type.unwrap_or(&ast::TypeName::Unit) {
            ast::TypeName::Result(ok_ty, err_ty, _) => {
                let ok_ty = match ok_ty.as_ref() {
                    ast::TypeName::Unit => Ok(writeable_or_unit),
                    ty => self
                        .lower_out_type(ty, &mut return_ltl, in_path)
                        .map(SuccessType::OutType),
                };
                let err_ty = match err_ty.as_ref() {
                    ast::TypeName::Unit => Ok(None),
                    ty => self.lower_out_type(ty, &mut return_ltl, in_path).map(Some),
                };

                match (ok_ty, err_ty) {
                    (Ok(ok_ty), Ok(err_ty)) => Ok(ReturnType::Fallible(ok_ty, err_ty)),
                    _ => Err(()),
                }
            }
            ty @ ast::TypeName::Option(value_ty) => match &**value_ty {
                ast::TypeName::Box(..) | ast::TypeName::Reference(..) => self
                    .lower_out_type(ty, &mut return_ltl, in_path)
                    .map(SuccessType::OutType)
                    .map(ReturnType::Infallible),
                _ => self
                    .lower_out_type(value_ty, &mut return_ltl, in_path)
                    .map(SuccessType::OutType)
                    .map(ReturnType::Nullable),
            },
            ast::TypeName::Unit => Ok(ReturnType::Infallible(writeable_or_unit)),
            ty => self
                .lower_out_type(ty, &mut return_ltl, in_path)
                .map(|ty| ReturnType::Infallible(SuccessType::OutType(ty))),
        }
        .map(|r_ty| (r_ty, return_ltl.finish()))
    }

    fn lower_named_lifetime(
        &mut self,
        lifetime: &ast::lifetimes::LifetimeNode,
    ) -> Result<BoundedLifetime, ()> {
        Ok(BoundedLifetime {
            ident: self.lower_ident(lifetime.lifetime.name(), "lifetime")?,
            longer: lifetime.longer.iter().copied().map(Lifetime::new).collect(),
            shorter: lifetime
                .shorter
                .iter()
                .copied()
                .map(Lifetime::new)
                .collect(),
        })
    }

    /// Lowers a lifetime env found on a type
    ///
    /// Should not be extended to return LifetimeEnv<Method>, which needs to use the lifetime
    /// lowerers to handle elision.
    fn lower_type_lifetime_env(&mut self, ast: &ast::LifetimeEnv) -> Result<LifetimeEnv, ()> {
        let nodes = ast
            .nodes
            .iter()
            .map(|lt| self.lower_named_lifetime(lt))
            .collect::<Result<_, ()>>()?;

        Ok(LifetimeEnv::new(nodes, ast.nodes.len()))
    }
}
