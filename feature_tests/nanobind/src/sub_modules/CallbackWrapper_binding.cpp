#include "diplomat_nanobind_common.hpp"


#include "CallbackWrapper.hpp"


void add_CallbackWrapper_binding(nb::handle mod) {
    nb::class_<CallbackWrapper>(mod, "CallbackWrapper")
        .def(nb::init<>())
        .def(nb::init<bool>(), "cant_be_empty"_a.none())
        .def_rw("cant_be_empty", &CallbackWrapper::cant_be_empty)
    	.def_static("test_cb_with_struct", &CallbackWrapper::test_cb_with_struct, "f"_a)
    	.def_static("test_multi_arg_callback", &CallbackWrapper::test_multi_arg_callback, "f"_a, "x"_a)
    	.def_static("test_multiple_cb_args", &CallbackWrapper::test_multiple_cb_args, "f"_a, "g"_a)
    	.def_static("test_no_args", &CallbackWrapper::test_no_args, "h"_a)
    	.def_static("test_opaque_cb_arg", &CallbackWrapper::test_opaque_cb_arg, "cb"_a, "a"_a)
    	.def_static("test_slice_cb_arg", &CallbackWrapper::test_slice_cb_arg, "arg"_a, "f"_a)
    	.def_static("test_str_cb_arg", &CallbackWrapper::test_str_cb_arg, "f"_a);
}

