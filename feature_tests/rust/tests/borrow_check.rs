//! Borrowed opaques are checked by the compiler: misuse fails to compile, and
//! correct use compiles and runs.

#[test]
fn borrow_check() {
    let t = trybuild::TestCases::new();
    t.pass("tests/borrow_check/pass/*.rs");
    t.compile_fail("tests/borrow_check/fail/*.rs");
}
