#include "diplomat_nanobind_common.hpp"


#include "ns/RenamedTestOpaque.hpp"


namespace ns{

void add_RenamedTestOpaque_binding(nb::module_ mod) {
    PyType_Slot ns_RenamedTestOpaque_slots[] = {
        {Py_tp_free, (void *)ns::RenamedTestOpaque::operator delete },
        {Py_tp_dealloc, (void *)diplomat_tp_dealloc},
        {0, nullptr}};
    
    nb::class_<ns::RenamedTestOpaque>(mod, "RenamedTestOpaque", nb::type_slots(ns_RenamedTestOpaque_slots));
}


}
