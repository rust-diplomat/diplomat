#include "diplomat_nanobind_common.hpp"


#include "OptionEnum.hpp"
#include "OptionInputStruct.hpp"
#include "OptionOpaque.hpp"


void add_OptionOpaque_binding(nb::module_ mod) {
    PyType_Slot OptionOpaque_slots[] = {
        {Py_tp_free, (void *)OptionOpaque::operator delete },
        {Py_tp_dealloc, (void *)diplomat_tp_dealloc},
        {0, nullptr}};
    
    nb::class_<OptionOpaque>(mod, "OptionOpaque", nb::type_slots(OptionOpaque_slots))
        .def_static("accepts_multiple_option_enum", &OptionOpaque::accepts_multiple_option_enum, "sentinel1"_a, "arg1"_a= nb::none(), "arg2"_a= nb::none(), "arg3"_a= nb::none(), "sentinel2"_a)
        .def_static("accepts_option_enum", &OptionOpaque::accepts_option_enum, "arg"_a= nb::none(), "sentinel"_a)
        .def_static("accepts_option_input_struct", &OptionOpaque::accepts_option_input_struct, "arg"_a= nb::none(), "sentinel"_a)
        .def_static("accepts_option_primitive", &OptionOpaque::accepts_option_primitive, "arg"_a= nb::none(), "sentinel"_a)
        .def_static("accepts_option_str", &OptionOpaque::accepts_option_str, "arg"_a= nb::none(), "sentinel"_a)
        .def_static("accepts_option_str_slice", &OptionOpaque::accepts_option_str_slice, "arg"_a= nb::none(), "sentinel"_a)
        .def_static("accepts_option_u8", &OptionOpaque::accepts_option_u8, "arg"_a= nb::none(), "sentinel"_a)
        .def("assert_integer", &OptionOpaque::assert_integer, "i"_a)
        .def_static("new", &OptionOpaque::new_, "i"_a)
        .def_static("new_none", &OptionOpaque::new_none)
        .def_static("new_struct", &OptionOpaque::new_struct)
        .def_static("new_struct_nones", &OptionOpaque::new_struct_nones)
        .def("option_i32", &OptionOpaque::option_i32)
        .def("option_isize", &OptionOpaque::option_isize)
        .def_static("option_opaque_argument", &OptionOpaque::option_opaque_argument, "arg"_a= nb::none())
        .def("option_u32", &OptionOpaque::option_u32)
        .def("option_usize", &OptionOpaque::option_usize)
        .def_static("returns", &OptionOpaque::returns)
        .def("returns_none_self", &OptionOpaque::returns_none_self, nb::rv_policy::reference_internal)
        .def_static("returns_option_input_struct", &OptionOpaque::returns_option_input_struct)
        .def("returns_some_self", &OptionOpaque::returns_some_self, nb::rv_policy::reference_internal);
}

