#include <iostream>
#include "../include/MyStruct.hpp"
#include "../include/MyEnum.hpp"
#include "../include/Opaque.hpp"
#include "../include/StructArithmetic.hpp"
#include "../include/ns/RenamedOpaqueArithmetic.hpp"
#include "../include/BigStructWithStuff.hpp"
#include "assert.hpp"

int main(int argc, char* argv[]) {

    static_assert(std::is_same_v<diplomat::as_ffi_t<ns::RenamedOpaqueArithmetic>, ns::capi::RenamedOpaqueArithmetic*>);

    std::unique_ptr<Opaque> o = Opaque::new_();
    MyStruct s = MyStruct::new_();

    o->assert_struct(s);

    simple_assert_eq("struct values", s.a, 17);
    simple_assert_eq("struct values", s.b, true);
    simple_assert_eq("struct values", s.c, 209);
    simple_assert_eq("struct values", s.d, 1234);
    simple_assert_eq("struct values", s.e, 5991);
    simple_assert_eq("struct values", (uint32_t)s.f, (uint32_t)U'餐');
    simple_assert_eq("struct values", (uint32_t)s.g.AsFFI(), (uint32_t)MyEnum(MyEnum::B).AsFFI());

    simple_assert_eq("enum fn", s.g.into_value(), -1);
    simple_assert_eq("struct fn", s.into_a(), 17);


    MyStruct default_s;
    simple_assert_eq("default struct values", default_s.g.into_value(), MyEnum(MyEnum::D).into_value());

    auto a = StructArithmetic{ 1, 2 };
    auto b = StructArithmetic{ 2, 3 };
    {
        auto r = a + b;

        simple_assert_eq("adding x", r.x, 3);
        simple_assert_eq("adding y", r.y, 5);
        r += a;
        simple_assert_eq("self-adding x", r.x, 4);
        simple_assert_eq("self-adding y", r.y, 7);
    }

    diplomat::span<const BigStructWithStuff> in({
        .first=0,
        .second=1,
        .third=2,
        .fourth = {
            .first=200,
            .second=100
        },
        .fifth=3
    }, {
        .first=5,
        .second=4,
        .third=3,
        .fourth = {
            .first = 100,
            .second = 200
        },
        .fifth = 2
    });


    BigStructWithStuff::assert_slice(in, in.size(), 4);
}
