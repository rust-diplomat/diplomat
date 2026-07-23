#include "diplomat_nanobind_common.hpp"


#include "OptionOpaque.hpp"
#include "OptionOpaqueChar.hpp"
#include "OptionStruct.hpp"

namespace somelib {
void add_OptionStruct_binding(nb::module_ mod) {
    nb::class_<somelib::OptionStruct> st(mod, "OptionStruct");
    st
        .def_ro("a", &somelib::OptionStruct::a)
        .def_ro("b", &somelib::OptionStruct::b)
        .def_ro("c", &somelib::OptionStruct::c)
        .def_ro("d", &somelib::OptionStruct::d);
}

} 