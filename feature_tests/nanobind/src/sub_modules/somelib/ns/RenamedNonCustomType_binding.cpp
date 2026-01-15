#include "diplomat_nanobind_common.hpp"


#include "ns/RenamedNonCustomType.hpp"

namespace somelib::ns {
void add_RenamedNonCustomType_binding(nb::module_ mod) {
    PyType_Slot somelib_ns_RenamedNonCustomType_slots[] = {
        {Py_tp_free, (void *)somelib::ns::RenamedNonCustomType::operator delete },
        {Py_tp_dealloc, (void *)diplomat_tp_dealloc},
        {0, nullptr}};
    
    nb::class_<somelib::ns::RenamedNonCustomType>(mod, "RenamedNonCustomType", nb::type_slots(somelib_ns_RenamedNonCustomType_slots));
}

} 