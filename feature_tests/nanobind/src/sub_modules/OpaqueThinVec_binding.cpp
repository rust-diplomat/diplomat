#include "diplomat_nanobind_common.hpp"


#include "OpaqueThinVec.hpp"


void add_OpaqueThinVec_binding(nb::module_ mod) {
    PyType_Slot OpaqueThinVec_slots[] = {
        {Py_tp_free, (void *)OpaqueThinVec::operator delete },
        {Py_tp_dealloc, (void *)diplomat_tp_dealloc},
        {0, nullptr}};
    
    nb::class_<OpaqueThinVec>(mod, "OpaqueThinVec", nb::type_slots(OpaqueThinVec_slots))
        .def("__len__", &OpaqueThinVec::__len__)
        .def(nb::new_(&OpaqueThinVec::create), "a"_a, "b"_a, "c"_a)
        .def_prop_ro("first", &OpaqueThinVec::first)
        .def("__getitem__", &OpaqueThinVec::operator[], "idx"_a, nb::rv_policy::reference_internal)
        .def("__iter__", &OpaqueThinVec::iter, nb::keep_alive<0, 1>());
}

