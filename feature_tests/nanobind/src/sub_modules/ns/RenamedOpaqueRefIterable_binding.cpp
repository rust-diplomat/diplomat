#include "diplomat_nanobind_common.hpp"


#include "ns/RenamedOpaqueRefIterable.hpp"


namespace ns{

void add_RenamedOpaqueRefIterable_binding(nb::module_ mod) {
    PyType_Slot ns_RenamedOpaqueRefIterable_slots[] = {
        {Py_tp_free, (void *)ns::RenamedOpaqueRefIterable::operator delete },
        {Py_tp_dealloc, (void *)diplomat_tp_dealloc},
        {0, nullptr}};
    
    nb::class_<ns::RenamedOpaqueRefIterable>(mod, "RenamedOpaqueRefIterable", nb::type_slots(ns_RenamedOpaqueRefIterable_slots))
        .def("__iter__", &ns::RenamedOpaqueRefIterable::iter, nb::keep_alive<0, 1>())
        .def(nb::new_(&ns::RenamedOpaqueRefIterable::new_), "size"_a);
}


}
