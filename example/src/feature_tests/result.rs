#[diplomat::bridge]
pub mod ffi {

    use diplomat_runtime::DiplomatResult;

    #[diplomat::opaque]
    pub struct ResultOpaque(i32);

    #[derive(PartialEq, Eq, Debug)]
    pub enum ErrorEnum {
        Foo,
        Bar,
    }

    #[derive(Debug)]
    pub struct ErrorStruct {
        i: i32,
    }
    impl ResultOpaque {
        pub fn new(i: i32) -> DiplomatResult<Box<ResultOpaque>, ErrorEnum> {
            Ok(Box::new(ResultOpaque(i))).into()
        }

        pub fn new_failing_foo() -> DiplomatResult<Box<ResultOpaque>, ErrorEnum> {
            Err(ErrorEnum::Foo).into()
        }

        pub fn new_failing_bar() -> DiplomatResult<Box<ResultOpaque>, ErrorEnum> {
            Err(ErrorEnum::Bar).into()
        }

        pub fn new_failing_unit() -> DiplomatResult<Box<ResultOpaque>, ()> {
            Err(()).into()
        }

        pub fn new_failing_struct(i: i32) -> DiplomatResult<Box<ResultOpaque>, ErrorStruct> {
            Err(ErrorStruct { i }).into()
        }

        pub fn new_in_err(i: i32) -> DiplomatResult<(), Box<ResultOpaque>> {
            Err(Box::new(ResultOpaque(i))).into()
        }

        pub fn new_in_enum_err(i: i32) -> DiplomatResult<ErrorEnum, Box<ResultOpaque>> {
            Err(Box::new(ResultOpaque(i))).into()
        }

        pub fn assert_integer(&self, i: i32) {
            assert_eq!(i, self.0);
        }
    }
}
