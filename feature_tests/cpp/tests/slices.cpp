#include <iostream>
#include "../include/MyString.hpp"
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
}