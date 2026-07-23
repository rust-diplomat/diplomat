#include "diplomat_nanobind_common.hpp"


#include "OptionOpaqueChar.hpp"

namespace somelib {
void add_OptionOpaqueChar_binding(nb::module_ mod) {
    nb::class_<somelib::OptionOpaqueChar> opaque(mod, "OptionOpaqueChar");
    opaque
        .def("assert_char", &somelib::OptionOpaqueChar::assert_char, "ch"_a);
}

} 