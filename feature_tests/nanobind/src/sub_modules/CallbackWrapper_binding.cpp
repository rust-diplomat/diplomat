#include "diplomat_nanobind_common.hpp"


#include "CallbackWrapper.hpp"
#include "MyString.hpp"


void add_CallbackWrapper_binding(nb::module_ mod) {
    nb::class_<CallbackWrapper>(mod, "CallbackWrapper")
        .def(nb::init<>())
        .def(nb::init<bool>(), "cant_be_empty"_a.none())
        .def_rw("cant_be_empty", &CallbackWrapper::cant_be_empty)
        .def_static("test_cb_with_struct", &CallbackWrapper::test_cb_with_struct, "f"_a)
        .def_static("test_diplomat_option_output", &CallbackWrapper::test_diplomat_option_output, "t"_a)
        .def_static("test_diplomat_result", &CallbackWrapper::test_diplomat_result, "t"_a)
        .def_static("test_inner_conversion", &CallbackWrapper::test_inner_conversion, "t"_a)
        .def_static("test_multi_arg_callback", &CallbackWrapper::test_multi_arg_callback, "f"_a, "x"_a)
        .def_static("test_multiple_cb_args", &CallbackWrapper::test_multiple_cb_args, "f"_a, "g"_a)
        .def_static("test_no_args", &CallbackWrapper::test_no_args, "h"_a)
        .def_static("test_opaque_cb_arg", &CallbackWrapper::test_opaque_cb_arg, "cb"_a, "a"_a)
        .def_static("test_opaque_result_error", &CallbackWrapper::test_opaque_result_error, "t"_a)
        .def_static("test_option_opaque", &CallbackWrapper::test_option_opaque, "t"_a)
        .def_static("test_option_output", &CallbackWrapper::test_option_output, "t"_a)
        .def_static("test_result_opaque", &CallbackWrapper::test_result_opaque, "t"_a)
        .def_static("test_result_output", &CallbackWrapper::test_result_output, "t"_a)
        .def_static("test_result_usize_output", &CallbackWrapper::test_result_usize_output, "t"_a)
        .def_static("test_slice_cb_arg", &CallbackWrapper::test_slice_cb_arg, "arg"_a, "f"_a)
        .def_static("test_slice_conversion", &CallbackWrapper::test_slice_conversion, "t"_a)
        .def_static("test_str_cb_arg", &CallbackWrapper::test_str_cb_arg, "f"_a)
        .def_static("test_str_conversion", &CallbackWrapper::test_str_conversion, "t"_a)
        .def_static("test_struct_slice_conversion", &CallbackWrapper::test_struct_slice_conversion, "t"_a);
}

