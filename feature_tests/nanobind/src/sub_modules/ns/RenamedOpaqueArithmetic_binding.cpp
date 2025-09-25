#include "diplomat_nanobind_common.hpp"


#include "ns/RenamedOpaqueArithmetic.hpp"


namespace ns{

void add_RenamedOpaqueArithmetic_binding(nb::module_ mod) {
    PyType_Slot ns_RenamedOpaqueArithmetic_slots[] = {
        {Py_tp_free, (void *)ns::RenamedOpaqueArithmetic::operator delete },
        {Py_tp_dealloc, (void *)diplomat_tp_dealloc},
        {0, nullptr}};
    
    nb::class_<ns::RenamedOpaqueArithmetic>(mod, "RenamedOpaqueArithmetic", nb::type_slots(ns_RenamedOpaqueArithmetic_slots))
        .def(nb::self + nb::self)
        .def(nb::self += nb::self, nb::rv_policy::none)
        .def(nb::self / nb::self)
        .def(nb::self /= nb::self, nb::rv_policy::none)
        .def_static("make", nb::overload_cast<int32_t, int32_t>(&ns::RenamedOpaqueArithmetic::make), "x"_a, "y"_a)
        .def_static("make", nb::overload_cast<float, float>(&ns::RenamedOpaqueArithmetic::make), "x"_a, "y"_a)
        .def(nb::self * nb::self)
        .def(nb::self *= nb::self, nb::rv_policy::none)
        .def(nb::self - nb::self)
        .def(nb::self -= nb::self, nb::rv_policy::none)
        .def("x", &ns::RenamedOpaqueArithmetic::x)
        .def("y", &ns::RenamedOpaqueArithmetic::y);
}


}
