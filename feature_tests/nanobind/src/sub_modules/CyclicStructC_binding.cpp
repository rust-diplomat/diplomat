#include "diplomat_nanobind_common.hpp"


#include "CyclicStructA.hpp"
#include "CyclicStructC.hpp"


void add_CyclicStructC_binding(nb::module_ mod) {
    nb::class_<CyclicStructC>(mod, "CyclicStructC")
        .def(nb::init<>())
        .def(nb::init<CyclicStructA>(), "a"_a.none())
        .def_rw("a", &CyclicStructC::a)
        .def("cyclic_out", &CyclicStructC::cyclic_out)
        .def_static("takes_nested_parameters", &CyclicStructC::takes_nested_parameters, "c"_a);
}

