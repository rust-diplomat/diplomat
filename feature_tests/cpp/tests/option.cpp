#include <iostream>
#include "../include/OptionOpaqueChar.hpp"
#include "../include/OptionOpaque.hpp"
#include "../include/OptionStruct.hpp"
#include "assert.hpp"

int main(int argc, char *argv[]) {

    std::unique_ptr<OptionOpaque> o = OptionOpaque::new_(1415);
    o->assert_integer(1415);

    o = OptionOpaque::new_none();
    simple_assert("new_none() returns None", !o);

    OptionStruct s = OptionOpaque::new_struct();
    s.a->assert_integer(101); 
    s.b->assert_char(U'餐'); 
    simple_assert_eq("correct struct returned", s.c, 904);
    s.d->assert_integer(926535);

    s = OptionOpaque::new_struct_nones();

    simple_assert("new_struct_nones() returns None", !s.a);
    simple_assert("new_struct_nones() returns None", !s.b);
    simple_assert_eq("correct struct returned", s.c, 908);
    simple_assert("new_struct_nones() returns None", !s.d);

    auto opt_u8 = o.accepts_option_u8(std::nullopt);
    simple_assert("accepts_option_u8 is idempotent", !opt_u8.has_value());
    opt_u8 = o.accepts_option_u8(5);
    simple_assert("accepts_option_u8 is idempotent", opt_u8.value() == 5);
}
