#include "diplomat_nanobind_common.hpp"


#include "nested/ns/Nested.hpp"

namespace somelib::nested::ns {
void add_Nested_binding(nb::module_ mod) {
    PyType_Slot somelib_nested_ns_Nested_slots[] = {
        {Py_tp_free, (void *)somelib::nested::ns::Nested::operator delete },
        {Py_tp_dealloc, (void *)diplomat_tp_dealloc},
        {0, nullptr}};
    
    nb::class_<somelib::nested::ns::Nested> opaque(mod, "Nested", nb::type_slots(somelib_nested_ns_Nested_slots));
    ;
}

} 