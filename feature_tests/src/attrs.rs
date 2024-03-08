#[diplomat::bridge]
#[diplomat::abi_rename = "namespace_{0}"]
#[diplomat::attr(cpp2, rename = "CPPRenamed{0}")]
#[diplomat::attr(cpp2, namespace = "ns")]
pub mod ffi {
    #[diplomat::opaque]
    #[diplomat::attr(cpp2, rename = "AttrOpaque1Renamed")]
    pub struct AttrOpaque1;

    impl AttrOpaque1 {
        #[diplomat::attr(cpp2, rename = "totally_not_{0}")]
        #[diplomat::attr(supports = constructors, constructor)]
        pub fn new() -> Box<AttrOpaque1> {
            Box::new(AttrOpaque1)
        }

        #[diplomat::attr(cpp2, rename = "method_renamed")]
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
    #[diplomat::attr(cpp2, disable)]
    pub struct AttrOpaque2;

    pub enum AttrEnum {
        A,
        B,
        #[diplomat::attr(cpp2, rename = "CPPRenamed")]
        C,
    }

    #[diplomat::opaque]
    #[diplomat::attr(cpp2, namespace = "")]
    #[diplomat::attr(cpp2, rename = "Unnamespaced")]
    pub struct Unnamespaced;

    impl Unnamespaced {
        #[diplomat::attr(supports = constructors, named_constructor)]
        pub fn make(_e: AttrEnum) -> Box<Self> {
            Box::new(Self)
        }

        pub fn use_namespaced(&self, _n: &AttrOpaque1) {}
    }
}
