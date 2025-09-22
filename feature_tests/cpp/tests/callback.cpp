#include <iostream>
#include "../include/CallbackWrapper.hpp"
#include "../include/CallbackHolder.hpp"
#include "../include/MutableCallbackHolder.hpp"
#include "../include/MyString.hpp"
#include "../include/Opaque.hpp"
#include "../include/MyStructContainingAnOption.hpp"
#include "../include/PrimitiveStructVec.hpp"
#include "assert.hpp"

using namespace somelib;

int main(int argc, char *argv[])
{

    CallbackWrapper o;
    int32_t tmp = 0;
    {
        auto out = o.test_multi_arg_callback([&tmp](int32_t a)
                                             { tmp = a; return a+5; }, 5);
        simple_assert_eq("multi_arg_callback arg ", tmp, 15);
        simple_assert_eq("multi_arg_callback output", out, 20);
    }
    {
        tmp = 1;
        auto out = o.test_no_args([&tmp]()
                                  { tmp++; });
        simple_assert_eq("test_no_args arg ", tmp, 2);
        simple_assert_eq("test_no_args output", out, -5);
    }
    {
        tmp = 0;
        auto out = o.test_cb_with_struct([&tmp](CallbackTestingStruct a)
                                         { tmp = a.y; return a.x+a.y; });
        simple_assert_eq("test_cb_with_struct arg ", tmp, 5);
        simple_assert_eq("test_cb_with_struct output", out, 6);
    }
    {
        tmp = 0;
        int32_t tmp2 = 0;
        auto out = o.test_multiple_cb_args([&tmp]()
                                           { tmp = 4; return 10; }, [&tmp2](int32_t a)
                                           {tmp2 = a; return a+1; });
        simple_assert_eq("test_multiple_cb_args arg ", tmp, 4);
        simple_assert_eq("test_multiple_cb_args arg2 ", tmp2, 5);
        simple_assert_eq("test_multiple_cb_args output", out, 16);
    }
    {
        auto out = o.test_str_cb_arg([](std::string_view a)
                                     { return a.length(); });
        simple_assert_eq("test_str_cb_arg output", out, 7);
    }
    {
        std::vector<uint8_t> vector {1,2,3,4};

        o.test_slice_cb_arg(diplomat::span<const uint8_t>(vector.data(), vector.size()), [](diplomat::span<const uint8_t> sp){
            simple_assert_eq("test_cb_size", sp.size(), 4);
            simple_assert_eq("test_cb_entry", sp.data()[3], 4);
        });
    }
    {
        int copied = 0;
        // TODO: Make C++ reject this by using move_only_function in c++23.
        // We cannot reject this in earlier standards due to a defect in std::function.
        // See: https://lesleylai.info/en/const-correcness-std-function/
        auto cb = CallbackHolder::new_([copied](int32_t a) mutable { copied += a; return copied;});
        simple_assert_eq("mutable cb object", cb->call(5), 5);
        simple_assert_eq("mutable cb object", cb->call(5), 10);
    }
    {
        int copied = 0;
        auto cb = MutableCallbackHolder::new_([copied](int32_t a) mutable { copied += a; return copied;});
        simple_assert_eq("mutable cb object", cb->call(5), 5);
        simple_assert_eq("mutable cb object", cb->call(5), 10);
    }
    {
        auto opaque = MyString::new_("Bananna");
        simple_assert_eq("opaque cb arg", opaque->borrow(), "Bananna");
        o.test_opaque_cb_arg([](MyString& op) {
            op.set_str("split");
        }, *opaque);
        simple_assert_eq("opaque cb arg", opaque->borrow(), "split");
    }
    {
        std::array<diplomat::string_view_for_slice, 2> names{"Banana", "Apple"};
        auto opaque = MyString::new_from_first(names);
        simple_assert_eq("opaque cb arg", opaque->borrow(), "Banana");
    }

    {
        o.test_result_output([]() {
            return diplomat::Ok<std::monostate>();
        });
    }
    {
        o.test_result_usize_output([]() {
            return diplomat::Ok<size_t>(0);
        });
    }
    {
        o.test_option_output([]() {
            return std::optional<std::monostate>(std::nullopt);
        });
    }
    {
        o.test_diplomat_option_output([]() {
            return std::optional<uint32_t>(0);
        });
    }
    {
        o.test_diplomat_result([]() {
            return diplomat::Err<size_t>(10);
        });
    }
    auto a = Opaque::from_str("This is a test value.").ok().value();
    auto ptr = a.get();
    {
        auto str = o.test_option_opaque([ptr]() {
            return ptr;
        });
        simple_assert_eq("Test opaque string passing", str, "\"This is a test value.\"");
    }
    {
        auto str = o.test_result_opaque([ptr]() {
            return diplomat::Ok<const Opaque&>(*ptr);
        });
        simple_assert_eq("Test opaque string passing", str, "\"This is a test value.\"");
    }
    {
        auto str = o.test_opaque_result_error([ptr]() {
            return diplomat::Err<const Opaque&>(*ptr);
        });
        simple_assert_eq("Test opaque string passing", str, "\"This is a test value.\"");
    }
    {
        o.test_inner_conversion([]() {
            auto st = MyStructContainingAnOption::filled();
            st.a->a = 42;
            return diplomat::Ok(st);
        });
    }
    {
        o.test_str_conversion([]() {
            return diplomat::Ok<std::string_view>("Slice conversion test string");
        });
    }

    auto floatVec = std::vector<double>{ 1.f, 2.f, 3.f, 4.f };
    {
        o.test_slice_conversion([floatVec]() {
            return diplomat::Ok(diplomat::span<const double>({floatVec.data(), floatVec.size()}));
        });
    }

    auto primitive_vec = PrimitiveStructVec::new_();
    auto primitive_vec_ptr = primitive_vec.get();
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
        .b = 'f',
        .c = 0,
        .d = 0,
        .e = 0
    });
    primitive_vec->push({.x = -1.0f});

    {
        o.test_struct_slice_conversion([primitive_vec_ptr]() {
            return diplomat::Ok(primitive_vec_ptr->as_slice());
        });
    }
}
