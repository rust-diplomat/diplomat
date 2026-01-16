#include "diplomat_nanobind_common.hpp"


#include "ns/RenamedBlockOverride.hpp"

namespace somelib::ns {
void add_RenamedBlockOverride_binding(nb::module_ mod) {
    PyType_Slot somelib_ns_RenamedBlockOverride_slots[] = {
        {Py_tp_free, (void *)somelib::ns::RenamedBlockOverride::operator delete },
        {Py_tp_dealloc, (void *)diplomat_tp_dealloc},
        {0, nullptr}};
    
    nb::class_<somelib::ns::RenamedBlockOverride> opaque(mod, "RenamedBlockOverride", nb::type_slots(somelib_ns_RenamedBlockOverride_slots));
    ;
    opaque.def("special_function", &somelib::ns::RenamedBlockOverride::special_function);
}

} 