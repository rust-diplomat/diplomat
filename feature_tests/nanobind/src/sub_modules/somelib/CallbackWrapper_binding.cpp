#include "diplomat_nanobind_common.hpp"


#include "CallbackTestingStruct.hpp"
#include "CallbackWrapper.hpp"
#include "FFIError.hpp"
#include "MyString.hpp"
#include "MyStruct.hpp"
#include "Opaque.hpp"
#include "PrimitiveStruct.hpp"

namespace somelib {
void add_CallbackWrapper_binding(nb::module_ mod) {
    nb::class_<somelib::CallbackWrapper> st(mod, "CallbackWrapper");
    st
        .def(nb::init<>())
        .def(nb::init<bool>(), "cant_be_empty"_a.none())
        .def_rw("cant_be_empty", &somelib::CallbackWrapper::cant_be_empty)
        .def_static("test_cb_with_struct_fallible", &somelib::CallbackWrapper::test_cb_with_struct_fallible, "f"_a)
        .def_static("test_ffi_error", &somelib::CallbackWrapper::test_ffi_error, "t"_a)
        .def_static("test_ffi_error_as_ok", &somelib::CallbackWrapper::test_ffi_error_as_ok, "t"_a)
        .def_static("test_multi_arg_callback_fallible", &somelib::CallbackWrapper::test_multi_arg_callback_fallible, "f"_a, "x"_a)
        .def_static("test_multiple_cb_args_fallible", &somelib::CallbackWrapper::test_multiple_cb_args_fallible, "f"_a, "g"_a)
        .def_static("test_no_args_fallible", &somelib::CallbackWrapper::test_no_args_fallible, "h"_a)
        .def_static("test_opaque_cb_arg_fallible", swap_lvalue_wrap(&somelib::CallbackWrapper::test_opaque_cb_arg_fallible), "cb"_a, "a"_a)
        .def_static("test_result_opaque", &somelib::CallbackWrapper::test_result_opaque, "t"_a)
        .def_static("test_result_option_struct_conversion", &somelib::CallbackWrapper::test_result_option_struct_conversion, "t"_a)
        .def_static("test_slice_conversion", &somelib::CallbackWrapper::test_slice_conversion, "t"_a)
        .def_static("test_str_cb_arg_fallible", &somelib::CallbackWrapper::test_str_cb_arg_fallible, "f"_a)
        .def_static("test_str_conversion", &somelib::CallbackWrapper::test_str_conversion, "t"_a)
        .def_static("test_struct_slice_conversion", &somelib::CallbackWrapper::test_struct_slice_conversion, "t"_a);
}

} 