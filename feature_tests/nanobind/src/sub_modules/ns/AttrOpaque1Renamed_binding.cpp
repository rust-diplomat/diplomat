#include "diplomat_nanobind_common.hpp"


#include "ns/AttrOpaque1Renamed.hpp"


namespace ns{

void add_AttrOpaque1Renamed_binding(nb::handle mod) {
    PyType_Slot ns_AttrOpaque1Renamed_slots[] = {
        {Py_tp_free, (void *)ns::AttrOpaque1Renamed::operator delete },
        {Py_tp_dealloc, (void *)diplomat_tp_dealloc},
        {0, nullptr}};
    
    nb::class_<ns::AttrOpaque1Renamed>(mod, "AttrOpaque1Renamed", nb::type_slots(ns_AttrOpaque1Renamed_slots))
    	.def_prop_ro("abirenamed", &ns::AttrOpaque1Renamed::abirenamed)
    	.def_prop_ro("method", &ns::AttrOpaque1Renamed::method_renamed)
    	.def(nb::new_(&ns::AttrOpaque1Renamed::totally_not_new))
    	.def("use_namespaced", &ns::AttrOpaque1Renamed::use_namespaced, "_n"_a)
    	.def("use_unnamespaced", &ns::AttrOpaque1Renamed::use_unnamespaced, "_un"_a);
}


}
