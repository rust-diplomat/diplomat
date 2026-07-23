#include "diplomat_nanobind_common.hpp"


#include "ns/RenamedMyIndexer.hpp"

namespace somelib::ns {
void add_RenamedMyIndexer_binding(nb::module_ mod) {
    nb::class_<somelib::ns::RenamedMyIndexer> opaque(mod, "RenamedMyIndexer");
    opaque
        .def(nb::new_(std::move(maybe_op_unwrap(&somelib::ns::RenamedMyIndexer::new_))), "v"_a)
        .def("__getitem__", [](somelib::ns::RenamedMyIndexer* self, size_t index) {
                auto out = self->operator[] (index);
                if (!out.has_value()) {
                    throw nb::index_error("Could not get index.");
                } else {
                    return out;
                }
            }, "i"_a)
        .def("__getitem__", [](somelib::ns::RenamedMyIndexer* self, std::string_view index) {
                auto out = self->operator[] (index);
                if (!out.has_value()) {
                    throw nb::index_error("Could not get index.");
                } else {
                    return out;
                }
            }, "s"_a);
}

} 