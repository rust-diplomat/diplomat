#include "diplomat_nanobind_common.hpp"


#include "CyclicStructA.hpp"
#include "CyclicStructC.hpp"

namespace somelib {
void add_CyclicStructC_binding(nb::module_ mod) {
    nb::class_<somelib::CyclicStructC> st(mod, "CyclicStructC");
    st
        .def(nb::init<>())
        .def(nb::init<somelib::CyclicStructA>(), "a"_a.none())
        .def_rw("a", &somelib::CyclicStructC::a)
        .def("cyclic_out", &somelib::CyclicStructC::cyclic_out)
        .def_static("takes_nested_parameters", &somelib::CyclicStructC::takes_nested_parameters, "c"_a);
}

} 