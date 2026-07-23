#include "diplomat_nanobind_common.hpp"


#include "OpaqueMut.hpp"

namespace somelib {
void add_OpaqueMut_binding(nb::module_ mod) {
    nb::class_<somelib::OpaqueMut> opaque(mod, "OpaqueMut");
    opaque
        .def(nb::new_(std::move(maybe_op_unwrap(&somelib::OpaqueMut::new_))));
}

} 