Opaque(OpaqueDef { docs: Docs("", []), name: Check { _marker: PhantomData, buf: "MyString" }, methods: [Method { docs: Docs("", []), name: Check { _marker: PhantomData, buf: "new" }, lifetime_env: LifetimeEnv { nodes: [], num_lifetimes: 1 }, param_self: None, params: [Param { name: Check { _marker: PhantomData, buf: "v" }, ty: Slice(Str(NonStatic(TypeLifetime(0)))) }], output: Infallible(Some(OutType(Opaque(OpaquePath { lifetimes: TypeLifetimes { indices: [] }, optional: Optional(true), owner: Own, tcx_id: OpaqueId(10) })))) }, Method { docs: Docs("", []), name: Check { _marker: PhantomData, buf: "set_str" }, lifetime_env: LifetimeEnv { nodes: [], num_lifetimes: 2 }, param_self: Some(ParamSelf { ty: Opaque(OpaquePath { lifetimes: TypeLifetimes { indices: [] }, optional: NonOptional, owner: Borrow { lifetime: NonStatic(TypeLifetime(0)), mutability: Mutable }, tcx_id: OpaqueId(10) }) }), params: [Param { name: Check { _marker: PhantomData, buf: "new_str" }, ty: Slice(Str(NonStatic(TypeLifetime(1)))) }], output: Infallible(None) }, Method { docs: Docs("", []), name: Check { _marker: PhantomData, buf: "get_str" }, lifetime_env: LifetimeEnv { nodes: [], num_lifetimes: 1 }, param_self: Some(ParamSelf { ty: Opaque(OpaquePath { lifetimes: TypeLifetimes { indices: [] }, optional: NonOptional, owner: Borrow { lifetime: NonStatic(TypeLifetime(0)), mutability: Immutable }, tcx_id: OpaqueId(10) }) }), params: [], output: Infallible(Some(Writeable)) }] })