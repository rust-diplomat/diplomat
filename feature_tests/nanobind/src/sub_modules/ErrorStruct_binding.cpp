#include "diplomat_nanobind_common.hpp"


#include "ErrorStruct.hpp"


void add_ErrorStruct_binding(nb::module_ mod) {
    nb::class_<ErrorStruct>(mod, "ErrorStruct")
        .def(nb::init<>())
        .def(nb::init<int32_t, int32_t>(), "i"_a.none(),  "j"_a.none())
        .def_rw("i", &ErrorStruct::i)
        .def_rw("j", &ErrorStruct::j);
}

