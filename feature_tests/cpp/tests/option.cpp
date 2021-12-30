#include <iostream>
#include "../include/OptionOpaqueChar.hpp"
#include "../include/OptionOpaque.hpp"
#include "assert.hpp"

int main(int argc, char *argv[]) {

    std::optional<OptionOpaque> o = OptionOpaque::new_(1415);
    o.value().assert_integer(1415);

    o = OptionOpaque::new_none();
    simple_assert("new_none() returns None", !o.has_value());

    OptionStruct s = OptionOpaque::new_struct();
    s.a.value().assert_integer(101); 
    s.b.value().assert_char(U'餐'); 
    simple_assert_eq("correct struct returned", s.c, 904);
    s.d.value().assert_integer(926535);

    s = OptionOpaque::new_struct_nones();

    simple_assert("new_struct_nones() returns None", !s.a.has_value());
    simple_assert("new_struct_nones() returns None", !s.b.has_value());
    simple_assert_eq("correct struct returned", s.c, 908);
    simple_assert("new_struct_nones() returns None", !s.d.has_value());
}
