#include "diplomat_nanobind_common.hpp"


#include "OpaqueThin.hpp"

namespace somelib {
void add_OpaqueThin_binding(nb::module_ mod) {
    nb::class_<somelib::OpaqueThin> opaque(mod, "OpaqueThin");
    opaque
        .def_prop_ro("a", &somelib::OpaqueThin::a)
        .def_prop_ro("b", &somelib::OpaqueThin::b)
        .def_prop_ro("c", &somelib::OpaqueThin::c);
}

} 