#[diplomat::bridge]
#[diplomat::abi_rename = "namespace_{0}"]
#[diplomat::attr(not(any(c, kotlin)), rename = "Renamed{0}")]
#[diplomat::attr(auto, namespace = "ns")]
pub mod ffi {
    #[diplomat::macro_rules]
    macro_rules! impl_mac {
        ($arg1:ident, $arg2:ident, $arg3:block) => {
            pub fn $arg1() -> i32 {
                $arg3
            }

            pub fn $arg2() -> i32 {
                println!("Test");
                0
            }
        };
    }

    #[diplomat::macro_rules]
    macro_rules! create_vec {
        ($vec_name:ident contains "hello"; [$ty:ident]) => {
            #[diplomat::opaque]
            pub struct $vec_name(Vec<$ty>);

            impl $vec_name {
                #[diplomat::attr(auto, constructor)]
                pub fn new() -> Box<$vec_name> {
                    println!("{}", stringify!($vec_name));
                    Box::new(Self(Vec::new()))
                }

                #[diplomat::attr(auto, getter)]
                pub fn len(&self) -> usize {
                    self.0.len()
                }

                #[diplomat::attr(auto, indexer)]
                pub fn get(&self, idx: usize) -> Option<$ty> {
                    self.0.get(idx).cloned()
                }

                pub fn push(&mut self, value: $ty) {
                    self.0.push(value)
                }
            }
        };
    }

    create_vec!(VectorTest contains "hello"; [f64]);

    #[derive(Clone)]
    #[diplomat::opaque]
    // Attr for generating mocking interface in kotlin backend to enable JVM test fakes.
    #[diplomat::attr(kotlin, generate_mocking_interface)]
    #[diplomat::attr(not(kotlin), rename = "AttrOpaque1Renamed")]
    /// Some example docs
    #[diplomat::docs(any(nanobind, cpp))]
    /// Some Nanobind/C++ example docs
    #[diplomat::docs(js)]
    /// Some JS example docs
    #[diplomat::docs(*)]
    /// Back to all docs
    pub struct AttrOpaque1;

    impl AttrOpaque1 {
        #[diplomat::attr(not(kotlin), rename = "totally_not_{0}")]
        #[diplomat::attr(auto, constructor)]
        /// More example docs
        pub fn new() -> Box<AttrOpaque1> {
            Box::new(AttrOpaque1)
        }

        #[diplomat::attr(any(not(supports=callbacks), kotlin), disable)]
        pub fn test_namespaced_callback(_t: impl Fn() -> Result<(), ()>) {
            todo!()
        }

        impl_mac!(mac_test, hello, {
            println!("Hello world!");
            10
        });

        #[diplomat::attr(not(kotlin), rename = "method_renamed")]
        #[diplomat::attr(auto, getter = "method")]
        pub fn method(&self) -> u8 {
            77
        }

        #[diplomat::abi_rename("renamed_on_abi_only")]
        #[diplomat::attr(auto, getter = "abirenamed")]
        pub fn abirenamed(&self) -> u8 {
            123
        }

        #[diplomat::attr(*, disable)]
        pub fn method_disabled(&self) {
            println!("disabled in hir");
        }

        pub fn use_unnamespaced(&self, _un: &Unnamespaced) {}
        pub fn use_namespaced(&self, _n: AttrEnum) {}
    }

    #[diplomat::opaque]
    pub struct AttrOpaque2;

    pub enum AttrEnum {
        A,
        B,
        #[diplomat::attr(*, rename = "Renamed")]
        C,
    }

    #[diplomat::opaque]
    #[diplomat::attr(auto, namespace = "")]
    #[diplomat::attr(not(kotlin), rename = "Unnamespaced")]
    pub struct Unnamespaced;

    impl Unnamespaced {
        #[diplomat::attr(auto, named_constructor)]
        pub fn make(_e: AttrEnum) -> Box<Self> {
            Box::new(Self)
        }

        pub fn use_namespaced(&self, _n: &AttrOpaque1) {}
    }

    #[diplomat::opaque]
    #[diplomat::attr(auto, namespace = "nested::ns")]
    #[diplomat::attr(supports = namespacing, rename = "Nested")]
    pub struct Nested;

    #[diplomat::opaque]
    #[diplomat::attr(auto, namespace = "nested::ns2")]
    #[diplomat::attr(supports = namespacing, rename = "Nested")]
    pub struct Nested2;

    #[diplomat::opaque]
    #[diplomat::cfg(supports = comparators)]
    pub struct Comparable(u8);

    impl Comparable {
        pub fn new(int: u8) -> Box<Self> {
            Box::new(Self(int))
        }
        #[diplomat::attr(auto, comparison)]
        pub fn cmp(&self, other: &Comparable) -> core::cmp::Ordering {
            self.0.cmp(&other.0)
        }
    }

    #[diplomat::opaque]
    #[diplomat::cfg(supports = indexing)]
    pub struct MyIndexer(Vec<String>);

    #[diplomat::opaque]
    #[diplomat::cfg(supports = iterators)]
    pub struct MyIterable(Vec<u8>);

    impl MyIterable {
        #[diplomat::attr(auto, constructor)]
        pub fn new(x: &[u8]) -> Box<Self> {
            Box::new(Self(x.into()))
        }
        #[diplomat::attr(auto, iterable)]
        pub fn iter<'a>(&'a self) -> Box<MyIterator<'a>> {
            Box::new(MyIterator(self.0.iter()))
        }
        #[diplomat::attr(nanobind, rename = "__len__")]
        #[diplomat::cfg(nanobind)]
        pub fn len(&self) -> usize {
            self.0.len()
        }
    }

    #[diplomat::opaque]
    #[diplomat::cfg(supports = iterators)]
    pub struct MyIterator<'a>(std::slice::Iter<'a, u8>);
    impl<'a> MyIterator<'a> {
        #[diplomat::attr(auto, iterator)]
        pub fn next(&mut self) -> Option<u8> {
            self.0.next().copied()
        }
    }

    impl MyIndexer {
        #[diplomat::attr(auto, indexer)]
        pub fn get<'a>(&'a self, i: usize) -> Option<&'a DiplomatStr> {
            self.0.get(i).as_ref().map(|string| string.as_bytes())
        }
    }

    #[diplomat::opaque]
    #[diplomat::cfg(supports = iterators)]
    struct OpaqueIterable(Vec<AttrOpaque1>);

    impl OpaqueIterable {
        #[diplomat::attr(auto, constructor)]
        pub fn new(size: usize) -> Box<Self> {
            Box::new(Self(vec![AttrOpaque1; size]))
        }

        #[diplomat::attr(auto, iterable)]
        pub fn iter<'a>(&'a self) -> Box<OpaqueIterator<'a>> {
            Box::new(OpaqueIterator(Box::new(self.0.iter().cloned())))
        }
    }

    #[diplomat::opaque]
    #[diplomat::cfg(supports = iterators)]
    struct OpaqueIterator<'a>(Box<dyn Iterator<Item = AttrOpaque1> + 'a>);
    impl<'a> OpaqueIterator<'a> {
        #[diplomat::attr(auto, iterator)]
        pub fn next(&'a mut self) -> Option<Box<AttrOpaque1>> {
            self.0.next().map(Box::new)
        }
    }

    #[diplomat::opaque]
    #[diplomat::cfg(supports = iterators)]
    struct OpaqueRefIterable(Vec<AttrOpaque1>);

    impl OpaqueRefIterable {
        #[diplomat::attr(auto, constructor)]
        pub fn new(size: usize) -> Box<Self> {
            Box::new(Self(vec![AttrOpaque1; size]))
        }

        #[diplomat::attr(auto, iterable)]
        pub fn iter<'a>(&'a self) -> Box<OpaqueRefIterator<'a>> {
            Box::new(OpaqueRefIterator(self.0.iter()))
        }
    }

    #[diplomat::opaque]
    #[diplomat::cfg(supports = iterators)]
    struct OpaqueRefIterator<'a>(std::slice::Iter<'a, AttrOpaque1>);
    impl<'a> OpaqueRefIterator<'a> {
        #[diplomat::attr(auto, iterator)]
        pub fn next(&'a mut self) -> Option<&'a AttrOpaque1> {
            self.0.next()
        }
    }

    #[diplomat::opaque]
    #[diplomat::cfg(supports = arithmetic)]
    pub(crate) struct OpaqueArithmetic {
        x: i32,
        y: i32,
    }

    impl OpaqueArithmetic {
        pub fn make(x: i32, #[diplomat::attr(auto, default_value = 12)] y: i32) -> Box<Self> {
            Box::new(Self { x, y })
        }

        #[diplomat::attr(supports=method_overloading, rename="make")]
        pub fn make_overload(
            x: f32,
            #[diplomat::attr(auto, default_value = 14.48)] y: f32,
        ) -> Box<Self> {
            Box::new(Self {
                x: (x as i32) + 2,
                y: y as i32,
            })
        }

        #[diplomat::attr(supports=method_overloading, rename="make")]
        pub fn make_overload_rename_arg(x: f32, z: bool) -> Box<Self> {
            Box::new(Self {
                x: (x as i32) + 2,
                y: z as i32,
            })
        }

        pub fn x(&self) -> i32 {
            self.x
        }

        pub fn y(&self) -> i32 {
            self.y
        }

        #[diplomat::attr(auto, add)]
        pub fn add(&self, o: &Self) -> Box<Self> {
            Box::new(Self {
                x: self.x + o.x,
                y: self.y + o.y,
            })
        }

        #[diplomat::attr(auto, sub)]
        pub fn sub(&self, o: &Self) -> Box<Self> {
            Box::new(Self {
                x: self.x - o.x,
                y: self.y - o.y,
            })
        }

        #[diplomat::attr(auto, mul)]
        pub fn mul(&self, o: &Self) -> Box<Self> {
            Box::new(Self {
                x: self.x * o.x,
                y: self.y * o.y,
            })
        }

        #[diplomat::attr(auto, div)]
        pub fn div(&self, o: &Self) -> Box<Self> {
            Box::new(Self {
                x: self.x / o.x,
                y: self.y / o.y,
            })
        }

        #[diplomat::attr(auto, add_assign)]
        pub fn addassign(&mut self, o: &Self) {
            self.x += o.x;
            self.y += o.y;
        }

        #[diplomat::attr(auto, sub_assign)]
        pub fn subassign(&mut self, o: &Self) {
            self.x -= o.x;
            self.y -= o.y;
        }

        #[diplomat::attr(auto, mul_assign)]
        pub fn mulassign(&mut self, o: &Self) {
            self.x *= o.x;
            self.y *= o.y;
        }

        #[diplomat::attr(auto, div_assign)]
        pub fn divassign(&mut self, o: &Self) {
            self.x /= o.x;
            self.y /= o.y;
        }
    }

    #[diplomat::attr(auto, abi_compatible)]
    pub struct StructWithAttrs {
        a: bool,
        b: u32,
    }

    impl StructWithAttrs {
        #[diplomat::attr(supports = fallible_constructors, constructor)]
        #[diplomat::attr(dart, disable)] // This has the same signature as the default dart ctor
        pub fn new_fallible(a: bool, b: u32) -> Result<StructWithAttrs, ()> {
            if a {
                Ok(Self { a, b })
            } else {
                Err(())
            }
        }

        #[diplomat::attr(auto, getter)]
        pub fn c(self) -> u32 {
            5
        }

        #[deprecated(note = "use Foo")]
        pub fn deprecated(self) {}
    }

    #[deprecated(note = "use Foo")]
    pub struct DeprecatedStruct;

    #[deprecated(note = "use Foo")]
    pub enum DeprecatedEnum {
        A,
    }

    #[diplomat::opaque]
    #[deprecated(note = "use Foo")]
    pub struct DeprecatedOpaque;

    #[diplomat::macro_rules]
    macro_rules! macro_frag_spec_test {
        (BLOCK $b:block [EXPR $e:expr, IDENT $i:ident] LT $lt:lifetime literal $l:literal <=> $m:meta $p:path; $t:tt $ty:ty, $vis:vis, $it:item) => {
            struct $i {
                a: usize,
            }

            $it

            use $p;
            impl $i {
                #[allow(clippy::extra_unused_lifetimes)]
                $vis fn test_func<$lt>(w : &mut DiplomatWrite) -> usize {
                    let a = $e;
                    write!(w, $l).unwrap();
                    a
                }

                #[$m]
                $vis fn test_meta() -> $i {
                    $b
                    $i { a: 0 }
                }
            }

            #[diplomat::opaque]
            struct TestOpaque($ty);

            impl TestOpaque $t
        };
    }

    macro_frag_spec_test! {BLOCK {
        println!("Hello world");
    } [EXPR 0, IDENT TestMacroStruct] LT 'a literal "Testing" <=> diplomat::attr(auto, constructor) std::fmt::Write; {
        fn hello() {}
    } f64, pub, const IT:usize = 0;}

    #[diplomat::attr(not(supports = free_functions), disable)]
    #[diplomat::attr(cpp, custom_extra_code(source = "//Test", location = "pre_impl_block"))]
    #[diplomat::attr(cpp, custom_extra_code(source = "//End Test", location = "impl_block"))]
    pub fn free_func_test(x: i32) -> i32 {
        x + 5
    }

    #[diplomat::attr(not(supports = free_functions), disable)]
    #[diplomat::attr(auto, namespace = "nested::ns")]
    pub fn nested_ns_fn(#[diplomat::attr(auto, default_value = true)] x: bool) -> bool {
        !x
    }

    /// Testing support for List[str] in Nanobind
    #[diplomat::opaque]
    #[diplomat::cfg(supports = custom_bindings)]
    #[diplomat::attr(
        cpp,
        custom_extra_code(
            file = "custom_binds/cpp/RenamedStringList.d.hpp",
            location = "def_block"
        )
    )]
    #[diplomat::attr(
        cpp,
        custom_extra_code(
            file = "custom_binds/cpp/RenamedStringList.hpp",
            location = "impl_block"
        )
    )]
    #[diplomat::attr(
        nanobind,
        custom_extra_code(
            file = "custom_binds/nanobind/RenamedStringList.hpp",
            location = "impl_block"
        )
    )]
    #[repr(C)]
    pub struct StringList(DiplomatOwnedStrSlice);

    impl StringList {
        // We want to generate the bindings for this ourselves:
        #[diplomat::attr(cpp, disable)]
        pub fn return_new() -> Box<Self> {
            let sl: Box<[u8]> = Box::new(*b"Test!");
            Box::new(Self(sl.into()))
        }
    }

    #[diplomat::opaque]
    #[diplomat::cfg(supports = custom_bindings)]
    #[diplomat::attr(
        any(nanobind, cpp),
        custom_extra_code(
            source = "public:
    const static bool custom_bool = false;
    static std::string special_function();",
            location = "def_block"
        )
    )]
    #[diplomat::attr(
        any(nanobind, cpp),
        custom_extra_code(source = "//Pre Test", location = "pre_def_block")
    )]
    #[diplomat::attr(
        any(nanobind, cpp),
        custom_extra_code(source = "//Post Test", location = "post_def_block")
    )]
    #[diplomat::attr(
        any(nanobind, cpp),
        custom_extra_code(
            source = r#"std::string somelib::ns::RenamedBlockOverride::special_function() {
    return "This is a custom binding.";
}"#,
            location = "impl_block"
        )
    )]
    #[diplomat::attr(
        nanobind,
        custom_extra_code(
            source = r#"opaque.def("special_function", &somelib::ns::RenamedBlockOverride::special_function);"#,
            location = "init_block"
        )
    )]
    #[diplomat::attr(
        nanobind,
        custom_extra_code(source = "//Pre-Init Test", location = "pre_init_block")
    )]
    pub struct BlockOverride();

    // C++ will not generate this, since it has all features disabled by default (see lib.rs)
    #[diplomat::attr(not(feature=some_feature), disable)]
    pub struct FeatureTest();

    #[diplomat::attr(not(nanobind), disable)]
    #[diplomat::opaque]
    /// Tests for https://github.com/rust-diplomat/diplomat/issues/1050.
    /// C++ generates unique_ptrs for Opaque ZSTs, and Nanobind
    /// expects every unique_ptr it converts to wrap a unique pointer type. It errors otherwise.
    /// This is not the case, as in Rust pointers to ZSTs are always the same address.
    pub struct OpaqueZST;

    impl OpaqueZST {
        #[diplomat::attr(auto, constructor)]
        pub fn ctor() -> Box<Self> {
            Box::new(Self)
        }

        pub fn make() -> Box<Self> {
            Box::new(Self)
        }

        #[diplomat::attr(auto, getter)]
        pub fn out_string(w: &mut DiplomatWrite) {
            write!(w, "Test!").expect("Could not write");
        }

        pub fn member(&self) -> Box<Self> {
            Box::new(Self)
        }

        pub fn mut_member(&mut self) -> Box<Self> {
            Box::new(Self)
        }

        #[diplomat::attr(auto, add)]
        pub fn add(&self, _o: &Self) -> Box<Self> {
            Box::new(OpaqueZST)
        }

        #[diplomat::attr(auto, sub)]
        pub fn sub(&self, _o: &Self) -> Box<Self> {
            Box::new(Self)
        }

        #[diplomat::attr(auto, mul)]
        pub fn mul(&self, _o: &Self) -> Box<Self> {
            Box::new(Self)
        }

        #[diplomat::attr(auto, div)]
        pub fn div(&self, _o: &Self) -> Box<Self> {
            Box::new(Self)
        }

        pub fn success_zst(return_success: bool) -> Result<Box<Self>, ()> {
            if return_success {
                Ok(Box::new(Self))
            } else {
                Err(())
            }
        }

        pub fn fail_zst(return_success: bool) -> Result<(), Box<Self>> {
            if return_success {
                Ok(())
            } else {
                Err(Box::new(Self))
            }
        }

        pub fn success_fail_zst(return_success: bool) -> Result<Box<Self>, Box<Self>> {
            if return_success {
                Ok(Box::new(Self))
            } else {
                Err(Box::new(Self))
            }
        }

        pub fn optional_zst(is_some: bool) -> Option<Box<Self>> {
            if is_some {
                Some(Box::new(Self))
            } else {
                None
            }
        }

        #[diplomat::attr(auto, getter)]
        pub fn static_getter() -> Box<Self> {
            Box::new(Self)
        }

        #[diplomat::attr(auto, setter = "static_getter")]
        pub fn static_setter(_a: &Self) {}

        #[diplomat::attr(auto, getter)]
        pub fn getter(&self) -> Box<Self> {
            Box::new(Self)
        }

        #[diplomat::attr(auto, setter = "getter")]
        pub fn setter(&self, _a: &Self) {}

        #[diplomat::attr(auto, iterable)]
        pub fn iter(&self) -> Box<OpaqueZSTIterator> {
            Box::new(OpaqueZSTIterator)
        }

        #[diplomat::attr(auto, indexer)]
        pub fn indexer(&self, _idx: usize) -> Box<Self> {
            Box::new(Self)
        }
    }

    #[diplomat::attr(not(nanobind), disable)]
    #[diplomat::opaque]
    /// Tests for https://github.com/rust-diplomat/diplomat/issues/1050.
    pub struct OpaqueZSTIterator;

    impl OpaqueZSTIterator {
        #[diplomat::attr(auto, constructor)]
        pub fn ctor() -> Box<Self> {
            Box::new(Self)
        }

        #[diplomat::attr(auto, iterator)]
        pub fn next(&self) -> Option<Box<Self>> {
            Some(Box::new(Self))
        }

        #[diplomat::attr(auto, indexer)]
        pub fn nullable_indexer(&self, _idx: usize) -> Option<Box<Self>> {
            Some(Box::new(Self))
        }

        #[diplomat::attr(auto, stringifier)]
        pub fn stringify(&self, _w: &mut DiplomatWrite) -> Result<(), Box<OpaqueZST>> {
            Err(Box::new(OpaqueZST))
        }
    }
}
