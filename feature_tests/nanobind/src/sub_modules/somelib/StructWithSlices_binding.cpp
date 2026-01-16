#include "diplomat_nanobind_common.hpp"


#include "StructWithSlices.hpp"

namespace somelib {
void add_StructWithSlices_binding(nb::module_ mod) {
    nb::class_<somelib::StructWithSlices> st(mod, "StructWithSlices");
    st
        .def(nb::init<>())
        .def(nb::init<std::string_view, somelib::diplomat::span<const uint16_t>>(), "first"_a.none(),  "second"_a.none())
        .def_rw("first", &somelib::StructWithSlices::first)
        .def_rw("second", &somelib::StructWithSlices::second)
        .def("return_last", &somelib::StructWithSlices::return_last);
}

} 