#include "diplomat_nanobind_common.hpp"


#include "MutableCallbackHolder.hpp"

namespace somelib {
void add_MutableCallbackHolder_binding(nb::module_ mod) {
    nb::class_<somelib::MutableCallbackHolder> opaque(mod, "MutableCallbackHolder");
    opaque
        .def(nb::new_(std::move(maybe_op_unwrap(&somelib::MutableCallbackHolder::new_))), "func"_a)
        .def("call", &somelib::MutableCallbackHolder::call, "a"_a);
}

} 