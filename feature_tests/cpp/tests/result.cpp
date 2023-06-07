#include <iostream>
#include "../include/ResultOpaque.hpp"
#include "assert.hpp"

int main(int argc, char *argv[]) {
    ResultOpaque r;
    ResultOpaque r2 = ResultOpaque::new_(5).ok().value();
    r2.assert_integer(5);
    auto foo = ResultOpaque::new_failing_foo().err().value();
    simple_assert_eq("foo error", (int)foo, (int)ErrorEnum::Foo);

    auto bar = ResultOpaque::new_failing_bar().err().value();
    simple_assert_eq("bar error", (int)bar, (int)ErrorEnum::Bar);

    auto unit_err = ResultOpaque::new_failing_unit();
    simple_assert("unit error", unit_err.is_err())

    auto struc = ResultOpaque::new_failing_struct(109).err().value();
    simple_assert_eq("struct error", struc.i, 109);


    auto integer = ResultOpaque::new_int(109).ok().value();
    simple_assert_eq("int ok", integer, 109);

    auto in_err = ResultOpaque::new_in_err(198).err().value();
    in_err.assert_integer(198);

    auto in_enum_err = ResultOpaque::new_in_enum_err(989).err().value();
    in_enum_err.assert_integer(989);
}
