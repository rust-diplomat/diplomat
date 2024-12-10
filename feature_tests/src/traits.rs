#[diplomat::bridge]
mod ffi {
    #[diplomat::attr(not(supports = "traits"), disable)]
    pub struct TraitTestingStruct {
        x: i32,
        y: i32,
    }
    pub trait TesterTrait {
        fn test_trait_fn(&self, x: u32) -> u32;
        fn test_void_trait_fn(&self);
        fn test_struct_trait_fn(&self, s: TraitTestingStruct) -> i32;
    }
    #[diplomat::attr(not(supports = "traits"), disable)]
    pub struct TraitWrapper {
        cant_be_empty: bool,
    }

    impl TraitWrapper {
        pub fn test_with_trait(t: impl TesterTrait, x: i32) -> i32 {
            t.test_void_trait_fn();
            t.test_trait_fn(x.try_into().unwrap()).try_into().unwrap()
        }

        pub fn test_trait_with_struct(t: impl TesterTrait) -> i32 {
            let arg = TraitTestingStruct { x: 1, y: 5 };
            t.test_struct_trait_fn(arg)
        }
    }
}
