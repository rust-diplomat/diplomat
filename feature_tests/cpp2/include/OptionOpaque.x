Opaque(OpaqueDef { docs: Docs("", []), name: Check { _marker: PhantomData, buf: "OptionOpaque" }, methods: [Method { docs: Docs("", []), name: Check { _marker: PhantomData, buf: "new" }, lifetime_env: LifetimeEnv { nodes: [], num_lifetimes: 0 }, param_self: None, params: [Param { name: Check { _marker: PhantomData, buf: "i" }, ty: Primitive(Int(I32)) }], output: Infallible(Some(OutType(Opaque(OpaquePath { lifetimes: TypeLifetimes { indices: [] }, optional: Optional(true), owner: Own, tcx_id: OpaqueId(4) })))) }, Method { docs: Docs("", []), name: Check { _marker: PhantomData, buf: "new_none" }, lifetime_env: LifetimeEnv { nodes: [], num_lifetimes: 0 }, param_self: None, params: [], output: Infallible(Some(OutType(Opaque(OpaquePath { lifetimes: TypeLifetimes { indices: [] }, optional: Optional(true), owner: Own, tcx_id: OpaqueId(4) })))) }, Method { docs: Docs("", []), name: Check { _marker: PhantomData, buf: "new_struct" }, lifetime_env: LifetimeEnv { nodes: [], num_lifetimes: 0 }, param_self: None, params: [], output: Infallible(Some(OutType(Struct(OutStruct(StructPath { lifetimes: TypeLifetimes { indices: [] }, tcx_id: OutStructId(0) }))))) }, Method { docs: Docs("", []), name: Check { _marker: PhantomData, buf: "new_struct_nones" }, lifetime_env: LifetimeEnv { nodes: [], num_lifetimes: 0 }, param_self: None, params: [], output: Infallible(Some(OutType(Struct(OutStruct(StructPath { lifetimes: TypeLifetimes { indices: [] }, tcx_id: OutStructId(0) }))))) }, Method { docs: Docs("", []), name: Check { _marker: PhantomData, buf: "assert_integer" }, lifetime_env: LifetimeEnv { nodes: [], num_lifetimes: 1 }, param_self: Some(ParamSelf { ty: Opaque(OpaquePath { lifetimes: TypeLifetimes { indices: [] }, optional: NonOptional, owner: Borrow { lifetime: NonStatic(TypeLifetime(0)), mutability: Immutable }, tcx_id: OpaqueId(4) }) }), params: [Param { name: Check { _marker: PhantomData, buf: "i" }, ty: Primitive(Int(I32)) }], output: Infallible(None) }] })