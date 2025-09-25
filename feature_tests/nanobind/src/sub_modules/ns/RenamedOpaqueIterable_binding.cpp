#include "diplomat_nanobind_common.hpp"


#include "ns/RenamedOpaqueIterable.hpp"


namespace ns{

void add_RenamedOpaqueIterable_binding(nb::module_ mod) {
    PyType_Slot ns_RenamedOpaqueIterable_slots[] = {
        {Py_tp_free, (void *)ns::RenamedOpaqueIterable::operator delete },
        {Py_tp_dealloc, (void *)diplomat_tp_dealloc},
        {0, nullptr}};
    
    nb::class_<ns::RenamedOpaqueIterable>(mod, "RenamedOpaqueIterable", nb::type_slots(ns_RenamedOpaqueIterable_slots))
        .def("__iter__", &ns::RenamedOpaqueIterable::iter, nb::keep_alive<0, 1>())
        .def(nb::new_(&ns::RenamedOpaqueIterable::new_), "size"_a);
}


}
