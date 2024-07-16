#[diplomat::bridge]
#[diplomat::abi_rename = "namespace_{0}"]
#[diplomat::attr(supports = renaming, rename = "Renamed{0}")]
#[diplomat::attr(cpp2, namespace = "ns")]
pub mod ffi {
    #[derive(Clone)]
    #[diplomat::opaque]
    #[diplomat::attr(supports = renaming, rename = "AttrOpaque1Renamed")]
    pub struct AttrOpaque1;

    impl AttrOpaque1 {
        #[diplomat::attr(supports = renaming, rename = "totally_not_{0}")]
        #[diplomat::attr(supports = constructors, constructor)]
        pub fn new() -> Box<AttrOpaque1> {
            Box::new(AttrOpaque1)
        }

        #[diplomat::attr(supports = renaming, rename = "method_renamed")]
        #[diplomat::attr(supports = accessors, getter = "method")]
        pub fn method(&self) -> u8 {
            77
        }

        #[diplomat::abi_rename("renamed_on_abi_only")]
        #[diplomat::attr(supports = accessors, getter = "abirenamed")]
        pub fn abirenamed(&self) -> u8 {
            123
        }

        #[diplomat::attr(cpp2, disable)]
        pub fn method_disabledcpp(&self) {
            println!("disabled in cpp");
        }

        pub fn use_unnamespaced(&self, _un: &Unnamespaced) {}
        pub fn use_namespaced(&self, _n: AttrEnum) {}
    }

    #[diplomat::opaque]
    pub struct AttrOpaque2;

    pub enum AttrEnum {
        A,
        B,
        #[diplomat::attr(supports = renaming, rename = "Renamed")]
        C,
    }

    #[diplomat::opaque]
    #[diplomat::attr(cpp2, namespace = "")]
    #[diplomat::attr(supports = renaming, rename = "Unnamespaced")]
    pub struct Unnamespaced;

    impl Unnamespaced {
        #[diplomat::attr(supports = constructors, named_constructor)]
        pub fn make(_e: AttrEnum) -> Box<Self> {
            Box::new(Self)
        }

        pub fn use_namespaced(&self, _n: &AttrOpaque1) {}
    }

    #[diplomat::opaque]
    #[diplomat::attr(not(supports = comparators), disable)]
    pub struct Comparable(u8);

    impl Comparable {
        pub fn new(int: u8) -> Box<Self> {
            Box::new(Self(int))
        }
        #[diplomat::attr(*, comparison)]
        pub fn cmp(&self, other: &Comparable) -> core::cmp::Ordering {
            self.0.cmp(&other.0)
        }
    }

    #[diplomat::opaque]
    #[diplomat::attr(not(supports = iterables), disable)]
    pub struct MyIterable(Vec<u8>);

    #[diplomat::opaque]
    #[diplomat::attr(not(supports = iterators), disable)]
    pub struct MyIterator<'a>(std::slice::Iter<'a, u8>);

    #[diplomat::opaque]
    #[diplomat::attr(not(supports = indexing), disable)]
    #[diplomat::attr(dart, disable)]
    pub struct MyIndexer(Vec<String>);

    impl MyIterable {
        #[diplomat::attr(supports = constructors, constructor)]
        pub fn new(x: &[u8]) -> Box<Self> {
            Box::new(Self(x.into()))
        }
        #[diplomat::attr(*, iterable)]
        pub fn iter<'a>(&'a self) -> Box<MyIterator<'a>> {
            Box::new(MyIterator(self.0.iter()))
        }
    }

    impl<'a> MyIterator<'a> {
        #[diplomat::attr(*, iterator)]
        pub fn next(&mut self) -> Option<u8> {
            self.0.next().copied()
        }
    }

    impl MyIndexer {
        #[diplomat::attr(*, indexer)]
        pub fn get<'a>(&'a self, i: usize) -> Option<&'a DiplomatStr> {
            self.0.get(i).as_ref().map(|string| string.as_bytes())
        }
    }

    #[diplomat::opaque]
    #[diplomat::attr(not(supports = iterables), disable)]
    struct OpaqueIterable(Vec<AttrOpaque1>);

    #[diplomat::opaque]
    #[diplomat::attr(not(supports = iterators), disable)]
    struct OpaqueIterator<'a>(Box<dyn Iterator<Item = AttrOpaque1> + 'a>);

    impl OpaqueIterable {
        #[diplomat::attr(*, iterable)]
        pub fn iter<'a>(&'a self) -> Box<OpaqueIterator<'a>> {
            Box::new(OpaqueIterator(Box::new(self.0.iter().cloned())))
        }
    }

    impl<'a> OpaqueIterator<'a> {
        #[diplomat::attr(*, iterator)]
        pub fn next(&'a mut self) -> Option<Box<AttrOpaque1>> {
            self.0.next().map(Box::new)
        }
    }
}
