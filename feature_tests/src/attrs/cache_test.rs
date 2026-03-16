#[diplomat::macro_rules]
#[macro_export]
macro_rules! cache_test_macro {
    ($t:ty) => {
        pub struct $t;
    };
}
