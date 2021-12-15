#include <iostream>
#include "../ResultOpaque.hpp"
#include "assert.hpp"

int main(int argc, char *argv[]) {
    ResultOpaque r;
    ResultOpaque r2 = ResultOpaque::new_(5).ok().value();
    r2.assert_integer(5);
    auto foo_err = ResultOpaque::new_failing_foo();
    auto foo = foo_err.err().value();
    simple_assert_eq("foo error", (int)foo, (int)ErrorEnum::Foo);

    auto bar_err = ResultOpaque::new_failing_bar();
    auto bar = bar_err.err().value();
    simple_assert_eq("bar error", (int)bar, (int)ErrorEnum::Bar);

    auto unit_err = ResultOpaque::new_failing_unit();
    simple_assert("unit error", unit_err.is_err())

    auto struc_err = ResultOpaque::new_failing_struct(109);
    auto struc = struc_err.err().value();
    simple_assert_eq("struct error", struc.i, 109);

    auto in_err = ResultOpaque::new_in_err(198);
    auto in_err_val = in_err.err().value();
    in_err_val.assert_integer(198);

    auto in_enum_err = ResultOpaque::new_in_enum_err(989);
    auto in_enum_err_val = in_enum_err.err().value();
    in_enum_err_val.assert_integer(989);
}
