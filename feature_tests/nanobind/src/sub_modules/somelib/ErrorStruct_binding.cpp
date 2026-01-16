#include "diplomat_nanobind_common.hpp"


#include "ErrorStruct.hpp"

namespace somelib {
void add_ErrorStruct_binding(nb::module_ mod) {
    nb::class_<somelib::ErrorStruct> st(mod, "ErrorStruct");
    st
        .def(nb::init<>())
        .def(nb::init<int32_t, int32_t>(), "i"_a.none(),  "j"_a.none())
        .def_rw("i", &somelib::ErrorStruct::i)
        .def_rw("j", &somelib::ErrorStruct::j);
}

} 