#include "diplomat_nanobind_common.hpp"


#include "ns/RenamedMyIterable.hpp"

namespace somelib::ns {
void add_RenamedMyIterable_binding(nb::module_ mod) {
    nb::class_<somelib::ns::RenamedMyIterable> opaque(mod, "RenamedMyIterable");
    opaque
        .def("__len__", &somelib::ns::RenamedMyIterable::__len__)
        .def(nb::new_(std::move(maybe_op_unwrap(&somelib::ns::RenamedMyIterable::new_))), "x"_a)
        .def("__iter__", std::move(maybe_op_unwrap(&somelib::ns::RenamedMyIterable::iter)), nb::keep_alive<0, 1>());
}

} 