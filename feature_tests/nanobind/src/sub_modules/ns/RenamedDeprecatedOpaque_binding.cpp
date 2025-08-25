#include "diplomat_nanobind_common.hpp"


#include "ns/RenamedDeprecatedOpaque.hpp"


namespace ns{

void add_RenamedDeprecatedOpaque_binding(nb::handle mod) {
    PyType_Slot ns_RenamedDeprecatedOpaque_slots[] = {
        {Py_tp_free, (void *)ns::RenamedDeprecatedOpaque::operator delete },
        {Py_tp_dealloc, (void *)diplomat_tp_dealloc},
        {0, nullptr}};
    
    nb::class_<ns::RenamedDeprecatedOpaque>(mod, "RenamedDeprecatedOpaque", nb::type_slots(ns_RenamedDeprecatedOpaque_slots));
}


}
