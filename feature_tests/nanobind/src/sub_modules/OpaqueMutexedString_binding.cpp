#include "diplomat_nanobind_common.hpp"


#include "OpaqueMutexedString.hpp"


void add_OpaqueMutexedString_binding(nb::module_ mod) {
    PyType_Slot OpaqueMutexedString_slots[] = {
        {Py_tp_free, (void *)OpaqueMutexedString::operator delete },
        {Py_tp_dealloc, (void *)diplomat_tp_dealloc},
        {0, nullptr}};
    
    nb::class_<OpaqueMutexedString>(mod, "OpaqueMutexedString", nb::type_slots(OpaqueMutexedString_slots))
        .def("borrow", &OpaqueMutexedString::borrow, nb::rv_policy::reference_internal)
        .def_static("borrow_other", &OpaqueMutexedString::borrow_other, "other"_a, nb::rv_policy::reference)
        .def("borrow_self_or_other", &OpaqueMutexedString::borrow_self_or_other, "other"_a, nb::rv_policy::reference_internal)
        .def("change", &OpaqueMutexedString::change, "number"_a)
        .def("dummy_str", &OpaqueMutexedString::dummy_str)
        .def_static("from_usize", &OpaqueMutexedString::from_usize, "number"_a)
        .def("get_len_and_add", &OpaqueMutexedString::get_len_and_add, "other"_a)
        .def("to_unsigned_from_unsigned", &OpaqueMutexedString::to_unsigned_from_unsigned, "input"_a)
        .def("wrapper", &OpaqueMutexedString::wrapper);
}

