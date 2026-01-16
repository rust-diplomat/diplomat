#include "diplomat_nanobind_common.hpp"


#include "OpaqueMutexedString.hpp"

namespace somelib {
void add_OpaqueMutexedString_binding(nb::module_ mod) {
    PyType_Slot somelib_OpaqueMutexedString_slots[] = {
        {Py_tp_free, (void *)somelib::OpaqueMutexedString::operator delete },
        {Py_tp_dealloc, (void *)diplomat_tp_dealloc},
        {0, nullptr}};
    
    nb::class_<somelib::OpaqueMutexedString> opaque(mod, "OpaqueMutexedString", nb::type_slots(somelib_OpaqueMutexedString_slots));
    opaque
        .def("borrow", &somelib::OpaqueMutexedString::borrow, nb::rv_policy::reference_internal)
        .def_static("borrow_other", &somelib::OpaqueMutexedString::borrow_other, "other"_a, nb::rv_policy::reference)
        .def("borrow_self_or_other", &somelib::OpaqueMutexedString::borrow_self_or_other, "other"_a, nb::rv_policy::reference_internal)
        .def("change", &somelib::OpaqueMutexedString::change, "number"_a)
        .def("dummy_str", &somelib::OpaqueMutexedString::dummy_str)
        .def_static("from_usize", &somelib::OpaqueMutexedString::from_usize, "number"_a)
        .def("get_len_and_add", &somelib::OpaqueMutexedString::get_len_and_add, "other"_a)
        .def("to_unsigned_from_unsigned", &somelib::OpaqueMutexedString::to_unsigned_from_unsigned, "input"_a)
        .def("wrapper", &somelib::OpaqueMutexedString::wrapper);
}

} 