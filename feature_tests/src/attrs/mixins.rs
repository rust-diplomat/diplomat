#[diplomat::macro_rules]
#[macro_export]
macro_rules! mixin_macro {
    () => {
        #[diplomat::opaque]
        pub struct MixinTest(super::mixins::NonMacroMixin);

        impl MixinTest {
            pub fn hello(w: &mut DiplomatWrite) {
                write!(w, "Hello!").unwrap();
            }
        }
    };
}

/// This will not appear in the include.
pub struct NonMacroMixin;
