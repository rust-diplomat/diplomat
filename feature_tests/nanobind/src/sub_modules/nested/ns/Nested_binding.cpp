#include "diplomat_nanobind_common.hpp"


#include "nested/ns/Nested.hpp"


namespace nested::ns{

void add_Nested_binding(nb::module_ mod) {
    PyType_Slot nested_ns_Nested_slots[] = {
        {Py_tp_free, (void *)nested::ns::Nested::operator delete },
        {Py_tp_dealloc, (void *)diplomat_tp_dealloc},
        {0, nullptr}};
    
    nb::class_<nested::ns::Nested>(mod, "Nested", nb::type_slots(nested_ns_Nested_slots));
}


}
