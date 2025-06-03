#include "diplomat_nanobind_common.hpp"


#include "CyclicStructA.hpp"
#include "CyclicStructB.hpp"


void add_CyclicStructA_binding(nb::handle mod) {
    nb::class_<CyclicStructA>(mod, "CyclicStructA")
        .def(nb::init<>())
        .def(nb::init<CyclicStructB>(), "a"_a.none())
        .def_rw("a", &CyclicStructA::a)
    	.def("cyclic_out", &CyclicStructA::cyclic_out)
    	.def("double_cyclic_out", &CyclicStructA::double_cyclic_out, "cyclic_struct_a"_a)
    	.def_static("get_b", &CyclicStructA::get_b)
    	.def_prop_ro("getter_out", &CyclicStructA::getter_out);
}

