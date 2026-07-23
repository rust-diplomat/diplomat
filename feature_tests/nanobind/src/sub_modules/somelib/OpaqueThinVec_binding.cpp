#include "diplomat_nanobind_common.hpp"


#include "OpaqueThinVec.hpp"

namespace somelib {
void add_OpaqueThinVec_binding(nb::module_ mod) {
    nb::class_<somelib::OpaqueThinVec> opaque(mod, "OpaqueThinVec");
    opaque
        .def("__len__", &somelib::OpaqueThinVec::__len__)
        .def(nb::new_(std::move(maybe_op_unwrap(&somelib::OpaqueThinVec::create))), "a"_a, "b"_a, "c"_a)
        .def_prop_ro("first", &somelib::OpaqueThinVec::first)
        .def("__getitem__", [](somelib::OpaqueThinVec* self, size_t index) {
                auto out = self->operator[] (index);
                if (out == nullptr) {
                    throw nb::index_error("Could not get index.");
                } else {
                    return out;
                }}, "idx"_a, nb::rv_policy::reference_internal)
        .def("__iter__", std::move(maybe_op_unwrap(&somelib::OpaqueThinVec::iter)), nb::keep_alive<0, 1>());
}

} 