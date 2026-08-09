#[diplomat::bridge]
mod ffi {
    #[diplomat::cfg(supports = "traits")]
    pub struct TraitTestingStruct {
        x: i32,
        y: i32,
    }

    pub trait TesterTrait {
        fn test_trait_fn(&self, x: u32) -> u32;
        fn test_void_trait_fn(&self);
        fn test_struct_trait_fn(&self, s: TraitTestingStruct) -> i32;
        #[diplomat::attr(kotlin, disable)]
        fn test_result_output(&self) -> Result<u32, ()>;
        fn test_taking_mutable_opaque(&self, mo: &mut MutableBytes);
    }

    #[diplomat::cfg(supports = "traits")]
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

        pub fn test_taking_mutable_opaque(t: impl TesterTrait) {
            let mut mutable_bytes = MutableBytes::new();
            t.test_taking_mutable_opaque(&mut mutable_bytes);
            assert_eq!(mutable_bytes.get_as_bytes(), &[1, 2, 3]);
        }
    }

    #[diplomat::opaque_mut]
    pub struct MutableBytes {
        inner: Option<Vec<u8>>,
    }

    impl MutableBytes {
        pub fn get_as_bytes<'a>(&'a self) -> &'a [u8] {
            match &self.inner {
                Some(bytes) => bytes.as_slice(),
                None => {
                    panic!("MutableBytes have not been set");
                }
            }
        }

        pub fn set_bytes(&mut self, new_bytes: &[u8]) {
            self.inner = Some(new_bytes.to_vec());
        }

        pub fn new() -> Box<Self> {
            Box::new(Self { inner: None })
        }
    }
}
