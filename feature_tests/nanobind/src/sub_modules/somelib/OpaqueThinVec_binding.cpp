#include "diplomat_nanobind_common.hpp"


#include "OpaqueThinVec.hpp"

namespace somelib {
void add_OpaqueThinVec_binding(nb::module_ mod) {
    PyType_Slot somelib_OpaqueThinVec_slots[] = {
        {Py_tp_free, (void *)somelib::OpaqueThinVec::operator delete },
        {Py_tp_dealloc, (void *)diplomat_tp_dealloc},
        {0, nullptr}};
    
    nb::class_<somelib::OpaqueThinVec> opaque(mod, "OpaqueThinVec", nb::type_slots(somelib_OpaqueThinVec_slots));
    opaque
        .def("__len__", &somelib::OpaqueThinVec::__len__)
        .def(nb::new_(&somelib::OpaqueThinVec::create), "a"_a, "b"_a, "c"_a)
        .def_prop_ro("first", &somelib::OpaqueThinVec::first)
        .def("__getitem__", &somelib::OpaqueThinVec::operator[], "idx"_a, nb::rv_policy::reference_internal)
        .def("__iter__", &somelib::OpaqueThinVec::iter, nb::keep_alive<0, 1>());
}

} 