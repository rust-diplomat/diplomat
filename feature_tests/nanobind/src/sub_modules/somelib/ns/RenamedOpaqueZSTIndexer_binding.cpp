#include "diplomat_nanobind_common.hpp"


#include "ns/RenamedOpaqueZSTIndexer.hpp"

namespace somelib::ns {
void add_RenamedOpaqueZSTIndexer_binding(nb::module_ mod) {
    nb::class_<somelib::ns::RenamedOpaqueZSTIndexer> opaque(mod, "RenamedOpaqueZSTIndexer");
    opaque
        .def(nb::new_(std::move(maybe_op_unwrap(&somelib::ns::RenamedOpaqueZSTIndexer::new_))))
        .def("__getitem__", [](somelib::ns::RenamedOpaqueZSTIndexer* self, size_t index) {
                auto out = map_inner(self->operator[] (index));
                if (out == nullptr) {
                    throw nb::index_error("Could not get index.");
                } else {
                    return out;
                }}, "idx"_a);
}

} 