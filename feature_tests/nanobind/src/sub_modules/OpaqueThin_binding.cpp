#include "diplomat_nanobind_common.hpp"


#include "OpaqueThin.hpp"


void add_OpaqueThin_binding(nb::module_ mod) {
    PyType_Slot OpaqueThin_slots[] = {
        {Py_tp_free, (void *)OpaqueThin::operator delete },
        {Py_tp_dealloc, (void *)diplomat_tp_dealloc},
        {0, nullptr}};
    
    nb::class_<OpaqueThin>(mod, "OpaqueThin", nb::type_slots(OpaqueThin_slots))
        .def_prop_ro("a", &OpaqueThin::a)
        .def_prop_ro("b", &OpaqueThin::b)
        .def_prop_ro("c", &OpaqueThin::c);
}

