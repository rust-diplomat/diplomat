#include "diplomat_nanobind_common.hpp"


#include "ImmutableStructOfOpaque.hpp"
#include "Opaque.hpp"

namespace somelib {
void add_ImmutableStructOfOpaque_binding(nb::module_ mod) {
    nb::class_<somelib::ImmutableStructOfOpaque> st(mod, "ImmutableStructOfOpaque");
    st
        .def(nb::init<>())
        .def(nb::init<somelib::Opaque*>(), "i"_a.none())
        .def_rw("i", &somelib::ImmutableStructOfOpaque::i)
        .def("take_in", &somelib::ImmutableStructOfOpaque::take_in);
}

} 