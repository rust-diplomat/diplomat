#include "diplomat_nanobind_common.hpp"


#include "OptionOpaqueChar.hpp"


void add_OptionOpaqueChar_binding(nb::module_ mod) {
    PyType_Slot OptionOpaqueChar_slots[] = {
        {Py_tp_free, (void *)OptionOpaqueChar::operator delete },
        {Py_tp_dealloc, (void *)diplomat_tp_dealloc},
        {0, nullptr}};
    
    nb::class_<OptionOpaqueChar>(mod, "OptionOpaqueChar", nb::type_slots(OptionOpaqueChar_slots))
        .def("assert_char", &OptionOpaqueChar::assert_char, "ch"_a);
}

