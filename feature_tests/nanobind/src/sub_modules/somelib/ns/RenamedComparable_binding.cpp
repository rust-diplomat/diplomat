#include "diplomat_nanobind_common.hpp"


#include "ns/RenamedComparable.hpp"

namespace somelib::ns {
void add_RenamedComparable_binding(nb::module_ mod) {
    PyType_Slot somelib_ns_RenamedComparable_slots[] = {
        {Py_tp_free, (void *)somelib::ns::RenamedComparable::operator delete },
        {Py_tp_dealloc, (void *)diplomat_tp_dealloc},
        {0, nullptr}};
    
    nb::class_<somelib::ns::RenamedComparable> opaque(mod, "RenamedComparable", nb::type_slots(somelib_ns_RenamedComparable_slots));
    opaque
        .def(nb::self == nb::self)
            .def(nb::self != nb::self)
            .def(nb::self <= nb::self)
            .def(nb::self >= nb::self)
            .def(nb::self < nb::self)
            .def(nb::self > nb::self)
        .def_static("new", &somelib::ns::RenamedComparable::new_, "int"_a);
}

} 