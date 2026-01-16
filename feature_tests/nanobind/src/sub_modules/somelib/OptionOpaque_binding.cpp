#include "diplomat_nanobind_common.hpp"


#include "BorrowingOptionStruct.hpp"
#include "OptionEnum.hpp"
#include "OptionInputStruct.hpp"
#include "OptionOpaque.hpp"

namespace somelib {
void add_OptionOpaque_binding(nb::module_ mod) {
    PyType_Slot somelib_OptionOpaque_slots[] = {
        {Py_tp_free, (void *)somelib::OptionOpaque::operator delete },
        {Py_tp_dealloc, (void *)diplomat_tp_dealloc},
        {0, nullptr}};
    
    nb::class_<somelib::OptionOpaque> opaque(mod, "OptionOpaque", nb::type_slots(somelib_OptionOpaque_slots));
    opaque
        .def_static("accepts_borrowing_option_struct", &somelib::OptionOpaque::accepts_borrowing_option_struct, "arg"_a)
        .def_static("accepts_multiple_option_enum", &somelib::OptionOpaque::accepts_multiple_option_enum, "sentinel1"_a, "arg1"_a= nb::none(), "arg2"_a= nb::none(), "arg3"_a= nb::none(), "sentinel2"_a)
        .def_static("accepts_option_enum", &somelib::OptionOpaque::accepts_option_enum, "arg"_a= nb::none(), "sentinel"_a)
        .def_static("accepts_option_input_struct", &somelib::OptionOpaque::accepts_option_input_struct, "arg"_a= nb::none(), "sentinel"_a)
        .def_static("accepts_option_primitive", &somelib::OptionOpaque::accepts_option_primitive, "arg"_a= nb::none(), "sentinel"_a)
        .def_static("accepts_option_str", &somelib::OptionOpaque::accepts_option_str, "arg"_a= nb::none(), "sentinel"_a)
        .def_static("accepts_option_str_slice", &somelib::OptionOpaque::accepts_option_str_slice, "arg"_a= nb::none(), "sentinel"_a)
        .def_static("accepts_option_u8", &somelib::OptionOpaque::accepts_option_u8, "arg"_a= nb::none(), "sentinel"_a)
        .def("assert_integer", &somelib::OptionOpaque::assert_integer, "i"_a)
        .def_static("new", &somelib::OptionOpaque::new_, "i"_a)
        .def_static("new_none", &somelib::OptionOpaque::new_none)
        .def_static("new_struct", &somelib::OptionOpaque::new_struct)
        .def_static("new_struct_nones", &somelib::OptionOpaque::new_struct_nones)
        .def("option_i32", &somelib::OptionOpaque::option_i32)
        .def("option_isize", &somelib::OptionOpaque::option_isize)
        .def_static("option_opaque_argument", &somelib::OptionOpaque::option_opaque_argument, "arg"_a= nb::none())
        .def("option_u32", &somelib::OptionOpaque::option_u32)
        .def("option_usize", &somelib::OptionOpaque::option_usize)
        .def_static("returns", &somelib::OptionOpaque::returns)
        .def("returns_none_self", &somelib::OptionOpaque::returns_none_self, nb::rv_policy::reference_internal)
        .def_static("returns_option_input_struct", &somelib::OptionOpaque::returns_option_input_struct)
        .def("returns_some_self", &somelib::OptionOpaque::returns_some_self, nb::rv_policy::reference_internal);
}

} 