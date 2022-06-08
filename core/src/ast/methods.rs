use serde::{Deserialize, Serialize};
use std::ops::ControlFlow;

use super::docs::Docs;
use super::lifetimes::{Lifetime, LifetimeEnv};
use super::{Ident, Mutability, Path, PathType, TypeName, ValidityError};
use crate::Env;

/// A method declared in the `impl` associated with an FFI struct.
/// Includes both static and non-static methods, which can be distinguished
/// by inspecting [`Method::self_param`].
#[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
pub struct Method {
    /// The name of the method as initially declared.
    pub name: Ident,

    /// Lines of documentation for the method.
    pub docs: Docs,

    /// The name of the FFI function wrapping around the method.
    pub full_path_name: Ident,

    /// The `self` param of the method, if any.
    pub self_param: Option<SelfParam>,

    /// All non-`self` params taken by the method.
    pub params: Vec<Param>,

    /// The return type of the method, if any.
    pub return_type: Option<TypeName>,

    /// The lifetimes introduced in this method and surrounding impl block.
    pub lifetime_env: LifetimeEnv,
}

impl Method {
    /// Extracts a [`Method`] from an AST node inside an `impl`.
    pub fn from_syn(
        m: &syn::ImplItemMethod,
        self_path_type: PathType,
        impl_generics: Option<&syn::Generics>,
    ) -> Method {
        let self_ident = self_path_type.path.elements.last().unwrap();
        let method_ident = &m.sig.ident;
        let extern_ident = syn::Ident::new(
            format!("{}_{}", self_ident, method_ident).as_str(),
            m.sig.ident.span(),
        );

        let mut lifetime_env = LifetimeEnv::from(&m.sig.generics);
        if let Some(generics) = impl_generics {
            lifetime_env.extend_generics(generics);
        }

        let all_params = m
            .sig
            .inputs
            .iter()
            .filter_map(|a| match a {
                syn::FnArg::Receiver(_) => None,
                syn::FnArg::Typed(ref t) => Some(Param::from_syn(t, self_path_type.clone())),
            })
            .collect::<Vec<_>>();

        let self_param = m.sig.receiver().map(|rec| match rec {
            syn::FnArg::Receiver(ref rec) => SelfParam::from_syn(rec, self_path_type.clone()),
            _ => panic!("Unexpected self param type"),
        });

        let return_ty = match &m.sig.output {
            syn::ReturnType::Type(_, return_typ) => {
                // When we allow lifetime elision, this is where we would want to
                // support it so we can insert the expanded explicit lifetimes.
                Some(TypeName::from_syn(
                    return_typ.as_ref(),
                    Some(self_path_type),
                ))
            }
            syn::ReturnType::Default => None,
        };

        Method {
            name: Ident::from(method_ident),
            docs: Docs::from_attrs(&m.attrs),
            full_path_name: Ident::from(&extern_ident),
            self_param,
            params: all_params,
            return_type: return_ty,
            lifetime_env,
        }
    }

    /// Returns the parameters that the output is lifetime-bound to.
    ///
    /// # Examples
    ///
    /// Given the following method:
    /// ```ignore
    /// fn foo<'a, 'b: 'a, 'c>(&'a self, bar: Bar<'b>, baz: Baz<'c>) -> FooBar<'a> { ... }
    /// ```
    /// Then this method would return the `&'a self` and `bar: Bar<'b>` params
    /// because `'a` is in the return type, and `'b` must live at least as long
    /// as `'a`. It wouldn't include `baz: Baz<'c>` though, because the return
    /// type isn't bound by `'c` in any way.
    ///
    /// # Panics
    ///
    /// This method may panic if `TypeName::check_result_type_validity` (called by
    /// `Method::check_validity`) doesn't pass first, since the result type may
    /// contain elided lifetimes that we depend on for this method. The validity
    /// checks ensure that the return type doesn't elide any lifetimes, ensuring
    /// that this method will produce correct results.
    pub fn params_held_by_output(&self) -> (Option<&SelfParam>, Vec<&Param>) {
        // To determine which params the return type is bound to, we just have to
        // find the params that contain a lifetime that's also in the return type.
        if let Some(ref return_type) = self.return_type {
            // The lifetimes that must outlive the return type
            let lifetimes = {
                // We can think about each lifetime as a node in a directed
                // graph, where each edge represents the "doesn't outlive"
                // relationship. Note that this doesn't strictly mean "outlived by",
                // since you can write `'a: 'a`, which is perfectly valid. Typically,
                // we can think about how `'a` pointing to `'b` would represent
                // `'b: 'a`, since `'a` doesn't outlive `'b`. Since lifetimes are
                // transitive, then for all `'a`, `'b`, there's a path of some
                // length from `'a` to `'b` if and only if `'a` doesn't outlive `'b`.
                //
                // To determine which lifetimes outlive the return type, we can
                // use this property by performing DFS to record all lifetimes
                // reachable from some lifetime of the return type.
                // In traditional DFS, you start with one root node in the stack,
                // but the return type could have more than one lifetime that we
                // want to explore. Since we only care about which lifetimes are
                // reachable, the path is irrelevant. Therefore, we can use the
                // _same stack_ to DFS on potentially multiple directed graphs
                // at the _same time_.
                //
                // Since we want to traverse lifetimes that live at least as long
                // as the return type, we just remove edges that we've already
                // traversed, allowing us to break cycles before getting stuck in
                // them forever, while still fully exploring all lifetimes that
                // outlive the return type.
                let mut root_lifetimes = vec![];

                // Push the root node(s) to the stack.
                return_type.visit_lifetimes(&mut |lifetime| -> ControlFlow<()> {
                    match lifetime {
                        Lifetime::Named(named) => root_lifetimes.push(named),
                        Lifetime::Anonymous => {
                            // TODO(160): We can remove this once resolved.
                            // This is also caught in the validity check, so this should
                            // never happen. If we're guaranteed to always run the
                            // check first, we can change this to `unreachable!()`.
                            panic!("Anonymous lifetimes not yet allowed in return types")
                        }
                        Lifetime::Static => {
                            // If the output depends on the static lifetime, this
                            // tells us nothing about what params it might need,
                            // so do nothing.
                        }
                    }
                    ControlFlow::Continue(())
                });

                self.lifetime_env.outlives(root_lifetimes)
            };

            let held_self_param = self.self_param.as_ref().filter(|self_param| {
                // Check if `self` is a reference with a lifetime in the return type.
                if let Some((Lifetime::Named(ref name), _)) = self_param.reference {
                    if lifetimes.contains(&name) {
                        return true;
                    }
                }
                self_param.path_type.lifetimes.iter().any(|lt| {
                    if let Lifetime::Named(name) = lt {
                        lifetimes.contains(&name)
                    } else {
                        false
                    }
                })
            });

            // Collect all the params that contain a named lifetime that's also
            // in the return type.
            let held_params = self
                .params
                .iter()
                .filter(|param| {
                    param
                        .ty
                        .visit_lifetimes(&mut |lt| {
                            // Thanks to `TypeName::visit_lifetimes`, we can
                            // traverse the lifetimes without allocations and
                            // short-circuit if we find a match.
                            if let Lifetime::Named(name) = lt {
                                if lifetimes.contains(&name) {
                                    return ControlFlow::Break(());
                                }
                            }
                            ControlFlow::Continue(())
                        })
                        .is_break()
                })
                .collect();

            (held_self_param, held_params)
        } else {
            Default::default()
        }
    }

    /// Performs type-specific validity checks (see [TypeName::check_validity()])
    pub fn check_validity<'a>(
        &'a self,
        in_path: &Path,
        env: &Env,
        errors: &mut Vec<ValidityError>,
    ) {
        // validity check that if the self type is nonopaque, that it is
        // behind a reference
        if let Some(ref self_param) = self.self_param {
            self_param
                .to_typename()
                .check_validity(in_path, env, errors);
        }
        for m in self.params.iter() {
            m.ty.check_validity(in_path, env, errors);
        }
        if let Some(ref t) = self.return_type {
            t.check_return_type_validity(in_path, env, errors);
        }
    }

    /// Checks whether the method qualifies for special writeable handling.
    /// To qualify, a method must:
    ///  - not return any value
    ///  - have the last argument be an `&mut diplomat_runtime::DiplomatWriteable`
    ///
    /// Typically, methods of this form will be transformed in the bindings to a
    /// method that doesn't take the writeable as an argument but instead creates
    /// one locally and just returns the final string.
    pub fn is_writeable_out(&self) -> bool {
        let return_compatible = self
            .return_type
            .as_ref()
            .map(|return_type| match return_type {
                TypeName::Unit => true,
                TypeName::Result(ok, _) => matches!(ok.as_ref(), TypeName::Unit),
                _ => false,
            })
            .unwrap_or(true);

        return_compatible && self.params.last().map(Param::is_writeable).unwrap_or(false)
    }

    /// Checks if any parameters are writeable (regardless of other compatibilities for writeable output)
    pub fn has_writeable_param(&self) -> bool {
        self.params.iter().any(|p| p.is_writeable())
    }
}

/// The `self` parameter taken by a [`Method`].
#[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
pub struct SelfParam {
    /// The lifetime and mutability of the `self` param, if it's a reference.
    pub reference: Option<(Lifetime, Mutability)>,

    /// The type of the parameter, which will be a named reference to
    /// the associated struct,
    pub path_type: PathType,
}

impl SelfParam {
    pub fn to_typename(&self) -> TypeName {
        let typ = TypeName::Named(self.path_type.clone());
        if let Some((ref lifetime, ref mutability)) = self.reference {
            return TypeName::Reference(lifetime.clone(), mutability.clone(), Box::new(typ));
        }
        typ
    }

    pub fn from_syn(rec: &syn::Receiver, path_type: PathType) -> Self {
        SelfParam {
            reference: rec
                .reference
                .as_ref()
                .map(|(_, lt)| (lt.into(), Mutability::from_syn(&rec.mutability))),
            path_type,
        }
    }
}

/// A parameter taken by a [`Method`], not including `self`.
#[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
pub struct Param {
    /// The name of the parameter in the original method declaration.
    pub name: Ident,

    /// The type of the parameter.
    pub ty: TypeName,
}

impl Param {
    /// Check if this parameter is a Writeable
    pub fn is_writeable(&self) -> bool {
        match self.ty {
            TypeName::Reference(_, Mutability::Mutable, ref w) => **w == TypeName::Writeable,
            _ => false,
        }
    }

    pub fn from_syn(t: &syn::PatType, self_path_type: PathType) -> Self {
        let ident = match t.pat.as_ref() {
            syn::Pat::Ident(ident) => ident,
            _ => panic!("Unexpected param type"),
        };

        Param {
            name: (&ident.ident).into(),
            ty: TypeName::from_syn(&t.ty, Some(self_path_type)),
        }
    }
}

#[cfg(test)]
mod tests {
    use insta;

    use syn;

    use crate::ast::Ident;

    use super::{Method, Path, PathType};

    #[test]
    fn static_methods() {
        insta::assert_yaml_snapshot!(Method::from_syn(
            &syn::parse_quote! {
                /// Some docs.
                #[diplomat::rust_link(foo::Bar::batz, FnInStruct)]
                fn foo(x: u64, y: MyCustomStruct) {

                }
            },
            PathType::new(Path::empty().sub_path(Ident::from("MyStructContainingMethod"))),
            None,
        ));

        insta::assert_yaml_snapshot!(Method::from_syn(
            &syn::parse_quote! {
                /// Some docs.
                /// Some more docs.
                ///
                /// Even more docs.
                #[diplomat::rust_link(foo::Bar::batz, FnInEnum)]
                fn foo(x: u64, y: MyCustomStruct) -> u64 {
                    x
                }
            },
            PathType::new(Path::empty().sub_path(Ident::from("MyStructContainingMethod"))),
            None,
        ));
    }

    #[test]
    fn nonstatic_methods() {
        insta::assert_yaml_snapshot!(Method::from_syn(
            &syn::parse_quote! {
                fn foo(&self, x: u64, y: MyCustomStruct) {

                }
            },
            PathType::new(Path::empty().sub_path(Ident::from("MyStructContainingMethod"))),
            None,
        ));

        insta::assert_yaml_snapshot!(Method::from_syn(
            &syn::parse_quote! {
                #[diplomat::rust_link(foo::Bar::batz, FnInStruct)]
                fn foo(&mut self, x: u64, y: MyCustomStruct) -> u64 {
                    x
                }
            },
            PathType::new(Path::empty().sub_path(Ident::from("MyStructContainingMethod"))),
            None,
        ));
    }

    macro_rules! assert_params_held_by_output {
        ([self $(,$($param:ident),+)?] => $($tokens:tt)* ) => {{
            let method = Method::from_syn(
                &syn::parse_quote! { $($tokens)* },
                PathType::new(Path::empty().sub_path(Ident::from("MyStructContainingMethod"))),
                None,
            );

            let (self_param, params) = method.params_held_by_output();
            assert!(self_param.is_some(), "expected `self` param to be held");

            let actual: Vec<&str> = params
                .iter()
                .map(|p| p.name.as_str())
                .collect();

            let expected: &[&str] = &[$($(stringify!($param)),*)?];

            assert_eq!(actual, expected);
        }};
        ([$($param:ident),*] => $($tokens:tt)* ) => {{
            let method = Method::from_syn(
                &syn::parse_quote! { $($tokens)* },
                PathType::new(Path::empty().sub_path(Ident::from("MyStructContainingMethod"))),
                None,
            );

            let (self_param, params) = method.params_held_by_output();
            assert!(self_param.is_none(), "didn't expect `self` param to be held");

            let actual = params
                .iter()
                .map(|p| p.name.as_str())
                .collect::<Vec<_>>();

            let expected: &[&str] = &[$(stringify!($param)),*];

            assert_eq!(actual, expected);
        }};
    }

    #[test]
    fn static_params_held_by_return_type() {
        assert_params_held_by_output! { [first, second] =>
            #[diplomat::rust_link(foo::Bar::batz, FnInStruct)]
            fn foo<'a, 'b>(first: &'a First, second: &'b Second, third: &Third) -> Foo<'a, 'b> {
                unimplemented!()
            }
        }

        assert_params_held_by_output! { [hold] =>
            #[diplomat::rust_link(Foo, FnInStruct)]
            fn transitivity<'a, 'b: 'a, 'c: 'b, 'd: 'c, 'e: 'd, 'x>(hold: &'x One<'e>, nohold: &One<'x>) -> Box<Foo<'a>> {
                unimplemented!()
            }
        }

        assert_params_held_by_output! { [hold] =>
            #[diplomat::rust_link(Foo, FnInStruct)]
            fn a_le_b_and_b_le_a<'a: 'b, 'b: 'a>(hold: &'b Bar, nohold: &'c Bar) -> Box<Foo<'a>> {
                unimplemented!()
            }
        }

        assert_params_held_by_output! { [a, b, c, d] =>
            #[diplomat::rust_link(Foo, FnInStruct)]
            fn many_dependents<'a, 'b: 'a, 'c: 'a, 'd: 'b, 'x, 'y>(a: &'x One<'a>, b: &'b One<'x>, c: &Two<'x, 'c>, d: &'x Two<'d, 'y>, nohold: &'x Two<'x, 'y>) -> Box<Foo<'a>> {
                unimplemented!()
            }
        }

        assert_params_held_by_output! { [hold] =>
            #[diplomat::rust_link(Foo, FnInStruct)]
            fn return_outlives_param<'short, 'long: 'short>(hold: &Two<'long, 'short>, nohold: &'short One<'short>) -> Box<Foo<'long>> {
                unimplemented!()
            }
        }

        assert_params_held_by_output! { [hold] =>
            #[diplomat::rust_link(Foo, FnInStruct)]
            fn transitivity_deep_types<'a, 'b: 'a, 'c: 'b, 'd: 'c>(hold: Option<Box<Bar<'d>>>, nohold: &'a Box<Option<Baz<'a>>>) -> DiplomatResult<Box<Foo<'b>>, Error> {
                unimplemented!()
            }
        }

        assert_params_held_by_output! { [top, left, right, bottom] =>
            #[diplomat::rust_link(Foo, FnInStruct)]
            fn diamond_top<'top, 'left: 'top, 'right: 'top, 'bottom: 'left + 'right>(top: One<'top>, left: One<'left>, right: One<'right>, bottom: One<'bottom>) -> Box<Foo<'top>> {
                unimplemented!()
            }
        }

        assert_params_held_by_output! { [left, bottom] =>
            #[diplomat::rust_link(Foo, FnInStruct)]
            fn diamond_left<'top, 'left: 'top, 'right: 'top, 'bottom: 'left + 'right>(top: One<'top>, left: One<'left>, right: One<'right>, bottom: One<'bottom>) -> Box<Foo<'left>> {
                unimplemented!()
            }
        }

        assert_params_held_by_output! { [right, bottom] =>
            #[diplomat::rust_link(Foo, FnInStruct)]
            fn diamond_right<'top, 'left: 'top, 'right: 'top, 'bottom: 'left + 'right>(top: One<'top>, left: One<'left>, right: One<'right>, bottom: One<'bottom>) -> Box<Foo<'right>> {
                unimplemented!()
            }
        }

        assert_params_held_by_output! { [bottom] =>
            #[diplomat::rust_link(Foo, FnInStruct)]
            fn diamond_bottom<'top, 'left: 'top, 'right: 'top, 'bottom: 'left + 'right>(top: One<'top>, left: One<'left>, right: One<'right>, bottom: One<'bottom>) -> Box<Foo<'bottom>> {
                unimplemented!()
            }
        }

        assert_params_held_by_output! { [a, b, c, d] =>
            #[diplomat::rust_link(Foo, FnInStruct)]
            fn diamond_and_nested_types<'b: 'a, 'c: 'b, 'd: 'b + 'c, 'x, 'y>(a: &'x One<'a>, b: &'y One<'b>, c: &One<'c>, d: &One<'d>, nohold: &One<'x>) -> Box<Foo<'a>> {
                unimplemented!()
            }
        }
    }

    #[test]
    fn nonstatic_params_held_by_return_type() {
        assert_params_held_by_output! { [self] =>
            #[diplomat::rust_link(foo::Bar::batz, FnInStruct)]
            fn foo<'a>(&'a self) -> Foo<'a> {
                unimplemented!()
            }
        }

        assert_params_held_by_output! { [self, foo, bar] =>
            #[diplomat::rust_link(foo::Bar::batz, FnInStruct)]
            fn foo<'x, 'y>(&'x self, foo: &'x Foo, bar: &Bar<'y>, baz: &Baz) -> Foo<'x, 'y> {
                unimplemented!()
            }
        }

        assert_params_held_by_output! { [self, bar] =>
            #[diplomat::rust_link(foo::Bar::batz, FnInStruct)]
            fn foo<'a, 'b>(&'a self, bar: Bar<'b>) -> Foo<'a, 'b> {
                unimplemented!()
            }
        }

        // Test that being dependent on 'static doesn't make you dependent on 'static params.
        assert_params_held_by_output! { [self, bar] =>
            #[diplomat::rust_link(foo::Bar::batz, FnInStruct)]
            fn foo<'a, 'b>(&'a self, bar: Bar<'b>, baz: &'static str) -> Foo<'a, 'b, 'static> {
                unimplemented!()
            }
        }
    }
}
