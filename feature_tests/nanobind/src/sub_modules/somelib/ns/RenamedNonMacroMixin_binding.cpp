#include "diplomat_nanobind_common.hpp"


#include "ns/RenamedNonMacroMixin.hpp"

namespace somelib::ns {
void add_RenamedNonMacroMixin_binding(nb::module_ mod) {
    nb::class_<somelib::ns::RenamedNonMacroMixin> st(mod, "RenamedNonMacroMixin");
    st
        .def(nb::init<>());
}

} 