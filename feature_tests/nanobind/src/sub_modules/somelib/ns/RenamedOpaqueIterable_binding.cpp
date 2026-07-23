#include "diplomat_nanobind_common.hpp"


#include "ns/RenamedOpaqueIterable.hpp"

namespace somelib::ns {
void add_RenamedOpaqueIterable_binding(nb::module_ mod) {
    nb::class_<somelib::ns::RenamedOpaqueIterable> opaque(mod, "RenamedOpaqueIterable");
    opaque
        .def(nb::new_(std::move(maybe_op_unwrap(&somelib::ns::RenamedOpaqueIterable::new_))), "size"_a)
        .def("__iter__", std::move(maybe_op_unwrap(&somelib::ns::RenamedOpaqueIterable::iter)), nb::keep_alive<0, 1>());
}

} 