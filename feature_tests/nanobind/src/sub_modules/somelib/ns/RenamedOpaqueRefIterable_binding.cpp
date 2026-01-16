#include "diplomat_nanobind_common.hpp"


#include "ns/RenamedOpaqueRefIterable.hpp"

namespace somelib::ns {
void add_RenamedOpaqueRefIterable_binding(nb::module_ mod) {
    PyType_Slot somelib_ns_RenamedOpaqueRefIterable_slots[] = {
        {Py_tp_free, (void *)somelib::ns::RenamedOpaqueRefIterable::operator delete },
        {Py_tp_dealloc, (void *)diplomat_tp_dealloc},
        {0, nullptr}};
    
    nb::class_<somelib::ns::RenamedOpaqueRefIterable> opaque(mod, "RenamedOpaqueRefIterable", nb::type_slots(somelib_ns_RenamedOpaqueRefIterable_slots));
    opaque
        .def("__iter__", &somelib::ns::RenamedOpaqueRefIterable::iter, nb::keep_alive<0, 1>())
        .def(nb::new_(&somelib::ns::RenamedOpaqueRefIterable::new_), "size"_a);
}

} 