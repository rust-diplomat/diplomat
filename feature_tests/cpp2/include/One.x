Opaque(OpaqueDef { docs: Docs("", []), name: Check { _marker: PhantomData, buf: "One" }, methods: [Method { docs: Docs("", []), name: Check { _marker: PhantomData, buf: "transitivity" }, lifetime_env: LifetimeEnv { nodes: [Lifetime { ident: Check { _marker: PhantomData, buf: "o" }, longer: [], shorter: [] }, Lifetime { ident: Check { _marker: PhantomData, buf: "a" }, longer: [MethodLifetime(2)], shorter: [] }, Lifetime { ident: Check { _marker: PhantomData, buf: "b" }, longer: [MethodLifetime(3)], shorter: [MethodLifetime(1)] }, Lifetime { ident: Check { _marker: PhantomData, buf: "c" }, longer: [MethodLifetime(4)], shorter: [MethodLifetime(2)] }, Lifetime { ident: Check { _marker: PhantomData, buf: "d" }, longer: [MethodLifetime(5)], shorter: [MethodLifetime(3)] }, Lifetime { ident: Check { _marker: PhantomData, buf: "e" }, longer: [], shorter: [MethodLifetime(4), MethodLifetime(6)] }, Lifetime { ident: Check { _marker: PhantomData, buf: "x" }, longer: [MethodLifetime(5)], shorter: [] }], num_lifetimes: 8 }, param_self: None, params: [Param { name: Check { _marker: PhantomData, buf: "hold" }, ty: Opaque(OpaquePath { lifetimes: TypeLifetimes { indices: [NonStatic(TypeLifetime(5))] }, optional: Optional(false), owner: Borrow { lifetime: NonStatic(TypeLifetime(6)), mutability: Immutable }, tcx_id: OpaqueId(2) }) }, Param { name: Check { _marker: PhantomData, buf: "nohold" }, ty: Opaque(OpaquePath { lifetimes: TypeLifetimes { indices: [NonStatic(TypeLifetime(6))] }, optional: Optional(false), owner: Borrow { lifetime: NonStatic(TypeLifetime(7)), mutability: Immutable }, tcx_id: OpaqueId(2) }) }], output: Infallible(Some(OutType(Opaque(OpaquePath { lifetimes: TypeLifetimes { indices: [NonStatic(TypeLifetime(1))] }, optional: Optional(true), owner: Own, tcx_id: OpaqueId(2) })))) }, Method { docs: Docs("", []), name: Check { _marker: PhantomData, buf: "cycle" }, lifetime_env: LifetimeEnv { nodes: [Lifetime { ident: Check { _marker: PhantomData, buf: "o" }, longer: [], shorter: [] }, Lifetime { ident: Check { _marker: PhantomData, buf: "a" }, longer: [MethodLifetime(3)], shorter: [MethodLifetime(2)] }, Lifetime { ident: Check { _marker: PhantomData, buf: "b" }, longer: [MethodLifetime(1)], shorter: [MethodLifetime(3)] }, Lifetime { ident: Check { _marker: PhantomData, buf: "c" }, longer: [MethodLifetime(2)], shorter: [MethodLifetime(1)] }, Lifetime { ident: Check { _marker: PhantomData, buf: "x" }, longer: [], shorter: [] }], num_lifetimes: 6 }, param_self: None, params: [Param { name: Check { _marker: PhantomData, buf: "hold" }, ty: Opaque(OpaquePath { lifetimes: TypeLifetimes { indices: [NonStatic(TypeLifetime(4)), NonStatic(TypeLifetime(2))] }, optional: Optional(false), owner: Borrow { lifetime: NonStatic(TypeLifetime(5)), mutability: Immutable }, tcx_id: OpaqueId(3) }) }, Param { name: Check { _marker: PhantomData, buf: "nohold" }, ty: Opaque(OpaquePath { lifetimes: TypeLifetimes { indices: [NonStatic(TypeLifetime(4))] }, optional: Optional(false), owner: Borrow { lifetime: NonStatic(TypeLifetime(4)), mutability: Immutable }, tcx_id: OpaqueId(2) }) }], output: Infallible(Some(OutType(Opaque(OpaquePath { lifetimes: TypeLifetimes { indices: [NonStatic(TypeLifetime(1))] }, optional: Optional(true), owner: Own, tcx_id: OpaqueId(2) })))) }, Method { docs: Docs("", []), name: Check { _marker: PhantomData, buf: "many_dependents" }, lifetime_env: LifetimeEnv { nodes: [Lifetime { ident: Check { _marker: PhantomData, buf: "o" }, longer: [], shorter: [] }, Lifetime { ident: Check { _marker: PhantomData, buf: "a" }, longer: [MethodLifetime(2), MethodLifetime(3)], shorter: [MethodLifetime(5), MethodLifetime(2)] }, Lifetime { ident: Check { _marker: PhantomData, buf: "b" }, longer: [MethodLifetime(4), MethodLifetime(1)], shorter: [MethodLifetime(1)] }, Lifetime { ident: Check { _marker: PhantomData, buf: "c" }, longer: [], shorter: [MethodLifetime(1)] }, Lifetime { ident: Check { _marker: PhantomData, buf: "d" }, longer: [], shorter: [MethodLifetime(2)] }, Lifetime { ident: Check { _marker: PhantomData, buf: "x" }, longer: [MethodLifetime(1), MethodLifetime(6)], shorter: [] }, Lifetime { ident: Check { _marker: PhantomData, buf: "y" }, longer: [], shorter: [MethodLifetime(5)] }], num_lifetimes: 8 }, param_self: None, params: [Param { name: Check { _marker: PhantomData, buf: "a" }, ty: Opaque(OpaquePath { lifetimes: TypeLifetimes { indices: [NonStatic(TypeLifetime(1))] }, optional: Optional(false), owner: Borrow { lifetime: NonStatic(TypeLifetime(5)), mutability: Immutable }, tcx_id: OpaqueId(2) }) }, Param { name: Check { _marker: PhantomData, buf: "b" }, ty: Opaque(OpaquePath { lifetimes: TypeLifetimes { indices: [NonStatic(TypeLifetime(1))] }, optional: Optional(false), owner: Borrow { lifetime: NonStatic(TypeLifetime(2)), mutability: Immutable }, tcx_id: OpaqueId(2) }) }, Param { name: Check { _marker: PhantomData, buf: "c" }, ty: Opaque(OpaquePath { lifetimes: TypeLifetimes { indices: [NonStatic(TypeLifetime(5)), NonStatic(TypeLifetime(3))] }, optional: Optional(false), owner: Borrow { lifetime: NonStatic(TypeLifetime(7)), mutability: Immutable }, tcx_id: OpaqueId(3) }) }, Param { name: Check { _marker: PhantomData, buf: "d" }, ty: Opaque(OpaquePath { lifetimes: TypeLifetimes { indices: [NonStatic(TypeLifetime(4)), NonStatic(TypeLifetime(6))] }, optional: Optional(false), owner: Borrow { lifetime: NonStatic(TypeLifetime(5)), mutability: Immutable }, tcx_id: OpaqueId(3) }) }, Param { name: Check { _marker: PhantomData, buf: "nohold" }, ty: Opaque(OpaquePath { lifetimes: TypeLifetimes { indices: [NonStatic(TypeLifetime(5)), NonStatic(TypeLifetime(6))] }, optional: Optional(false), owner: Borrow { lifetime: NonStatic(TypeLifetime(5)), mutability: Immutable }, tcx_id: OpaqueId(3) }) }], output: Infallible(Some(OutType(Opaque(OpaquePath { lifetimes: TypeLifetimes { indices: [NonStatic(TypeLifetime(1))] }, optional: Optional(true), owner: Own, tcx_id: OpaqueId(2) })))) }, Method { docs: Docs("", []), name: Check { _marker: PhantomData, buf: "return_outlives_param" }, lifetime_env: LifetimeEnv { nodes: [Lifetime { ident: Check { _marker: PhantomData, buf: "o" }, longer: [], shorter: [] }, Lifetime { ident: Check { _marker: PhantomData, buf: "short" }, longer: [MethodLifetime(2)], shorter: [] }, Lifetime { ident: Check { _marker: PhantomData, buf: "long" }, longer: [], shorter: [MethodLifetime(1)] }], num_lifetimes: 4 }, param_self: None, params: [Param { name: Check { _marker: PhantomData, buf: "hold" }, ty: Opaque(OpaquePath { lifetimes: TypeLifetimes { indices: [NonStatic(TypeLifetime(2)), NonStatic(TypeLifetime(1))] }, optional: Optional(false), owner: Borrow { lifetime: NonStatic(TypeLifetime(3)), mutability: Immutable }, tcx_id: OpaqueId(3) }) }, Param { name: Check { _marker: PhantomData, buf: "nohold" }, ty: Opaque(OpaquePath { lifetimes: TypeLifetimes { indices: [NonStatic(TypeLifetime(1))] }, optional: Optional(false), owner: Borrow { lifetime: NonStatic(TypeLifetime(1)), mutability: Immutable }, tcx_id: OpaqueId(2) }) }], output: Infallible(Some(OutType(Opaque(OpaquePath { lifetimes: TypeLifetimes { indices: [NonStatic(TypeLifetime(2))] }, optional: Optional(true), owner: Own, tcx_id: OpaqueId(2) })))) }, Method { docs: Docs("", []), name: Check { _marker: PhantomData, buf: "diamond_top" }, lifetime_env: LifetimeEnv { nodes: [Lifetime { ident: Check { _marker: PhantomData, buf: "o" }, longer: [], shorter: [] }, Lifetime { ident: Check { _marker: PhantomData, buf: "top" }, longer: [MethodLifetime(2), MethodLifetime(3)], shorter: [] }, Lifetime { ident: Check { _marker: PhantomData, buf: "left" }, longer: [MethodLifetime(4)], shorter: [MethodLifetime(1)] }, Lifetime { ident: Check { _marker: PhantomData, buf: "right" }, longer: [MethodLifetime(4)], shorter: [MethodLifetime(1)] }, Lifetime { ident: Check { _marker: PhantomData, buf: "bottom" }, longer: [], shorter: [MethodLifetime(2), MethodLifetime(3)] }], num_lifetimes: 9 }, param_self: None, params: [Param { name: Check { _marker: PhantomData, buf: "top" }, ty: Opaque(OpaquePath { lifetimes: TypeLifetimes { indices: [NonStatic(TypeLifetime(1))] }, optional: Optional(false), owner: Borrow { lifetime: NonStatic(TypeLifetime(5)), mutability: Immutable }, tcx_id: OpaqueId(2) }) }, Param { name: Check { _marker: PhantomData, buf: "left" }, ty: Opaque(OpaquePath { lifetimes: TypeLifetimes { indices: [NonStatic(TypeLifetime(2))] }, optional: Optional(false), owner: Borrow { lifetime: NonStatic(TypeLifetime(6)), mutability: Immutable }, tcx_id: OpaqueId(2) }) }, Param { name: Check { _marker: PhantomData, buf: "right" }, ty: Opaque(OpaquePath { lifetimes: TypeLifetimes { indices: [NonStatic(TypeLifetime(3))] }, optional: Optional(false), owner: Borrow { lifetime: NonStatic(TypeLifetime(7)), mutability: Immutable }, tcx_id: OpaqueId(2) }) }, Param { name: Check { _marker: PhantomData, buf: "bottom" }, ty: Opaque(OpaquePath { lifetimes: TypeLifetimes { indices: [NonStatic(TypeLifetime(4))] }, optional: Optional(false), owner: Borrow { lifetime: NonStatic(TypeLifetime(8)), mutability: Immutable }, tcx_id: OpaqueId(2) }) }], output: Infallible(Some(OutType(Opaque(OpaquePath { lifetimes: TypeLifetimes { indices: [NonStatic(TypeLifetime(1))] }, optional: Optional(true), owner: Own, tcx_id: OpaqueId(2) })))) }, Method { docs: Docs("", []), name: Check { _marker: PhantomData, buf: "diamond_left" }, lifetime_env: LifetimeEnv { nodes: [Lifetime { ident: Check { _marker: PhantomData, buf: "o" }, longer: [], shorter: [] }, Lifetime { ident: Check { _marker: PhantomData, buf: "top" }, longer: [MethodLifetime(2), MethodLifetime(3)], shorter: [] }, Lifetime { ident: Check { _marker: PhantomData, buf: "left" }, longer: [MethodLifetime(4)], shorter: [MethodLifetime(1)] }, Lifetime { ident: Check { _marker: PhantomData, buf: "right" }, longer: [MethodLifetime(4)], shorter: [MethodLifetime(1)] }, Lifetime { ident: Check { _marker: PhantomData, buf: "bottom" }, longer: [], shorter: [MethodLifetime(2), MethodLifetime(3)] }], num_lifetimes: 9 }, param_self: None, params: [Param { name: Check { _marker: PhantomData, buf: "top" }, ty: Opaque(OpaquePath { lifetimes: TypeLifetimes { indices: [NonStatic(TypeLifetime(1))] }, optional: Optional(false), owner: Borrow { lifetime: NonStatic(TypeLifetime(5)), mutability: Immutable }, tcx_id: OpaqueId(2) }) }, Param { name: Check { _marker: PhantomData, buf: "left" }, ty: Opaque(OpaquePath { lifetimes: TypeLifetimes { indices: [NonStatic(TypeLifetime(2))] }, optional: Optional(false), owner: Borrow { lifetime: NonStatic(TypeLifetime(6)), mutability: Immutable }, tcx_id: OpaqueId(2) }) }, Param { name: Check { _marker: PhantomData, buf: "right" }, ty: Opaque(OpaquePath { lifetimes: TypeLifetimes { indices: [NonStatic(TypeLifetime(3))] }, optional: Optional(false), owner: Borrow { lifetime: NonStatic(TypeLifetime(7)), mutability: Immutable }, tcx_id: OpaqueId(2) }) }, Param { name: Check { _marker: PhantomData, buf: "bottom" }, ty: Opaque(OpaquePath { lifetimes: TypeLifetimes { indices: [NonStatic(TypeLifetime(4))] }, optional: Optional(false), owner: Borrow { lifetime: NonStatic(TypeLifetime(8)), mutability: Immutable }, tcx_id: OpaqueId(2) }) }], output: Infallible(Some(OutType(Opaque(OpaquePath { lifetimes: TypeLifetimes { indices: [NonStatic(TypeLifetime(2))] }, optional: Optional(true), owner: Own, tcx_id: OpaqueId(2) })))) }, Method { docs: Docs("", []), name: Check { _marker: PhantomData, buf: "diamond_right" }, lifetime_env: LifetimeEnv { nodes: [Lifetime { ident: Check { _marker: PhantomData, buf: "o" }, longer: [], shorter: [] }, Lifetime { ident: Check { _marker: PhantomData, buf: "top" }, longer: [MethodLifetime(2), MethodLifetime(3)], shorter: [] }, Lifetime { ident: Check { _marker: PhantomData, buf: "left" }, longer: [MethodLifetime(4)], shorter: [MethodLifetime(1)] }, Lifetime { ident: Check { _marker: PhantomData, buf: "right" }, longer: [MethodLifetime(4)], shorter: [MethodLifetime(1)] }, Lifetime { ident: Check { _marker: PhantomData, buf: "bottom" }, longer: [], shorter: [MethodLifetime(2), MethodLifetime(3)] }], num_lifetimes: 9 }, param_self: None, params: [Param { name: Check { _marker: PhantomData, buf: "top" }, ty: Opaque(OpaquePath { lifetimes: TypeLifetimes { indices: [NonStatic(TypeLifetime(1))] }, optional: Optional(false), owner: Borrow { lifetime: NonStatic(TypeLifetime(5)), mutability: Immutable }, tcx_id: OpaqueId(2) }) }, Param { name: Check { _marker: PhantomData, buf: "left" }, ty: Opaque(OpaquePath { lifetimes: TypeLifetimes { indices: [NonStatic(TypeLifetime(2))] }, optional: Optional(false), owner: Borrow { lifetime: NonStatic(TypeLifetime(6)), mutability: Immutable }, tcx_id: OpaqueId(2) }) }, Param { name: Check { _marker: PhantomData, buf: "right" }, ty: Opaque(OpaquePath { lifetimes: TypeLifetimes { indices: [NonStatic(TypeLifetime(3))] }, optional: Optional(false), owner: Borrow { lifetime: NonStatic(TypeLifetime(7)), mutability: Immutable }, tcx_id: OpaqueId(2) }) }, Param { name: Check { _marker: PhantomData, buf: "bottom" }, ty: Opaque(OpaquePath { lifetimes: TypeLifetimes { indices: [NonStatic(TypeLifetime(4))] }, optional: Optional(false), owner: Borrow { lifetime: NonStatic(TypeLifetime(8)), mutability: Immutable }, tcx_id: OpaqueId(2) }) }], output: Infallible(Some(OutType(Opaque(OpaquePath { lifetimes: TypeLifetimes { indices: [NonStatic(TypeLifetime(3))] }, optional: Optional(true), owner: Own, tcx_id: OpaqueId(2) })))) }, Method { docs: Docs("", []), name: Check { _marker: PhantomData, buf: "diamond_bottom" }, lifetime_env: LifetimeEnv { nodes: [Lifetime { ident: Check { _marker: PhantomData, buf: "o" }, longer: [], shorter: [] }, Lifetime { ident: Check { _marker: PhantomData, buf: "top" }, longer: [MethodLifetime(2), MethodLifetime(3)], shorter: [] }, Lifetime { ident: Check { _marker: PhantomData, buf: "left" }, longer: [MethodLifetime(4)], shorter: [MethodLifetime(1)] }, Lifetime { ident: Check { _marker: PhantomData, buf: "right" }, longer: [MethodLifetime(4)], shorter: [MethodLifetime(1)] }, Lifetime { ident: Check { _marker: PhantomData, buf: "bottom" }, longer: [], shorter: [MethodLifetime(2), MethodLifetime(3)] }], num_lifetimes: 9 }, param_self: None, params: [Param { name: Check { _marker: PhantomData, buf: "top" }, ty: Opaque(OpaquePath { lifetimes: TypeLifetimes { indices: [NonStatic(TypeLifetime(1))] }, optional: Optional(false), owner: Borrow { lifetime: NonStatic(TypeLifetime(5)), mutability: Immutable }, tcx_id: OpaqueId(2) }) }, Param { name: Check { _marker: PhantomData, buf: "left" }, ty: Opaque(OpaquePath { lifetimes: TypeLifetimes { indices: [NonStatic(TypeLifetime(2))] }, optional: Optional(false), owner: Borrow { lifetime: NonStatic(TypeLifetime(6)), mutability: Immutable }, tcx_id: OpaqueId(2) }) }, Param { name: Check { _marker: PhantomData, buf: "right" }, ty: Opaque(OpaquePath { lifetimes: TypeLifetimes { indices: [NonStatic(TypeLifetime(3))] }, optional: Optional(false), owner: Borrow { lifetime: NonStatic(TypeLifetime(7)), mutability: Immutable }, tcx_id: OpaqueId(2) }) }, Param { name: Check { _marker: PhantomData, buf: "bottom" }, ty: Opaque(OpaquePath { lifetimes: TypeLifetimes { indices: [NonStatic(TypeLifetime(4))] }, optional: Optional(false), owner: Borrow { lifetime: NonStatic(TypeLifetime(8)), mutability: Immutable }, tcx_id: OpaqueId(2) }) }], output: Infallible(Some(OutType(Opaque(OpaquePath { lifetimes: TypeLifetimes { indices: [NonStatic(TypeLifetime(4))] }, optional: Optional(true), owner: Own, tcx_id: OpaqueId(2) })))) }, Method { docs: Docs("", []), name: Check { _marker: PhantomData, buf: "diamond_and_nested_types" }, lifetime_env: LifetimeEnv { nodes: [Lifetime { ident: Check { _marker: PhantomData, buf: "o" }, longer: [], shorter: [] }, Lifetime { ident: Check { _marker: PhantomData, buf: "a" }, longer: [MethodLifetime(2)], shorter: [] }, Lifetime { ident: Check { _marker: PhantomData, buf: "b" }, longer: [MethodLifetime(3), MethodLifetime(4)], shorter: [MethodLifetime(1), MethodLifetime(6)] }, Lifetime { ident: Check { _marker: PhantomData, buf: "c" }, longer: [MethodLifetime(4)], shorter: [MethodLifetime(2)] }, Lifetime { ident: Check { _marker: PhantomData, buf: "d" }, longer: [], shorter: [MethodLifetime(2), MethodLifetime(3)] }, Lifetime { ident: Check { _marker: PhantomData, buf: "x" }, longer: [], shorter: [] }, Lifetime { ident: Check { _marker: PhantomData, buf: "y" }, longer: [MethodLifetime(2)], shorter: [] }], num_lifetimes: 11 }, param_self: None, params: [Param { name: Check { _marker: PhantomData, buf: "a" }, ty: Opaque(OpaquePath { lifetimes: TypeLifetimes { indices: [NonStatic(TypeLifetime(1))] }, optional: Optional(false), owner: Borrow { lifetime: NonStatic(TypeLifetime(7)), mutability: Immutable }, tcx_id: OpaqueId(2) }) }, Param { name: Check { _marker: PhantomData, buf: "b" }, ty: Opaque(OpaquePath { lifetimes: TypeLifetimes { indices: [NonStatic(TypeLifetime(2))] }, optional: Optional(false), owner: Borrow { lifetime: NonStatic(TypeLifetime(6)), mutability: Immutable }, tcx_id: OpaqueId(2) }) }, Param { name: Check { _marker: PhantomData, buf: "c" }, ty: Opaque(OpaquePath { lifetimes: TypeLifetimes { indices: [NonStatic(TypeLifetime(3))] }, optional: Optional(false), owner: Borrow { lifetime: NonStatic(TypeLifetime(8)), mutability: Immutable }, tcx_id: OpaqueId(2) }) }, Param { name: Check { _marker: PhantomData, buf: "d" }, ty: Opaque(OpaquePath { lifetimes: TypeLifetimes { indices: [NonStatic(TypeLifetime(4))] }, optional: Optional(false), owner: Borrow { lifetime: NonStatic(TypeLifetime(9)), mutability: Immutable }, tcx_id: OpaqueId(2) }) }, Param { name: Check { _marker: PhantomData, buf: "nohold" }, ty: Opaque(OpaquePath { lifetimes: TypeLifetimes { indices: [NonStatic(TypeLifetime(5))] }, optional: Optional(false), owner: Borrow { lifetime: NonStatic(TypeLifetime(10)), mutability: Immutable }, tcx_id: OpaqueId(2) }) }], output: Infallible(Some(OutType(Opaque(OpaquePath { lifetimes: TypeLifetimes { indices: [NonStatic(TypeLifetime(1))] }, optional: Optional(true), owner: Own, tcx_id: OpaqueId(2) })))) }, Method { docs: Docs("", []), name: Check { _marker: PhantomData, buf: "implicit_bounds" }, lifetime_env: LifetimeEnv { nodes: [Lifetime { ident: Check { _marker: PhantomData, buf: "o" }, longer: [], shorter: [] }, Lifetime { ident: Check { _marker: PhantomData, buf: "a" }, longer: [MethodLifetime(2)], shorter: [] }, Lifetime { ident: Check { _marker: PhantomData, buf: "b" }, longer: [MethodLifetime(3)], shorter: [MethodLifetime(1)] }, Lifetime { ident: Check { _marker: PhantomData, buf: "c" }, longer: [MethodLifetime(4)], shorter: [MethodLifetime(2)] }, Lifetime { ident: Check { _marker: PhantomData, buf: "d" }, longer: [MethodLifetime(5)], shorter: [MethodLifetime(3)] }, Lifetime { ident: Check { _marker: PhantomData, buf: "x" }, longer: [], shorter: [MethodLifetime(4)] }, Lifetime { ident: Check { _marker: PhantomData, buf: "y" }, longer: [], shorter: [] }], num_lifetimes: 9 }, param_self: None, params: [Param { name: Check { _marker: PhantomData, buf: "explicit_hold" }, ty: Opaque(OpaquePath { lifetimes: TypeLifetimes { indices: [NonStatic(TypeLifetime(5))] }, optional: Optional(false), owner: Borrow { lifetime: NonStatic(TypeLifetime(4)), mutability: Immutable }, tcx_id: OpaqueId(2) }) }, Param { name: Check { _marker: PhantomData, buf: "implicit_hold" }, ty: Opaque(OpaquePath { lifetimes: TypeLifetimes { indices: [NonStatic(TypeLifetime(5))] }, optional: Optional(false), owner: Borrow { lifetime: NonStatic(TypeLifetime(7)), mutability: Immutable }, tcx_id: OpaqueId(2) }) }, Param { name: Check { _marker: PhantomData, buf: "nohold" }, ty: Opaque(OpaquePath { lifetimes: TypeLifetimes { indices: [NonStatic(TypeLifetime(6))] }, optional: Optional(false), owner: Borrow { lifetime: NonStatic(TypeLifetime(8)), mutability: Immutable }, tcx_id: OpaqueId(2) }) }], output: Infallible(Some(OutType(Opaque(OpaquePath { lifetimes: TypeLifetimes { indices: [NonStatic(TypeLifetime(1))] }, optional: Optional(true), owner: Own, tcx_id: OpaqueId(2) })))) }, Method { docs: Docs("", []), name: Check { _marker: PhantomData, buf: "implicit_bounds_deep" }, lifetime_env: LifetimeEnv { nodes: [Lifetime { ident: Check { _marker: PhantomData, buf: "o" }, longer: [], shorter: [] }, Lifetime { ident: Check { _marker: PhantomData, buf: "a" }, longer: [MethodLifetime(2)], shorter: [] }, Lifetime { ident: Check { _marker: PhantomData, buf: "b" }, longer: [MethodLifetime(3)], shorter: [MethodLifetime(1)] }, Lifetime { ident: Check { _marker: PhantomData, buf: "c" }, longer: [MethodLifetime(4)], shorter: [MethodLifetime(2)] }, Lifetime { ident: Check { _marker: PhantomData, buf: "d" }, longer: [], shorter: [MethodLifetime(3)] }, Lifetime { ident: Check { _marker: PhantomData, buf: "x" }, longer: [], shorter: [] }], num_lifetimes: 6 }, param_self: None, params: [Param { name: Check { _marker: PhantomData, buf: "explicit_" }, ty: Opaque(OpaquePath { lifetimes: TypeLifetimes { indices: [NonStatic(TypeLifetime(2))] }, optional: Optional(false), owner: Borrow { lifetime: NonStatic(TypeLifetime(1)), mutability: Immutable }, tcx_id: OpaqueId(2) }) }, Param { name: Check { _marker: PhantomData, buf: "implicit_1" }, ty: Opaque(OpaquePath { lifetimes: TypeLifetimes { indices: [NonStatic(TypeLifetime(3))] }, optional: Optional(false), owner: Borrow { lifetime: NonStatic(TypeLifetime(2)), mutability: Immutable }, tcx_id: OpaqueId(2) }) }, Param { name: Check { _marker: PhantomData, buf: "implicit_2" }, ty: Opaque(OpaquePath { lifetimes: TypeLifetimes { indices: [NonStatic(TypeLifetime(4))] }, optional: Optional(false), owner: Borrow { lifetime: NonStatic(TypeLifetime(3)), mutability: Immutable }, tcx_id: OpaqueId(2) }) }, Param { name: Check { _marker: PhantomData, buf: "nohold" }, ty: Opaque(OpaquePath { lifetimes: TypeLifetimes { indices: [NonStatic(TypeLifetime(5))] }, optional: Optional(false), owner: Borrow { lifetime: NonStatic(TypeLifetime(5)), mutability: Immutable }, tcx_id: OpaqueId(2) }) }], output: Infallible(Some(OutType(Opaque(OpaquePath { lifetimes: TypeLifetimes { indices: [NonStatic(TypeLifetime(1))] }, optional: Optional(true), owner: Own, tcx_id: OpaqueId(2) })))) }] })