#include "diplomat_nanobind_common.hpp"


#include "RefListParameter.hpp"


void add_RefListParameter_binding(nb::module_ mod) {
    PyType_Slot RefListParameter_slots[] = {
        {Py_tp_free, (void *)RefListParameter::operator delete },
        {Py_tp_dealloc, (void *)diplomat_tp_dealloc},
        {0, nullptr}};
    
    nb::class_<RefListParameter>(mod, "RefListParameter", nb::type_slots(RefListParameter_slots));
}

