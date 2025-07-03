#include "diplomat_nanobind_common.hpp"


#include "CyclicStructB.hpp"
NB_MAKE_OPAQUE(std::vector<CyclicStructB>)


void add_CyclicStructB_binding(nb::handle mod) {
    
    nb::bind_vector<std::vector<CyclicStructB>>(mod, "CyclicStructBSlice"); 
    nb::class_<CyclicStructB>(mod, "CyclicStructB")
        .def(nb::init<>())
        .def(nb::init<uint8_t>(), "field"_a.none())
        .def_rw("field", &CyclicStructB::field)
    	.def_static("get_a", &CyclicStructB::get_a)
    	.def_static("get_a_option", &CyclicStructB::get_a_option);
}

