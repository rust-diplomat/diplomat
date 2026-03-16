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

/// Diplomat will prepend this whole block to the start of attrs.rs,
/// but we currently cannot do the same for proc_macro (until we hit MSRV >= 1.88).
/// So the workaround is to use the path to the module whenever referring to the imported type (as seen above).
pub struct NonMacroMixin;
