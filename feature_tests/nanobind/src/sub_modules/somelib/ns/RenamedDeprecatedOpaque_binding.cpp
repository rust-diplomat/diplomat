#include "diplomat_nanobind_common.hpp"


#include "ns/RenamedDeprecatedOpaque.hpp"

namespace somelib::ns {
void add_RenamedDeprecatedOpaque_binding(nb::module_ mod) {
    PyType_Slot somelib_ns_RenamedDeprecatedOpaque_slots[] = {
        {Py_tp_free, (void *)somelib::ns::RenamedDeprecatedOpaque::operator delete },
        {Py_tp_dealloc, (void *)diplomat_tp_dealloc},
        {0, nullptr}};
    
    nb::class_<somelib::ns::RenamedDeprecatedOpaque> opaque(mod, "RenamedDeprecatedOpaque", nb::type_slots(somelib_ns_RenamedDeprecatedOpaque_slots));
    ;
}

} 