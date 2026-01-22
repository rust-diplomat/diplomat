#include "diplomat_nanobind_common.hpp"


#include "CallbackWrapper.hpp"
#include "MyString.hpp"

namespace somelib {
void add_CallbackWrapper_binding(nb::module_ mod) {
    nb::class_<somelib::CallbackWrapper> st(mod, "CallbackWrapper");
    st
        .def(nb::init<>())
        .def(nb::init<bool>(), "cant_be_empty"_a.none())
        .def_rw("cant_be_empty", &somelib::CallbackWrapper::cant_be_empty)
        .def_static("test_cb_with_struct", &somelib::CallbackWrapper::test_cb_with_struct, "f"_a)
        .def_static("test_diplomat_option_output", &somelib::CallbackWrapper::test_diplomat_option_output, "t"_a)
        .def_static("test_diplomat_result", &somelib::CallbackWrapper::test_diplomat_result, "t"_a)
        .def_static("test_inner_conversion", &somelib::CallbackWrapper::test_inner_conversion, "t"_a)
        .def_static("test_multi_arg_callback", &somelib::CallbackWrapper::test_multi_arg_callback, "f"_a, "x"_a)
        .def_static("test_multiple_cb_args", &somelib::CallbackWrapper::test_multiple_cb_args, "f"_a, "g"_a)
        .def_static("test_no_args", &somelib::CallbackWrapper::test_no_args, "h"_a)
        .def_static("test_opaque_cb_arg", &somelib::CallbackWrapper::test_opaque_cb_arg, "cb"_a, "a"_a)
        .def_static("test_opaque_result_error", &somelib::CallbackWrapper::test_opaque_result_error, "t"_a)
        .def_static("test_option_opaque", &somelib::CallbackWrapper::test_option_opaque, "t"_a)
        .def_static("test_option_output", &somelib::CallbackWrapper::test_option_output, "t"_a)
        .def_static("test_result_opaque", &somelib::CallbackWrapper::test_result_opaque, "t"_a)
        .def_static("test_result_output", &somelib::CallbackWrapper::test_result_output, "t"_a)
        .def_static("test_result_usize_output", &somelib::CallbackWrapper::test_result_usize_output, "t"_a)
        .def_static("test_slice_cb_arg", &somelib::CallbackWrapper::test_slice_cb_arg, "arg"_a, "f"_a)
        .def_static("test_slice_conversion", &somelib::CallbackWrapper::test_slice_conversion, "t"_a)
        .def_static("test_str_cb_arg", &somelib::CallbackWrapper::test_str_cb_arg, "f"_a)
        .def_static("test_str_conversion", &somelib::CallbackWrapper::test_str_conversion, "t"_a)
        .def_static("test_struct_slice_conversion", &somelib::CallbackWrapper::test_struct_slice_conversion, "t"_a);
}

} 