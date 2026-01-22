#include "diplomat_nanobind_common.hpp"


#include "ns/RenamedMyIterable.hpp"

namespace somelib::ns {
void add_RenamedMyIterable_binding(nb::module_ mod) {
    PyType_Slot somelib_ns_RenamedMyIterable_slots[] = {
        {Py_tp_free, (void *)somelib::ns::RenamedMyIterable::operator delete },
        {Py_tp_dealloc, (void *)diplomat_tp_dealloc},
        {0, nullptr}};
    
    nb::class_<somelib::ns::RenamedMyIterable> opaque(mod, "RenamedMyIterable", nb::type_slots(somelib_ns_RenamedMyIterable_slots));
    opaque
        .def("__len__", &somelib::ns::RenamedMyIterable::__len__)
        .def("__iter__", &somelib::ns::RenamedMyIterable::iter, nb::keep_alive<0, 1>())
        .def(nb::new_(&somelib::ns::RenamedMyIterable::new_), "x"_a);
}

} 