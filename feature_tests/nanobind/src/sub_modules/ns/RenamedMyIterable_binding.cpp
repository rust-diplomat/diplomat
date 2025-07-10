#include "diplomat_nanobind_common.hpp"


#include "ns/RenamedMyIterable.hpp"


namespace ns{

void add_RenamedMyIterable_binding(nb::handle mod) {
    PyType_Slot ns_RenamedMyIterable_slots[] = {
        {Py_tp_free, (void *)ns::RenamedMyIterable::operator delete },
        {Py_tp_dealloc, (void *)diplomat_tp_dealloc},
        {0, nullptr}};
    
    nb::class_<ns::RenamedMyIterable>(mod, "RenamedMyIterable", nb::type_slots(ns_RenamedMyIterable_slots))
    	.def("__len__", &ns::RenamedMyIterable::__len__)
    	.def("__iter__", &ns::RenamedMyIterable::iter, nb::rv_policy::reference_internal)
    	.def(nb::new_(&ns::RenamedMyIterable::new_), "x"_a);
}


}
