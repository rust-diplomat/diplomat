#include "diplomat_nanobind_common.hpp"


#include "Two.hpp"


void add_Two_binding(nb::module_ mod) {
    PyType_Slot Two_slots[] = {
        {Py_tp_free, (void *)Two::operator delete },
        {Py_tp_dealloc, (void *)diplomat_tp_dealloc},
        {0, nullptr}};
    
    nb::class_<Two>(mod, "Two", nb::type_slots(Two_slots));
}

