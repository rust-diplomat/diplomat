#include "diplomat_nanobind_common.hpp"


#include "FallibleOpaqueConstructor.hpp"

namespace somelib {
void add_FallibleOpaqueConstructor_binding(nb::module_ mod) {
    nb::class_<somelib::FallibleOpaqueConstructor> st(mod, "FallibleOpaqueConstructor");
    st
        .def_rw("x", &somelib::FallibleOpaqueConstructor::x)
        .def(nb::new_(std::move(maybe_op_unwrap(&somelib::FallibleOpaqueConstructor::ctor))));
}

} 