#include "diplomat_nanobind_common.hpp"


#include "mylib/MethodOverloading.hpp"

namespace somelib::mylib {
void add_MethodOverloading_binding(nb::module_ mod) {
    nb::class_<somelib::mylib::MethodOverloading> opaque(mod, "MethodOverloading");
    opaque
        .def_static("from_", std::move(maybe_op_unwrap(nb::overload_cast<int32_t>(&somelib::mylib::MethodOverloading::from))), "_v"_a)
        .def_static("from_", std::move(maybe_op_unwrap(nb::overload_cast<int64_t>(&somelib::mylib::MethodOverloading::from))), "_v"_a)
        .def_static("from_", std::move(maybe_op_unwrap(nb::overload_cast<uint32_t>(&somelib::mylib::MethodOverloading::from))), "_v"_a);
}

} 