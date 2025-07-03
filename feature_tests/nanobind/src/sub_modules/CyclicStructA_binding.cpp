#include "diplomat_nanobind_common.hpp"


#include "CyclicStructA.hpp"
#include "CyclicStructB.hpp"
NB_MAKE_OPAQUE(std::vector<CyclicStructA>)


void add_CyclicStructA_binding(nb::handle mod) {
    
    nb::bind_vector<std::vector<CyclicStructA>>(mod, "CyclicStructASlice"); 
    nb::class_<CyclicStructA>(mod, "CyclicStructA")
        .def(nb::init<>())
        .def(nb::init<CyclicStructB>(), "a"_a.none())
        .def_rw("a", &CyclicStructA::a)
    	.def("cyclic_out", &CyclicStructA::cyclic_out)
    	.def("double_cyclic_out", &CyclicStructA::double_cyclic_out, "cyclic_struct_a"_a)
    	.def_static("get_b", &CyclicStructA::get_b)
    	.def_prop_ro("getter_out", &CyclicStructA::getter_out)
    	.def_static("nested_slice", &CyclicStructA::nested_slice, "sl"_a);
}

