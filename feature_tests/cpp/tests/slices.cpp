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

    std::vector<double> written = float_vec_a->write_to();
    simple_assert_eq("write_to len", written.size(), 3);
    simple_assert_eq("write_to[0]", written[0], 1.0);
    simple_assert_eq("write_to[1]", written[1], 2.0);
    simple_assert_eq("write_to[2]", written[2], 3.0);

    float_vec_b->write_to_write(written);
    simple_assert_eq("write_to_write len", written.size(), 6);
    simple_assert_eq("write_to_write[3]", written[3], 4.5);
    simple_assert_eq("write_to_write[4]", written[4], 6.2);
    simple_assert_eq("write_to_write[5]", written[5], 3.4);

    std::vector<double> multi = float_vec_a->write_multi();
    simple_assert_eq("write_multi len", multi.size(), 6);
    simple_assert_eq("write_multi[0]", multi[0], 1.0);
    simple_assert_eq("write_multi[2]", multi[2], 3.0);
    simple_assert_eq("write_multi[3]", multi[3], 2.0);
    simple_assert_eq("write_multi[5]", multi[5], 6.0);

    auto ok_res = float_vec_a->try_write_to(true);
    simple_assert("try_write_to ok", ok_res.is_ok());
    std::vector<double> ok_vec = std::move(ok_res).ok().value();
    simple_assert_eq("try_write_to ok len", ok_vec.size(), 3);
    simple_assert_eq("try_write_to ok[1]", ok_vec[1], 2.0);

    auto err_res = float_vec_a->try_write_to(false);
    simple_assert("try_write_to err", err_res.is_err());

    std::vector<double> try_out;
    auto ok_write_res = float_vec_b->try_write_to_write(true, try_out);
    simple_assert("try_write_to_write ok", ok_write_res.is_ok());
    simple_assert_eq("try_write_to_write len", try_out.size(), 3);
    simple_assert_eq("try_write_to_write[0]", try_out[0], 4.5);

    auto some_res = float_vec_a->maybe_write_to(true);
    simple_assert("maybe_write_to some", some_res.has_value());
    simple_assert_eq("maybe_write_to some len", some_res.value().size(), 3);

    auto none_res = float_vec_a->maybe_write_to(false);
    simple_assert("maybe_write_to none", !none_res.has_value());
}