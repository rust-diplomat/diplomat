#include <iostream>
#include "../include/MyStruct.hpp"
#include "../include/Opaque.hpp"
#include "assert.hpp"

int main(int argc, char *argv[]) {
    Opaque o = Opaque::new_();
    std::string str = "hello";
    MyStruct s = MyStruct::new_(str);

    o.assert_struct(s);

    simple_assert_eq("struct values", s.a, 17);
    simple_assert_eq("struct values", s.b, true);
    simple_assert_eq("struct values", s.c, 209);
    simple_assert_eq("struct values", s.d, 1234);
    simple_assert_eq("struct values", s.e, 5991);
    simple_assert_eq("struct values", (uint32_t)s.f, (uint32_t)U'餐');
}
