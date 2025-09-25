#include "diplomat_nanobind_common.hpp"


#include "MutableCallbackHolder.hpp"


void add_MutableCallbackHolder_binding(nb::module_ mod) {
    PyType_Slot MutableCallbackHolder_slots[] = {
        {Py_tp_free, (void *)MutableCallbackHolder::operator delete },
        {Py_tp_dealloc, (void *)diplomat_tp_dealloc},
        {0, nullptr}};
    
    nb::class_<MutableCallbackHolder>(mod, "MutableCallbackHolder", nb::type_slots(MutableCallbackHolder_slots))
        .def("call", &MutableCallbackHolder::call, "a"_a)
        .def(nb::new_(&MutableCallbackHolder::new_), "func"_a);
}

