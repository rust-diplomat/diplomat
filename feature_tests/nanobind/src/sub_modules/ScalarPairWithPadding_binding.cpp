#include "diplomat_nanobind_common.hpp"


#include "ScalarPairWithPadding.hpp"


void add_ScalarPairWithPadding_binding(nb::handle mod) {
    
    NB_MAKE_OPAQUE(std::vector<ScalarPairWithPadding>)
    nb::bind_vector<std::vector<ScalarPairWithPadding>>(mod, "ScalarPairWithPaddingSlice"); 
    nb::class_<ScalarPairWithPadding>(mod, "ScalarPairWithPadding")
        .def(nb::init<>())
        .def(nb::init<uint8_t, uint32_t>(), "first"_a.none(),  "second"_a.none())
        .def_rw("first", &ScalarPairWithPadding::first)
        .def_rw("second", &ScalarPairWithPadding::second)
    	.def("assert_value", &ScalarPairWithPadding::assert_value);
}

