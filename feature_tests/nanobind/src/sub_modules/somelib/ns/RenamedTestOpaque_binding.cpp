#include "diplomat_nanobind_common.hpp"


#include "ns/RenamedTestOpaque.hpp"

namespace somelib::ns {
void add_RenamedTestOpaque_binding(nb::module_ mod) {
    PyType_Slot somelib_ns_RenamedTestOpaque_slots[] = {
        {Py_tp_free, (void *)somelib::ns::RenamedTestOpaque::operator delete },
        {Py_tp_dealloc, (void *)diplomat_tp_dealloc},
        {0, nullptr}};
    
    nb::class_<somelib::ns::RenamedTestOpaque> opaque(mod, "RenamedTestOpaque", nb::type_slots(somelib_ns_RenamedTestOpaque_slots));
    ;
}

} 