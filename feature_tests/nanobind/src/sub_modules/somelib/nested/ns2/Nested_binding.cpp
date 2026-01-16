#include "diplomat_nanobind_common.hpp"


#include "nested/ns2/Nested.hpp"

namespace somelib::nested::ns2 {
void add_Nested_binding(nb::module_ mod) {
    PyType_Slot somelib_nested_ns2_Nested_slots[] = {
        {Py_tp_free, (void *)somelib::nested::ns2::Nested::operator delete },
        {Py_tp_dealloc, (void *)diplomat_tp_dealloc},
        {0, nullptr}};
    
    nb::class_<somelib::nested::ns2::Nested> opaque(mod, "Nested", nb::type_slots(somelib_nested_ns2_Nested_slots));
    ;
}

} 