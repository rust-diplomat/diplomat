#include <iostream>
#include "../include/MyStruct.hpp"
#include "../include/MyEnum.hpp"
#include "../include/Opaque.hpp"
#include "../include/StructArithmetic.hpp"
#include "../include/ns/RenamedOpaqueArithmetic.hpp"
#include "../include/BigStructWithStuff.hpp"
#include "../include/PrimitiveStructVec.hpp"
#include "../include/PrimitiveStruct.hpp"
#include "../include/CyclicStructA.hpp"
#include "assert.hpp"

using namespace somelib;

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

    BigStructWithStuff bigArr[] {
        {
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
        }
    };

    diplomat::span<const BigStructWithStuff> in(bigArr, 2);


    BigStructWithStuff::assert_slice(in, 4);

    auto primitive_vec = PrimitiveStructVec::new_();
    primitive_vec->push({
            .x = 1.0f,
            .a = true,
            .b = 'a',
            .c = 0,
            .d = 0,
            .e = 0
        });
    primitive_vec->push({
        .x = 2.0f,
        .a = false,
        .b = '\0',
        .c = 0,
        .d = 0,
        .e = 0
    });
    primitive_vec->push({.x = -1.0f});

    PrimitiveStruct::mutable_slice(primitive_vec->as_slice_mut());
    simple_assert_eq("primitiveArr cumulative sum", primitive_vec->get(2).x, 2.0f);
    simple_assert_eq("primitiveArr alternating bool", primitive_vec->get(0).a, false);
    simple_assert_eq("primitiveArr alternating bool 2", primitive_vec->get(1).a, true);
    simple_assert_eq("primitiveArr DiplomatChar", (int)primitive_vec->get(2).b, 2);
    simple_assert_eq("primitiveArr isize", primitive_vec->get(0).d, 101);
    simple_assert_eq("primitiveArr DiplomatByte", primitive_vec->get(1).e, 3);

    CyclicStructA cyclic_arr[] {
        {
            .a = {
                .field = 3
            }
        },
        {
            .a = {
                .field = 2
            }
        },
        {
            .a = {
                .field = 4
            }
        },
        {
            .a = {
                .field = 5
            }
        }
    };

    diplomat::span<const CyclicStructA> cyclic_span(cyclic_arr, 4);
    uint8_t cyclic_a_sum = CyclicStructA::nested_slice(cyclic_span);
    simple_assert_eq("CyclicStructA slice sum", cyclic_a_sum, 14);

    PrimitiveStruct primitive_one = {
        .x = 0.0f,
        .a = true,
        .b = 'a',
    };
    PrimitiveStruct primitive_two = {
        .d = 0
    };
    primitive_one.mutable_ref(primitive_two);
    simple_assert_eq("Mutable ref in", primitive_one.a, false);
    simple_assert_eq("Mutable ref in", primitive_two.d, 1);

    MyStruct struct_ref_one = MyStruct {
        .a = 25,
    };
    MyStruct struct_ref_two = MyStruct {};

    struct_ref_one.takes_mut(struct_ref_two);
    simple_assert_eq("Mutable ref cloned in", struct_ref_one.a, 0);
    simple_assert_eq("Mutable ref cloned in", struct_ref_two.c, 100);

    struct_ref_one.takes_const(struct_ref_two);
    simple_assert_eq("Mutable ref cloned in", struct_ref_two.c, 0);
}
