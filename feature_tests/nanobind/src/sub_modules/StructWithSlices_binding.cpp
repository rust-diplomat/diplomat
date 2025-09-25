#include "diplomat_nanobind_common.hpp"


#include "StructWithSlices.hpp"


void add_StructWithSlices_binding(nb::module_ mod) {
    nb::class_<StructWithSlices>(mod, "StructWithSlices")
        .def(nb::init<>())
        .def(nb::init<std::string_view, diplomat::span<const uint16_t>>(), "first"_a.none(),  "second"_a.none())
        .def_rw("first", &StructWithSlices::first)
        .def_rw("second", &StructWithSlices::second)
        .def("return_last", &StructWithSlices::return_last);
}

