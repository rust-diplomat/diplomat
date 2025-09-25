#include "diplomat_nanobind_common.hpp"


#include "CallbackHolder.hpp"


void add_CallbackHolder_binding(nb::module_ mod) {
    PyType_Slot CallbackHolder_slots[] = {
        {Py_tp_free, (void *)CallbackHolder::operator delete },
        {Py_tp_dealloc, (void *)diplomat_tp_dealloc},
        {0, nullptr}};
    
    nb::class_<CallbackHolder>(mod, "CallbackHolder", nb::type_slots(CallbackHolder_slots))
        .def("call", &CallbackHolder::call, "a"_a)
        .def(nb::new_(&CallbackHolder::new_), "func"_a);
}

