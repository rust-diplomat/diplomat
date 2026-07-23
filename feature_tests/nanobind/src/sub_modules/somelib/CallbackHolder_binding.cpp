#include "diplomat_nanobind_common.hpp"


#include "CallbackHolder.hpp"

namespace somelib {
void add_CallbackHolder_binding(nb::module_ mod) {
    nb::class_<somelib::CallbackHolder> opaque(mod, "CallbackHolder");
    opaque
        .def(nb::new_(std::move(maybe_op_unwrap(&somelib::CallbackHolder::new_))), "func"_a)
        .def("call", &somelib::CallbackHolder::call, "a"_a);
}

} 