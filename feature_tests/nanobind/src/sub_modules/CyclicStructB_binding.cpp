#include "diplomat_nanobind_common.hpp"


#include "CyclicStructB.hpp"


void add_CyclicStructB_binding(nb::handle mod) {
    nb::class_<CyclicStructB>(mod, "CyclicStructB")
        .def(nb::init<>())
        .def(nb::init<uint8_t>(), "field"_a.none())
        .def_rw("field", &CyclicStructB::field)
    	.def_static("get_a", &CyclicStructB::get_a)
    	.def_static("get_a_option", &CyclicStructB::get_a_option);
}

