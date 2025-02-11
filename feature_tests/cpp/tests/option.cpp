#include <iostream>
#include "../include/OptionOpaqueChar.hpp"
#include "../include/OptionOpaque.hpp"
#include "../include/OptionStruct.hpp"
#include "assert.hpp"

int main(int argc, char *argv[])
{

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

    auto opt_u8 = OptionOpaque::accepts_option_u8(std::nullopt);
    simple_assert("accepts_option_u8 is idempotent", !opt_u8.has_value());
    opt_u8 = OptionOpaque::accepts_option_u8(5);
    simple_assert("accepts_option_u8 is idempotent", opt_u8.value() == 5);
    auto opt_enum = OptionOpaque::accepts_option_enum(std::nullopt);
    simple_assert("accepts_option_enum is idempotent", !opt_enum.has_value());
    opt_enum = OptionOpaque::accepts_option_enum(OptionEnum::Foo);
    simple_assert("accepts_option_enum is idempotent", opt_enum.value() == OptionEnum::Foo);
    auto opt_struct = OptionOpaque::accepts_option_input_struct(std::nullopt);
    simple_assert("accepts_option_input_struct is idempotent", !opt_struct.has_value());
    opt_struct = OptionOpaque::accepts_option_input_struct(std::optional<OptionInputStruct>({std::optional(1), std::nullopt, std::optional(OptionEnum::Foo)}));
    simple_assert("accepts_option_input_struct is idempotent", opt_struct.value().a == 1);
    simple_assert("accepts_option_input_struct is idempotent", !opt_struct.value().b.has_value());
    simple_assert("accepts_option_input_struct is idempotent", opt_struct.value().c == OptionEnum::Foo);

    opt_struct = OptionOpaque::returns_option_input_struct();
    simple_assert("returns_option_input_struct returns the right values", opt_struct.value().a == 6);
    simple_assert("returns_option_input_struct returns the right values", !opt_struct.value().b.has_value());
    simple_assert("returns_option_input_struct returns the right values", opt_struct.value().c == OptionEnum::Bar);

    using namespace std::string_view_literals;

    std::array<std::string_view, 2> string_array{"string1"sv, "string2"sv};
    diplomat::span<const std::string_view> arg{string_array};
    auto str_slice_result = OptionOpaque::accepts_option_str_slice(std::make_optional(std::move(arg)));
    simple_assert("option_str_slice functions", str_slice_result);

    simple_assert_eq("Optional string param (Some)", OptionOpaque::accepts_option_str(std::make_optional("accepts optional string!")), 24);
    simple_assert_eq("Optional string param (None)", OptionOpaque::accepts_option_str(std::nullopt), 0);

    constexpr uint32_t array[]{1, 2, 3, 4};
    simple_assert_eq("Optional primitive param (Some)", OptionOpaque::accepts_option_primitive(std::make_optional(diplomat::span{array, 4})), 10);
    simple_assert_eq("Optional primitive param (None)", OptionOpaque::accepts_option_primitive(std::nullopt), -1);
}
