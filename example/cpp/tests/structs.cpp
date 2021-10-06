#include <iostream>
#include "../MyStruct.hpp"
#include "../Opaque.hpp"
#include "assert.hpp"

int main(int argc, char *argv[]) {
    Opaque o = Opaque::new_();
    MyStruct s = MyStruct::new_();

    o.assert_struct(s);

    assert_eq("struct values", s.a, 17);
    assert_eq("struct values", s.b, true);
    assert_eq("struct values", s.c, 209);
    assert_eq("struct values", s.d, 1234);
    assert_eq("struct values", s.e, 5991);
}
