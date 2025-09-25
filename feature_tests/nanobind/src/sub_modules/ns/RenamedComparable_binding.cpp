#include "diplomat_nanobind_common.hpp"


#include "ns/RenamedComparable.hpp"


namespace ns{

void add_RenamedComparable_binding(nb::module_ mod) {
    PyType_Slot ns_RenamedComparable_slots[] = {
        {Py_tp_free, (void *)ns::RenamedComparable::operator delete },
        {Py_tp_dealloc, (void *)diplomat_tp_dealloc},
        {0, nullptr}};
    
    nb::class_<ns::RenamedComparable>(mod, "RenamedComparable", nb::type_slots(ns_RenamedComparable_slots))
        .def(nb::self == nb::self)
            .def(nb::self != nb::self)
            .def(nb::self <= nb::self)
            .def(nb::self >= nb::self)
            .def(nb::self < nb::self)
            .def(nb::self > nb::self)
        .def_static("new", &ns::RenamedComparable::new_, "int"_a);
}


}
