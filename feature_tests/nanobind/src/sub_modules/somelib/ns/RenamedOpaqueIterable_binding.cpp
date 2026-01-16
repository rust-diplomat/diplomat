#include "diplomat_nanobind_common.hpp"


#include "ns/RenamedOpaqueIterable.hpp"

namespace somelib::ns {
void add_RenamedOpaqueIterable_binding(nb::module_ mod) {
    PyType_Slot somelib_ns_RenamedOpaqueIterable_slots[] = {
        {Py_tp_free, (void *)somelib::ns::RenamedOpaqueIterable::operator delete },
        {Py_tp_dealloc, (void *)diplomat_tp_dealloc},
        {0, nullptr}};
    
    nb::class_<somelib::ns::RenamedOpaqueIterable> opaque(mod, "RenamedOpaqueIterable", nb::type_slots(somelib_ns_RenamedOpaqueIterable_slots));
    opaque
        .def("__iter__", &somelib::ns::RenamedOpaqueIterable::iter, nb::keep_alive<0, 1>())
        .def(nb::new_(&somelib::ns::RenamedOpaqueIterable::new_), "size"_a);
}

} 