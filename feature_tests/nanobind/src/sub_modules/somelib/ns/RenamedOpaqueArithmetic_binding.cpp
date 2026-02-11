#include "diplomat_nanobind_common.hpp"


#include "ns/RenamedOpaqueArithmetic.hpp"

namespace somelib::ns {
void add_RenamedOpaqueArithmetic_binding(nb::module_ mod) {
    PyType_Slot somelib_ns_RenamedOpaqueArithmetic_slots[] = {
        {Py_tp_free, (void *)somelib::ns::RenamedOpaqueArithmetic::operator delete },
        {Py_tp_dealloc, (void *)diplomat_tp_dealloc},
        {0, nullptr}};
    
    nb::class_<somelib::ns::RenamedOpaqueArithmetic> opaque(mod, "RenamedOpaqueArithmetic", nb::type_slots(somelib_ns_RenamedOpaqueArithmetic_slots));
    opaque
        .def("__add__", std::move(maybe_op_unwrap(&somelib::ns::RenamedOpaqueArithmetic::operator+)), nb::is_operator())
        .def(nb::self += nb::self, nb::rv_policy::none)
        .def("__truediv__", std::move(maybe_op_unwrap(&somelib::ns::RenamedOpaqueArithmetic::operator/)), nb::is_operator())
        .def(nb::self /= nb::self, nb::rv_policy::none)
        .def_static("make", std::move(maybe_op_unwrap(nb::overload_cast<int32_t, int32_t>(&somelib::ns::RenamedOpaqueArithmetic::make))), "x"_a, "y"_a)
        .def_static("make", std::move(maybe_op_unwrap(nb::overload_cast<float, float>(&somelib::ns::RenamedOpaqueArithmetic::make))), "x"_a, "y"_a)
        .def_static("make", std::move(maybe_op_unwrap(nb::overload_cast<float, bool>(&somelib::ns::RenamedOpaqueArithmetic::make))), "x"_a, "z"_a)
        .def("__mul__", std::move(maybe_op_unwrap(&somelib::ns::RenamedOpaqueArithmetic::operator*)), nb::is_operator())
        .def(nb::self *= nb::self, nb::rv_policy::none)
        .def("__sub__", std::move(maybe_op_unwrap(&somelib::ns::RenamedOpaqueArithmetic::operator-)), nb::is_operator())
        .def(nb::self -= nb::self, nb::rv_policy::none)
        .def("x", &somelib::ns::RenamedOpaqueArithmetic::x)
        .def("y", &somelib::ns::RenamedOpaqueArithmetic::y);
}

} 