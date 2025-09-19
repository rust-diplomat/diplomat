#include "diplomat_nanobind_common.hpp"


#include "nested/ns2/Nested.hpp"


namespace nested::ns2{

void add_Nested_binding(nb::module_ mod) {
    PyType_Slot nested_ns2_Nested_slots[] = {
        {Py_tp_free, (void *)nested::ns2::Nested::operator delete },
        {Py_tp_dealloc, (void *)diplomat_tp_dealloc},
        {0, nullptr}};
    
    nb::class_<nested::ns2::Nested>(mod, "Nested", nb::type_slots(nested_ns2_Nested_slots));
}


}
