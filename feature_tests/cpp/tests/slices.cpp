#include <iostream>
#include <array>
#include "../include/MyString.hpp"
#include "../include/Float64Vec.hpp"
#include "assert.hpp"

using namespace somelib;


int main(int argc, char* argv[]) {
    std::array<MyString, 3> arr{MyString::new_("Test"), MyString::new_(" String "),MyString::new_("end.") };

    diplomat::span<MyString> in(arr.data(), arr.size());
    simple_assert_eq("Slice of opaques", MyString::slice_of_opaques(in), "Test String end.");

    std::vector<diplomat::Optional<MyStringRef>> optional_vec{diplomat::Optional(arr[0].as_ref()), diplomat::Optional<MyStringRef>(std::nullopt), diplomat::Optional(arr[1].as_ref())};
    diplomat::span<diplomat::Optional<MyStringRef>> optional_in(optional_vec.data(), optional_vec.size());
    simple_assert_eq("Optional slice of opaques", MyString::optional_slice_of_opaques(optional_in), "Some(MyString(\"Test\")) None Some(MyString(\" String \")) ");

    std::array float_arr{1.0, 2.0, 3.0};
    std::array other_float_arr{4.5, 6.2, 3.4};
    auto float_vec_a = Float64Vec::new_(diplomat::span<const double>(float_arr.data(), float_arr.size()));
    auto float_vec_b = Float64Vec::new_(diplomat::span<const double>(other_float_arr.data(), other_float_arr.size()));
    std::array array_of_vec{std::move(float_vec_a ), std::move(float_vec_b) };
    simple_assert_eq("Include other opaque", MyString::other_opaque_type(diplomat::span<Float64Vec>(array_of_vec.data(), array_of_vec.size())), "Float64Vec([1.0, 2.0, 3.0])Float64Vec([4.5, 6.2, 3.4])");
}
