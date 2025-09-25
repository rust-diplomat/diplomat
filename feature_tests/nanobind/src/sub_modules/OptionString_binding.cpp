#include "diplomat_nanobind_common.hpp"


#include "OptionString.hpp"


void add_OptionString_binding(nb::module_ mod) {
    PyType_Slot OptionString_slots[] = {
        {Py_tp_free, (void *)OptionString::operator delete },
        {Py_tp_dealloc, (void *)diplomat_tp_dealloc},
        {0, nullptr}};
    
    nb::class_<OptionString>(mod, "OptionString", nb::type_slots(OptionString_slots))
        .def("borrow", &OptionString::borrow)
        .def_static("new", &OptionString::new_, "diplomat_str"_a)
        .def("write", &OptionString::write);
}

