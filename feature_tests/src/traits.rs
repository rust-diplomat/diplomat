#[diplomat::bridge]
mod ffi {
    use crate::callbacks::ffi::FFIError;

    #[diplomat::cfg(supports = "traits")]
    pub struct TraitTestingStruct {
        x: i32,
        y: i32,
    }

    #[diplomat::cfg(all(
        supports = "traits",
        not(supports = "callback_returns_must_be_fallible")
    ))]
    pub trait TesterTrait {
        fn test_trait_fn(&self, x: u32) -> u32;
        fn test_void_trait_fn(&self);
        fn test_struct_trait_fn(&self, s: TraitTestingStruct) -> i32;
        #[diplomat::attr(kotlin, disable)]
        fn test_result_output(&self) -> Result<u32, ()>;
        #[diplomat::attr(kotlin, disable)]
        fn test_optional_output(&self, x: u32) -> Option<u32>;
        #[diplomat::attr(kotlin, disable)]
        fn test_result_of_optional(&self, is_ok: bool) -> Result<u32, DiplomatOption<u32>>;
    }

    #[diplomat::cfg(all(
        supports = "traits",
        not(supports = "callback_returns_must_be_fallible")
    ))]
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

        #[diplomat::attr(kotlin, disable)]
        pub fn test_result_output(t: impl TesterTrait) {
            assert_eq!(t.test_result_output(), Ok(0));
        }

        #[diplomat::attr(kotlin, disable)]
        pub fn test_optional_output(t: impl TesterTrait, x: u32) {
            assert_eq!(t.test_optional_output(x), Some(5));
        }

        #[diplomat::attr(kotlin, disable)]
        pub fn test_result_of_optional(
            t: impl TesterTrait,
            is_ok: bool,
        ) -> Result<u32, DiplomatOption<u32>> {
            t.test_result_of_optional(is_ok)
        }
    }

    #[diplomat::cfg(all(supports = "traits", supports = "callback_returns_must_be_fallible"))]
    pub trait FallibleTesterTrait {
        fn test_void_trait_fn(&self) -> Result<(), FFIError>;
        fn test_result_output(&self, x: u32) -> Result<u32, FFIError>;
    }

    #[diplomat::cfg(all(supports = "traits", supports = "callback_returns_must_be_fallible"))]
    pub struct FallibleTraitWrapper {
        cant_be_empty: bool,
    }

    impl FallibleTraitWrapper {
        pub fn test_with_trait(t: impl FallibleTesterTrait) {
            let _ = t.test_void_trait_fn();
        }

        pub fn test_result_output(t: impl FallibleTesterTrait, x: u32) -> Result<u32, FFIError> {
            t.test_result_output(x)
        }
    }
}
