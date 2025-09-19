#include "diplomat_nanobind_common.hpp"


#include "CallbackTestingStruct.hpp"


void add_CallbackTestingStruct_binding(nb::module_ mod) {
    nb::class_<CallbackTestingStruct>(mod, "CallbackTestingStruct")
        .def(nb::init<>())
        .def(nb::init<int32_t, int32_t>(), "x"_a.none(),  "y"_a.none())
        .def_rw("x", &CallbackTestingStruct::x)
        .def_rw("y", &CallbackTestingStruct::y);
}

