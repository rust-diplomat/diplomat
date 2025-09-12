#include "diplomat_nanobind_common.hpp"


#include "ns/RenamedAttrOpaque2.hpp"


namespace ns{

void add_RenamedAttrOpaque2_binding(nb::module_ mod) {
    PyType_Slot ns_RenamedAttrOpaque2_slots[] = {
        {Py_tp_free, (void *)ns::RenamedAttrOpaque2::operator delete },
        {Py_tp_dealloc, (void *)diplomat_tp_dealloc},
        {0, nullptr}};
    
    nb::class_<ns::RenamedAttrOpaque2>(mod, "RenamedAttrOpaque2", nb::type_slots(ns_RenamedAttrOpaque2_slots));
}


}
