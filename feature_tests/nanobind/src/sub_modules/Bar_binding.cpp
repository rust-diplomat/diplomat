#include "diplomat_nanobind_common.hpp"


#include "Bar.hpp"


void add_Bar_binding(nb::module_ mod) {
    PyType_Slot Bar_slots[] = {
        {Py_tp_free, (void *)Bar::operator delete },
        {Py_tp_dealloc, (void *)diplomat_tp_dealloc},
        {0, nullptr}};
    
    nb::class_<Bar>(mod, "Bar", nb::type_slots(Bar_slots))
        .def_prop_ro("foo", &Bar::foo);
}

