#include "diplomat_nanobind_common.hpp"


#include "ns/RenamedAttrOpaque2.hpp"

namespace somelib::ns {
void add_RenamedAttrOpaque2_binding(nb::module_ mod) {
    PyType_Slot somelib_ns_RenamedAttrOpaque2_slots[] = {
        {Py_tp_free, (void *)somelib::ns::RenamedAttrOpaque2::operator delete },
        {Py_tp_dealloc, (void *)diplomat_tp_dealloc},
        {0, nullptr}};
    
    nb::class_<somelib::ns::RenamedAttrOpaque2> opaque(mod, "RenamedAttrOpaque2", nb::type_slots(somelib_ns_RenamedAttrOpaque2_slots));
    ;
}

} 