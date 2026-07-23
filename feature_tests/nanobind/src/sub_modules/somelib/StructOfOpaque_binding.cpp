#include "diplomat_nanobind_common.hpp"


#include "Opaque.hpp"
#include "OpaqueMut.hpp"
#include "StructOfOpaque.hpp"

namespace somelib {
void add_StructOfOpaque_binding(nb::module_ mod) {
    nb::class_<somelib::StructOfOpaque> st(mod, "StructOfOpaque");
    maybe_bind_default_init(st);
    st
        .def(nb::init<somelib::OpaqueRef, somelib::OpaqueMutRefMut>(), "i"_a.none(),  "j"_a.none())
        .def_rw("i", &somelib::StructOfOpaque::i)
        .def_rw("j", &somelib::StructOfOpaque::j)
        .def("take_in", &somelib::StructOfOpaque::take_in, "other"_a);
}

} 