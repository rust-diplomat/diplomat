#include <iostream>
#include "../include/MyString.hpp"
#include "../include/Float64Vec.hpp"
#include "assert.hpp"

using namespace somelib;


int main(int argc, char* argv[]) {
    auto a = MyString::new_("Test");
    auto b = MyString::new_(" String ");
    auto c = MyString::new_("end.");

    const MyString* arr[] = {
        a.get(), b.get(), c.get()
    };
    diplomat::span<const MyString*> in(arr, 3);
    simple_assert_eq("Slice of opaques", MyString::slice_of_opaques(in), "Test String end.");

    const MyString* optional_arr[] = {
        a.get(), nullptr, b.get()
    };
    diplomat::span<const MyString*> optional_in(optional_arr, 3);
    simple_assert_eq("Optional slice of opaques", MyString::optional_slice_of_opaques(optional_in), "Some(MyString(\"Test\")) None Some(MyString(\" String \")) ");

    const double float_arr[] = { 1.0, 2.0, 3.0 };
    const double other_float_arr[] = {4.5, 6.2, 3.4};
    auto float_vec_a = Float64Vec::new_(diplomat::span<const double>(float_arr, 3));
    auto float_vec_b = Float64Vec::new_(diplomat::span<const double>(other_float_arr, 3));
    const Float64Vec* array_of_vec[] = {
        float_vec_a.get(),
        float_vec_b.get()
    };
    simple_assert_eq("Include other opaque", MyString::other_opaque_type(diplomat::span<const Float64Vec*>(array_of_vec, 2)), "Float64Vec([1.0, 2.0, 3.0])Float64Vec([4.5, 6.2, 3.4])");
}